use crate::client_api::error::ApiResult;
use crate::client_api::IPCApi;
use crate::types::files::GetFileTagsRequest;
use crate::types::identifier::FileIdentifier;
use crate::types::tags::TagResponse;
use async_trait::async_trait;
use rmp_ipc::context::{PoolGuard, PooledContext};
use rmp_ipc::ipc::context::Context;

#[derive(Clone)]
pub struct TagApi {
    ctx: PooledContext,
}

#[async_trait]
impl IPCApi for TagApi {
    fn namespace() -> &'static str {
        "tags"
    }

    fn ctx(&self) -> PoolGuard<Context> {
        self.ctx.acquire()
    }
}

impl TagApi {
    pub fn new(ctx: PooledContext) -> Self {
        Self { ctx }
    }

    /// Returns a list of all tags stored in the repo
    #[tracing::instrument(level = "debug", skip(self))]
    pub async fn get_all_tags(&self) -> ApiResult<Vec<TagResponse>> {
        self.emit_and_get("all_tags", ()).await
    }

    /// Returns a list of all tags for a file
    #[tracing::instrument(level = "debug", skip(self))]
    pub async fn get_tags_for_file(&self, hash: String) -> ApiResult<Vec<TagResponse>> {
        self.emit_and_get(
            "tags_for_file",
            GetFileTagsRequest {
                id: FileIdentifier::Hash(hash),
            },
        )
        .await
    }
}
