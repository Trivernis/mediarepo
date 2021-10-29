use crate::types::identifier::FileIdentifier;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AddFileRequest {
    pub path: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ReadFileRequest {
    pub id: FileIdentifier,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GetFileThumbnailsRequest {
    pub id: FileIdentifier,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GetFileTagsRequest {
    pub id: FileIdentifier,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FindFilesByTagsRequest {
    pub tags: Vec<TagQuery>,
    pub sort_expression: Vec<SortKey>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TagQuery {
    pub negate: bool,
    pub name: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum SortKey {
    Namespace(SortNamespace),
    FileName(SortDirection),
    FileSize(SortDirection),
    FileImportedTime(SortDirection),
    FileCreatedTime(SortDirection),
    FileChangeTime(SortDirection),
    FileType(SortDirection),
    NumTags(SortDirection),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SortNamespace {
    pub tag: String,
    pub direction: SortDirection,
}

#[derive(Clone, Debug, Serialize, Deserialize, Ord, PartialOrd, PartialEq)]
pub enum SortDirection {
    Ascending,
    Descending,
}

impl Eq for SortDirection {}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FileMetadataResponse {
    pub id: i64,
    pub name: Option<String>,
    pub comment: Option<String>,
    pub hash: String,
    pub file_type: u32,
    pub mime_type: Option<String>,
    pub creation_time: NaiveDateTime,
    pub change_time: NaiveDateTime,
    pub import_time: NaiveDateTime,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ThumbnailMetadataResponse {
    pub id: i64,
    pub hash: String,
    pub height: i32,
    pub width: i32,
    pub mime_type: Option<String>,
}
