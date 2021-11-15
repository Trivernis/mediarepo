use crate::error::RepoResult;
use serde::{Deserialize, Serialize};
use std::net::IpAddr;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Settings {
    pub listen_address: IpAddr,
    pub port_range: (u16, u16),
    pub database_path: String,
    pub default_file_store: String,
    pub thumbnail_store: String,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            listen_address: IpAddr::from([127, 0, 0, 1]),
            port_range: (3400, 3500),
            database_path: "./db/repo.db".to_string(),
            default_file_store: "Main".to_string(),
            thumbnail_store: "./thumbnails".to_string(),
        }
    }
}

impl Settings {
    /// Parses settings from a string
    pub fn from_toml_string(s: &str) -> RepoResult<Self> {
        let settings = toml::from_str(s)?;
        Ok(settings)
    }

    /// Converts the settings into a toml string
    pub fn to_toml_string(&self) -> RepoResult<String> {
        let string = toml::to_string(&self)?;
        Ok(string)
    }
}
