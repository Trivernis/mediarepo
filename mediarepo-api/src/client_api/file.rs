use crate::client_api::error::ApiResult;
use crate::client_api::IPCApi;
use crate::types::files::{
    FileMetadataResponse, FindFilesByTagsRequest, GetFileThumbnailsRequest, ReadFileRequest,
    SortKey, TagQuery, ThumbnailMetadataResponse,
};
use crate::types::identifier::FileIdentifier;
use async_trait::async_trait;
use rmp_ipc::context::{PoolGuard, PooledContext};
use rmp_ipc::payload::{BytePayload, EventSendPayload};
use rmp_ipc::prelude::Context;

#[derive(Clone)]
pub struct FileApi {
    ctx: PooledContext,
}

#[async_trait]
impl IPCApi for FileApi {
    fn namespace() -> &'static str {
        "files"
    }

    fn ctx(&self) -> PoolGuard<Context> {
        self.ctx.acquire()
    }
}

impl FileApi {
    /// Creates a new file api client
    pub fn new(ctx: PooledContext) -> Self {
        Self { ctx }
    }

    /// Returns all known files
    #[tracing::instrument(level = "debug", skip(self))]
    pub async fn all_files(&self) -> ApiResult<Vec<FileMetadataResponse>> {
        self.emit_and_get("all_files", ()).await
    }

    /// Searches for a file by a list of tags
    #[tracing::instrument(level = "debug", skip(self))]
    pub async fn find_files(
        &self,
        tags: Vec<TagQuery>,
        sort_expression: Vec<SortKey>,
    ) -> ApiResult<Vec<FileMetadataResponse>> {
        self.emit_and_get(
            "find_files",
            FindFilesByTagsRequest {
                tags,
                sort_expression,
            },
        )
        .await
    }

    /// Reads the file and returns its contents as bytes
    #[tracing::instrument(level = "debug", skip(self))]
    pub async fn read_file_by_hash(&self, hash: String) -> ApiResult<Vec<u8>> {
        let payload: BytePayload = self
            .emit_and_get(
                "read_file",
                ReadFileRequest {
                    id: FileIdentifier::Hash(hash),
                },
            )
            .await?;

        Ok(payload.to_payload_bytes()?)
    }

    /// Returns a list of all thumbnails of the file
    #[tracing::instrument(level = "debug", skip(self))]
    pub async fn get_file_thumbnails(
        &self,
        hash: String,
    ) -> ApiResult<Vec<ThumbnailMetadataResponse>> {
        self.emit_and_get(
            "get_thumbnails",
            GetFileThumbnailsRequest {
                id: FileIdentifier::Hash(hash),
            },
        )
        .await
    }

    /// Reads the thumbnail of the file and returns its contents in bytes
    #[tracing::instrument(level = "debug", skip(self))]
    pub async fn read_thumbnail(&self, hash: String) -> ApiResult<Vec<u8>> {
        let payload: BytePayload = self.emit_and_get("read_thumbnail", hash).await?;
        Ok(payload.to_payload_bytes()?)
    }
}
