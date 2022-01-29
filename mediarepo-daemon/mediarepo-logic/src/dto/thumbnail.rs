use std::path::PathBuf;

use tokio::fs;
use tokio::fs::{File, OpenOptions};
use tokio::io::BufReader;

use mediarepo_core::error::RepoResult;
use mediarepo_core::fs::thumbnail_store::Dimensions;

#[derive(Clone, Debug)]
pub struct ThumbnailDto {
    path: PathBuf,
    parent_cd: String,
    size: Dimensions,
    mime_type: String,
}

impl ThumbnailDto {
    pub fn new(path: PathBuf, parent_cd: String, size: Dimensions, mime_type: String) -> Self {
        Self {
            path,
            parent_cd,
            size,
            mime_type,
        }
    }

    pub fn parent_cd(&self) -> &String {
        &self.parent_cd
    }

    pub fn size(&self) -> &Dimensions {
        &self.size
    }

    pub fn mime_type(&self) -> &String {
        &self.mime_type
    }

    #[tracing::instrument(level = "debug")]
    pub async fn get_reader(&self) -> RepoResult<BufReader<File>> {
        let file = OpenOptions::new().read(true).open(&self.path).await?;
        Ok(BufReader::new(file))
    }

    /// Deletes the thumbnail
    #[tracing::instrument(level = "debug")]
    pub async fn delete(self) -> RepoResult<()> {
        fs::remove_file(&self.path).await?;

        Ok(())
    }
}
