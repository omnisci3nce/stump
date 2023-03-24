use super::{
	utils::persist_job_state, JobDetail, JobError, JobManagerShutdownSignal, JobState,
	JobTrait, WorkerCtx,
};

#[async_trait::async_trait]
pub trait JobExecutorTrait: Send + Sync {
	fn name(&self) -> &'static str;
	fn description(&self) -> Option<Box<&str>>;
	fn detail(&self) -> &Option<JobDetail>;
	fn detail_mut(&mut self) -> &mut Option<JobDetail>;
	async fn execute(&mut self, ctx: WorkerCtx) -> Result<(), JobError>;
	async fn finish(
		&self,
		result: Result<(), JobError>,
		ctx: WorkerCtx,
	) -> Result<(), JobError>;
}

pub struct Job<InnerJob: JobTrait> {
	detail: Option<JobDetail>,
	state: JobState<InnerJob>,
	inner_job: InnerJob,
}

impl<InnerJob: JobTrait> Job<InnerJob> {
	pub fn new(inner_job: InnerJob) -> Box<Self> {
		Box::new(Self {
			detail: Some(JobDetail::new(
				"test".to_string(),
				inner_job.name().to_string(),
				inner_job.description().map(|s| s.to_string()),
			)),
			state: JobState::<InnerJob>::default(),
			inner_job,
		})
	}
}

#[async_trait::async_trait]
impl<InnerJob: JobTrait> JobExecutorTrait for Job<InnerJob> {
	fn detail(&self) -> &Option<JobDetail> {
		&self.detail
	}

	fn detail_mut(&mut self) -> &mut Option<JobDetail> {
		&mut self.detail
	}

	fn name(&self) -> &'static str {
		self.inner_job.name()
	}

	fn description(&self) -> Option<Box<&str>> {
		self.inner_job.description()
	}

	async fn execute(&mut self, ctx: WorkerCtx) -> Result<(), JobError> {
		let mut shutdown_rx = ctx.shutdown_rx();
		let shutdown_rx_fut = shutdown_rx.recv();
		tokio::pin!(shutdown_rx_fut);

		// let job_fn = self.inner_job.run(ctx.clone(), &mut self.state);
		// tokio::pin!(job_fn);

		let start = std::time::Instant::now();

		let mut running = true;
		while running {
			tokio::select! {
				job_result = self.inner_job.run(ctx.clone(), &mut self.state) => {
					// self.state.duration = duration;
					// self.state.status = JobStatus::Finished;
					let duration = start.elapsed();
					running = false;

					if let Err(err) = job_result {
						unimplemented!()
					} else {
						unimplemented!()
					}
				}
				// FIXME: I think this might be wrong, in that even if the signal is
				// meant to pause, it will kill the future above? Unless I pin it maybe?
				shutdown_result = &mut shutdown_rx_fut => {
					let duration = start.elapsed();
					// self.state.duration = duration;
					if let Ok(signal_type) = shutdown_result {
						match signal_type {
							JobManagerShutdownSignal::Worker(id) if &id == ctx.job_id()  => {
								let state = serde_json::to_vec(&self.state).expect("Failed to serialize job state");
								return Err(JobError::Paused(state));
							}
							JobManagerShutdownSignal::All => {
								return Err(JobError::Cancelled);
							}
							_ => {}
						}
					} else if let Err(err) = shutdown_result {
						return Err(JobError::Unknown(err.to_string()));
					}
				}
			}
		}

		unimplemented!()
	}

	async fn finish(
		&self,
		result: Result<(), JobError>,
		ctx: WorkerCtx,
	) -> Result<(), JobError> {
		let resolved_state = if let Err(e) = result {
			match e {
				JobError::Paused(state) => state,
				_ => serde_json::to_vec(&self.state)
					.expect("Failed to serialize job state"),
			}
		} else {
			serde_json::to_vec(&self.state).expect("Failed to serialize job state")
		};

		// TODO: pass JobStatus to persist utility
		// TODO: error handling...
		let _ =
			persist_job_state(ctx.core_ctx.clone(), resolved_state, ctx.job_id.clone())
				.await;

		Ok(())
	}
}
