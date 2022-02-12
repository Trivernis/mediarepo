use std::path::PathBuf;

use futures::future;
use tokio::fs::{self, OpenOptions};
use tokio::io::{AsyncBufReadExt, BufReader};

use crate::error::RepoResult;

/// Parses a normalized tag into its two components of namespace and tag
pub fn parse_namespace_and_tag(norm_tag: String) -> (Option<String>, String) {
    norm_tag
        .to_lowercase()
        .split_once(':')
        .map(|(n, t)| (Some(n.trim().to_string()), t.trim().to_string()))
        .unwrap_or((None, norm_tag.trim().to_string()))
}

/// Parses all tags from a file
pub async fn parse_tags_file(path: PathBuf) -> RepoResult<Vec<(Option<String>, String)>> {
    let file = OpenOptions::new().read(true).open(path).await?;
    let mut lines = BufReader::new(file).lines();
    let mut tags = Vec::new();

    while let Some(line) = lines.next_line().await? {
        tags.push(parse_namespace_and_tag(line));
    }

    Ok(tags)
}

/// Iteratively scans the size of a folder
#[tracing::instrument(level = "debug")]
pub async fn get_folder_size(path: PathBuf) -> RepoResult<u64> {
    let mut unchecked_dirs = vec![path];
    let mut all_files = Vec::new();

    while !unchecked_dirs.is_empty() {
        let dir = unchecked_dirs.remove(0);

        match get_files_and_dirs_for_dir(&dir).await {
            Ok((mut files, mut dirs)) => {
                all_files.append(&mut files);
                unchecked_dirs.append(&mut dirs);
            }
            Err(e) => {
                tracing::warn!("failed to read entries for directory {:?}: {}", dir, e);
            }
        }
    }
    let futures = all_files.into_iter().map(|f| read_file_size(f));
    let results = future::join_all(futures).await;

    let size = results.into_iter().filter_map(|r| r.ok()).sum();

    Ok(size)
}

async fn get_files_and_dirs_for_dir(dir: &PathBuf) -> RepoResult<(Vec<PathBuf>, Vec<PathBuf>)> {
    let mut files = Vec::new();
    let mut directories = Vec::new();

    let mut read_dir = fs::read_dir(dir).await?;

    while let Some(entry) = read_dir.next_entry().await? {
        let file_type = entry.file_type().await?;

        if file_type.is_file() {
            files.push(entry.path());
        } else if file_type.is_dir() {
            directories.push(entry.path())
        }
    }

    Ok((files, directories))
}

async fn read_file_size(path: PathBuf) -> RepoResult<u64> {
    let metadata = fs::metadata(path).await?;

    Ok(metadata.len())
}
