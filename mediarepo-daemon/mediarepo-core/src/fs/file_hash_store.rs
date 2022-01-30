use std::path::PathBuf;

use tokio::fs;
use tokio::fs::{File, OpenOptions};
use tokio::io::{AsyncRead, AsyncReadExt, BufReader};

use crate::content_descriptor::{create_content_descriptor, encode_content_descriptor};
use crate::error::RepoResult;
use crate::utils::get_folder_size;

#[derive(Clone, Debug)]
pub struct FileHashStore {
    path: PathBuf,
}

impl FileHashStore {
    pub fn new(path: PathBuf) -> Self {
        Self { path }
    }

    /// Adds a file that can be read to the hash store and returns the resulting hash identifier
    pub async fn add_file<R: AsyncRead + Unpin>(
        &self,
        mut reader: R,
        extension: Option<&str>,
    ) -> RepoResult<Vec<u8>> {
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf).await?;
        let descriptor = create_content_descriptor(&buf);
        let file_path = self.descriptor_to_file_path(&descriptor);
        let folder_path = file_path.parent().unwrap();

        if !folder_path.exists() {
            fs::create_dir(folder_path).await?;
        }
        let mut file_path = self.descriptor_to_file_path(&descriptor);
        if let Some(extension) = extension {
            file_path.set_extension(extension);
        }
        fs::write(file_path, buf).await?;

        Ok(descriptor)
    }

    /// Returns the file extension and a reader for the file by hash
    pub async fn get_file(
        &self,
        descriptor: &[u8],
    ) -> RepoResult<(Option<String>, BufReader<File>)> {
        let file_path = self.descriptor_to_file_path(descriptor);
        tracing::debug!("Opening file {:?}", file_path);
        let extension = file_path
            .extension()
            .and_then(|s| s.to_str())
            .map(|s| s.to_string());
        let file = OpenOptions::new().read(true).open(file_path).await?;
        let reader = BufReader::new(file);

        Ok((extension, reader))
    }

    /// Renames a file
    pub async fn rename_file(
        &self,
        src_descriptor: &[u8],
        dst_descriptor: &[u8],
    ) -> RepoResult<()> {
        let src_path = self.descriptor_to_file_path(src_descriptor);
        if !src_path.exists() {
            tracing::warn!("file {:?} doesn't exist", src_path);
            return Ok(());
        }
        let dst_path = self.descriptor_to_file_path(dst_descriptor);
        let dst_parent = dst_path.parent().unwrap();
        if !dst_parent.exists() {
            fs::create_dir(dst_parent).await?;
        }
        fs::rename(src_path, dst_path).await?;

        Ok(())
    }

    pub async fn delete_file(&self, descriptor: &[u8]) -> RepoResult<()> {
        let path = self.descriptor_to_file_path(descriptor);
        if !path.exists() {
            tracing::warn!("file {:?} doesn't exist", path);
            return Ok(());
        }
        fs::remove_file(path).await?;

        Ok(())
    }

    /// Scans the size of the folder
    #[inline]
    pub async fn get_size(&self) -> RepoResult<u64> {
        get_folder_size(self.path.to_owned()).await
    }

    fn descriptor_to_file_path(&self, descriptor: &[u8]) -> PathBuf {
        let descriptor_string = encode_content_descriptor(descriptor);
        let mut path = self.descriptor_string_to_folder_path(&descriptor_string);
        path.push(descriptor_string);

        path
    }

    fn descriptor_string_to_folder_path(&self, descriptor: &str) -> PathBuf {
        assert!(descriptor.len() >= 2);
        let mut path = self.path.clone();
        path.push(&descriptor[descriptor.len() - 3..descriptor.len() - 1]);

        path
    }
}
