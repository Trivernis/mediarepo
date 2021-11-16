use crate::from_model::FromModel;
use crate::utils::{file_by_identifier, get_repo_from_context, hash_by_identifier};
use chrono::NaiveDateTime;
use compare::Compare;
use mediarepo_api::types::files::{
    AddFileRequestHeader, FileMetadataResponse, FindFilesByTagsRequest,
    GetFileThumbnailOfSizeRequest, GetFileThumbnailsRequest, ReadFileRequest, SortDirection,
    SortKey, ThumbnailMetadataResponse, UpdateFileNameRequest,
};
use mediarepo_api::types::identifier::FileIdentifier;
use mediarepo_core::fs::thumbnail_store::Dimensions;
use mediarepo_core::itertools::Itertools;
use mediarepo_core::rmp_ipc::prelude::*;
use mediarepo_core::thumbnailer::ThumbnailSize;
use mediarepo_core::utils::parse_namespace_and_tag;
use mediarepo_database::queries::tags::{
    get_hashes_with_namespaced_tags, get_hashes_with_tag_count,
};
use std::cmp::Ordering;
use std::collections::HashMap;
use tokio::io::AsyncReadExt;

pub struct FilesNamespace;
pub struct FileSortContext {
    name: Option<String>,
    size: u64,
    mime_type: Option<String>,
    namespaces: HashMap<String, Vec<String>>,
    tag_count: u32,
    import_time: NaiveDateTime,
    create_time: NaiveDateTime,
    change_time: NaiveDateTime,
}

