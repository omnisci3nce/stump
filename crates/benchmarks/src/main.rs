use job::{Job, JobManager, JobState};
use scanner::LibraryScanJob;

mod job;
mod scanner;

fn main() {
	let job_manager = JobManager::new().arced();

	let lib_job = LibraryScanJob {
		library_path: String::from("test"),
	};
	let job = Job::new(lib_job);

	let job_manager_cpy = job_manager.clone();
	JobManager::enqueue_job(job_manager_cpy, job);
}
