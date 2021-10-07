#[derive(Clone, Debug)]
pub struct Settings {
    pub repo_path: String,
    pub database_path: String,
    pub default_file_store: String,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            repo_path: "".to_string(),
            database_path: "".to_string(),
            default_file_store: "".to_string(),
        }
    }
}
