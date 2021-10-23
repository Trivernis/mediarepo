use crate::types::requests::{
    AddFileRequest, FindFilesByTagsRequest, GetFileThumbnailsRequest, ReadFileRequest,
};
use crate::types::responses::{FileResponse, ThumbnailResponse};
use crate::utils::{file_by_identifier, get_repo_from_context};
use mediarepo_core::error::RepoError;
use mediarepo_core::rmp_ipc::prelude::*;
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
            "find_files" => Self::find_files,
            "add_file" => Self::add_file,
            "read_file" => Self::read_file,
            "get_thumbnails" => Self::thumbnails,
            "read_thumbnail" => Self::read_thumbnail
        );
    }
}

impl FilesNamespace {
    /// Returns a list of all files
    #[tracing::instrument(skip_all)]
    async fn all_files(ctx: &Context, event: Event) -> IPCResult<()> {
        let repo = get_repo_from_context(ctx).await;
        let files = repo.files().await?;
        let responses: Vec<FileResponse> = files.into_iter().map(FileResponse::from).collect();

        ctx.emitter
            .emit_response_to(event.id(), Self::name(), "all_files", responses)
            .await?;

        Ok(())
    }

    /// Searches for files by tags
    #[tracing::instrument(skip_all)]
    async fn find_files(ctx: &Context, event: Event) -> IPCResult<()> {
        let tags = event.data::<FindFilesByTagsRequest>()?;
        let repo = get_repo_from_context(ctx).await;
        let files = repo.find_files_by_tags(tags.tags).await?;
        let responses: Vec<FileResponse> = files.into_iter().map(FileResponse::from).collect();
        ctx.emitter
            .emit_response_to(event.id(), Self::name(), "find_files", responses)
            .await?;
        Ok(())
    }

    /// Adds a file to the repository
    #[tracing::instrument(skip_all)]
    async fn add_file(ctx: &Context, event: Event) -> IPCResult<()> {
        let request = event.data::<AddFileRequest>()?;
        let path = PathBuf::from(request.path);
        let repo = get_repo_from_context(ctx).await;
        let file = repo.add_file_by_path(path).await?;

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
    #[tracing::instrument(skip_all)]
    async fn read_file(ctx: &Context, event: Event) -> IPCResult<()> {
        let request = event.data::<ReadFileRequest>()?;

        let repo = get_repo_from_context(ctx).await;
        let file = file_by_identifier(request, &repo).await?;
        let mut reader = file.get_reader().await?;
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf).await?;

        ctx.emitter
            .emit_response_to(event.id(), Self::name(), "read_file", BytePayload::new(buf))
            .await?;

        Ok(())
    }

    /// Returns a list of available thumbnails of a file
    #[tracing::instrument(skip_all)]
    async fn thumbnails(ctx: &Context, event: Event) -> IPCResult<()> {
        let request = event.data::<GetFileThumbnailsRequest>()?;
        let repo = get_repo_from_context(ctx).await;
        let file = file_by_identifier(request, &repo).await?;
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
    #[tracing::instrument(skip_all)]
    async fn read_thumbnail(ctx: &Context, event: Event) -> IPCResult<()> {
        let hash = event.data::<String>()?;
        let repo = get_repo_from_context(ctx).await;
        let thumbnail = repo
            .thumbnail_by_hash(&hash)
            .await?
            .ok_or_else(|| RepoError::from("Thumbnail not found"))?;
        let mut reader = thumbnail.get_reader().await?;
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf).await?;
        ctx.emitter
            .emit_response_to(
                event.id(),
                Self::name(),
                "read_thumbnail",
                BytePayload::new(buf),
            )
            .await?;

        Ok(())
    }
}
