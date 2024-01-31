use std::net::SocketAddr;

use tokio::net::TcpListener;
use tokio::task::JoinHandle;

use crate::encrypted::EncryptedListener;
use mediarepo_core::bromine::prelude::*;
use mediarepo_core::error::{RepoError, RepoResult};
use mediarepo_core::mediarepo_api::types::misc::InfoResponse;
use mediarepo_core::settings::{PortSetting, Settings};
use mediarepo_core::tokio_graceful_shutdown::SubsystemHandle;
use mediarepo_core::trait_bound_typemap::{SendSyncTypeMap, TypeMap};
use mediarepo_core::type_keys::{SizeMetadataKey, SubsystemKey};

mod from_model;
mod namespaces;
mod utils;

#[tracing::instrument(skip_all)]
pub fn start_tcp_server(
    subsystem: SubsystemHandle,
    settings: Settings,
    shared_data: SendSyncTypeMap,
) -> RepoResult<(String, JoinHandle<()>)> {
    let port = match &settings.server.tcp.port {
        PortSetting::Fixed(p) => {
            if port_check::is_local_port_free(*p) {
                *p
            } else {
                return Err(RepoError::PortUnavailable);
            }
        }
        PortSetting::Range((l, r)) => {
            port_check::free_local_port_in_range(*l, *r).ok_or(RepoError::PortUnavailable)?
        }
    };
    let ip = settings.server.tcp.listen_address.to_owned();
    let address = SocketAddr::new(ip, port);
    let address_string = address.to_string();

    let join_handle = tokio::task::spawn(async move {
        get_builder::<EncryptedListener<TcpListener>>(address)
            .insert::<SubsystemKey>(subsystem)
            .insert_all(shared_data)
            .insert::<SizeMetadataKey>(Default::default())
            .build_server()
            .await
            .expect("Failed to start tcp server")
    });

    Ok((address_string, join_handle))
}

#[cfg(unix)]
#[tracing::instrument(skip_all)]
pub fn create_unix_socket(
    subsystem: SubsystemHandle,
    path: std::path::PathBuf,
    shared_data: SendSyncTypeMap,
) -> RepoResult<JoinHandle<()>> {
    use std::fs;
    use tokio::net::UnixListener;

    if path.exists() {
        fs::remove_file(&path)?;
    }
    let join_handle = tokio::task::spawn(async move {
        get_builder::<UnixListener>(path)
            .insert::<SubsystemKey>(subsystem)
            .insert_all(shared_data)
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
async fn info(ctx: &Context, _: Event) -> IPCResult<Response> {
    let response = InfoResponse::new(
        env!("CARGO_PKG_NAME").to_string(),
        env!("CARGO_PKG_VERSION").to_string(),
    );

    ctx.response(response)
}

#[tracing::instrument(skip_all)]
async fn shutdown(ctx: &Context, _: Event) -> IPCResult<Response> {
    ctx.clone().stop().await?;
    {
        let data = ctx.data.read().await;
        let subsystem = data.get::<SubsystemKey>().unwrap();
        subsystem.request_shutdown();
        subsystem.on_shutdown_requested().await;
    }

    Ok(Response::empty())
}
