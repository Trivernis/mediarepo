use crate::client_api::protocol::ApiProtocolListener;
use crate::client_api::ApiClient;
use crate::tauri_plugin::commands::{ApiAccess, AppAccess, BufferAccess};
use crate::tauri_plugin::error::{PluginError, PluginResult};
use crate::tauri_plugin::settings::{save_settings, Repository};
use crate::types::repo::FrontendState;
use serde::{Deserialize, Serialize};
use std::mem;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};
use tokio::fs;
use tokio::time::Duration;

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
    let mut repositories: Vec<Repository> = settings.repositories.values().cloned().collect();
    repositories.sort_by_key(|r| r.last_opened.unwrap_or(0));
    repositories.reverse(); // the last opened repository should always be on top

    Ok(repositories)
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
        last_opened: None,
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
pub async fn remove_repository(app_state: AppAccess<'_>, name: String) -> PluginResult<()> {
    let mut settings = app_state.settings.write().await;

    if let Some(_repo) = settings.repositories.remove(&name) {
        save_settings(&settings)?;
        Ok(())
    } else {
        Err(PluginError::from(format!(
            "The repository '{}' does not exist.",
            name
        )))
    }
}

#[tauri::command]
pub async fn delete_repository(app_state: AppAccess<'_>, name: String) -> PluginResult<()> {
    let settings = app_state.settings.read().await;

    if let Some(repository) = settings.repositories.get(&name) {
        if let Some(path) = &repository.path {
            fs::remove_dir_all(PathBuf::from(path)).await?;

            Ok(())
        } else {
            Err(PluginError::from(format!(
                "The repository '{}' is a remote repository",
                name
            )))
        }
    } else {
        Err(PluginError::from(format!(
            "The repository '{}' does not exist.",
            name
        )))
    }
}

#[tauri::command]
pub async fn check_local_repository_exists(path: String) -> PluginResult<bool> {
    let config_path = PathBuf::from(path).join(REPO_CONFIG_FILE);

    if !config_path.exists() {
        Ok(false)
    } else {
        Ok(true)
    }
}

#[tauri::command]
pub async fn disconnect_repository(
    app_state: AppAccess<'_>,
    api_state: ApiAccess<'_>,
    buffer_state: BufferAccess<'_>,
) -> PluginResult<()> {
    api_state.disconnect().await;
    let mut active_repo = app_state.active_repo.write().await;
    mem::take(&mut *active_repo);
    buffer_state.clear();

    Ok(())
}

#[tauri::command]
pub async fn close_local_repository(
    app_state: AppAccess<'_>,
    api_state: ApiAccess<'_>,
    buffer_state: BufferAccess<'_>,
) -> PluginResult<()> {
    let mut active_repo = app_state.active_repo.write().await;

    if let Some(path) = mem::take(&mut *active_repo).and_then(|r| r.path) {
        app_state.stop_running_daemon(&path).await?;
    }
    api_state.disconnect().await;
    mem::take(&mut *active_repo);
    buffer_state.clear();

    Ok(())
}

#[tauri::command]
pub async fn select_repository(
    name: String,
    app_state: AppAccess<'_>,
    api_state: ApiAccess<'_>,
) -> PluginResult<()> {
    let mut settings = app_state.settings.write().await;
    let repo = settings
        .repositories
        .get_mut(&name)
        .ok_or(PluginError::from(
            format!("Repository '{}' not found", name).as_str(),
        ))?;
    close_selected_repository(&app_state).await?;
    let address = if let Some(address) = &repo.address {
        address.clone()
    } else {
        tracing::debug!("Reading repo address from local file.");
        let path = repo
            .path
            .clone()
            .ok_or_else(|| PluginError::from("Missing repo path or address in config."))?;
        get_repo_address(path).await?
    };
    let client = ApiClient::connect::<ApiProtocolListener>(address).await?;
    api_state.set_api(client).await;

    let mut active_repo = app_state.active_repo.write().await;
    repo.last_opened = Some(
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs(),
    );

    *active_repo = Some(repo.clone());
    save_settings(&settings)?;

    Ok(())
}

#[tauri::command]
pub async fn get_frontend_state(api_state: ApiAccess<'_>) -> PluginResult<Option<String>> {
    let api = api_state.api().await?;
    let state = api.repo.get_frontend_state().await?;

    Ok(state.state)
}

#[tauri::command]
pub async fn set_frontend_state(api_state: ApiAccess<'_>, state: String) -> PluginResult<()> {
    let api = api_state.api().await?;
    api.repo
        .set_frontend_state(FrontendState { state: Some(state) })
        .await?;

    Ok(())
}

async fn get_repo_address(path: String) -> PluginResult<String> {
    let tcp_path = PathBuf::from(&path).join("repo.tcp");
    let socket_path = PathBuf::from(&path).join("repo.sock");

    let mut address = String::from("127.0.0.1:2400");
    for _ in 0..10 {
        #[cfg(unix)]
        if socket_path.exists() {
            address = socket_path.to_str().unwrap().to_string();
            break;
        }
        if tcp_path.exists() {
            address = fs::read_to_string(tcp_path).await?;
            break;
        }
        tokio::time::sleep(Duration::from_millis(250)).await;
    }

    Ok(address)
}

async fn close_selected_repository(app_state: &AppAccess<'_>) -> PluginResult<()> {
    if let Some(path) = app_state
        .active_repo
        .read()
        .await
        .clone()
        .and_then(|r| r.path)
    {
        app_state.stop_running_daemon(&path).await?;
    }

    Ok(())
}
