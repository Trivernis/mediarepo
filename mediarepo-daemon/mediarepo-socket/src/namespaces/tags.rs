use crate::from_model::FromModel;
use crate::utils::{file_by_identifier, get_repo_from_context};
use mediarepo_api::types::files::{GetFileTagsRequest, GetFilesTagsRequest};
use mediarepo_api::types::tags::TagResponse;
use mediarepo_core::rmp_ipc::prelude::*;

pub struct TagsNamespace;

impl NamespaceProvider for TagsNamespace {
    fn name() -> &'static str {
        "tags"
    }

    fn register(handler: &mut EventHandler) {
        events!(handler,
            "all_tags" => Self::all_tags,
            "tags_for_file" => Self::tags_for_file,
            "tags_for_files" => Self::tags_for_files
        );
    }
}

impl TagsNamespace {
    /// Returns a list of all tags in the database
    #[tracing::instrument(skip_all)]
    async fn all_tags(ctx: &Context, event: Event) -> IPCResult<()> {
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
    async fn tags_for_file(ctx: &Context, event: Event) -> IPCResult<()> {
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
    async fn tags_for_files(ctx: &Context, event: Event) -> IPCResult<()> {
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
}
