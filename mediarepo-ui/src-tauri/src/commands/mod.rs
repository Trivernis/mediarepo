use crate::context::Context;
use crate::error::AppResult;

pub mod repo;

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
