use std::sync::Arc;

use tokio::sync::{broadcast, Mutex};

use crate::{event::CoreEvent, prelude::Ctx};

use super::{
	job_manager::{JobManager, JobManagerShutdownSignal},
	JobDetail, JobError, JobExecutorTrait, JobUpdate,
};

#[derive(Clone)]
pub struct WorkerCtx {
	pub job_id: String,
	pub shutdown_tx: Arc<broadcast::Sender<JobManagerShutdownSignal>>,
	pub core_ctx: Arc<Ctx>,
}

impl WorkerCtx {
	pub fn shutdown_rx(&self) -> broadcast::Receiver<JobManagerShutdownSignal> {
		self.shutdown_tx.subscribe()
	}

	pub fn job_id(&self) -> &str {
		&self.job_id
	}

	pub fn emit_progress(&self, progress: JobUpdate) {
		self.core_ctx.emit_event(CoreEvent::JobProgress(progress))
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
		worker_ctx: WorkerCtx,
		job_manager: Arc<JobManager>,
		worker_mtx: Arc<Mutex<Self>>,
	) -> Result<(), JobError> {
		let job_id = worker_ctx.job_id.clone();
		let mut job = worker_mtx
			.lock()
			.await
			.job
			.take()
			.ok_or(JobError::SpawnFailed)?;

		tokio::spawn(async move {
			let result = job.execute(worker_ctx.clone()).await;

			// TODO: emit errors without expect...
			job.finish(result, worker_ctx)
				.await
				.expect("Failed to finish job!");

			job_manager
				.dequeue_job(job_id)
				.await
				.expect("Failed to dequeue job!");
		});

		Ok(())
	}
}
