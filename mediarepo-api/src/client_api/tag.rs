use crate::client_api::error::ApiResult;
use crate::client_api::IPCApi;
use crate::types::files::{GetFileTagMapRequest, GetFileTagsRequest, GetFilesTagsRequest};
use crate::types::identifier::FileIdentifier;
use crate::types::tags::{
    AddTagImplicationsRequest, ChangeFileTagsRequest, DeleteTagImplicationsRequest,
    NamespaceResponse, TagImplication, TagResponse,
};
use async_trait::async_trait;
use bromine::context::{PoolGuard, PooledContext};
use bromine::ipc::context::Context;
use std::collections::HashMap;
use std::time::Duration;

pub struct TagApi {
    ctx: PooledContext,
}

impl Clone for TagApi {
    fn clone(&self) -> Self {
        Self {
            ctx: self.ctx.clone(),
        }
    }
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
        self.emit_and_get("all_tags", (), Some(Duration::from_secs(2)))
            .await
    }

    /// Returns a list of all namespaces stored in the repo
    #[tracing::instrument(level = "debug", skip(self))]
    pub async fn get_all_namespaces(&self) -> ApiResult<Vec<NamespaceResponse>> {
        self.emit_and_get("all_namespaces", (), Some(Duration::from_secs(2)))
            .await
    }

    /// Returns a list of all tags for a file
    #[tracing::instrument(level = "debug", skip(self))]
    pub async fn get_tags_for_file(&self, id: FileIdentifier) -> ApiResult<Vec<TagResponse>> {
        self.emit_and_get(
            "tags_for_file",
            GetFileTagsRequest { id },
            Some(Duration::from_secs(1)),
        )
        .await
    }

    /// Returns a list of all tags that are assigned to the list of files
    #[tracing::instrument(level = "debug", skip_all)]
    pub async fn get_tags_for_files(&self, cds: Vec<String>) -> ApiResult<Vec<TagResponse>> {
        self.emit_and_get(
            "tags_for_files",
            GetFilesTagsRequest { cds },
            Some(Duration::from_secs(10)),
        )
        .await
    }

    /// Returns a map from files to assigned tags
    #[tracing::instrument(level = "debug", skip_all)]
    pub async fn get_file_tag_map(
        &self,
        cds: Vec<String>,
    ) -> ApiResult<HashMap<String, Vec<TagResponse>>> {
        self.emit_and_get(
            "file_tag_map",
            GetFileTagMapRequest { cds },
            Some(Duration::from_secs(10)),
        )
        .await
    }

    /// Creates a new tag and returns the created tag object
    #[tracing::instrument(level = "debug", skip(self))]
    pub async fn create_tags(&self, tags: Vec<String>) -> ApiResult<Vec<TagResponse>> {
        self.emit_and_get("create_tags", tags, Some(Duration::from_secs(10)))
            .await
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
            Some(Duration::from_secs(10)),
        )
        .await
    }

    /// Add implications for the given tags
    #[tracing::instrument(level = "debug", skip(self))]
    pub async fn add_tag_implications(&self, implications: Vec<TagImplication>) -> ApiResult<()> {
        self.emit_and_get(
            "add_tag_implications",
            AddTagImplicationsRequest { implications },
            None,
        )
        .await
    }

    /// Add implications for the given tags
    #[tracing::instrument(level = "debug", skip(self))]
    pub async fn delete_tag_implications(
        &self,
        implications: Vec<TagImplication>,
    ) -> ApiResult<()> {
        self.emit_and_get(
            "delete_tag_implications",
            DeleteTagImplicationsRequest { implications },
            None,
        )
        .await
    }
}
