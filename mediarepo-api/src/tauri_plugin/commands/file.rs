use crate::tauri_plugin::commands::{add_once_buffer, ApiAccess, BufferAccess};
use crate::tauri_plugin::error::PluginResult;
use crate::tauri_plugin::utils::system_time_to_naive_date_time;
use crate::types::files::{
    FileMetadataResponse, FileOSMetadata, SortKey, TagQuery, ThumbnailMetadataResponse,
};
use crate::types::identifier::FileIdentifier;
use std::path::PathBuf;
use tokio::fs;
use tokio::fs::DirEntry;

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
pub async fn get_thumbnail_of_size(
    api_state: ApiAccess<'_>,
    buffer_state: BufferAccess<'_>,
    file_id: i64,
    min_size: (u32, u32),
    max_size: (u32, u32),
) -> PluginResult<String> {
    let api = api_state.api().await?;
    let (thumb, data) = api
        .file
        .get_thumbnail_of_size(FileIdentifier::ID(file_id), min_size, max_size)
        .await?;
    let uri = add_once_buffer(
        buffer_state,
        thumb.hash,
        thumb.mime_type.unwrap_or(String::from("image/png")),
        data,
    );

    Ok(uri)
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

#[tauri::command]
pub async fn resolve_paths_to_files(paths: Vec<String>) -> PluginResult<Vec<FileOSMetadata>> {
    let mut files = Vec::new();

    for path in paths {
        let path = PathBuf::from(path);
        if path.exists() {
            files.append(&mut resolve_path_to_files(path).await?);
        }
    }

    Ok(files)
}

/// Resolves a path into several file metadata objects
#[tracing::instrument(level = "debug")]
async fn resolve_path_to_files(path: PathBuf) -> PluginResult<Vec<FileOSMetadata>> {
    let mut files = Vec::new();

    if path.is_dir() {
        let mut read_dir = fs::read_dir(path).await?;

        while let Some(entry) = read_dir.next_entry().await? {
            let subdir_entries = resolve_subdir(entry).await?;
            for entry in subdir_entries {
                let metadata = retrieve_file_information(entry.path()).await?;
                files.push(metadata);
            }
        }
    } else {
        let metadata = retrieve_file_information(path).await?;
        files.push(metadata);
    }

    Ok(files)
}

/// Iteratively resolves a directory into its sub components
#[tracing::instrument(level = "debug")]
async fn resolve_subdir(entry: DirEntry) -> PluginResult<Vec<DirEntry>> {
    let mut entries = vec![entry];

    for i in 0..entries.len() {
        let entry = &entries[i];

        if entry.path().is_dir() {
            let mut read_dir = fs::read_dir(entry.path()).await?;
            while let Some(entry) = read_dir.next_entry().await? {
                entries.push(entry);
            }
        }
    }

    Ok(entries)
}

/// Retrieves information about a path that MUST be a file and returns
/// metadata for it
#[tracing::instrument(level = "trace")]
async fn retrieve_file_information(path: PathBuf) -> PluginResult<FileOSMetadata> {
    let mime = mime_guess::from_path(&path)
        .first()
        .ok_or_else(|| format!("Could not guess mime for file {:?}", path))?;
    let metadata = fs::metadata(&path).await?;
    let creation_time = metadata.created()?;
    let change_time = metadata.modified()?;
    let name = path
        .file_name()
        .ok_or_else(|| "Could not retrieve file name")?;

    let os_metadata = FileOSMetadata {
        path: path.to_string_lossy().to_string(),
        name: name.to_string_lossy().to_string(),
        mime_type: mime.to_string(),
        creation_time: system_time_to_naive_date_time(creation_time),
        change_time: system_time_to_naive_date_time(change_time),
    };

    Ok(os_metadata)
}
