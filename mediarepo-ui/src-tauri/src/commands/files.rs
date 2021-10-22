use crate::commands::get_ipc;
use mediarepo::requests::{FindFilesByTagsRequest, GetFileThumbnailsRequest, ReadFileRequest};
use mediarepo::responses::{FileResponse, ThumbnailResponse};

use crate::context::Context;
use crate::error::{AppError, AppResult};

#[tauri::command]
pub async fn get_all_files(context: tauri::State<'_, Context>) -> AppResult<Vec<FileResponse>> {
  let ipc = get_ipc(context).await?;
  let response = ipc
    .emitter
    .emit_to("files", "all_files", ())
    .await?
    .await_reply(&ipc)
    .await?;

  Ok(response.data::<Vec<FileResponse>>()?)
}

#[tauri::command]
pub async fn find_files(
  tags: Vec<String>,
  context: tauri::State<'_, Context>,
) -> AppResult<Vec<FileResponse>> {
  let ipc = get_ipc(context).await?;
  let response = ipc
    .emitter
    .emit_to("files", "find_files", FindFilesByTagsRequest { tags })
    .await?
    .await_reply(&ipc)
    .await?;
  Ok(response.data::<Vec<FileResponse>>()?)
}

#[tauri::command]
pub async fn read_file_by_hash(
  hash: String,
  context: tauri::State<'_, Context>,
) -> AppResult<Vec<u8>> {
  let ipc = get_ipc(context).await?;
  let response = ipc
    .emitter
    .emit_to("files", "read_file", ReadFileRequest::Hash(hash))
    .await?
    .await_reply(&ipc)
    .await?;

  Ok(response.data_raw().to_vec())
}

#[tauri::command]
pub async fn get_thumbnails(
  hash: String,
  context: tauri::State<'_, Context>,
) -> AppResult<Vec<ThumbnailResponse>> {
  let ipc = get_ipc(context).await?;
  let response = ipc
    .emitter
    .emit_to(
      "files",
      "get_thumbnails",
      GetFileThumbnailsRequest::Hash(hash),
    )
    .await?
    .await_reply(&ipc)
    .await?;

  Ok(response.data::<Vec<ThumbnailResponse>>()?)
}

#[tauri::command]
pub async fn read_thumbnail(
  hash: String,
  context: tauri::State<'_, Context>,
) -> AppResult<Vec<u8>> {
  let ipc = context.ipc.read().await;
  if let Some(ipc) = &*ipc {
    let response = ipc
      .emitter
      .emit_to("files", "read_thumbnail", hash)
      .await?
      .await_reply(&ipc)
      .await?;

    Ok(response.data_raw().to_vec())
  } else {
    Err(AppError::new("No ipc connection."))
  }
}
