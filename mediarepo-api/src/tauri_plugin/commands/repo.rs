use crate::client_api::ApiClient;
use crate::tauri_plugin::commands::{ApiAccess, AppAccess};
use crate::tauri_plugin::error::{PluginError, PluginResult};
use crate::tauri_plugin::settings::{save_settings, Repository};
use serde::{Deserialize, Serialize};
use std::mem;
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
    path: Option<String>,
    address: Option<String>,
    local: bool,
    app_state: AppAccess<'_>,
) -> PluginResult<Vec<Repository>> {
    if path.is_none() && address.is_none() {
        return Err(PluginError::from(
            "Either a path or an address needs to be specified for the repository",
        ));
    }
    let repo = Repository {
        name,
        path,
        address,
        local,
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
pub async fn disconnect_repository(api_state: ApiAccess<'_>) -> PluginResult<()> {
    api_state.disconnect().await;

    Ok(())
}

#[tauri::command]
pub async fn close_local_repository(
    app_state: AppAccess<'_>,
    api_state: ApiAccess<'_>,
) -> PluginResult<()> {
    let mut active_repo = app_state.active_repo.write().await;
    if let Some(path) = mem::take(&mut *active_repo).and_then(|r| r.path) {
        app_state.stop_running_daemon(&path).await?;
    }
    api_state.disconnect().await;

    Ok(())
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
    let address = if let Some(address) = &repo.address {
        address.clone()
    } else {
        tracing::debug!("Reading repo address from config.");
        let path = repo
            .path
            .clone()
            .ok_or_else(|| PluginError::from("Missing repo path or address in config."))?;
        let config = read_repo_config(PathBuf::from(path).join(REPO_CONFIG_FILE)).await?;

        config.listen_address
    };
    let client = ApiClient::connect(&address).await?;
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
