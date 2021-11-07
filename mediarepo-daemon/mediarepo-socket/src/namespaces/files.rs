use crate::from_model::FromModel;
use crate::utils::{file_by_identifier, get_repo_from_context};
use compare::Compare;
use mediarepo_api::types::files::{
    AddFileRequest, FileMetadataResponse, FindFilesByTagsRequest, GetFileThumbnailsRequest,
    ReadFileRequest, SortDirection, SortKey, ThumbnailMetadataResponse, UpdateFileNameRequest,
};
use mediarepo_core::error::RepoError;
use mediarepo_core::rmp_ipc::prelude::*;
use mediarepo_database::queries::tags::get_hashes_with_namespaced_tags;
use mediarepo_model::file::File;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::path::PathBuf;
use tokio::io::AsyncReadExt;

pub struct FilesNamespace;

impl NamespaceProvider for FilesNamespace {
    fn name() -> &'static str {
        "files"
    }

    fn register<S: AsyncProtocolStream>(handler: &mut EventHandler<S>) {
        events!(handler,
            "all_files" => Self::all_files,
            "find_files" => Self::find_files,
            "add_file" => Self::add_file,
            "read_file" => Self::read_file,
            "get_thumbnails" => Self::thumbnails,
            "read_thumbnail" => Self::read_thumbnail,
            "update_file_name" => Self::update_file_name
        );
    }
}

impl FilesNamespace {
    /// Returns a list of all files
    #[tracing::instrument(skip_all)]
    async fn all_files<S: AsyncProtocolStream>(ctx: &Context<S>, event: Event) -> IPCResult<()> {
        let repo = get_repo_from_context(ctx).await;
        let files = repo.files().await?;

        let responses: Vec<FileMetadataResponse> = files
            .into_iter()
            .map(FileMetadataResponse::from_model)
            .collect();

        ctx.emitter
            .emit_response_to(event.id(), Self::name(), "all_files", responses)
            .await?;

        Ok(())
    }

    /// Searches for files by tags
    #[tracing::instrument(skip_all)]
    async fn find_files<S: AsyncProtocolStream>(ctx: &Context<S>, event: Event) -> IPCResult<()> {
        let req = event.data::<FindFilesByTagsRequest>()?;
        let repo = get_repo_from_context(ctx).await;
        let tags = req.tags.into_iter().map(|t| (t.name, t.negate)).collect();
        let mut files = repo.find_files_by_tags(tags).await?;
        let hash_ids = files.iter().map(|f| f.hash_id()).collect();

        let hash_nsp: HashMap<i64, HashMap<String, String>> =
            get_hashes_with_namespaced_tags(repo.db(), hash_ids).await?;

        let sort_expression = req.sort_expression;
        tracing::debug!("sort_expression = {:?}", sort_expression);
        let empty_map = HashMap::with_capacity(0);

        files.sort_by(|a, b| {
            compare_files(
                a,
                hash_nsp.get(&a.hash_id()).unwrap_or(&empty_map),
                b,
                hash_nsp.get(&b.hash_id()).unwrap_or(&empty_map),
                &sort_expression,
            )
        });

        let responses: Vec<FileMetadataResponse> = files
            .into_iter()
            .map(FileMetadataResponse::from_model)
            .collect();
        ctx.emitter
            .emit_response_to(event.id(), Self::name(), "find_files", responses)
            .await?;
        Ok(())
    }

    /// Adds a file to the repository
    #[tracing::instrument(skip_all)]
    async fn add_file<S: AsyncProtocolStream>(ctx: &Context<S>, event: Event) -> IPCResult<()> {
        let request = event.data::<AddFileRequest>()?;
        let path = PathBuf::from(request.path);
        let repo = get_repo_from_context(ctx).await;
        let file = repo.add_file_by_path(path).await?;

        ctx.emitter
            .emit_response_to(
                event.id(),
                Self::name(),
                "add_file",
                FileMetadataResponse::from_model(file),
            )
            .await?;

        Ok(())
    }

