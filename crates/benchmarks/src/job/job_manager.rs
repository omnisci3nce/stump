use std::{
	collections::{HashMap, VecDeque},
	sync::Arc,
};
use tokio::sync::{broadcast, Mutex, RwLock};

use super::{worker::Worker, JobExecutorTrait};

#[derive(Debug, Clone)]
pub enum JobManagerShutdownSignal {
	All,
	Worker(String),
}

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

	pub async fn pause_job(self: Arc<Self>, job_id: String) {
		// TODO: write vs read here? I guess it depends on how I wind up *actually* pause a job.
		let workers = self.workers.read().await;
		if workers.get(&job_id).is_some() {
			self.shutdown_tx
				.send(JobManagerShutdownSignal::Worker(job_id.clone()))
				.expect("Failed to send shutdown signal to worker!");
		} else {
			println!("Job not found: {}", job_id);
		}
		drop(workers);
	}
}
