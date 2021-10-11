use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct AddFileRequest {
    pub path: String,
}

#[derive(Serialize, Deserialize)]
pub enum ReadFileRequest {
    ID(i64),
    Hash(String),
}
