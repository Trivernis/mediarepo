use crate::error::RepoResult;
use futures::future;
use std::path::PathBuf;
use tokio::fs::{self, OpenOptions};
use tokio::io::{AsyncBufReadExt, BufReader};

/// Parses a normalized tag into its two components of namespace and tag
pub fn parse_namespace_and_tag(norm_tag: String) -> (Option<String>, String) {
    norm_tag
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
        let mut read_dir = fs::read_dir(dir).await?;

        while let Some(entry) = read_dir.next_entry().await? {
            let file_type = entry.file_type().await?;

            if file_type.is_file() {
                all_files.push(entry.path());
            } else if file_type.is_dir() {
                unchecked_dirs.push(entry.path())
            }
        }
    }
    let futures = all_files.into_iter().map(|f| read_file_size(f));
    let results = future::join_all(futures).await;

    let size = results
        .into_iter()
        .filter_map(|r| r.ok())
        .fold(0u64, |acc, val| acc + val);

    Ok(size)
}

async fn read_file_size(path: PathBuf) -> RepoResult<u64> {
    let metadata = fs::metadata(path).await?;

    Ok(metadata.len())
}