impl NamespaceProvider for FilesNamespace {
    fn name() -> &'static str {
        "files"
    }

    fn register<S: AsyncProtocolStream>(handler: &mut EventHandler<S>) {
        events!(handler,
            "all_files" => Self::all_files,
            "get_file" => Self::get_file,
            "find_files" => Self::find_files,
            "add_file" => Self::add_file,
            "read_file" => Self::read_file,
            "get_thumbnails" => Self::thumbnails,
            "get_thumbnail_of_size" => Self::get_thumbnail_of_size,
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

    /// Returns a file by id
    #[tracing::instrument(skip_all)]
    async fn get_file<S: AsyncProtocolStream>(ctx: &Context<S>, event: Event) -> IPCResult<()> {
        let id = event.data::<FileIdentifier>()?;
        let repo = get_repo_from_context(ctx).await;
        let file = file_by_identifier(id, &repo).await?;
        let response = FileMetadataResponse::from_model(file);
        ctx.emitter
            .emit_response_to(event.id(), Self::name(), "get_file", response)
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
        let hash_ids: Vec<i64> = files.iter().map(|f| f.hash_id()).collect();

        let mut hash_nsp: HashMap<i64, HashMap<String, Vec<String>>> =
            get_hashes_with_namespaced_tags(repo.db(), hash_ids.clone()).await?;
        let mut hash_tag_counts = get_hashes_with_tag_count(repo.db(), hash_ids).await?;

        let mut contexts = HashMap::new();

        for file in &files {
            let context = FileSortContext {
                name: file.name().to_owned(),
                size: file.get_size().await?,
                mime_type: file.mime_type().to_owned(),
                namespaces: hash_nsp
                    .remove(&file.hash_id())
                    .unwrap_or(HashMap::with_capacity(0)),
                tag_count: hash_tag_counts.remove(&file.hash_id()).unwrap_or(0),
                import_time: file.import_time().to_owned(),
                create_time: file.import_time().to_owned(),
                change_time: file.change_time().to_owned(),
            };
            contexts.insert(file.id(), context);
        }
        let sort_expression = req.sort_expression;
        tracing::debug!("sort_expression = {:?}", sort_expression);

        files.sort_by(|a, b| {
            compare_files(
                contexts.get(&a.id()).unwrap(),
                contexts.get(&b.id()).unwrap(),
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
        let (request, bytes) = event
            .data::<TandemPayload<AddFileRequestHeader, BytePayload>>()?
            .into_inner();
        let AddFileRequestHeader { metadata, tags } = request;
        let repo = get_repo_from_context(ctx).await;

        let mut file = repo
            .add_file(
                metadata.mime_type,
                bytes.into_inner(),
                metadata.creation_time,
                metadata.change_time,
            )
            .await?;
        file.set_name(metadata.name).await?;

        let tags = repo
            .add_all_tags(tags.into_iter().map(parse_namespace_and_tag).collect())
            .await?;
        let tag_ids: Vec<i64> = tags.into_iter().map(|t| t.id()).unique().collect();
        file.add_tags(tag_ids).await?;

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
        let file_hash = hash_by_identifier(request.id.clone(), &repo).await?;
        let mut thumbnails = repo.get_file_thumbnails(file_hash).await?;

        if thumbnails.is_empty() {
            tracing::debug!("No thumbnails for file found. Creating thumbnails...");
            let file = file_by_identifier(request.id, &repo).await?;
            thumbnails = repo.create_thumbnails_for_file(&file).await?;
            tracing::debug!("Thumbnails for file created.");
        }

        let thumb_responses: Vec<ThumbnailMetadataResponse> = thumbnails
            .into_iter()
            .map(ThumbnailMetadataResponse::from_model)
            .collect();
        ctx.emitter
            .emit_response_to(event.id(), Self::name(), "get_thumbnails", thumb_responses)
            .await?;

        Ok(())
    }

    /// Returns a thumbnail that is within the range of the requested sizes
    #[tracing::instrument(skip_all)]
    async fn get_thumbnail_of_size<S: AsyncProtocolStream>(
        ctx: &Context<S>,
        event: Event,
    ) -> IPCResult<()> {
        let request = event.data::<GetFileThumbnailOfSizeRequest>()?;
        let repo = get_repo_from_context(ctx).await;
        let file_hash = hash_by_identifier(request.id.clone(), &repo).await?;
        let thumbnails = repo.get_file_thumbnails(file_hash).await?;
        let min_size = request.min_size;
        let max_size = request.max_size;

        let found_thumbnail = thumbnails.into_iter().find(|thumb| {
            let Dimensions { height, width } = thumb.size;

            height >= min_size.0
                && height <= max_size.0
                && width >= min_size.1
                && width <= max_size.1
        });

        let thumbnail = if let Some(thumbnail) = found_thumbnail {
            thumbnail
        } else {
            let file = file_by_identifier(request.id, &repo).await?;
            let middle_size = ((max_size.0 + min_size.0) / 2, (max_size.1 + min_size.1) / 2);
            let thumbnail = repo
                .create_file_thumbnail(&file, ThumbnailSize::Custom(middle_size))
                .await?;

            thumbnail
        };
        let mut buf = Vec::new();
        thumbnail.get_reader().await?.read_to_end(&mut buf).await?;
        let byte_payload = BytePayload::new(buf);
        let thumb_payload = ThumbnailMetadataResponse::from_model(thumbnail);
        ctx.emitter
            .emit_response_to(
                event.id(),
                Self::name(),
                "get_thumbnail_of_size",
                TandemPayload::new(thumb_payload, byte_payload),
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
    ctx_a: &FileSortContext,
    ctx_b: &FileSortContext,
    expression: &Vec<SortKey>,
) -> Ordering {
    let cmp_date = compare::natural();
    let cmp_u64 = compare::natural();
    let cmp_u32 = compare::natural();

    for sort_key in expression {
        let ordering = match sort_key {
            SortKey::Namespace(namespace) => {
                let list_a = ctx_a.namespaces.get(&namespace.name);
                let list_b = ctx_b.namespaces.get(&namespace.name);

                let cmp_result = if let (Some(list_a), Some(list_b)) = (list_a, list_b) {
                    compare_tag_lists(list_a, list_b)
                } else if list_a.is_some() {
                    Ordering::Greater
                } else if list_b.is_some() {
                    Ordering::Less
                } else {
                    Ordering::Equal
                };
                adjust_for_dir(cmp_result, &namespace.direction)
            }
            SortKey::FileName(direction) => {
                adjust_for_dir(compare_opts(&ctx_a.name, &ctx_b.name), direction)
            }
            SortKey::FileSize(direction) => {
                adjust_for_dir(cmp_u64.compare(&ctx_a.size, &ctx_b.size), direction)
            }
            SortKey::FileImportedTime(direction) => adjust_for_dir(
                cmp_date.compare(&ctx_a.import_time, &ctx_b.import_time),
                direction,
            ),
            SortKey::FileCreatedTime(direction) => adjust_for_dir(
                cmp_date.compare(&ctx_a.create_time, &ctx_b.create_time),
                direction,
            ),
            SortKey::FileChangeTime(direction) => adjust_for_dir(
                cmp_date.compare(&ctx_a.change_time, &ctx_b.change_time),
                direction,
            ),
            SortKey::FileType(direction) => {
                adjust_for_dir(compare_opts(&ctx_a.mime_type, &ctx_b.mime_type), direction)
            }
            SortKey::NumTags(direction) => adjust_for_dir(
                cmp_u32.compare(&ctx_a.tag_count, &ctx_b.tag_count),
                direction,
            ),
        };
        if !ordering.is_eq() {
            return ordering;
        }
    }

    Ordering::Equal
}

fn compare_opts<T: Ord + Sized>(opt_a: &Option<T>, opt_b: &Option<T>) -> Ordering {
    let cmp = compare::natural();
    if let (Some(a), Some(b)) = (opt_a, opt_b) {
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

fn compare_tag_lists(list_a: &Vec<String>, list_b: &Vec<String>) -> Ordering {
    let first_diff = list_a
        .into_iter()
        .zip(list_b.into_iter())
        .find(|(a, b)| *a != *b);
    if let Some(diff) = first_diff {
        if let (Some(num_a), Some(num_b)) = (diff.0.parse::<f32>().ok(), diff.1.parse::<f32>().ok())
        {
            compare_f32(num_a, num_b)
        } else {
            let cmp = compare::natural();
            cmp.compare(diff.0, diff.1)
        }
    } else {
        Ordering::Equal
    }
}
