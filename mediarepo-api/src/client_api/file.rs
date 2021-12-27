use crate::client_api::error::ApiResult;
use crate::client_api::IPCApi;
use crate::types::files::{
    AddFileRequestHeader, FileMetadataResponse, FileOSMetadata, FilterExpression, FindFilesRequest,
    GetFileThumbnailOfSizeRequest, GetFileThumbnailsRequest, ReadFileRequest, SortKey,
    ThumbnailMetadataResponse, UpdateFileNameRequest,
};
use crate::types::identifier::FileIdentifier;
use async_trait::async_trait;
use bromine::context::{PoolGuard, PooledContext};
use bromine::payload::BytePayload;
use bromine::prelude::*;
use tokio::time::Duration;

pub struct FileApi {
    ctx: PooledContext,
}

impl Clone for FileApi {
    fn clone(&self) -> Self {
        Self {
            ctx: self.ctx.clone(),
        }
    }
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
        self.emit_and_get("all_files", (), Some(Duration::from_secs(30))).await
    }

    /// Returns a file by identifier
    pub async fn get_file(&self, id: FileIdentifier) -> ApiResult<FileMetadataResponse> {
        self.emit_and_get("get_file", id, Some(Duration::from_secs(2))).await
    }

    /// Searches for a file by a list of tags
    #[tracing::instrument(level = "debug", skip(self))]
    pub async fn find_files(
        &self,
        filters: Vec<FilterExpression>,
        sort_expression: Vec<SortKey>,
    ) -> ApiResult<Vec<FileMetadataResponse>> {
        self.emit_and_get(
            "find_files",
            FindFilesRequest {
                filters,
                sort_expression,
            },
            Some(Duration::from_secs(20))
        )
        .await
    }

    /// Reads the file and returns its contents as bytes
    #[tracing::instrument(level = "debug", skip(self))]
    pub async fn read_file(&self, id: FileIdentifier) -> ApiResult<Vec<u8>> {
        let payload: BytePayload = self
            .emit_and_get("read_file", ReadFileRequest { id }, Some(Duration::from_secs(10)))
            .await?;

        Ok(payload.into_inner())
    }

    /// Returns a list of all thumbnails of the file
    #[tracing::instrument(level = "debug", skip(self))]
    pub async fn get_file_thumbnails(
        &self,
        id: FileIdentifier,
    ) -> ApiResult<Vec<ThumbnailMetadataResponse>> {
        self.emit_and_get("get_thumbnails", GetFileThumbnailsRequest { id }, Some(Duration::from_secs(2)))
            .await
    }

    /// Returns a thumbnail of size that is within the specified range
    #[tracing::instrument(level = "debug", skip(self))]
    pub async fn get_thumbnail_of_size(
        &self,
        file_id: FileIdentifier,
        min_size: (u32, u32),
        max_size: (u32, u32),
    ) -> ApiResult<(ThumbnailMetadataResponse, Vec<u8>)> {
        let payload: TandemPayload<SerdePayload<ThumbnailMetadataResponse>, BytePayload> = self
            .emit_and_get(
                "get_thumbnail_of_size",
                GetFileThumbnailOfSizeRequest {
                    id: file_id,
                    min_size,
                    max_size,
                },
                Some(Duration::from_secs(2))
            )
            .await?;
        let (metadata, bytes) = payload.into_inner();

        Ok((metadata.data(), bytes.into_inner()))
    }

    /// Updates a files name
    #[tracing::instrument(level = "debug", skip(self))]
    pub async fn update_file_name(
        &self,
        file_id: FileIdentifier,
        name: String,
    ) -> ApiResult<FileMetadataResponse> {
        self.emit_and_get("update_file_name", UpdateFileNameRequest { file_id, name }, Some(Duration::from_secs(1)))
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

        self.emit_and_get("add_file", payload, Some(Duration::from_secs(5))).await
    }

    /// Deletes all thumbnails of a file to regenerate them when requested
    #[tracing::instrument(level = "debug", skip(self))]
    pub async fn delete_thumbnails(&self, file_id: FileIdentifier) -> ApiResult<()> {
        self.emit("delete_thumbnails", file_id).await?;

        Ok(())
    }
}
