use mediarepo_core::error::RepoResult;
use mediarepo_core::fs::thumbnail_store::Dimensions;
use std::path::PathBuf;
use tokio::fs::{File, OpenOptions};
use tokio::io::BufReader;

pub struct Thumbnail {
    pub file_hash: String,
    pub path: PathBuf,
    pub size: Dimensions,
    pub mime_type: String,
}

impl Thumbnail {
    /// Returns the reader of the thumbnail file
    #[tracing::instrument(level = "debug", skip(self))]
    pub async fn get_reader(&self) -> RepoResult<BufReader<File>> {
        let file = OpenOptions::new().read(true).open(&self.path).await?;
        Ok(BufReader::new(file))
    }
}
