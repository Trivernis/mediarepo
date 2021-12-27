use crate::types::identifier::FileIdentifier;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TagResponse {
    pub id: i64,
    pub namespace: Option<String>,
    pub name: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NamespaceResponse {
    pub id: i64,
    pub name: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ChangeFileTagsRequest {
    pub file_id: FileIdentifier,
    pub removed_tags: Vec<i64>,
    pub added_tags: Vec<i64>,
}
