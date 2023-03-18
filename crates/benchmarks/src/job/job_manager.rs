use std::{
	collections::{HashMap, VecDeque},
	sync::Arc,
};
use tokio::sync::{broadcast, Mutex, RwLock};

use super::{worker::Worker, JobExecutorTrait};

// TODO: add pause variant for a single worker.
#[derive(Debug, Clone)]
pub enum JobManagerShutdownSignal {
	All,
	Worker(String),
}

#[derive(Debug)]
pub enum JobManagerError {
	WorkerNotFound(String),
}

pub type JobManagerResult<T> = Result<T, JobManagerError>;

pub struct JobManager {
	/// Queue of jobs waiting to be run in a worker thread.
	job_queue: RwLock<VecDeque<Box<dyn JobExecutorTrait>>>,
	/// Worker threads with a running job.
	workers: RwLock<HashMap<String, Arc<Mutex<Worker>>>>,
	/// A channel to send shutdown signals to all or some workers.
	shutdown_tx: Arc<broadcast::Sender<JobManagerShutdownSignal>>,
}

impl JobManager {
	pub fn new() -> Self {
		let (shutdown_tx, _) = broadcast::channel(1024);

		Self {
			job_queue: RwLock::new(VecDeque::new()),
			workers: RwLock::new(HashMap::new()),
			shutdown_tx: Arc::new(shutdown_tx),
		}
	}

	pub fn arced(self) -> Arc<Self> {
		Arc::new(self)
	}

	pub fn get_shutdown_tx(&self) -> Arc<broadcast::Sender<JobManagerShutdownSignal>> {
		Arc::clone(&self.shutdown_tx)
	}

	pub async fn cancel_job(self: Arc<Self>, job_id: String) -> JobManagerResult<()> {
		let mut workers = self.workers.write().await;
		if workers.get(&job_id).is_some() {
			self.shutdown_tx
				.send(JobManagerShutdownSignal::Worker(job_id.clone()))
				.expect("Failed to send shutdown signal to worker!");
			// TOOD: store the job state to DB as paused? This will likely just be handled in the worker...
			workers.remove(&job_id);
			drop(workers);
			return Ok(());
		}

		let mut job_queue = self.job_queue.write().await;
		let maybe_index = job_queue.iter().position(|job| {
			let job_detail = job
				.detail()
				.as_ref()
				.map(|job_detail| job_detail.id == job_id);
			job_detail.unwrap_or(false)
		});
		if let Some(job_index) = maybe_index {
			let job = job_queue
				.get_mut(job_index)
				.expect("Job not found in queue!");
			// TODO: store the job state to DB as cancelled...
			job_queue.remove(job_index);
			return Ok(());
		}

		Err(JobManagerError::WorkerNotFound(job_id))
	}

	pub async fn pause_job(self: Arc<Self>, job_id: String) -> JobManagerResult<()> {
		let workers = self.workers.read().await;
		if workers.get(&job_id).is_some() {
			self.shutdown_tx
				.send(JobManagerShutdownSignal::Worker(job_id.clone()))
				.expect("Failed to send shutdown signal to worker!");
			// // TOOD: store the job state to DB as paused? This will likely just be handled in the worker...
			// workers.remove(&job_id);
			drop(workers);
			Ok(())
		} else {
			Err(JobManagerError::WorkerNotFound(job_id))
		}
	}

	pub async fn enqueue_job(self: Arc<Self>, mut job: Box<dyn JobExecutorTrait>) {
		let mut workers = self.workers.write().await;

		if workers.is_empty() {
			println!("Starting job: {}", job.name());

			let job_detail = job
				.detail_mut()
				.take()
				.expect("Job initialized without state!");

			let job_id = job_detail.id.clone();
			let worker = Worker::new(job, job_detail);
			let worker_mtx = Arc::new(Mutex::new(worker));

			let spawn_result =
				Worker::spawn(job_id.clone(), Arc::clone(&self), Arc::clone(&worker_mtx))
					.await;
			if let Err(err) = spawn_result {
				println!("Error spawning worker: {:?}", err);
			} else {
				workers.insert(job_id, worker_mtx);
			}
		} else {
			self.job_queue.write().await.push_back(job);
		}

		drop(workers);
	}

	/// Dequeues a job from the queue
	pub async fn dequeue_job(self: Arc<Self>, job_id: String) -> JobManagerResult<()> {
		let remove_result = self.workers.write().await.remove(&job_id);

		if remove_result.is_none() {
			if let Some(index) = self.get_queued_job_index(&job_id).await {
				return self.dequeue_pending_job(index).await;
			}

			return Err(JobManagerError::WorkerNotFound(job_id));
		}

		let next_job = self.job_queue.write().await.pop_front();
		if let Some(job) = next_job {
			// TODO: dispatch to event handler
		}

		Ok(())
	}

	async fn dequeue_pending_job(self: Arc<Self>, index: usize) -> JobManagerResult<()> {
		self.job_queue.write().await.remove(index);
		Ok(())
	}

	async fn get_queued_job_index(&self, job_id: &str) -> Option<usize> {
		let job_queue = self.job_queue.read().await;
		job_queue.iter().position(|job| {
			let job_detail = job
				.detail()
				.as_ref()
				.map(|job_detail| job_detail.id == job_id);
			job_detail.unwrap_or(false)
		})
	}
}
