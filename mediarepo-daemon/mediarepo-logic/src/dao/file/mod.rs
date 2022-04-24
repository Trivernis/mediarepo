use sea_orm::prelude::*;
use tokio::io::AsyncReadExt;

use crate::dao_provider;
use mediarepo_core::error::RepoResult;
use mediarepo_database::entities::{content_descriptor, file, file_metadata};

use crate::dto::{FileDto, FileMetadataDto, ThumbnailDto};

pub mod add;
pub mod delete;
pub mod find;
pub mod update;

dao_provider!(FileDao);

impl FileDao {
    #[tracing::instrument(level = "debug", skip(self))]
    pub async fn all(&self) -> RepoResult<Vec<FileDto>> {
        let files = file::Entity::find()
            .find_also_related(content_descriptor::Entity)
            .all(&self.ctx.db)
            .await?
            .into_iter()
            .filter_map(map_file_and_cd)
            .collect();

        Ok(files)
    }

    #[tracing::instrument(level = "debug", skip(self))]
    #[inline]
    pub async fn by_id(&self, id: i64) -> RepoResult<Option<FileDto>> {
        self.all_by_id(vec![id]).await.map(|f| f.into_iter().next())
    }

    #[tracing::instrument(level = "debug", skip(self))]
    #[inline]
    pub async fn by_cd(&self, cd: Vec<u8>) -> RepoResult<Option<FileDto>> {
        self.all_by_cd(vec![cd]).await.map(|f| f.into_iter().next())
    }

    #[tracing::instrument(level = "debug", skip(self))]
    pub async fn all_by_cd(&self, cds: Vec<Vec<u8>>) -> RepoResult<Vec<FileDto>> {
        if cds.is_empty() {
            return Ok(vec![]);
        }

        let files = file::Entity::find()
            .find_also_related(content_descriptor::Entity)
            .filter(content_descriptor::Column::Descriptor.is_in(cds))
            .all(&self.ctx.db)
            .await?
            .into_iter()
            .filter_map(map_file_and_cd)
            .collect();

        Ok(files)
    }

    #[tracing::instrument(level = "debug", skip(self))]
    pub async fn all_by_id(&self, ids: Vec<i64>) -> RepoResult<Vec<FileDto>> {
        if ids.is_empty() {
            return Ok(vec![]);
        }

        let files = file::Entity::find()
            .find_also_related(content_descriptor::Entity)
            .filter(file::Column::Id.is_in(ids))
            .all(&self.ctx.db)
            .await?
            .into_iter()
            .filter_map(map_file_and_cd)
            .collect();

        Ok(files)
    }

    pub async fn metadata(&self, file_id: i64) -> RepoResult<Option<FileMetadataDto>> {
        self.all_metadata(vec![file_id])
            .await
            .map(|m| m.into_iter().next())
    }

    #[tracing::instrument(level = "debug", skip(self))]
    pub async fn all_metadata(&self, file_ids: Vec<i64>) -> RepoResult<Vec<FileMetadataDto>> {
        if file_ids.is_empty() {
            return Ok(vec![]);
        }

        let metadata = file_metadata::Entity::find()
            .filter(file_metadata::Column::FileId.is_in(file_ids))
            .all(&self.ctx.db)
            .await?
            .into_iter()
            .map(FileMetadataDto::new)
            .collect();

        Ok(metadata)
    }

    /// Returns all thumbnails for a cd
    #[tracing::instrument(level = "debug", skip(self))]
    pub async fn thumbnails(&self, encoded_cd: String) -> RepoResult<Vec<ThumbnailDto>> {
        let thumbnails = self
            .ctx
            .thumbnail_storage
            .get_thumbnails(&encoded_cd)
            .await?
            .into_iter()
            .map(|(size, path)| {
                ThumbnailDto::new(path, encoded_cd.clone(), size, String::from("image/png"))
            })
            .collect();

        Ok(thumbnails)
    }

    #[tracing::instrument(level = "debug", skip(self))]
    pub async fn get_bytes(&self, cd: &[u8]) -> RepoResult<Vec<u8>> {
        let mut buf = Vec::new();
        let mut reader = self.ctx.main_storage.get_file(cd).await?.1;
        reader.read_to_end(&mut buf).await?;

        Ok(buf)
    }
}

fn map_file_and_cd(
    (file, cd): (file::Model, Option<content_descriptor::Model>),
) -> Option<FileDto> {
    cd.map(|c| FileDto::new(file, c, None))
}

fn map_cd_and_file(
    (cd, file): (content_descriptor::Model, Option<file::Model>),
) -> Option<FileDto> {
    file.map(|f| FileDto::new(f, cd, None))
}
