use tokio::io::AsyncReadExt;

use mediarepo_core::bromine::prelude::*;
use mediarepo_core::content_descriptor::{create_content_descriptor, encode_content_descriptor};
use mediarepo_core::error::RepoError;
use mediarepo_core::fs::thumbnail_store::Dimensions;
use mediarepo_core::itertools::Itertools;
use mediarepo_core::mediarepo_api::types::files::{
    AddFileRequestHeader, FileBasicDataResponse, FileMetadataResponse,
    GetFileThumbnailOfSizeRequest, GetFileThumbnailsRequest, ReadFileRequest,
    ThumbnailMetadataResponse, UpdateFileNameRequest, UpdateFileStatusRequest,
};
use mediarepo_core::mediarepo_api::types::filtering::FindFilesRequest;
use mediarepo_core::mediarepo_api::types::identifier::FileIdentifier;
use mediarepo_core::thumbnailer::ThumbnailSize;
use mediarepo_core::utils::parse_namespace_and_tag;
use mediarepo_logic::dao::DaoProvider;
use mediarepo_logic::dto::{AddFileDto, UpdateFileDto, UpdateFileMetadataDto};

use crate::from_model::FromModel;
use crate::namespaces::files::searching::find_files_for_filters;
use crate::namespaces::files::sorting::sort_files_by_properties;
use crate::utils::{cd_by_identifier, file_by_identifier, get_repo_from_context};

mod searching;
mod sorting;

pub struct FilesNamespace;

impl NamespaceProvider for FilesNamespace {
    fn name() -> &'static str {
        "files"
    }

    fn register(handler: &mut EventHandler) {
        events!(handler,
            "all_files" => Self::all_files,
            "get_file" => Self::get_file,
            "get_file_metadata" => Self::get_file_metadata,
            "get_files" => Self::get_files,
            "find_files" => Self::find_files,
            "add_file" => Self::add_file,
            "read_file" => Self::read_file,
            "get_thumbnails" => Self::thumbnails,
            "get_thumbnail_of_size" => Self::get_thumbnail_of_size,
            "update_file_name" => Self::update_file_name,
            "delete_thumbnails" => Self::delete_thumbnails,
            "update_file_status" => Self::update_status,
            "delete_file" => Self::delete_file
        );
    }
}

impl FilesNamespace {
    /// Returns a list of all files
    #[tracing::instrument(skip_all)]
    async fn all_files(ctx: &Context, _event: Event) -> IPCResult<()> {
        let repo = get_repo_from_context(ctx).await;
        let files = repo.file().all().await?;

        let responses: Vec<FileBasicDataResponse> = files
            .into_iter()
            .map(FileBasicDataResponse::from_model)
            .collect();

        ctx.emit_to(Self::name(), "all_files", responses).await?;

        Ok(())
    }

    /// Returns a file by id
    #[tracing::instrument(skip_all)]
    async fn get_file(ctx: &Context, event: Event) -> IPCResult<()> {
        let id = event.payload::<FileIdentifier>()?;
        let repo = get_repo_from_context(ctx).await;
        let file = file_by_identifier(id, &repo).await?;
        let response = FileBasicDataResponse::from_model(file);
        ctx.emit_to(Self::name(), "get_file", response).await?;

        Ok(())
    }

    /// Returns metadata for a given file
    #[tracing::instrument(skip_all)]
    async fn get_file_metadata(ctx: &Context, event: Event) -> IPCResult<()> {
        let id = event.payload::<FileIdentifier>()?;
        let repo = get_repo_from_context(ctx).await;
        let file = file_by_identifier(id, &repo).await?;
        let file_id = file.id();

        let metadata = if let Some(metadata) = file.into_metadata() {
            metadata
        } else {
            repo.file()
                .metadata(file_id)
                .await?
                .ok_or_else(|| RepoError::from("file metadata not found"))?
        };

        ctx.emit_to(
            Self::name(),
            "get_file_metadata",
            FileMetadataResponse::from_model(metadata),
        )
        .await?;

        Ok(())
    }

