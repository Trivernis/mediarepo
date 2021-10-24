use crate::client_api::error::ApiResult;
use crate::client_api::IPCApi;
use crate::types::files::GetFileTagsRequest;
use crate::types::identifier::FileIdentifier;
use crate::types::tags::TagResponse;
use async_trait::async_trait;
use rmp_ipc::ipc::context::Context;

#[derive(Clone)]
pub struct TagApi {
    ctx: Context,
}

#[async_trait]
impl IPCApi for TagApi {
    fn namespace() -> &'static str {
        "tags"
    }

    fn ctx(&self) -> &Context {
        &self.ctx
    }
}

impl TagApi {
    pub fn new(ctx: Context) -> Self {
        Self { ctx }
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
