use std::collections::VecDeque;

use serde::{Deserialize, Serialize};

use crate::job::{JobError, JobState, JobTrait, WorkerCtx};

pub const LIBRARY_SCAN_JOB_NAME: &str = "library_scan";

#[derive(Serialize, Deserialize)]
pub struct LibraryScanTask {
	pub series_path: String,
	pub files_to_process: VecDeque<String>,
}

#[derive(Serialize, Deserialize)]
pub struct LibraryScanJob {
	pub library_path: String,
}

#[async_trait::async_trait]
impl JobTrait for LibraryScanJob {
	type Task = LibraryScanTask;

	fn name(&self) -> &'static str {
		LIBRARY_SCAN_JOB_NAME
	}

	fn description(&self) -> Option<Box<&str>> {
		Some(Box::new(self.library_path.as_str()))
	}

	async fn run(
		&mut self,
		ctx: WorkerCtx,
		state: &mut JobState<Self>,
	) -> Result<(), JobError> {
		unimplemented!()
	}
}
