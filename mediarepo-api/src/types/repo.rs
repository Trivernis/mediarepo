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
    pub total_size: u64,
    pub image_size: u64,
    pub database_size: u64,
    pub thumbnail_size: u64,
}