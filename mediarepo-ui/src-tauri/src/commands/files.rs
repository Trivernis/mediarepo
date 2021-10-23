use crate::commands::{add_once_buffer, get_ipc};
use mediarepo::requests::{FindFilesByTagsRequest, GetFileThumbnailsRequest, ReadFileRequest};
use mediarepo::responses::{FileResponse, ThumbnailResponse};

use crate::context::Context;
use crate::error::AppResult;

#[tauri::command]
pub async fn get_all_files(context: tauri::State<'_, Context>) -> AppResult<Vec<FileResponse>> {
  let ipc = get_ipc(&context).await?;
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
  let ipc = get_ipc(&context).await?;
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
  mime: String,
  context: tauri::State<'_, Context>,
) -> AppResult<String> {
  let ipc = get_ipc(&context).await?;
  let response = ipc
    .emitter
    .emit_to("files", "read_file", ReadFileRequest::Hash(hash.clone()))
    .await?
    .await_reply(&ipc)
    .await?;
  let raw_data = response.data_raw().to_vec();
  let uri = add_once_buffer(&context, hash, mime, raw_data);

  Ok(uri)
}

#[tauri::command]
pub async fn get_thumbnails(
  hash: String,
  context: tauri::State<'_, Context>,
) -> AppResult<Vec<ThumbnailResponse>> {
  let ipc = get_ipc(&context).await?;
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
  mime: String,
  context: tauri::State<'_, Context>,
) -> AppResult<String> {
  let ipc = get_ipc(&context).await?;
  let response = ipc
    .emitter
    .emit_to("files", "read_thumbnail", hash.clone())
    .await?
    .await_reply(&ipc)
    .await?;
  let raw_data = response.data_raw().to_vec();
  let uri = add_once_buffer(&context, hash, mime, raw_data);

  Ok(uri)
}
