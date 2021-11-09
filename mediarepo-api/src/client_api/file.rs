use crate::client_api::error::ApiResult;
use crate::client_api::IPCApi;
use crate::types::files::{
    AddFileRequestHeader, FileMetadataResponse, FileOSMetadata, FindFilesByTagsRequest,
    GetFileThumbnailOfSizeRequest, GetFileThumbnailsRequest, ReadFileRequest, SortKey, TagQuery,
    ThumbnailMetadataResponse, UpdateFileNameRequest,
};
use crate::types::identifier::FileIdentifier;
use async_trait::async_trait;
use rmp_ipc::context::{PoolGuard, PooledContext};
use rmp_ipc::payload::{BytePayload, EventSendPayload};
use rmp_ipc::prelude::*;

pub struct FileApi<S: AsyncProtocolStream> {
    ctx: PooledContext<S>,
}

impl<S> Clone for FileApi<S>
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
impl<S> IPCApi<S> for FileApi<S>
where
    S: AsyncProtocolStream,
{
    fn namespace() -> &'static str {
        "files"
    }

    fn ctx(&self) -> PoolGuard<Context<S>> {
        self.ctx.acquire()
    }
}

impl<S> FileApi<S>
where
    S: AsyncProtocolStream,
{
    /// Creates a new file api client
    pub fn new(ctx: PooledContext<S>) -> Self {
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
    pub async fn read_file_by_hash(&self, id: FileIdentifier) -> ApiResult<Vec<u8>> {
        let payload: BytePayload = self
            .emit_and_get("read_file", ReadFileRequest { id })
            .await?;

        Ok(payload.to_payload_bytes()?)
    }

    /// Returns a list of all thumbnails of the file
    #[tracing::instrument(level = "debug", skip(self))]
    pub async fn get_file_thumbnails(
        &self,
        id: FileIdentifier,
    ) -> ApiResult<Vec<ThumbnailMetadataResponse>> {
        self.emit_and_get("get_thumbnails", GetFileThumbnailsRequest { id })
            .await
    }

    /// Reads the thumbnail of the file and returns its contents in bytes
    #[tracing::instrument(level = "debug", skip(self))]
    pub async fn read_thumbnail(&self, hash: String) -> ApiResult<Vec<u8>> {
        let payload: BytePayload = self.emit_and_get("read_thumbnail", hash).await?;
        Ok(payload.into_inner())
    }

    /// Returns a thumbnail of size that is within the specified range
    #[tracing::instrument(level = "debug", skip(self))]
    pub async fn get_thumbnail_of_size(
        &self,
        file_id: FileIdentifier,
        min_size: (u32, u32),
        max_size: (u32, u32),
    ) -> ApiResult<(ThumbnailMetadataResponse, Vec<u8>)> {
        let payload: TandemPayload<ThumbnailMetadataResponse, BytePayload> = self
            .emit_and_get(
                "get_thumbnail_of_size",
                GetFileThumbnailOfSizeRequest {
                    id: file_id,
                    min_size,
                    max_size,
                },
            )
            .await?;
        let (metadata, bytes) = payload.into_inner();

        Ok((metadata, bytes.into_inner()))
    }

    /// Updates a files name
    #[tracing::instrument(level = "debug", skip(self))]
    pub async fn update_file_name(
        &self,
        file_id: FileIdentifier,
        name: String,
    ) -> ApiResult<FileMetadataResponse> {
        self.emit_and_get("update_file_name", UpdateFileNameRequest { file_id, name })
            .await
    }

    /// Adds a file with predefined tags
    #[tracing::instrument(level = "debug", skip(self, bytes))]
    pub async fn add_file(
        &self,
        metadata: FileOSMetadata,
        tags: Vec<String>,
        bytes: Vec<u8>,
    ) -> ApiResult<FileMetadataResponse> {
        let payload = TandemPayload::new(
            AddFileRequestHeader { metadata, tags },
            BytePayload::new(bytes),
        );

        self.emit_and_get("add_file", payload).await
    }
}
