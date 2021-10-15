use crate::types::requests::{
    AddFileRequest, FileIdentifier, GetFileThumbnailsRequest, ReadFileRequest,
};
use crate::types::responses::FileResponse;
use mediarepo_core::error::{RepoError, RepoResult};
use mediarepo_model::file::File;
use mediarepo_model::repo::Repo;
use mediarepo_model::type_keys::RepoKey;
use rmp_ipc::context::Context;
use rmp_ipc::error::Result;
use rmp_ipc::{Event, NamespaceBuilder};
use std::path::PathBuf;
use tokio::io::AsyncReadExt;

pub const FILES_NAMESPACE: &str = "files";

pub fn build(builder: NamespaceBuilder) -> NamespaceBuilder {
    builder
        .on("all_files", |c, e| Box::pin(all_files(c, e)))
        .on("add_file", |c, e| Box::pin(add_file(c, e)))
        .on("read_file", |c, e| Box::pin(read_file(c, e)))
        .on("get_thumbnails", |c, e| Box::pin(get_file_thumbnails(c, e)))
        .on("read_thumbnail", |c, e| Box::pin(read_thumbnail(c, e)))
}

/// Returns a list of all files
async fn all_files(ctx: &Context, event: Event) -> Result<()> {
    let files = {
        let data = ctx.data.read().await;
        let repo = data.get::<RepoKey>().unwrap();
        repo.files().await?
    };
    let responses: Vec<FileResponse> = files.into_iter().map(FileResponse::from).collect();
    ctx.emitter
        .emit_response_to(event.id(), FILES_NAMESPACE, "all_files", responses)
        .await?;

    Ok(())
}

/// Adds a file to the repository
async fn add_file(ctx: &Context, event: Event) -> Result<()> {
    let request = event.data::<AddFileRequest>()?;
    let path = PathBuf::from(request.path);
    let file = {
        let data = ctx.data.read().await;
        let repo = data.get::<RepoKey>().unwrap();
        repo.add_file_by_path(path).await?
    };
    ctx.emitter
        .emit_response_to(
            event.id(),
            FILES_NAMESPACE,
            "add_file",
            FileResponse::from(file),
        )
        .await?;

    Ok(())
}

/// Reads the binary contents of a file
async fn read_file(ctx: &Context, event: Event) -> Result<()> {
    let request = event.data::<ReadFileRequest>()?;
    let mut reader = {
        let data = ctx.data.read().await;
        let repo = data.get::<RepoKey>().unwrap();
        let file = file_by_identifier(request, repo)
            .await?
            .ok_or_else(|| RepoError::from("File not found"));
        file?.get_reader().await?
    };
    let mut buf = Vec::new();
    reader.read_to_end(&mut buf).await?;
    ctx.emitter
        .emit_response_to(event.id(), FILES_NAMESPACE, "read_file", buf)
        .await?;

    Ok(())
}

/// Returns a list of available thumbnails of a file
async fn get_file_thumbnails(ctx: &Context, event: Event) -> Result<()> {
    let request = event.data::<GetFileThumbnailsRequest>()?;
    let data = ctx.data.read().await;
    let repo = data.get::<RepoKey>().unwrap();
    let file = file_by_identifier(request, repo)
        .await?
        .ok_or_else(|| RepoError::from("File not found"))?;
    let thumbnails = file.thumbnails().await?;
    let thumb_hashes: Vec<String> = thumbnails
        .into_iter()
        .map(|thumb| thumb.hash().clone())
        .collect();
    ctx.emitter
        .emit_response_to(event.id(), FILES_NAMESPACE, "get_thumbnails", thumb_hashes)
        .await?;

    Ok(())
}

/// Reads a thumbnail for the given thumbnail hash
async fn read_thumbnail(ctx: &Context, event: Event) -> Result<()> {
    let hash = event.data::<String>()?;
    let mut reader = {
        let data = ctx.data.read().await;
        let repo = data.get::<RepoKey>().unwrap();
        let thumbnail = repo
            .thumbnail_by_hash(&hash)
            .await?
            .ok_or_else(|| RepoError::from("Thumbnail not found"))?;
        thumbnail.get_reader().await?
    };
    let mut buf = Vec::new();
    reader.read_to_end(&mut buf).await?;
    ctx.emitter
        .emit_response_to(event.id(), FILES_NAMESPACE, "read_thumbnail", buf)
        .await?;

    Ok(())
}

async fn file_by_identifier(identifier: FileIdentifier, repo: &Repo) -> RepoResult<Option<File>> {
    match identifier {
        FileIdentifier::ID(id) => repo.file_by_id(id).await,
        FileIdentifier::Hash(hash) => repo.file_by_hash(hash).await,
    }
}
