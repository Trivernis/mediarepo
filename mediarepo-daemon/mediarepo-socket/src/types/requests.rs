use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct AddFileRequest {
    pub path: String,
}
