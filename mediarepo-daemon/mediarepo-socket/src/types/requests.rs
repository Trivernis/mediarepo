use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct AddFileRequest {
    pub path: String,
}

#[derive(Serialize, Deserialize)]
pub enum FileIdentifier {
    ID(i64),
    Hash(String),
}

pub type ReadFileRequest = FileIdentifier;
pub type GetFileThumbnailsRequest = FileIdentifier;
pub type GetFileTagsRequest = FileIdentifier;

#[derive(Serialize, Deserialize)]
pub struct FindFilesByTagsRequest {
    pub tags: Vec<String>,
}
