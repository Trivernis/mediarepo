use crate::tauri_plugin::commands::ApiAccess;
use crate::tauri_plugin::error::PluginResult;
use crate::types::tags::TagResponse;

#[tauri::command]
pub async fn get_tags_for_file(
    hash: String,
    api_state: ApiAccess<'_>,
) -> PluginResult<Vec<TagResponse>> {
    let api = api_state.api().await?;
    let tags = api.tag.get_tags_for_file(hash).await?;

    Ok(tags)
}