    /// Returns a list of files by identifier
    #[tracing::instrument(skip_all)]
    async fn get_files(ctx: &Context, event: Event) -> IPCResult<()> {
        let ids = event.payload::<Vec<FileIdentifier>>()?;
        let repo = get_repo_from_context(ctx).await;
        let mut responses = Vec::new();

        for id in ids {
            responses.push(
                file_by_identifier(id, &repo)
                    .await
                    .map(FileBasicDataResponse::from_model)?,
            );
        }
        ctx.emit_to(Self::name(), "get_files", responses).await?;

        Ok(())
    }

    /// Searches for files by tags
    #[tracing::instrument(skip_all)]
    async fn find_files(ctx: &Context, event: Event) -> IPCResult<()> {
        let req = event.payload::<FindFilesRequest>()?;
        let repo = get_repo_from_context(ctx).await;

        let mut files = find_files_for_filters(&repo, req.filters).await?;
        sort_files_by_properties(&repo, req.sort_expression, &mut files).await?;

        let responses: Vec<FileBasicDataResponse> = files
            .into_iter()
            .map(FileBasicDataResponse::from_model)
            .collect();
        ctx.emit_to(Self::name(), "find_files", responses).await?;
        Ok(())
    }

    /// Adds a file to the repository
    #[tracing::instrument(skip_all)]
    async fn add_file(ctx: &Context, event: Event) -> IPCResult<()> {
        let (request, bytes) = event
            .payload::<TandemPayload<AddFileRequestHeader, BytePayload>>()?
            .into_inner();
        let AddFileRequestHeader { metadata, tags } = request;
        let repo = get_repo_from_context(ctx).await;
        let bytes = bytes.into_inner();
        let cd = create_content_descriptor(&bytes);

        let file = if let Some(file) = repo.file().by_cd(cd).await? {
            tracing::debug!("Inserted file already exists");
            file
        } else {
            let add_dto = AddFileDto {
                content: bytes,
                mime_type: metadata
                    .mime_type
                    .unwrap_or(String::from("application/octet-stream")),
                creation_time: metadata.creation_time,
                change_time: metadata.change_time,
                name: Some(metadata.name),
            };
            repo.file().add(add_dto).await?
        };

        let tags = repo
            .add_all_tags(tags.into_iter().map(parse_namespace_and_tag).collect())
            .await?;
        let tag_ids: Vec<i64> = tags.into_iter().map(|t| t.id()).unique().collect();
        repo.tag()
            .upsert_mappings(vec![file.cd_id()], tag_ids)
            .await?;

        ctx.emit_to(
            Self::name(),
            "add_file",
            FileBasicDataResponse::from_model(file),
        )
        .await?;

        Ok(())
    }

    #[tracing::instrument(skip_all)]
    async fn update_status(ctx: &Context, event: Event) -> IPCResult<()> {
        let request = event.payload::<UpdateFileStatusRequest>()?;
        let repo = get_repo_from_context(ctx).await;
        let mut file = file_by_identifier(request.file_id, &repo).await?;
        file = repo
            .file()
            .update(UpdateFileDto {
                id: file.id(),
                status: Some(request.status.into()),
                ..Default::default()
            })
            .await?;
        ctx.emit_to(
            Self::name(),
            "update_file_status",
            FileBasicDataResponse::from_model(file),
        )
        .await?;

        Ok(())
    }

    /// Reads the binary contents of a file
    #[tracing::instrument(skip_all)]
    async fn read_file(ctx: &Context, event: Event) -> IPCResult<()> {
        let request = event.payload::<ReadFileRequest>()?;
        let repo = get_repo_from_context(ctx).await;
        let file = file_by_identifier(request.id, &repo).await?;
        let bytes = repo.file().get_bytes(file.cd()).await?;

        ctx.emit_to(Self::name(), "read_file", BytePayload::new(bytes))
            .await?;

        Ok(())
    }

    /// Deletes a file
    #[tracing::instrument(skip_all)]
    async fn delete_file(ctx: &Context, event: Event) -> IPCResult<()> {
        let id = event.payload::<FileIdentifier>()?;
        let repo = get_repo_from_context(ctx).await;
        let file = file_by_identifier(id, &repo).await?;
        repo.file().delete(file).await?;

        ctx.emit_to(Self::name(), "delete_file", ()).await?;

        Ok(())
    }

