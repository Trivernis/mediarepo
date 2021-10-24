use chrono::NaiveDateTime;
use crate::types::identifier::FileIdentifier;

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
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TagQuery {
    pub negate: bool,
    pub name: String,
}

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
    pub hash: Strin,
    pub height: i32,
    pub width: i32,
    pub mime_type: Option<String>,
}