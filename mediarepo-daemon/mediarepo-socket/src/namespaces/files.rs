use crate::types::requests::{
    AddFileRequest, FileIdentifier, GetFileThumbnailsRequest, ReadFileRequest,
};
use crate::types::responses::{FileResponse, ThumbnailResponse};
use mediarepo_core::error::{RepoError, RepoResult};
use mediarepo_core::rmp_ipc::prelude::*;
use mediarepo_model::file::File;
use mediarepo_model::repo::Repo;
use mediarepo_model::type_keys::RepoKey;
use std::path::PathBuf;
use tokio::io::AsyncReadExt;

pub struct FilesNamespace;

impl NamespaceProvider for FilesNamespace {
    fn name() -> &'static str {
        "files"
    }

    fn register(handler: &mut EventHandler) {
        events!(handler,
            "all_files" => Self::all_files,
            "add_file" => Self::add_file,
            "read_file" => Self::read_file,
            "get_thumbnails" => Self::thumbnails,
            "read_thumbnail" => Self::read_thumbnail
        );
    }
}

impl FilesNamespace {
    /// Returns a list of all files
    async fn all_files(ctx: &Context, event: Event) -> IPCResult<()> {
        let files = {
            let data = ctx.data.read().await;
            let repo = data.get::<RepoKey>().unwrap();
            repo.files().await?
        };
        let responses: Vec<FileResponse> = files.into_iter().map(FileResponse::from).collect();
        ctx.emitter
            .emit_response_to(event.id(), Self::name(), "all_files", responses)
            .await?;

        Ok(())
    }

    /// Adds a file to the repository
    async fn add_file(ctx: &Context, event: Event) -> IPCResult<()> {
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
                Self::name(),
                "add_file",
                FileResponse::from(file),
            )
            .await?;

        Ok(())
    }

    /// Reads the binary contents of a file
    async fn read_file(ctx: &Context, event: Event) -> IPCResult<()> {
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
            .emit_response_to(event.id(), Self::name(), "read_file", buf)
            .await?;

        Ok(())
    }

    /// Returns a list of available thumbnails of a file
    async fn thumbnails(ctx: &Context, event: Event) -> IPCResult<()> {
        let request = event.data::<GetFileThumbnailsRequest>()?;
        let data = ctx.data.read().await;
        let repo = data.get::<RepoKey>().unwrap();
        let file = file_by_identifier(request, repo)
            .await?
            .ok_or_else(|| RepoError::from("File not found"))?;
        let thumbnails = file.thumbnails().await?;
        let thumb_responses: Vec<ThumbnailResponse> = thumbnails
            .into_iter()
            .map(ThumbnailResponse::from)
            .collect();
        ctx.emitter
            .emit_response_to(event.id(), Self::name(), "get_thumbnails", thumb_responses)
            .await?;

        Ok(())
    }

    /// Reads a thumbnail for the given thumbnail hash
    async fn read_thumbnail(ctx: &Context, event: Event) -> IPCResult<()> {
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
            .emit_response_to(event.id(), Self::name(), "read_thumbnail", buf)
            .await?;

        Ok(())
    }
}

async fn file_by_identifier(identifier: FileIdentifier, repo: &Repo) -> RepoResult<Option<File>> {
    match identifier {
        FileIdentifier::ID(id) => repo.file_by_id(id).await,
        FileIdentifier::Hash(hash) => repo.file_by_hash(hash).await,
    }
}
