use crate::error::RepoResult;
use crate::utils::get_folder_size;
use multibase::Base;
use multihash::{Code, MultihashDigest};
use std::path::PathBuf;
use tokio::fs;
use tokio::fs::{File, OpenOptions};
use tokio::io::{AsyncRead, AsyncReadExt, BufReader};

static STORE_BASE: Base = Base::Base32Lower;

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
    ) -> RepoResult<String> {
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf).await?;
        let hash: Vec<u8> = Code::Sha2_256.digest(&buf).to_bytes();
        let hash: String = multibase::encode(STORE_BASE, &hash);
        let folder_path = self.hash_to_folder_path(&hash);

        if !folder_path.exists() {
            fs::create_dir(folder_path).await?;
        }
        let mut file_path = self.hash_to_file_path(&hash);
        if let Some(extension) = extension {
            file_path.set_extension(extension);
        }
        fs::write(file_path, buf).await?;

        Ok(hash)
    }

    /// Returns the file extension and a reader for the file by hash
    pub async fn get_file(
        &self,
        mut hash: String,
    ) -> RepoResult<(Option<String>, BufReader<File>)> {
        let (base, data) = multibase::decode(&hash)?;
        if base != STORE_BASE {
            hash = multibase::encode(STORE_BASE, data);
        }
        let file_path = self.hash_to_file_path(&hash);
        let extension = file_path
            .extension()
            .and_then(|s| s.to_str())
            .map(|s| s.to_string());
        let file = OpenOptions::new().read(true).open(file_path).await?;
        let reader = BufReader::new(file);

        Ok((extension, reader))
    }

    /// Scans the size of the folder
    #[inline]
    pub async fn get_size(&self) -> RepoResult<u64> {
        get_folder_size(self.path.to_owned()).await
    }

    fn hash_to_file_path(&self, hash: &str) -> PathBuf {
        let mut path = self.hash_to_folder_path(hash);
        path.push(hash);

        path
    }

    fn hash_to_folder_path(&self, hash: &str) -> PathBuf {
        assert!(hash.len() >= 2);
        let mut path = self.path.clone();
        path.push(&hash[hash.len() - 3..hash.len() - 1]);

        path
    }
}
