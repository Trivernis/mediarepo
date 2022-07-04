use rayon::iter::{IntoParallelIterator, ParallelIterator};
use std::collections::HashMap;

use mediarepo_core::bromine::prelude::*;
use mediarepo_core::content_descriptor::{decode_content_descriptor, encode_content_descriptor};
use mediarepo_core::mediarepo_api::types::files::{
    GetFileTagMapRequest, GetFileTagsRequest, GetFilesTagsRequest,
};
use mediarepo_core::mediarepo_api::types::tags::{
    AddTagImplicationsRequest, ChangeFileTagsRequest, DeleteTagImplicationsRequest,
    NamespaceResponse, TagResponse,
};
use mediarepo_core::utils::parse_namespace_and_tag;
use mediarepo_logic::dao::DaoProvider;
use mediarepo_logic::dto::{AddTagDto, AddTagImplicationDto, DeleteTagImplicationDto};

use crate::from_model::FromModel;
use crate::utils::{file_by_identifier, get_repo_from_context};

pub struct TagsNamespace;

impl NamespaceProvider for TagsNamespace {
    fn name() -> &'static str {
        "tags"
    }

    fn register(handler: &mut EventHandler) {
        events!(handler,
            "all_tags" => Self::all_tags,
            "all_namespaces" => Self::all_namespaces,
            "tags_for_file" => Self::tags_for_file,
            "tags_for_files" => Self::tags_for_files,
            "file_tag_map" => Self::tag_cd_map_for_files,
            "create_tags" => Self::create_tags,
            "change_file_tags" => Self::change_file_tags,
            "add_tag_implications" => Self::add_tag_implications,
            "delete_tag_implications" => Self::delete_tag_implications
        );
    }
}

impl TagsNamespace {
    /// Returns a list of all tags in the database
    #[tracing::instrument(skip_all)]
    async fn all_tags(ctx: &Context, _event: Event) -> IPCResult<Response> {
        let repo = get_repo_from_context(ctx).await;
        let tags: Vec<TagResponse> = repo
            .tag()
            .all()
            .await?
            .into_iter()
            .map(TagResponse::from_model)
            .collect();

        ctx.response(tags)
    }

    /// Returns a list of all namespaces from the database
    #[tracing::instrument(skip_all)]
    async fn all_namespaces(ctx: &Context, _event: Event) -> IPCResult<Response> {
        let repo = get_repo_from_context(ctx).await;
        let namespaces: Vec<NamespaceResponse> = repo
            .tag()
            .all_namespaces()
            .await?
            .into_iter()
            .map(NamespaceResponse::from_model)
            .collect();

        ctx.response(namespaces)
    }

    /// Returns all tags for a single file
    #[tracing::instrument(skip_all)]
    async fn tags_for_file(ctx: &Context, event: Event) -> IPCResult<Response> {
        let repo = get_repo_from_context(ctx).await;
        let request = event.payload::<GetFileTagsRequest>()?;
        let file = file_by_identifier(request.id, &repo).await?;
        let tags = repo.tag().tags_for_cd(file.cd_id()).await?;
        let responses: Vec<TagResponse> = tags.into_iter().map(TagResponse::from_model).collect();

        ctx.response(responses)
    }

    /// Returns all tags for a given list of file hashes
    #[tracing::instrument(skip_all)]
    async fn tags_for_files(ctx: &Context, event: Event) -> IPCResult<Response> {
        let repo = get_repo_from_context(ctx).await;
        let request = event.payload::<GetFilesTagsRequest>()?;
        let tag_responses: Vec<TagResponse> = repo
            .tag()
            .all_for_cds(
                request
                    .cds
                    .into_par_iter()
                    .filter_map(|c| decode_content_descriptor(c).ok())
                    .collect(),
            )
            .await?
            .into_iter()
            .map(TagResponse::from_model)
            .collect();

        ctx.response(tag_responses)
    }

