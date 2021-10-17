use crate::types::responses::TagResponse;
use crate::utils::get_repo_from_context;
use mediarepo_core::rmp_ipc::prelude::*;

pub struct TagsNamespace;

impl NamespaceProvider for TagsNamespace {
    fn name() -> &'static str {
        "tags"
    }

    fn register(handler: &mut EventHandler) {
        events!(handler,
            "all_tags" => Self::all_tags
        );
    }
}

impl TagsNamespace {
    /// Returns a list of all tags in the database
    async fn all_tags(ctx: &Context, event: Event) -> IPCResult<()> {
        let repo = get_repo_from_context(ctx).await;
        let tags: Vec<TagResponse> = repo
            .tags()
            .await?
            .into_iter()
            .map(TagResponse::from)
            .collect();
        ctx.emitter
            .emit_response_to(event.id(), Self::name(), "all_tags", tags)
            .await?;

        Ok(())
    }
}
