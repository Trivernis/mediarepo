use crate::client_api::ApiClient;
use crate::tauri_plugin::commands::AppAccess;
use crate::tauri_plugin::error::PluginResult;

#[tauri::command]
pub async fn init_repository(app_state: AppAccess<'_>, repo_path: String) -> PluginResult<()> {
    let daemon = app_state.get_daemon_cli(repo_path).await;
    daemon.init_repo().await?;

    Ok(())
}

#[tauri::command]
pub async fn start_daemon(app_state: AppAccess<'_>, repo_path: String) -> PluginResult<()> {
    let mut daemon = app_state.get_daemon_cli(repo_path).await;
    daemon.start_daemon()?;
    app_state.add_started_daemon(daemon).await;

    Ok(())
}

#[tauri::command]
pub async fn stop_daemon(app_state: AppAccess<'_>, repo_path: String) -> PluginResult<()> {
    app_state.stop_running_daemon(&repo_path).await?;

    Ok(())
}

#[tauri::command]
pub async fn check_daemon_running(address: String) -> PluginResult<bool> {
    if let Ok(api_client) = ApiClient::connect(&address).await {
        Ok(api_client.info().await.is_ok())
    } else {
        Ok(false)
    }
}