    /// Returns a map of content descriptors to assigned tags
    #[tracing::instrument(skip_all)]
    async fn tag_cd_map_for_files(ctx: &Context, event: Event) -> IPCResult<Response> {
        let request = event.payload::<GetFileTagMapRequest>()?;
        let repo = get_repo_from_context(ctx).await;
        let cds = request
            .cds
            .into_iter()
            .filter_map(|c| decode_content_descriptor(c).ok())
            .collect();

        let mappings = repo
            .tag()
            .all_for_cds_map(cds)
            .await?
            .into_iter()
            .map(|(cd, tags)| (encode_content_descriptor(&cd), tags))
            .map(|(cd, tags)| {
                (
                    cd,
                    tags.into_iter()
                        .map(TagResponse::from_model)
                        .collect::<Vec<TagResponse>>(),
                )
            })
            .collect::<HashMap<String, Vec<TagResponse>>>();

        ctx.response(mappings)
    }

    /// Creates all tags given as input or returns the existing tags
    #[tracing::instrument(skip_all)]
    async fn create_tags(ctx: &Context, event: Event) -> IPCResult<Response> {
        let repo = get_repo_from_context(ctx).await;
        let tags = event.payload::<Vec<String>>()?;
        let created_tags = repo
            .tag()
            .add_all(
                tags.into_iter()
                    .map(parse_namespace_and_tag)
                    .map(AddTagDto::from_tuple)
                    .collect(),
            )
            .await?;

        let responses: Vec<TagResponse> = created_tags
            .into_iter()
            .map(TagResponse::from_model)
            .collect();

        ctx.response(responses)
    }

    /// Changes tags of a file
    /// it removes the tags from the removed list and adds the one from the add list
    #[tracing::instrument(skip_all)]
    async fn change_file_tags(ctx: &Context, event: Event) -> IPCResult<Response> {
        let repo = get_repo_from_context(ctx).await;
        let request = event.payload::<ChangeFileTagsRequest>()?;
        let file = file_by_identifier(request.file_id, &repo).await?;

        if !request.added_tags.is_empty() {
            repo.tag()
                .upsert_mappings(vec![file.cd_id()], request.added_tags)
                .await?;
        }
        if !request.removed_tags.is_empty() {
            repo.tag()
                .remove_mappings(vec![file.cd_id()], request.removed_tags)
                .await?;
        }

        let responses: Vec<TagResponse> = repo
            .tag()
            .tags_for_cd(file.cd_id())
            .await?
            .into_iter()
            .map(TagResponse::from_model)
            .collect();

        ctx.response(responses)
    }

    /// Adds multiple tag implications
    #[tracing::instrument(skip_all)]
    async fn add_tag_implications(ctx: &Context, event: Event) -> IPCResult<Response> {
        let repo = get_repo_from_context(ctx).await;
        let request = event.payload::<AddTagImplicationsRequest>()?;
        let add_dtos: Vec<AddTagImplicationDto> = request
            .implications
            .into_iter()
            .map(|i| AddTagImplicationDto {
                tag_id: i.tag_id,
                implied_tag_id: i.implied_tag_id,
            })
            .collect();
        repo.tag().add_implications(add_dtos).await?;

        ctx.response(())
    }

    /// Deletes multiple tag implications
    #[tracing::instrument(skip_all)]
    async fn delete_tag_implications(ctx: &Context, event: Event) -> IPCResult<Response> {
        let repo = get_repo_from_context(ctx).await;
        let request = event.payload::<DeleteTagImplicationsRequest>()?;
        let delete_dtos: Vec<DeleteTagImplicationDto> = request
            .implications
            .into_iter()
            .map(|i| DeleteTagImplicationDto {
                tag_id: i.tag_id,
                implied_tag_id: i.implied_tag_id,
            })
            .collect();
        repo.tag().delete_implications(delete_dtos).await?;

        ctx.response(())
    }
}
