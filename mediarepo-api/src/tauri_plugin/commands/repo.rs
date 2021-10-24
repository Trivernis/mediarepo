use crate::client_api::ApiClient;
use crate::tauri_plugin::commands::{ApiAccess, AppAccess};
use crate::tauri_plugin::error::{PluginError, PluginResult};
use crate::tauri_plugin::settings::{save_settings, Repository};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use tokio::fs;

static REPO_CONFIG_FILE: &str = "repo.toml";

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RepoConfig {
    pub listen_address: String,
    pub database_path: String,
    pub default_file_store: String,
}

#[tauri::command]
pub async fn get_repositories(app_state: AppAccess<'_>) -> PluginResult<Vec<Repository>> {
    let settings = app_state.settings.read().await;

    Ok(settings.repositories.values().cloned().collect())
}

#[tauri::command]
pub async fn get_active_repository(app_state: AppAccess<'_>) -> PluginResult<Option<Repository>> {
    let repo = app_state.active_repo.read().await;
    Ok(repo.clone())
}

#[tauri::command]
pub async fn add_repository(
    name: String,
    path: String,
    app_state: AppAccess<'_>,
) -> PluginResult<Vec<Repository>> {
    let repo_path = path.clone();
    let path = PathBuf::from(path);
    let RepoConfig { listen_address, .. } = read_repo_config(path.join(REPO_CONFIG_FILE)).await?;

    let repo = Repository {
        name,
        path: Some(repo_path),
        address: listen_address,
    };

    let mut repositories = Vec::new();
    {
        let mut settings = app_state.settings.write().await;
        settings.repositories.insert(repo.name.clone(), repo);
        save_settings(&settings)?;
        repositories.append(&mut settings.repositories.values().cloned().collect());
    }

    Ok(repositories)
}

#[tauri::command]
pub async fn select_repository(
    name: String,
    app_state: AppAccess<'_>,
    api_state: ApiAccess<'_>,
) -> PluginResult<()> {
    let settings = app_state.settings.read().await;
    let repo = settings.repositories.get(&name).ok_or(PluginError::from(
        format!("Repository '{}' not found", name).as_str(),
    ))?;
    let client = ApiClient::connect(&repo.address).await?;
    api_state.set_api(client).await;

    let mut active_repo = app_state.active_repo.write().await;
    *active_repo = Some(repo.clone());

    Ok(())
}

async fn read_repo_config(path: PathBuf) -> PluginResult<RepoConfig> {
    let toml_str = fs::read_to_string(path).await?;
    let config = toml::from_str(&toml_str)?;

    Ok(config)
}
