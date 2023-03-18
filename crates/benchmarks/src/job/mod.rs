mod job_manager;
mod worker;

use std::collections::VecDeque;

pub use job_manager::{JobManager, JobManagerShutdownSignal};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
pub use worker::{Worker, WorkerCtx};

#[derive(Clone, Debug)]
pub enum JobError {
	Paused(Vec<u8>),
	Cancelled,
	Unknown(String),
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub enum JobStatus {
	Running,
	Paused,
	Completed,
	Cancelled,
	Failed,
	#[default]
	Queued,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct JobDetail {
	/// The ID of the job
	pub id: String,
	/// The name of job, e.g. LibraryScanJob
	pub name: String,
	/// The extra details of the job, e.g. "/Users/oromei/Documents/Stump/MainLibrary"
	pub description: Option<String>,
	/// The status of the job. e.g. Running, Paused, Completed, Cancelled, Failed, Queued
	pub status: JobStatus,
	/// The state of the job. This is primarily stored in order to support pausing/resuming
	/// jobs, as it will hold the state of the job at the time it was paused.
	pub state: Option<Vec<u8>>,
	// A JSON blob of extra metadata
	pub extra_metadata: Option<serde_json::Value>,
	/// The total number of tasks
	pub task_count: Option<i32>,
	/// The total number of tasks completed (i.e. without error/failure)
	pub completed_task_count: Option<i32>,
	/// The time (in milliseconds) to complete the job
	pub ms_elapsed: Option<u64>,
	/// The datetime stamp of when the job completed
	pub completed_at: Option<String>,
}

impl JobDetail {
	pub fn new(id: String, name: String, description: Option<String>) -> Self {
		Self {
			id,
			name,
			description,
			status: JobStatus::Queued,
			state: None,
			extra_metadata: None,
			task_count: None,
			completed_task_count: None,
			ms_elapsed: None,
			completed_at: None,
		}
	}
}

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

#[derive(Serialize, Deserialize)]
pub struct JobState<J: JobTrait> {
	pub tasks: VecDeque<J::Task>,
	pub current_task: usize,
	pub ms_elapsed: u64,
}

impl<J: JobTrait> Default for JobState<J> {
	fn default() -> Self {
		Self {
			tasks: VecDeque::new(),
			current_task: 0,
			ms_elapsed: 0,
		}
	}
}

#[async_trait::async_trait]
pub trait JobTrait: Send + Sync + Sized {
	// state will be serialized and stored in DB on pause and/or any completed state
	// (including failure completion). So it needs to be serializable and deserializable.
	type Task: Serialize + DeserializeOwned + Send + Sync;

	fn name(&self) -> &'static str;
	fn description(&self) -> Option<Box<&str>>;
	// fn detail_mut(&mut self) -> &mut Option<JobDetail>;
	async fn run(
		&mut self,
		ctx: WorkerCtx,
		state: &mut JobState<Self>,
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
					let duration = start.elapsed();
					running = false;
					unimplemented!()
				}
				shutdown_result = &mut shutdown_rx_fut => {
					if let Ok(signal_type) = shutdown_result {
						match signal_type {
							JobManagerShutdownSignal::Worker(id) if &id == ctx.job_id()  => {
								return Err(JobError::Paused(serde_json::to_vec(&self.state).expect("Failed to serialize job state")));
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
		// if matches JobError::Paused, extract vector
		// else use self.state and turn into vec<u8>

		let resolved_state = if let Err(e) = result {
			match e {
				JobError::Paused(state) => state,
				_ => serde_json::to_vec(&self.state)
					.expect("Failed to serialize job state"),
			}
		} else {
			serde_json::to_vec(&self.state).expect("Failed to serialize job state")
		};

		// TODO: persist state to DB

		unimplemented!()
	}
}
