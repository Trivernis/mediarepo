mod files;

use crate::error::AppResult;
use mediarepo::responses::InfoResponse;
use rmp_ipc::error::Result;
use rmp_ipc::error_event::{ErrorEventData, ERROR_EVENT_NAME};
use rmp_ipc::event::Event;
use rmp_ipc::ipc::context::Context as IPCContext;
use rmp_ipc::IPCBuilder;
use tauri::Window;
use typemap_rev::TypeMapKey;

pub struct WindowKey;

impl TypeMapKey for WindowKey {
  type Value = Window;
}

pub async fn build_ipc_context(window: Window, address: &str) -> AppResult<IPCContext> {
  let ctx = IPCBuilder::new()
    .address(address)
    .insert::<WindowKey>(window)
    .on(ERROR_EVENT_NAME, |c, e| Box::pin(handle_error(c, e)))
    .on("info", |c, e| Box::pin(handle_info(c, e)))
    .build_client()
    .await?;

  Ok(ctx)
}

async fn handle_error(ctx: &IPCContext, event: Event) -> Result<()> {
  let error_data = event.data::<ErrorEventData>()?;
  let data = ctx.data.read().await;
  let window = data.get::<WindowKey>().unwrap();
  window
    .emit("error", error_data)
    .expect("Failed to emit error event");

  Ok(())
}

async fn handle_info(ctx: &IPCContext, event: Event) -> Result<()> {
  let info_data = event.data::<InfoResponse>()?;
  let data = ctx.data.read().await;
  let window = data.get::<WindowKey>().unwrap();
  window
    .emit("info", info_data)
    .expect("Failed to emit info event");

  Ok(())
}
