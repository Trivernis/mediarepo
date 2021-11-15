use std::io::Result;
use std::path::PathBuf;
use tokio::fs;
use tokio::fs::OpenOptions;
use tokio::io::{AsyncWriteExt, BufWriter};

#[derive(Clone, Debug)]
pub struct ThumbnailStore {
    path: PathBuf,
}

#[derive(Clone, Debug)]
pub struct Dimensions {
    pub height: u32,
    pub width: u32,
}

impl ThumbnailStore {
    pub fn new(path: PathBuf) -> Self {
        Self { path }
    }

    /// Adds a thumbnail to be stored for a parent id
    /// if the thumbnail already exists it will be recreated without warning
    pub async fn add_thumbnail<S: ToString>(
        &self,
        parent_id: S,
        size: Dimensions,
        data: &[u8],
    ) -> Result<PathBuf> {
        let parent_dir = self.path.join(parent_id.to_string());
        let entry_path = parent_dir.join(format!("{}-{}", size.height, size.width));

        if !parent_dir.exists() {
            fs::create_dir_all(parent_dir).await?;
        }

        let file = OpenOptions::new()
            .write(true)
            .create(true)
            .open(&entry_path)
            .await?;
        let mut writer = BufWriter::new(file);
        writer.write_all(data).await?;
        writer.flush().await?;

        Ok(entry_path)
    }

    /// Returns all thumbnails for a parent id
    pub async fn get_thumbnails<S: ToString>(
        &self,
        parent_id: S,
    ) -> Result<Vec<(Dimensions, PathBuf)>> {
        let mut entries = Vec::new();
        let parent_dir = self.path.join(parent_id.to_string());
        if !parent_dir.exists() {
            return Ok(vec![]);
        }
        let mut dir = fs::read_dir(parent_dir).await?;

        while let Ok(Some(entry)) = dir.next_entry().await {
            let file_name = entry.file_name();
            let name = file_name.to_string_lossy();

            let (height, width) = name
                .split_once("-")
                .and_then(|(height, width)| {
                    Some((height.parse::<u32>().ok()?, width.parse::<u32>().ok()?))
                })
                .unwrap_or((255, 255));
            entries.push((Dimensions { height, width }, entry.path()))
        }

        Ok(entries)
    }
}
