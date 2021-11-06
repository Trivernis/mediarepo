use crate::tauri_plugin::commands::{add_once_buffer, ApiAccess, BufferAccess};
use crate::tauri_plugin::error::PluginResult;
use crate::types::files::{FileMetadataResponse, SortKey, TagQuery, ThumbnailMetadataResponse};
use crate::types::identifier::FileIdentifier;

#[tauri::command]
pub async fn get_all_files(api_state: ApiAccess<'_>) -> PluginResult<Vec<FileMetadataResponse>> {
    let api = api_state.api().await?;
    let all_files = api.file.all_files().await?;

    Ok(all_files)
}

#[tauri::command]
pub async fn find_files(
    tags: Vec<TagQuery>,
    sort_by: Vec<SortKey>,
    api_state: ApiAccess<'_>,
) -> PluginResult<Vec<FileMetadataResponse>> {
    let api = api_state.api().await?;
    let files = api.file.find_files(tags, sort_by).await?;

    Ok(files)
}

#[tauri::command]
pub async fn read_file_by_hash(
    api_state: ApiAccess<'_>,
    buffer_state: BufferAccess<'_>,
    id: i64,
    hash: String,
    mime_type: String,
) -> PluginResult<String> {
    if buffer_state.reserve_entry(&hash) {
        Ok(format!("once://{}", hash)) // entry has been cached
    } else {
        let api = api_state.api().await?;
        let content = api.file.read_file_by_hash(FileIdentifier::ID(id)).await?;
        let uri = add_once_buffer(buffer_state, hash, mime_type, content);

        Ok(uri)
    }
}

#[tauri::command]
pub async fn get_file_thumbnails(
    api_state: ApiAccess<'_>,
    id: i64,
) -> PluginResult<Vec<ThumbnailMetadataResponse>> {
    let api = api_state.api().await?;
    let thumbs = api.file.get_file_thumbnails(FileIdentifier::ID(id)).await?;

    Ok(thumbs)
}

#[tauri::command]
pub async fn read_thumbnail(
    hash: String,
    mime_type: String,
    api_state: ApiAccess<'_>,
    buffer_state: BufferAccess<'_>,
) -> PluginResult<String> {
    if buffer_state.reserve_entry(&hash) {
        Ok(format!("once://{}", hash)) // entry has been cached
    } else {
        let api = api_state.api().await?;
        let content = api.file.read_thumbnail(hash.clone()).await?;
        let uri = add_once_buffer(buffer_state, hash, mime_type, content);

        Ok(uri)
    }
}

#[tauri::command]
pub async fn update_file_name(
    api_state: ApiAccess<'_>,
    id: i64,
    name: String,
) -> PluginResult<FileMetadataResponse> {
    let api = api_state.api().await?;
    let metadata = api
        .file
        .update_file_name(FileIdentifier::ID(id), name)
        .await?;

    Ok(metadata)
}
