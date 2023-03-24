use std::sync::Arc;

use prisma_client_rust::QueryError;

use crate::{prelude::Ctx, prisma::job};

pub(crate) async fn persist_job_state(
	core_ctx: Arc<Ctx>,
	state: Vec<u8>,
	job_id: String,
) -> Result<(), QueryError> {
	let client = core_ctx.get_db();

	client
		.job()
		.update(job::id::equals(job_id), vec![job::state::set(Some(state))])
		.exec()
		.await?;

	Ok(())
}
