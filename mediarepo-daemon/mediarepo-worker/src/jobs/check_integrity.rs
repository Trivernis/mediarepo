use crate::jobs::Job;
use crate::status_utils::SimpleProgress;
use async_trait::async_trait;
use mediarepo_core::error::RepoResult;
use mediarepo_logic::dao::repo::Repo;
use mediarepo_logic::dao::DaoProvider;
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Clone, Default)]
pub struct CheckIntegrityJob {
    progress: Arc<RwLock<SimpleProgress>>,
}

#[async_trait]
impl Job for CheckIntegrityJob {
    type JobStatus = SimpleProgress;
    type Result = ();

    fn status(&self) -> Arc<RwLock<Self::JobStatus>> {
        self.progress.clone()
    }

    async fn run(&self, repo: Arc<Repo>) -> RepoResult<Self::Result> {
        repo.job().check_integrity().await?;
        {
            let mut progress = self.progress.write().await;
            progress.set_total(100);
        }
        Ok(())
    }
}
