use mediarepo::requests::ReadFileRequest;
use mediarepo::responses::FileResponse;
use crate::context::Context;
use crate::error::{AppError, AppResult};

#[tauri::command]
pub async fn get_all_files(context: tauri::State<'_, Context>) -> AppResult<Vec<FileResponse>> {
  let ipc = context.ipc.read().await;
  if let Some(ipc) = &*ipc {
    let response = ipc.emitter.emit_to("files", "all_files", ()).await?.await_reply(&ipc).await?;

    Ok(response.data::<Vec<FileResponse>>()?)
  } else {
    Err(AppError::new("No ipc connection."))
  }
}

#[tauri::command]
pub async fn read_file_by_hash(hash: String, context: tauri::State<'_, Context>) -> AppResult<Vec<u8>> {
  let ipc = context.ipc.read().await;
  if let Some(ipc) = &*ipc {
    let response = ipc.emitter.emit_to("files", "read_file", ReadFileRequest::Hash(hash)).await?.await_reply(&ipc).await?;

    Ok(response.data::<Vec<u8>>()?)
  } else {
    Err(AppError::new("No ipc connection."))
  }

}
