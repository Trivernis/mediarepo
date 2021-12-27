use bromine::prelude::*;
use tokio::time::Duration;

use crate::client_api::error::ApiResult;
use crate::client_api::IPCApi;
use crate::types::repo::{FrontendState, RepositoryMetadata};

#[derive(Clone)]
pub struct RepoApi {
    ctx: PooledContext,
}

impl IPCApi for RepoApi {
    fn namespace() -> &'static str {
        "repo"
    }

    fn ctx(&self) -> PoolGuard<Context> {
        self.ctx.acquire()
    }
}

impl RepoApi {
    pub fn new(ctx: PooledContext) -> Self {
        Self { ctx }
    }

    /// Returns metadata about the repository
    #[tracing::instrument(level = "debug", skip(self))]
    pub async fn get_repo_metadata(&self) -> ApiResult<RepositoryMetadata> {
        let metadata = self.emit_and_get("repository_metadata", (), Some(Duration::from_secs(30))).await?;

        Ok(metadata)
    }

    /// Returns the state of the frontend that is stored in the repo
    #[tracing::instrument(level = "debug", skip(self))]
    pub async fn get_frontend_state(&self) -> ApiResult<FrontendState> {
        let state = self.emit_and_get("frontend_state", (), Some(Duration::from_secs(5))).await?;

        Ok(state)
    }

    /// Sets the state of the frontend
    #[tracing::instrument(level = "debug", skip(self))]
    pub async fn set_frontend_state(&self, state: FrontendState) -> ApiResult<()> {
        self.emit("set_frontend_state", state).await?;

        Ok(())
    }
}
