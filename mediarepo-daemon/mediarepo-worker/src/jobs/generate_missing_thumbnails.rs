use crate::jobs::Job;
use crate::status_utils::SimpleProgress;
use async_trait::async_trait;
use mediarepo_core::error::RepoResult;
use mediarepo_core::thumbnailer::ThumbnailSize;
use mediarepo_logic::dao::repo::Repo;
use mediarepo_logic::dao::DaoProvider;
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Clone, Default)]
pub struct GenerateMissingThumbsJob {
    state: Arc<RwLock<SimpleProgress>>,
}

#[async_trait]
impl Job for GenerateMissingThumbsJob {
    type JobStatus = SimpleProgress;
    type Result = ();

    fn status(&self) -> Arc<RwLock<Self::JobStatus>> {
        self.state.clone()
    }

    async fn run(&self, repo: Arc<Repo>) -> RepoResult<()> {
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

        Ok(())
    }
}
