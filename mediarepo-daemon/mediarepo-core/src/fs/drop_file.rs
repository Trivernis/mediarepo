use std::io::Result;
use std::path::{Path, PathBuf};
use tokio::fs::{File, OpenOptions};

/// A file that only exists while being owned.
/// Will automatically be deleted on Drop
pub struct DropFile {
    path: PathBuf,
}

impl DropFile {
    pub async fn new<P: AsRef<Path>>(path: P) -> Result<(File, Self)> {
        let file = OpenOptions::new()
            .write(true)
            .read(true)
            .create(true)
            .open(path.as_ref())
            .await?;
        Ok((file, Self::from_path(path)))
    }

    pub fn from_path<P: AsRef<Path>>(path: P) -> Self {
        Self {
            path: path.as_ref().to_path_buf(),
        }
    }
}

impl Drop for DropFile {
    fn drop(&mut self) {
        if let Err(e) = std::fs::remove_file(&self.path) {
            tracing::error!("failed to remove drop file '{}'", e);
        }
    }
}
