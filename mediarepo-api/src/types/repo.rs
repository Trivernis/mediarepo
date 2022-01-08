use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct FrontendState {
    pub state: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct RepositoryMetadata {
    pub version: String,
    pub file_count: u64,
    pub tag_count: u64,
    pub namespace_count: u64,
    pub mapping_count: u64,
    pub hash_count: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SizeMetadata {
    pub size_type: SizeType,
    pub size: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum SizeType {
    Total,
    FileFolder,
    ThumbFolder,
    DatabaseFile,
}
