use crate::types::identifier::FileIdentifier;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ReadFileRequest {
    pub id: FileIdentifier,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GetFileThumbnailsRequest {
    pub id: FileIdentifier,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GetFileThumbnailOfSizeRequest {
    pub id: FileIdentifier,
    pub min_size: (u32, u32),
    pub max_size: (u32, u32),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GetFileTagsRequest {
    pub id: FileIdentifier,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GetFilesTagsRequest {
    pub cds: Vec<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FileBasicDataResponse {
    pub id: i64,
    pub status: FileStatus,
    pub cd: String,
    pub mime_type: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum FileStatus {
    Imported,
    Archived,
    Deleted,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FileMetadataResponse {
    pub file_id: i64,
    pub name: Option<String>,
    pub comment: Option<String>,
    pub creation_time: NaiveDateTime,
    pub change_time: NaiveDateTime,
    pub import_time: NaiveDateTime,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FileOSMetadata {
    pub path: String,
    pub name: String,
    pub mime_type: Option<String>,
    pub creation_time: NaiveDateTime,
    pub change_time: NaiveDateTime,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ThumbnailMetadataResponse {
    pub file_hash: String,
    pub height: u32,
    pub width: u32,
    pub mime_type: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UpdateFileNameRequest {
    pub file_id: FileIdentifier,
    pub name: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AddFileRequestHeader {
    pub metadata: FileOSMetadata,
    pub tags: Vec<String>,
}
