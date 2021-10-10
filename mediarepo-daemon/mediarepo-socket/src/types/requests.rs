use serde::Deserialize;

#[derive(Deserialize)]
pub struct AddFileRequest {
    pub path: String,
}
