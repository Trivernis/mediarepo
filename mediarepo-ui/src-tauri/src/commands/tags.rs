use crate::commands::get_ipc;
use crate::context::Context;
use crate::error::AppResult;
use mediarepo::requests::FileIdentifier;
use mediarepo::responses::TagResponse;

#[tauri::command]
pub async fn get_tags_for_file(
  hash: String,
  context: tauri::State<'_, Context>,
) -> AppResult<Vec<TagResponse>> {
  let ipc = get_ipc(context).await?;
  let response = ipc
    .emitter
    .emit_to("tags", "tags_for_file", FileIdentifier::Hash(hash))
    .await?
    .await_reply(&ipc)
    .await?;

  Ok(response.data::<Vec<TagResponse>>()?)
}
