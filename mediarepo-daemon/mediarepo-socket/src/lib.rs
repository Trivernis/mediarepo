use mediarepo_api::types::misc::InfoResponse;
use mediarepo_core::error::{RepoError, RepoResult};
use mediarepo_core::rmp_ipc::prelude::*;
use mediarepo_core::settings::Settings;
use mediarepo_core::type_keys::SettingsKey;
use mediarepo_model::repo::Repo;
use mediarepo_model::type_keys::RepoKey;
use std::net::{IpAddr, SocketAddr};
use std::sync::Arc;
use tokio::net::TcpListener;
use tokio::task::JoinHandle;

mod from_model;
mod namespaces;
mod utils;

pub fn start_tcp_server(
    ip: IpAddr,
    port_range: (u16, u16),
    settings: Settings,
    repo: Repo,
) -> RepoResult<(String, JoinHandle<()>)> {
    let port = port_check::free_local_port_in_range(port_range.0, port_range.1)
        .ok_or_else(|| RepoError::PortUnavailable)?;
    let address = SocketAddr::new(ip, port);
    let address_string = address.to_string();

    let join_handle = tokio::task::spawn(async move {
        get_builder::<TcpListener>(address)
            .insert::<RepoKey>(Arc::new(repo))
            .insert::<SettingsKey>(settings)
            .build_server()
            .await
            .expect("Failed to start tcp server")
    });

    Ok((address_string, join_handle))
}

fn get_builder<L: AsyncStreamProtocolListener>(address: L::AddressType) -> IPCBuilder<L> {
    namespaces::build_namespaces(IPCBuilder::new().address(address)).on("info", callback!(info))
}

#[tracing::instrument(skip_all)]
async fn info<S: AsyncProtocolStream>(ctx: &Context<S>, event: Event) -> IPCResult<()> {
    let response = InfoResponse::new(
        env!("CARGO_PKG_NAME").to_string(),
        env!("CARGO_PKG_VERSION").to_string(),
    );
    ctx.emitter
        .emit_response(event.id(), "info", response)
        .await?;

    Ok(())
}