    /// Reads the binary contents of a file
    #[tracing::instrument(skip_all)]
    async fn read_file<S: AsyncProtocolStream>(ctx: &Context<S>, event: Event) -> IPCResult<()> {
        let request = event.data::<ReadFileRequest>()?;

        let repo = get_repo_from_context(ctx).await;
        let file = file_by_identifier(request.id, &repo).await?;
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
    async fn thumbnails<S: AsyncProtocolStream>(ctx: &Context<S>, event: Event) -> IPCResult<()> {
        let request = event.data::<GetFileThumbnailsRequest>()?;
        let repo = get_repo_from_context(ctx).await;
        let file = file_by_identifier(request.id, &repo).await?;
        let mut thumbnails = file.thumbnails().await?;

        if thumbnails.len() == 0 {
            tracing::debug!("No thumbnails for file found. Creating thumbnails...");
            repo.create_thumbnails_for_file(file.clone()).await?;
            tracing::debug!("Thumbnails for file created.");
        }
        thumbnails = file.thumbnails().await?;

        let thumb_responses: Vec<ThumbnailMetadataResponse> = thumbnails
            .into_iter()
            .map(ThumbnailMetadataResponse::from_model)
            .collect();
        ctx.emitter
            .emit_response_to(event.id(), Self::name(), "get_thumbnails", thumb_responses)
            .await?;

        Ok(())
    }

    /// Reads a thumbnail for the given thumbnail hash
    #[tracing::instrument(skip_all)]
    async fn read_thumbnail<S: AsyncProtocolStream>(
        ctx: &Context<S>,
        event: Event,
    ) -> IPCResult<()> {
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

    /// Updates the name of a file
    #[tracing::instrument(skip_all)]
    async fn update_file_name<S: AsyncProtocolStream>(
        ctx: &Context<S>,
        event: Event,
    ) -> IPCResult<()> {
        let repo = get_repo_from_context(ctx).await;
        let request = event.data::<UpdateFileNameRequest>()?;
        let mut file = file_by_identifier(request.file_id, &repo).await?;
        file.set_name(request.name).await?;
        ctx.emitter
            .emit_response_to(
                event.id(),
                Self::name(),
                "update_file_name",
                FileMetadataResponse::from_model(file),
            )
            .await?;

        Ok(())
    }
}

#[tracing::instrument(level = "trace", skip_all)]
fn compare_files(
    file_a: &File,
    nsp_a: &HashMap<String, String>,
    file_b: &File,
    nsp_b: &HashMap<String, String>,
    expression: &Vec<SortKey>,
) -> Ordering {
    let cmp_date = compare::natural();

    for sort_key in expression {
        let ordering = match sort_key {
            SortKey::Namespace(namespace) => {
                let tag_a = nsp_a.get(&namespace.name);
                let tag_b = nsp_b.get(&namespace.name);

                if let (Some(a), Some(b)) = (
                    tag_a.and_then(|a| a.parse::<f32>().ok()),
                    tag_b.and_then(|b| b.parse::<f32>().ok()),
                ) {
                    adjust_for_dir(compare_f32(a, b), &namespace.direction)
                } else {
                    adjust_for_dir(compare_opts(tag_a, tag_b), &namespace.direction)
                }
            }
            SortKey::FileName(direction) => adjust_for_dir(
                compare_opts(file_a.name().clone(), file_b.name().clone()),
                direction,
            ),
            SortKey::FileSize(_direction) => {
                Ordering::Equal // TODO: Retrieve file size
            }
            SortKey::FileImportedTime(direction) => adjust_for_dir(
                cmp_date.compare(file_a.import_time(), file_b.import_time()),
                direction,
            ),
            SortKey::FileCreatedTime(direction) => adjust_for_dir(
                cmp_date.compare(file_a.creation_time(), file_b.creation_time()),
                direction,
            ),
            SortKey::FileChangeTime(direction) => adjust_for_dir(
                cmp_date.compare(file_a.change_time(), file_b.change_time()),
                direction,
            ),
            SortKey::FileType(direction) => adjust_for_dir(
                compare_opts(file_a.mime_type().clone(), file_b.mime_type().clone()),
                direction,
            ),
            SortKey::NumTags(_) => {
                Ordering::Equal // TODO: Count tags
            }
        };
        if !ordering.is_eq() {
            return ordering;
        }
    }

    Ordering::Equal
}

fn compare_opts<T: Ord + Sized>(opt_a: Option<T>, opt_b: Option<T>) -> Ordering {
    let cmp = compare::natural();
    if let (Some(a), Some(b)) = (&opt_a, &opt_b) {
        cmp.compare(a, b)
    } else if opt_a.is_some() {
        Ordering::Greater
    } else if opt_b.is_some() {
        Ordering::Less
    } else {
        Ordering::Equal
    }
}

fn compare_f32(a: f32, b: f32) -> Ordering {
    if a > b {
        Ordering::Greater
    } else if b > a {
        Ordering::Less
    } else {
        Ordering::Equal
    }
}

fn adjust_for_dir(ordering: Ordering, direction: &SortDirection) -> Ordering {
    if *direction == SortDirection::Descending {
        ordering.reverse()
    } else {
        ordering
    }
}
