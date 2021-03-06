use std::fs;
use std::path::{Path, PathBuf};

use config::{Config, FileFormat};
use serde::{Deserialize, Serialize};

pub use logging::*;
pub use paths::*;
pub use server::*;

use crate::error::RepoResult;
use crate::settings::v1::SettingsV1;

mod logging;
mod paths;
mod server;
pub mod v1;

#[derive(Clone, Debug, Deserialize, Serialize, Default)]
pub struct Settings {
    pub server: ServerSettings,
    pub paths: PathSettings,
    pub logging: LoggingSettings,
}

impl Settings {
    pub fn read(root: &Path) -> RepoResult<Self> {
        let settings = Config::builder()
            .add_source(config::File::from_str(
                &*Settings::default().to_toml_string()?,
                FileFormat::Toml,
            ))
            .add_source(config::File::from(root.join("repo")))
            .add_source(config::Environment::with_prefix("MEDIAREPO").separator("."))
            .build()?;
        tracing::debug!("Settings are: {:#?}", settings);

        Ok(settings.try_deserialize()?)
    }

    /// Parses settings from a string
    pub fn from_v1(settings_v1: SettingsV1) -> RepoResult<Self> {
        let mut settings_main = Settings::default();
        settings_main.server.tcp.enabled = true;
        settings_main.server.tcp.port = PortSetting::Range(settings_v1.port_range);
        settings_main.server.tcp.listen_address = settings_v1.listen_address;
        settings_main.paths.thumbnail_directory = settings_v1.thumbnail_store;
        settings_main.paths.database_directory = PathBuf::from(settings_v1.database_path)
            .parent()
            .map(|p| p.to_string_lossy().to_string())
            .unwrap_or_else(|| String::from("./"));

        let settings = Config::builder()
            .add_source(config::File::from_str(
                &*settings_main.to_toml_string()?,
                FileFormat::Toml,
            ))
            .add_source(config::Environment::with_prefix("MEDIAREPO"))
            .build()?;
        tracing::debug!("Settings are: {:#?}", settings);

        Ok(settings.try_deserialize()?)
    }

    /// Converts the settings into a toml string
    pub fn to_toml_string(&self) -> RepoResult<String> {
        let string = toml::to_string(&self)?;

        Ok(string)
    }

    pub fn save(&self, root: &Path) -> RepoResult<()> {
        let string = toml::to_string_pretty(&self)?;
        fs::write(root.join("repo.toml"), string.into_bytes())?;

        Ok(())
    }
}
