use crate::types::responses::InfoResponse;
use mediarepo_core::rmp_ipc::prelude::*;

mod namespaces;
pub mod types;
mod utils;

pub fn get_builder(address: &str) -> IPCBuilder {
    namespaces::build_namespaces(IPCBuilder::new().address(address)).on("info", callback!(info))
}

async fn info(ctx: &Context, event: Event) -> IPCResult<()> {
    let response = InfoResponse {
        name: env!("CARGO_PKG_NAME").to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
    };
    ctx.emitter
        .emit_response(event.id(), "info", response)
        .await?;

    Ok(())
}
