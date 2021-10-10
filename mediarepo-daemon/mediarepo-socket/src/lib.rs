use crate::types::responses::InfoResponse;
use rmp_ipc::context::Context;
use rmp_ipc::error::Result;
use rmp_ipc::{Event, IPCBuilder};

mod namespaces;
pub mod types;

pub fn get_builder(address: &str) -> IPCBuilder {
    namespaces::build_namespaces(IPCBuilder::new().address(address))
        .on("info", |c, e| Box::pin(info(c, e)))
}

async fn info(ctx: &Context, event: Event) -> Result<()> {
    let response = InfoResponse {
        name: env!("CARGO_PKG_NAME").to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
    };
    ctx.emitter
        .emit_response(event.id(), "info", response)
        .await?;

    Ok(())
}
