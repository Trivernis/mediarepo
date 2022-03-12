use crate::jobs::{EmptyStatus, Job};
use async_trait::async_trait;
use mediarepo_core::error::RepoResult;
use mediarepo_logic::dao::job::JobDao;
use mediarepo_logic::dao::repo::Repo;
use mediarepo_logic::dao::DaoProvider;
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Default, Clone)]
pub struct VacuumJob;

#[async_trait]
impl Job for VacuumJob {
    type JobStatus = ();

    fn status(&self) -> Arc<RwLock<Self::JobStatus>> {
        EmptyStatus::default()
    }

    #[tracing::instrument(level = "debug", skip_all)]
    async fn run(&self, repo: Arc<Repo>) -> RepoResult<()> {
        repo.job().vacuum().await?;

        Ok(())
    }

    #[tracing::instrument(level = "debug", skip_all)]
    async fn save_status(&self, _: JobDao) -> RepoResult<()> {
        Ok(())
    }
}
