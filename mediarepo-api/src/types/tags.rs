#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TagResponse {
    pub id: i64,
    pub namespace: Option<String>,
    pub name: String,
}