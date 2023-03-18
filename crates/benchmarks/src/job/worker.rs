use std::sync::Arc;

use tokio::sync::{broadcast, Mutex};

use super::{
	job_manager::{JobManager, JobManagerShutdownSignal},
	JobDetail, JobError, JobExecutorTrait,
};

#[derive(Debug, Clone)]
pub struct WorkerCtx {
	job_id: String,
	shutdown_tx: Arc<broadcast::Sender<JobManagerShutdownSignal>>,
}

impl WorkerCtx {
	pub fn shutdown_rx(&self) -> broadcast::Receiver<JobManagerShutdownSignal> {
		self.shutdown_tx.subscribe()
	}

	pub fn job_id(&self) -> &str {
		&self.job_id
	}
}

pub struct Worker {
	job: Option<Box<dyn JobExecutorTrait>>,
	job_detail: JobDetail,
}

impl Worker {
	pub fn new(job: Box<dyn JobExecutorTrait>, initial_detail: JobDetail) -> Self {
		Self {
			job: Some(job),
			job_detail: initial_detail,
		}
	}

	pub fn job_detail(&self) -> JobDetail {
		self.job_detail.clone()
	}

	pub async fn spawn(
		job_id: String,
		job_manager: Arc<JobManager>,
		worker_mtx: Arc<Mutex<Self>>,
	) -> Result<(), JobError> {
		let worker_ctx = WorkerCtx {
			job_id: job_id.clone(),
			shutdown_tx: job_manager.get_shutdown_tx(),
		};

		let mut job = worker_mtx
			.lock()
			.await
			.job
			.take()
			.expect("Failed to take job from worker");

		tokio::spawn(async move {
			let result = job.execute(worker_ctx.clone()).await;
			job.finish(result, worker_ctx)
				.await
				.expect("Failed to finish job!");
			job_manager
				.dequeue_job(job_id)
				.await
				.expect("Failed to dequeue job!")
		});

		Ok(())
	}
}
