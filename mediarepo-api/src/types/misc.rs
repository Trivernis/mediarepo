#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct InfoResponse {
    pub name: String,
    pub version: String,
}