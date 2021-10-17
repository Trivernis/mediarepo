use crate::error::RepoResult;
use std::path::PathBuf;
use tokio::fs::OpenOptions;
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
