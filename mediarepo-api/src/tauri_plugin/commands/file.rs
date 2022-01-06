use crate::tauri_plugin::commands::{ApiAccess, BufferAccess};
use crate::tauri_plugin::error::PluginResult;
use crate::tauri_plugin::utils::system_time_to_naive_date_time;
use crate::types::files::{
    FileBasicDataResponse, FileMetadataResponse, FileOSMetadata, FilterExpression, SortKey,
    ThumbnailMetadataResponse,
};
use crate::types::identifier::FileIdentifier;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::time::SystemTime;
use tokio::fs;
use tokio::fs::DirEntry;

#[derive(Serialize, Deserialize, Debug)]
pub struct AddFileOptions {
    pub read_tags_from_txt: bool,
    pub delete_after_import: bool,
}

#[tauri::command]
pub async fn get_all_files(api_state: ApiAccess<'_>) -> PluginResult<Vec<FileBasicDataResponse>> {
    let api = api_state.api().await?;
    let all_files = api.file.all_files().await?;

    Ok(all_files)
}

#[tauri::command]
pub async fn add_local_file(
    api_state: ApiAccess<'_>,
    metadata: FileOSMetadata,
    options: AddFileOptions,
) -> PluginResult<FileBasicDataResponse> {
    let api = api_state.api().await?;
    let path = PathBuf::from(&metadata.path);
    let mut tags = Vec::new();
    let txt_path = PathBuf::from(format!("{}.txt", path.to_string_lossy()));

    if options.read_tags_from_txt {
        if txt_path.exists() {
            let content = fs::read_to_string(&txt_path).await?;
            tags.append(
                &mut content
                    .split('\n')
                    .map(|line| line.to_owned())
                    .collect::<Vec<String>>(),
            );
        }
    }

    let file_content = fs::read(&path).await?;
    let file = api.file.add_file(metadata, tags, file_content).await?;
    if options.delete_after_import {
        fs::remove_file(path).await?;

        if options.read_tags_from_txt {
            fs::remove_file(txt_path).await?;
        }
    }

    Ok(file)
}

#[tauri::command]
pub async fn find_files(
    filters: Vec<FilterExpression>,
    sort_by: Vec<SortKey>,
    api_state: ApiAccess<'_>,
) -> PluginResult<Vec<FileBasicDataResponse>> {
    let api = api_state.api().await?;
    let files = api.file.find_files(filters, sort_by).await?;

    Ok(files)
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
pub async fn read_file(
    api_state: ApiAccess<'_>,
    buffer_state: BufferAccess<'_>,
    hash: String,
) -> PluginResult<Vec<u8>> {
    if let Some(buffer) = buffer_state.get_entry(&hash) {
        Ok(buffer.buf)
    } else {
        let api = api_state.api().await?;
        let content = api
            .file
            .read_file(FileIdentifier::CID(hash.clone()))
            .await?;

        Ok(content)
    }
}

/// Saves a file on the local system
#[tauri::command]
pub async fn save_file_locally(
    api_state: ApiAccess<'_>,
    id: i64,
    path: String,
) -> PluginResult<()> {
    let api = api_state.api().await?;
    let content = api.file.read_file(FileIdentifier::ID(id)).await?;
    fs::write(PathBuf::from(path), content).await?;

    Ok(())
}

#[tauri::command]
pub async fn delete_thumbnails(api_state: ApiAccess<'_>, id: i64) -> PluginResult<()> {
    let api = api_state.api().await?;
    api.file.delete_thumbnails(FileIdentifier::ID(id)).await?;

    Ok(())
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
            for entry in subdir_entries.into_iter().filter(|e| !e.path().is_dir()) {
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

    let mut i = 0;

    while i < entries.len() {
        let entry = &entries[i];

        if entry.path().is_dir() {
            let mut read_dir = fs::read_dir(entry.path()).await?;
            while let Some(entry) = read_dir.next_entry().await? {
                entries.push(entry);
            }
        }
        i += 1;
    }

    Ok(entries)
}

/// Retrieves information about a path that MUST be a file and returns
/// metadata for it
#[tracing::instrument(level = "trace")]
async fn retrieve_file_information(path: PathBuf) -> PluginResult<FileOSMetadata> {
    let mime = mime_guess::from_path(&path).first();
    let metadata = fs::metadata(&path).await?;
    let creation_time = metadata.created().unwrap_or(SystemTime::now());
    let change_time = metadata.modified().unwrap_or(SystemTime::now());
    let name = path
        .file_name()
        .ok_or_else(|| "Could not retrieve file name")?;

    let os_metadata = FileOSMetadata {
        path: path.to_string_lossy().to_string(),
        name: name.to_string_lossy().to_string(),
        mime_type: mime.map(|m| m.to_string()),
        creation_time: system_time_to_naive_date_time(creation_time),
        change_time: system_time_to_naive_date_time(change_time),
    };

    Ok(os_metadata)
}