    /// Returns a list of available thumbnails of a file
    #[tracing::instrument(skip_all)]
    async fn thumbnails(ctx: &Context, event: Event) -> IPCResult<()> {
        let request = event.payload::<GetFileThumbnailsRequest>()?;
        let repo = get_repo_from_context(ctx).await;
        let file_cd = cd_by_identifier(request.id.clone(), &repo).await?;
        let mut thumbnails = repo
            .file()
            .thumbnails(encode_content_descriptor(&file_cd))
            .await?;

        if thumbnails.is_empty() {
            tracing::debug!("No thumbnails for file found. Creating thumbnails...");
            let file = file_by_identifier(request.id, &repo).await?;
            thumbnails = repo
                .file()
                .create_thumbnails(file, vec![ThumbnailSize::Medium])
                .await?;
            tracing::debug!("Thumbnails for file created.");
        }

        let thumb_responses: Vec<ThumbnailMetadataResponse> = thumbnails
            .into_iter()
            .map(ThumbnailMetadataResponse::from_model)
            .collect();
        ctx.emit_to(Self::name(), "get_thumbnails", thumb_responses)
            .await?;

        Ok(())
    }

    /// Returns a thumbnail that is within the range of the requested sizes
    #[tracing::instrument(skip_all)]
    async fn get_thumbnail_of_size(ctx: &Context, event: Event) -> IPCResult<()> {
        let request = event.payload::<GetFileThumbnailOfSizeRequest>()?;
        let repo = get_repo_from_context(ctx).await;
        let file_cd = cd_by_identifier(request.id.clone(), &repo).await?;
        let min_size = request.min_size;
        let max_size = request.max_size;
        let thumbnails = repo
            .file()
            .thumbnails(encode_content_descriptor(&file_cd))
            .await?;

        let found_thumbnail = thumbnails.into_iter().find(|thumb| {
            let Dimensions { height, width } = thumb.size();

            *height >= min_size.0
                && *height <= max_size.0
                && *width >= min_size.1
                && *width <= max_size.1
        });

        let thumbnail = if let Some(thumbnail) = found_thumbnail {
            thumbnail
        } else {
            let file = file_by_identifier(request.id, &repo).await?;
            let middle_size = ((max_size.0 + min_size.0) / 2, (max_size.1 + min_size.1) / 2);
            let thumbnail = repo
                .file()
                .create_thumbnails(file, vec![ThumbnailSize::Custom(middle_size)])
                .await?;

            thumbnail
                .into_iter()
                .next()
                .ok_or_else(|| RepoError::from("thumbnail could not be created"))?
        };
        let mut buf = Vec::new();
        thumbnail.get_reader().await?.read_to_end(&mut buf).await?;
        let byte_payload = BytePayload::new(buf);
        let thumb_payload = ThumbnailMetadataResponse::from_model(thumbnail);
        ctx.emit_to(
            Self::name(),
            "get_thumbnail_of_size",
            TandemPayload::new(thumb_payload, byte_payload),
        )
        .await?;

        Ok(())
    }

    /// Updates the name of a file
    #[tracing::instrument(skip_all)]
    async fn update_file_name(ctx: &Context, event: Event) -> IPCResult<()> {
        let repo = get_repo_from_context(ctx).await;
        let request = event.payload::<UpdateFileNameRequest>()?;
        let file = file_by_identifier(request.file_id, &repo).await?;

        let metadata = repo
            .file()
            .update_metadata(UpdateFileMetadataDto {
                file_id: file.id(),
                name: Some(Some(request.name)),
                ..Default::default()
            })
            .await?;

        ctx.emit_to(
            Self::name(),
            "update_file_name",
            FileMetadataResponse::from_model(metadata),
        )
        .await?;

        Ok(())
    }

    /// Deletes all thumbnails of a file
    #[tracing::instrument(skip_all)]
    async fn delete_thumbnails(ctx: &Context, event: Event) -> IPCResult<()> {
        let repo = get_repo_from_context(ctx).await;
        let id = event.payload::<FileIdentifier>()?;
        let file = file_by_identifier(id, &repo).await?;
        let thumbnails = repo.file().thumbnails(file.encoded_cd()).await?;

        for thumb in thumbnails {
            thumb.delete().await?;
        }

        Ok(())
    }
}
