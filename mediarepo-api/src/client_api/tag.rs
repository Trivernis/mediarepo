use crate::client_api::error::ApiResult;
use crate::client_api::IPCApi;
use crate::types::files::{GetFileTagsRequest, GetFilesTagsRequest};
use crate::types::identifier::FileIdentifier;
use crate::types::tags::{ChangeFileTagsRequest, TagResponse};
use async_trait::async_trait;
use rmp_ipc::context::{PoolGuard, PooledContext};
use rmp_ipc::ipc::context::Context;
use rmp_ipc::protocol::AsyncProtocolStream;

pub struct TagApi<S: AsyncProtocolStream> {
    ctx: PooledContext<S>,
}

impl<S> Clone for TagApi<S>
where
    S: AsyncProtocolStream,
{
    fn clone(&self) -> Self {
        Self {
            ctx: self.ctx.clone(),
        }
    }
}

#[async_trait]
impl<S> IPCApi<S> for TagApi<S>
where
    S: AsyncProtocolStream,
{
    fn namespace() -> &'static str {
        "tags"
    }

    fn ctx(&self) -> PoolGuard<Context<S>> {
        self.ctx.acquire()
    }
}

impl<S> TagApi<S>
where
    S: AsyncProtocolStream,
{
    pub fn new(ctx: PooledContext<S>) -> Self {
        Self { ctx }
    }

    /// Returns a list of all tags stored in the repo
    #[tracing::instrument(level = "debug", skip(self))]
    pub async fn get_all_tags(&self) -> ApiResult<Vec<TagResponse>> {
        self.emit_and_get("all_tags", ()).await
    }

    /// Returns a list of all tags for a file
    #[tracing::instrument(level = "debug", skip(self))]
    pub async fn get_tags_for_file(&self, id: FileIdentifier) -> ApiResult<Vec<TagResponse>> {
        self.emit_and_get("tags_for_file", GetFileTagsRequest { id })
            .await
    }

    /// Returns a list of all tags that are assigned to the list of files
    #[tracing::instrument(level = "debug", skip_all)]
    pub async fn get_tags_for_files(&self, hashes: Vec<String>) -> ApiResult<Vec<TagResponse>> {
        self.emit_and_get("tags_for_files", GetFilesTagsRequest { hashes })
            .await
    }

    /// Creates a new tag and returns the created tag object
    #[tracing::instrument(level = "debug", skip(self))]
    pub async fn create_tags(&self, tags: Vec<String>) -> ApiResult<Vec<TagResponse>> {
        self.emit_and_get("create_tags", tags).await
    }

    /// Changes the tags of a file
    #[tracing::instrument(level = "debug", skip(self))]
    pub async fn change_file_tags(
        &self,
        file_id: FileIdentifier,
        added_tags: Vec<i64>,
        removed_tags: Vec<i64>,
    ) -> ApiResult<Vec<TagResponse>> {
        self.emit_and_get(
            "change_file_tags",
            ChangeFileTagsRequest {
                file_id,
                added_tags,
                removed_tags,
            },
        )
        .await
    }
}
