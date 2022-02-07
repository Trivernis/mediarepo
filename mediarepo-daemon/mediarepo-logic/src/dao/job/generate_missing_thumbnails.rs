use crate::dao::job::JobDao;
use crate::dao::DaoProvider;
use mediarepo_core::error::RepoResult;
use mediarepo_core::futures;
use mediarepo_core::thumbnailer::ThumbnailSize;

impl JobDao {
    /// Generates thumbnails for files that are still missing some
    #[tracing::instrument(level = "debug", skip(self))]
    pub async fn generate_missing_thumbnails(&self) -> RepoResult<()> {
        let file_dao = self.file();
        let files = file_dao.all().await?;
        let mut missing_thumbnails = Vec::new();

        for file in files {
            if file_dao.thumbnails(file.encoded_cd()).await?.is_empty() {
                missing_thumbnails.push(file);
            }
        }

        futures::future::join_all(missing_thumbnails.into_iter().map(|f| async {
            file_dao
                .create_thumbnails(f, vec![ThumbnailSize::Medium])
                .await
        }))
        .await;

        Ok(())
    }
}
