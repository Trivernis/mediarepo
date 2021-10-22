use rmp_ipc::ipc::context::Context as IPCContext;

use crate::context::Context;
use crate::error::{AppError, AppResult};

pub mod files;
pub mod repo;
pub mod tags;

#[tauri::command]
pub async fn emit_info(context: tauri::State<'_, Context>) -> AppResult<()> {
  let ipc = context.ipc.read().await;
  if let Some(ipc) = &*ipc {
    ipc.emitter.emit("info", ()).await?;
    println!("Emitted info event.");
  } else {
    println!("No IPC Context");
  }

  Ok(())
}

pub async fn get_ipc(context: tauri::State<'_, Context>) -> AppResult<IPCContext> {
  let ipc = context.ipc.read().await;
  (ipc.clone()).ok_or(AppError::new("No ipc connection."))
}
