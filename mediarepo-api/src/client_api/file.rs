use crate::client_api::error::ApiResult;
use crate::client_api::IPCApi;
use crate::types::files::{
    FileMetadataResponse, FindFilesByTagsRequest, GetFileThumbnailsRequest, ReadFileRequest,
    TagQuery, ThumbnailMetadataResponse,
};
use crate::types::identifier::FileIdentifier;
use async_trait::async_trait;
use rmp_ipc::prelude::Context;

#[derive(Clone)]
pub struct FileApi {
    ctx: Context,
}

#[async_trait]
impl IPCApi for FileApi {
    fn namespace() -> &'static str {
        "files"
    }

    fn ctx(&self) -> &Context {
        &self.ctx
    }
}

impl FileApi {
    /// Creates a new file api client
    pub fn new(ctx: Context) -> Self {
        Self { ctx }
    }

    /// Returns all known files
    #[tracing::instrument(level = "debug", skip(self))]
    pub async fn all_files(&self) -> ApiResult<Vec<FileMetadataResponse>> {
        self.emit_and_get("all_files", ()).await
    }

    /// Searches for a file by a list of tags
    #[tracing::instrument(level = "debug", skip(self))]
    pub async fn find_files(&self, tags: Vec<String>) -> ApiResult<Vec<FileMetadataResponse>> {
        let tags = tags
            .into_iter()
            .map(|tag| TagQuery {
                name: tag,
                negate: false,
            })
            .collect();

        self.emit_and_get("find_files", FindFilesByTagsRequest { tags })
            .await
    }

    /// Reads the file and returns its contents as bytes
    #[tracing::instrument(level = "debug", skip(self))]
    pub async fn read_file_by_hash(&self, hash: String) -> ApiResult<Vec<u8>> {
        self.emit_and_get(
            "read_file",
            ReadFileRequest {
                id: FileIdentifier::Hash(hash),
            },
        )
        .await
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
        self.emit_and_get("read_thumbnail", hash).await
    }
}
