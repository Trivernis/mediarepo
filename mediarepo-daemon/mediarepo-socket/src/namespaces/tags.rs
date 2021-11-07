use crate::from_model::FromModel;
use crate::utils::{file_by_identifier, get_repo_from_context};
use mediarepo_api::types::files::{GetFileTagsRequest, GetFilesTagsRequest};
use mediarepo_api::types::tags::{ChangeFileTagsRequest, TagResponse};
use mediarepo_core::rmp_ipc::prelude::*;

pub struct TagsNamespace;

impl NamespaceProvider for TagsNamespace {
    fn name() -> &'static str {
        "tags"
    }

    fn register<S: AsyncProtocolStream>(handler: &mut EventHandler<S>) {
        events!(handler,
            "all_tags" => Self::all_tags,
            "tags_for_file" => Self::tags_for_file,
            "tags_for_files" => Self::tags_for_files,
            "create_tags" => Self::create_tags,
            "change_file_tags" => Self::change_file_tags
        );
    }
}

impl TagsNamespace {
    /// Returns a list of all tags in the database
    #[tracing::instrument(skip_all)]
    async fn all_tags<S: AsyncProtocolStream>(ctx: &Context<S>, event: Event) -> IPCResult<()> {
        let repo = get_repo_from_context(ctx).await;
        let tags: Vec<TagResponse> = repo
            .tags()
            .await?
            .into_iter()
            .map(TagResponse::from_model)
            .collect();
        ctx.emitter
            .emit_response_to(event.id(), Self::name(), "all_tags", tags)
            .await?;

        Ok(())
    }

    /// Returns all tags for a single file
    #[tracing::instrument(skip_all)]
    async fn tags_for_file<S: AsyncProtocolStream>(
        ctx: &Context<S>,
        event: Event,
    ) -> IPCResult<()> {
        let repo = get_repo_from_context(ctx).await;
        let request = event.data::<GetFileTagsRequest>()?;
        let file = file_by_identifier(request.id, &repo).await?;
        let tags = file.tags().await?;
        let responses: Vec<TagResponse> = tags.into_iter().map(TagResponse::from_model).collect();

        ctx.emitter
            .emit_response_to(event.id(), Self::name(), "tags_for_file", responses)
            .await?;

        Ok(())
    }

    /// Returns all tags for a given list of file hashes
    #[tracing::instrument(skip_all)]
    async fn tags_for_files<S: AsyncProtocolStream>(
        ctx: &Context<S>,
        event: Event,
    ) -> IPCResult<()> {
        let repo = get_repo_from_context(ctx).await;
        let request = event.data::<GetFilesTagsRequest>()?;
        let tag_responses: Vec<TagResponse> = repo
            .find_tags_for_hashes(request.hashes)
            .await?
            .into_iter()
            .map(TagResponse::from_model)
            .collect();
        ctx.emitter
            .emit_response_to(event.id(), Self::name(), "tags_for_files", tag_responses)
            .await?;

        Ok(())
    }

    /// Creates all tags given as input or returns the existing tag
    #[tracing::instrument(skip_all)]
    async fn create_tags<S: AsyncProtocolStream>(ctx: &Context<S>, event: Event) -> IPCResult<()> {
        let repo = get_repo_from_context(ctx).await;
        let tags = event.data::<Vec<String>>()?;
        let mut created_tags = Vec::new();

        for tag in tags {
            let created_tag = repo.add_or_find_tag(tag).await?;
            created_tags.push(created_tag);
        }
        let responses: Vec<TagResponse> = created_tags
            .into_iter()
            .map(TagResponse::from_model)
            .collect();
        ctx.emitter
            .emit_response_to(event.id(), Self::name(), "create_tags", responses)
            .await?;

        Ok(())
    }

    /// Changes tags of a file
    /// it removes the tags from the removed list and adds the one from the add list
    #[tracing::instrument(skip_all)]
    async fn change_file_tags<S: AsyncProtocolStream>(
        ctx: &Context<S>,
        event: Event,
    ) -> IPCResult<()> {
        let repo = get_repo_from_context(ctx).await;
        let request = event.data::<ChangeFileTagsRequest>()?;
        let file = file_by_identifier(request.file_id, &repo).await?;

        if !request.added_tags.is_empty() {
            file.add_tags(request.added_tags).await?;
        }
        if !request.removed_tags.is_empty() {
            file.remove_tags(request.removed_tags).await?;
        }

        let responses: Vec<TagResponse> = file
            .tags()
            .await?
            .into_iter()
            .map(TagResponse::from_model)
            .collect();
        ctx.emitter
            .emit_response_to(event.id(), Self::name(), "change_file_tags", responses)
            .await?;

        Ok(())
    }
}
