use mediarepo_core::bromine::prelude::*;
use mediarepo_core::error::{RepoError, RepoResult};
use mediarepo_core::mediarepo_api::types::misc::InfoResponse;
use mediarepo_core::settings::{PortSetting, Settings};
use mediarepo_core::tokio_graceful_shutdown::SubsystemHandle;
use mediarepo_core::type_keys::{RepoPathKey, SettingsKey, SizeMetadataKey, SubsystemKey};
use mediarepo_logic::repo::Repo;
use mediarepo_logic::type_keys::RepoKey;
use std::net::SocketAddr;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::net::TcpListener;
use tokio::task::JoinHandle;

mod from_model;
mod namespaces;
mod utils;

#[tracing::instrument(skip(subsystem, settings, repo))]
pub fn start_tcp_server(
    subsystem: SubsystemHandle,
    repo_path: PathBuf,
    settings: Settings,
    repo: Repo,
) -> RepoResult<(String, JoinHandle<()>)> {
    let port = match &settings.server.tcp.port {
        PortSetting::Fixed(p) => {
            if port_check::is_local_port_free(*p) {
                *p
            } else {
                return Err(RepoError::PortUnavailable);
            }
        }
        PortSetting::Range((l, r)) => port_check::free_local_port_in_range(*l, *r)
            .ok_or_else(|| RepoError::PortUnavailable)?,
    };
    let ip = settings.server.tcp.listen_address.to_owned();
    let address = SocketAddr::new(ip, port);
    let address_string = address.to_string();

    let join_handle = tokio::task::Builder::new()
        .name("mediarepo_tcp::listen")
        .spawn(async move {
            get_builder::<TcpListener>(address)
                .insert::<SubsystemKey>(subsystem)
                .insert::<RepoKey>(Arc::new(repo))
                .insert::<SettingsKey>(settings)
                .insert::<RepoPathKey>(repo_path)
                .insert::<SizeMetadataKey>(Default::default())
                .build_server()
                .await
                .expect("Failed to start tcp server")
        });

    Ok((address_string, join_handle))
}

#[cfg(unix)]
#[tracing::instrument(skip(subsystem, settings, repo))]
pub fn create_unix_socket(
    subsystem: SubsystemHandle,
    path: std::path::PathBuf,
    repo_path: PathBuf,
    settings: Settings,
    repo: Repo,
) -> RepoResult<JoinHandle<()>> {
    use std::fs;
    use tokio::net::UnixListener;

    if path.exists() {
        fs::remove_file(&path)?;
    }
    let join_handle = tokio::task::Builder::new()
        .name("mediarepo_unix_socket::listen")
        .spawn(async move {
            get_builder::<UnixListener>(path)
                .insert::<SubsystemKey>(subsystem)
                .insert::<RepoKey>(Arc::new(repo))
                .insert::<SettingsKey>(settings)
                .insert::<RepoPathKey>(repo_path)
                .insert::<SizeMetadataKey>(Default::default())
                .build_server()
                .await
                .expect("Failed to create unix domain socket");
        });

    Ok(join_handle)
}

fn get_builder<L: AsyncStreamProtocolListener>(address: L::AddressType) -> IPCBuilder<L> {
    namespaces::build_namespaces(IPCBuilder::new().address(address))
        .on("info", callback!(info))
        .on("shutdown", callback!(shutdown))
}

#[tracing::instrument(skip_all)]
async fn info(ctx: &Context, _: Event) -> IPCResult<()> {
    let response = InfoResponse::new(
        env!("CARGO_PKG_NAME").to_string(),
        env!("CARGO_PKG_VERSION").to_string(),
    );
    ctx.emit("info", response).await?;

    Ok(())
}

#[tracing::instrument(skip_all)]
async fn shutdown(ctx: &Context, _: Event) -> IPCResult<()> {
    ctx.clone().stop().await?;
    {
        let data = ctx.data.read().await;
        let subsystem = data.get::<SubsystemKey>().unwrap();
        subsystem.request_shutdown();
        subsystem.on_shutdown_requested().await;
    }
    ctx.emit("shutdown", ()).await?;

    Ok(())
}
