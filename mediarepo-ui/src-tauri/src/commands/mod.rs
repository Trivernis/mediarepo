use rmp_ipc::ipc::context::Context as IPCContext;

use crate::context::{Context, OnceBuffer};
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

pub async fn get_ipc(context: &tauri::State<'_, Context>) -> AppResult<IPCContext> {
  let ipc = context.ipc.read().await;
  (ipc.clone()).ok_or(AppError::new("No ipc connection."))
}

/// Adds a once-buffer to the buffer store
pub fn add_once_buffer(
  context: &tauri::State<'_, Context>,
  key: String,
  mime: String,
  buf: Vec<u8>,
) -> String {
  let uri = format!("once://{}", key);
  let once_buffer = OnceBuffer { mime, buf };
  let mut once_buffers = context.once_buffers.lock().unwrap();
  once_buffers.insert(key, once_buffer);

  uri
}
