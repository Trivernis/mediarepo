use crate::client_api::error::ApiResult;
use crate::client_api::IPCApi;
use crate::types::jobs::{JobType, RunJobRequest};
use bromine::context::{Context, PoolGuard, PooledContext};
use std::time::Duration;

#[derive(Clone)]
pub struct JobApi {
    ctx: PooledContext,
}

impl IPCApi for JobApi {
    fn namespace() -> &'static str {
        "jobs"
    }

    fn ctx(&self) -> PoolGuard<Context> {
        self.ctx.acquire()
    }
}

impl JobApi {
    pub fn new(ctx: PooledContext) -> Self {
        Self { ctx }
    }

    /// Runs a job of the given type and returns when it has finished
    #[tracing::instrument(level = "debug", skip(self))]
    pub async fn run_job(&self, job_type: JobType, sync: bool) -> ApiResult<()> {
        self.emit("run_job", RunJobRequest { job_type, sync })
            .await_reply()
            .with_timeout(Duration::from_secs(3600))
            .await?;

        Ok(())
    }

    /// Checks if a particular job is already running
    #[tracing::instrument(level = "debug", skip(self))]
    pub async fn is_job_running(&self, job_type: JobType) -> ApiResult<bool> {
        self.emit_and_get("is_job_running", job_type, None).await
    }
}
