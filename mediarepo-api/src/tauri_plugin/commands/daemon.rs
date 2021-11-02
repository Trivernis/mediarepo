use crate::tauri_plugin::commands::AppAccess;
use crate::tauri_plugin::error::PluginResult;
use rmp_ipc::prelude::IPCResult;
use rmp_ipc::IPCBuilder;

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
    let connect_result = try_connect_daemon(address).await;

    Ok(connect_result.is_ok())
}

async fn try_connect_daemon(address: String) -> IPCResult<()> {
    let ctx = IPCBuilder::new().address(address).build_client().await?;
    ctx.emitter
        .emit("info", ())
        .await?
        .await_reply(&ctx)
        .await?;
    ctx.stop().await?;
    Ok(())
}
