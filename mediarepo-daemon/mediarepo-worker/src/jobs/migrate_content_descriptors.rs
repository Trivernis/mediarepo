use crate::jobs::{deserialize_state, serialize_state, Job};
use crate::status_utils::SimpleProgress;
use async_trait::async_trait;
use mediarepo_core::error::RepoResult;
use mediarepo_database::entities::job_state::JobType;
use mediarepo_logic::dao::job::JobDao;
use mediarepo_logic::dao::repo::Repo;
use mediarepo_logic::dao::DaoProvider;
use serde::{Deserialize, Serialize};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Clone, Default)]
pub struct MigrateCDsJob {
    progress: Arc<RwLock<SimpleProgress>>,
    migrated: Arc<AtomicBool>,
}

#[async_trait]
impl Job for MigrateCDsJob {
    type JobStatus = SimpleProgress;
    type Result = ();

    fn status(&self) -> Arc<tokio::sync::RwLock<Self::JobStatus>> {
        self.progress.clone()
    }

    async fn load_state(&self, job_dao: JobDao) -> RepoResult<()> {
        if let Some(state) = job_dao.state_for_job_type(JobType::MigrateCDs).await? {
            let state = deserialize_state::<MigrationStatus>(state)?;
            self.migrated.store(state.migrated, Ordering::SeqCst);
        }

        Ok(())
    }

    async fn run(&self, repo: Arc<Repo>) -> RepoResult<Self::Result> {
        if self.migrated.load(Ordering::SeqCst) {
            return Ok(());
        }
        let job_dao = repo.job();

        job_dao.migrate_content_descriptors().await?;
        self.migrated.store(true, Ordering::Relaxed);
        {
            let mut progress = self.progress.write().await;
            progress.set_total(100);
        }
        Ok(())
    }

    async fn save_state(&self, job_dao: JobDao) -> RepoResult<()> {
        if self.migrated.load(Ordering::Relaxed) {
            let state = serialize_state(JobType::MigrateCDs, &MigrationStatus { migrated: true })?;
            job_dao.upsert_state(state).await?;
        }
        Ok(())
    }
}

#[derive(Serialize, Deserialize)]
struct MigrationStatus {
    pub migrated: bool,
}
