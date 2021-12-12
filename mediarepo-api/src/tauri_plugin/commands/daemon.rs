use crate::tauri_plugin::commands::AppAccess;
use crate::tauri_plugin::error::PluginResult;
use bromine::prelude::{IPCError, IPCResult};
use bromine::IPCBuilder;
use std::io::ErrorKind;
use std::net::{SocketAddr, ToSocketAddrs};
use tokio::net::TcpListener;

#[tauri::command]
pub async fn has_executable(app_state: AppAccess<'_>) -> PluginResult<bool> {
    let settings = app_state.settings.read().await;

    Ok(settings.daemon_path.is_some())
}

#[tauri::command]
pub async fn init_repository(app_state: AppAccess<'_>, repo_path: String) -> PluginResult<()> {
    let daemon = app_state.get_daemon_cli(repo_path).await?;
    daemon.init_repo().await?;

    Ok(())
}

#[tauri::command]
pub async fn start_daemon(app_state: AppAccess<'_>, repo_path: String) -> PluginResult<()> {
    let mut daemon = app_state.get_daemon_cli(repo_path).await?;
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
    let address = get_socket_address(address)?;
    let ctx = IPCBuilder::<TcpListener>::new()
        .address(address)
        .build_client()
        .await?;
    ctx.emit("info", ()).await?.await_reply(&ctx).await?;
    ctx.stop().await?;
    Ok(())
}

fn get_socket_address(address: String) -> IPCResult<SocketAddr> {
    address
        .to_socket_addrs()
        .ok()
        .and_then(|mut addr| addr.next())
        .ok_or_else(|| {
            IPCError::IoError(std::io::Error::new(
                ErrorKind::InvalidInput,
                "Invalid Socket address",
            ))
        })
}
