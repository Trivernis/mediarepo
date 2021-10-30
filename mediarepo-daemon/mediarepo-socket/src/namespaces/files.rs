use crate::from_model::FromModel;
use crate::utils::{file_by_identifier, get_repo_from_context};
use compare::Compare;
use mediarepo_api::types::files::{
    AddFileRequest, FileMetadataResponse, FindFilesByTagsRequest, GetFileThumbnailsRequest,
    ReadFileRequest, SortDirection, SortKey, ThumbnailMetadataResponse,
};
use mediarepo_core::error::{RepoError, RepoResult};
use mediarepo_core::futures::future;
use mediarepo_core::rmp_ipc::prelude::*;
use mediarepo_model::file::File;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::iter::FromIterator;
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
    async fn find_files(ctx: &Context, event: Event) -> IPCResult<()> {
        let req = event.data::<FindFilesByTagsRequest>()?;
        let repo = get_repo_from_context(ctx).await;
        let tags = req.tags.into_iter().map(|t| (t.name, t.negate)).collect();
        let mut files = repo.find_files_by_tags(tags).await?;

        let files_nsp: HashMap<String, HashMap<String, String>> = HashMap::from_iter(
            future::join_all(files.iter().map(|f| {
                let file = f.clone();
                async move {
                    let result: RepoResult<(String, HashMap<String, String>)> =
                        Ok((f.hash().clone(), get_namespaces_for_file(&file).await?));
                    result
                }
            }))
            .await
            .into_iter()
            .filter_map(|r| match r {
                Ok(value) => Some(value),
                Err(e) => {
                    tracing::error!("{:?}", e);
                    None
                }
            }),
        );

        let sort_expression = req.sort_expression;

        files.sort_by(|a, b| {
            compare_files(
                a,
                files_nsp.get(a.hash()).unwrap(),
                b,
                files_nsp.get(b.hash()).unwrap(),
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
                FileMetadataResponse::from_model(file),
            )
            .await?;

        Ok(())
    }

    /// Reads the binary contents of a file
    #[tracing::instrument(skip_all)]
    async fn read_file(ctx: &Context, event: Event) -> IPCResult<()> {
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
    async fn thumbnails(ctx: &Context, event: Event) -> IPCResult<()> {
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
                let tag_a = nsp_a.get(&namespace.tag);
                let tag_b = nsp_b.get(&namespace.tag);

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

async fn get_namespaces_for_file(file: &File) -> RepoResult<HashMap<String, String>> {
    let tags = file.tags().await?;
    let namespaces: HashMap<String, String> =
        HashMap::from_iter(tags.into_iter().filter_map(|tag| {
            let namespace = tag.namespace()?;
            Some((namespace.name().clone(), tag.name().clone()))
        }));

    Ok(namespaces)
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
