use crate::tauri_plugin::error::PluginResult;
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use serde_piecewise_default::DeserializePiecewiseDefault;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

static SETTINGS_FILE: &str = "settings.toml";

#[derive(Serialize, Debug, Deserialize, Clone)]
pub struct Repository {
    pub(crate) name: String,
    pub(crate) path: Option<String>,
    pub(crate) address: Option<String>,
    pub(crate) local: bool,
}

#[derive(DeserializePiecewiseDefault, Debug, Serialize)]
pub struct Settings {
    pub daemon_path: String,
    pub repositories: HashMap<String, Repository>,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            daemon_path: String::from("mediarepo-daemon"),
            repositories: HashMap::new(),
        }
    }
}

fn get_settings_path() -> PathBuf {
    let dirs = ProjectDirs::from("com", "trivernis", "mediarepo").unwrap();
    let config_path = dirs.config_dir().to_path_buf();

    config_path.join(SETTINGS_FILE)
}

/// Writes the settings to the file
#[tracing::instrument(level = "debug")]
pub fn save_settings(settings: &Settings) -> PluginResult<()> {
    let settings_path = get_settings_path();
    let settings_string = toml::to_string(&settings)?;
    fs::write(&settings_path, &settings_string.into_bytes())?;

    Ok(())
}

/// Loads the settings from the file
#[tracing::instrument(level = "debug")]
pub fn load_settings() -> PluginResult<Settings> {
    let dirs = ProjectDirs::from("com", "trivernis", "mediarepo")
        .expect("Failed to get project directories");
    let config_path = dirs.config_dir().to_path_buf();
    if !config_path.exists() {
        fs::create_dir_all(&config_path)?;
    }
    let settings_path = config_path.join(SETTINGS_FILE);
    if !settings_path.exists() {
        let settings = Settings::default();
        save_settings(&settings)?;
    }
    let config_str = fs::read_to_string(settings_path)?;
    let settings = toml::from_str(&config_str)?;

    Ok(settings)
}
