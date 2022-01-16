use crate::error::RepoResult;
use serde::{Deserialize, Serialize};
use std::net::IpAddr;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SettingsV1 {
    pub listen_address: IpAddr,
    pub port_range: (u16, u16),
    pub database_path: String,
    pub default_file_store: String,
    pub thumbnail_store: String,
}

impl SettingsV1 {
    /// Parses settings from a string
    pub fn from_toml_string(s: &str) -> RepoResult<Self> {
        let settings = toml::from_str(s)?;
        Ok(settings)
    }
}
