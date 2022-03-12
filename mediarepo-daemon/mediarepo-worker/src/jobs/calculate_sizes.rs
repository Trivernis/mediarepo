use crate::jobs::Job;
use crate::status_utils::SimpleProgress;
use async_trait::async_trait;
use mediarepo_core::error::{RepoError, RepoResult};
use mediarepo_core::mediarepo_api::types::repo::SizeType;
use mediarepo_core::settings::Settings;
use mediarepo_core::utils::get_folder_size;
use mediarepo_logic::dao::repo::Repo;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::fs;
use tokio::sync::broadcast::{self, Sender};
use tokio::sync::RwLock;

pub struct CalculateSizesState {
    pub progress: SimpleProgress,
    pub sizes_channel: Sender<(SizeType, u64)>,
}

#[derive(Clone)]
pub struct CalculateSizesJob {
    repo_path: PathBuf,
    settings: Settings,
    state: Arc<RwLock<CalculateSizesState>>,
}

impl CalculateSizesJob {
    pub fn new(repo_path: PathBuf, settings: Settings) -> Self {
        let (tx, _) = broadcast::channel(4);
        Self {
            repo_path,
            settings,
            state: Arc::new(RwLock::new(CalculateSizesState {
                sizes_channel: tx,
                progress: SimpleProgress::new(4),
            })),
        }
    }
}

#[async_trait]
impl Job for CalculateSizesJob {
    type JobState = CalculateSizesState;

    fn state(&self) -> Arc<RwLock<Self::JobState>> {
        self.state.clone()
    }

    #[tracing::instrument(level = "debug", skip_all)]
    async fn run(&self, repo: Arc<Repo>) -> RepoResult<()> {
        let size_types = vec![
            SizeType::Total,
            SizeType::FileFolder,
            SizeType::ThumbFolder,
            SizeType::DatabaseFile,
        ];
        for size_type in size_types {
            let size = calculate_size(&size_type, &repo, &self.repo_path, &self.settings).await?;
            let mut state = self.state.write().await;
            state
                .sizes_channel
                .send((size_type, size))
                .map_err(|_| RepoError::from("failed to broadcast new size"))?;
            state.progress.tick();
        }

        Ok(())
    }
}

async fn calculate_size(
    size_type: &SizeType,
    repo: &Repo,
    repo_path: &PathBuf,
    settings: &Settings,
) -> RepoResult<u64> {
    let size = match &size_type {
        SizeType::Total => get_folder_size(repo_path.clone()).await?,
        SizeType::FileFolder => repo.get_main_store_size().await?,
        SizeType::ThumbFolder => repo.get_thumb_store_size().await?,
        SizeType::DatabaseFile => {
            let db_path = settings.paths.db_file_path(repo_path);

            let database_metadata = fs::metadata(db_path).await?;
            database_metadata.len()
        }
    };

    Ok(size)
}
