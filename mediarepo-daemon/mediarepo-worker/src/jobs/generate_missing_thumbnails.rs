use crate::jobs::{deserialize_state, serialize_state, Job};
use crate::status_utils::SimpleProgress;
use async_trait::async_trait;
use mediarepo_core::error::RepoResult;
use mediarepo_core::thumbnailer::ThumbnailSize;
use mediarepo_database::entities::job_state::JobType;
use mediarepo_logic::dao::job::JobDao;
use mediarepo_logic::dao::repo::Repo;
use mediarepo_logic::dao::DaoProvider;
use serde::{Deserialize, Serialize};
use std::mem;
use std::sync::Arc;
use std::time::{Duration, SystemTime};
use tokio::sync::RwLock;

#[derive(Clone, Default)]
pub struct GenerateMissingThumbsJob {
    state: Arc<RwLock<SimpleProgress>>,
    inner_state: Arc<RwLock<GenerateThumbsState>>,
}

#[async_trait]
impl Job for GenerateMissingThumbsJob {
    type JobStatus = SimpleProgress;
    type Result = ();

    fn status(&self) -> Arc<RwLock<Self::JobStatus>> {
        self.state.clone()
    }

    async fn load_state(&self, job_dao: JobDao) -> RepoResult<()> {
        if let Some(state) = job_dao.state_for_job_type(JobType::GenerateThumbs).await? {
            let mut inner_state = self.inner_state.write().await;
            let state = deserialize_state::<GenerateThumbsState>(state)?;
            let _ = mem::replace(&mut *inner_state, state);
        }

        Ok(())
    }

    async fn run(&self, repo: Arc<Repo>) -> RepoResult<()> {
        if !self.needs_generation(&repo).await? {
            return Ok(());
        }
        let file_dao = repo.file();
        let all_files = file_dao.all().await?;
        {
            let mut progress = self.state.write().await;
            progress.set_total(all_files.len() as u64);
        }

        for file in all_files {
            if file_dao.thumbnails(file.encoded_cd()).await?.is_empty() {
                let _ = file_dao
                    .create_thumbnails(&file, vec![ThumbnailSize::Medium])
                    .await;
            }
            {
                let mut progress = self.state.write().await;
                progress.tick();
            }
        }

        self.refresh_state(&repo).await?;

        Ok(())
    }

    async fn save_state(&self, job_dao: JobDao) -> RepoResult<()> {
        let state = self.inner_state.read().await;
        let state = serialize_state(JobType::GenerateThumbs, &*state)?;
        job_dao.upsert_state(state).await
    }
}

impl GenerateMissingThumbsJob {
    async fn needs_generation(&self, repo: &Repo) -> RepoResult<bool> {
        let repo_counts = repo.get_counts().await?;
        let file_count = repo_counts.file_count as u64;
        let state = self.inner_state.read().await;

        Ok(state.file_count != file_count
            || state.last_run.elapsed().unwrap() > Duration::from_secs(60 * 60))
    }

    async fn refresh_state(&self, repo: &Repo) -> RepoResult<()> {
        let repo_counts = repo.get_counts().await?;
        let mut state = self.inner_state.write().await;
        state.last_run = SystemTime::now();
        state.file_count = repo_counts.file_count as u64;

        Ok(())
    }
}

#[derive(Serialize, Deserialize)]
struct GenerateThumbsState {
    file_count: u64,
    last_run: SystemTime,
}

impl Default for GenerateThumbsState {
    fn default() -> Self {
        Self {
            file_count: 0,
            last_run: SystemTime::now(),
        }
    }
}
