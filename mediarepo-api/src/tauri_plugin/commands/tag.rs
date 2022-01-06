use crate::tauri_plugin::commands::ApiAccess;
use crate::tauri_plugin::error::PluginResult;
use crate::types::identifier::FileIdentifier;
use crate::types::tags::{NamespaceResponse, TagResponse};

#[tauri::command]
pub async fn get_all_tags(api_state: ApiAccess<'_>) -> PluginResult<Vec<TagResponse>> {
    let api = api_state.api().await?;
    let all_tags = api.tag.get_all_tags().await?;

    Ok(all_tags)
}

#[tauri::command]
pub async fn get_all_namespaces(api_state: ApiAccess<'_>) -> PluginResult<Vec<NamespaceResponse>> {
    let api = api_state.api().await?;
    let all_namespaces = api.tag.get_all_namespaces().await?;

    Ok(all_namespaces)
}

#[tauri::command]
pub async fn get_tags_for_file(
    id: i64,
    api_state: ApiAccess<'_>,
) -> PluginResult<Vec<TagResponse>> {
    let api = api_state.api().await?;
    let tags = api.tag.get_tags_for_file(FileIdentifier::ID(id)).await?;

    Ok(tags)
}

#[tauri::command]
pub async fn get_tags_for_files(
    ids: Vec<i64>,
    api_state: ApiAccess<'_>,
) -> PluginResult<Vec<TagResponse>> {
    let api = api_state.api().await?;
    let tags = api.tag.get_tags_for_files(ids).await?;

    Ok(tags)
}

#[tauri::command]
pub async fn create_tags(
    api_state: ApiAccess<'_>,
    tags: Vec<String>,
) -> PluginResult<Vec<TagResponse>> {
    let api = api_state.api().await?;
    let tags = api.tag.create_tags(tags).await?;

    Ok(tags)
}

#[tauri::command]
pub async fn change_file_tags(
    api_state: ApiAccess<'_>,
    id: i64,
    added_tags: Vec<i64>,
    removed_tags: Vec<i64>,
) -> PluginResult<Vec<TagResponse>> {
    let api = api_state.api().await?;
    let tags = api
        .tag
        .change_file_tags(FileIdentifier::ID(id), added_tags, removed_tags)
        .await?;

    Ok(tags)
}
