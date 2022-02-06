use std::fmt::Debug;
use std::io::Cursor;
use std::str::FromStr;

use sea_orm::prelude::*;
use sea_orm::ActiveValue::{Set, Unchanged};
use sea_orm::{NotSet, TransactionTrait};

use mediarepo_core::error::{RepoError, RepoResult};
use mediarepo_core::fs::thumbnail_store::Dimensions;
use mediarepo_core::thumbnailer;
use mediarepo_core::thumbnailer::ThumbnailSize;
use mediarepo_database::entities::{content_descriptor, file, file_metadata};

use crate::dao::file::FileDao;
use crate::dao::opt_to_active_val;
use crate::dto::{FileDto, FileMetadataDto, ThumbnailDto, UpdateFileDto, UpdateFileMetadataDto};

impl FileDao {
    #[tracing::instrument(level = "debug", skip(self))]
    pub async fn update(&self, update_dto: UpdateFileDto) -> RepoResult<FileDto> {
        let trx = self.ctx.db.begin().await?;
        let model = file::ActiveModel {
            id: Set(update_dto.id),
            cd_id: update_dto.cd_id.map(|v| Set(v)).unwrap_or(NotSet),
            mime_type: update_dto.mime_type.map(|v| Set(v)).unwrap_or(NotSet),
            status: update_dto.status.map(|v| Set(v as i32)).unwrap_or(NotSet),
        };
        let file_model = model.update(&trx).await?;
        let cd = file_model
            .find_related(content_descriptor::Entity)
            .one(&trx)
            .await?
            .ok_or_else(|| RepoError::from("Content descriptor not found"))?;
        trx.commit().await?;

        Ok(FileDto::new(file_model, cd, None))
    }

    #[tracing::instrument(level = "debug", skip(self))]
    pub async fn update_metadata(
        &self,
        update_dto: UpdateFileMetadataDto,
    ) -> RepoResult<FileMetadataDto> {
        let model = file_metadata::ActiveModel {
            file_id: Unchanged(update_dto.file_id),
            name: opt_to_active_val(update_dto.name),
            comment: opt_to_active_val(update_dto.comment),
            size: opt_to_active_val(update_dto.size),
            change_time: opt_to_active_val(update_dto.change_time),
            ..Default::default()
        };
        let metadata = model.update(&self.ctx.db).await?;

        Ok(FileMetadataDto::new(metadata))
    }

    #[tracing::instrument(level = "debug", skip(self))]
    pub async fn create_thumbnails<I: IntoIterator<Item = ThumbnailSize> + Debug>(
        &self,
        file: FileDto,
        sizes: I,
    ) -> RepoResult<Vec<ThumbnailDto>> {
        let bytes = self.get_bytes(file.cd()).await?;
        let mime_type = mime::Mime::from_str(file.mime_type())
            .unwrap_or_else(|_| mime::APPLICATION_OCTET_STREAM);
        let thumbnails =
            thumbnailer::create_thumbnails(Cursor::new(bytes), mime_type.clone(), sizes)?;
        let mut dtos = Vec::new();

        for thumbnail in thumbnails {
            let mut buf = Cursor::new(Vec::new());
            let size = thumbnail.size();
            let size = Dimensions {
                height: size.1,
                width: size.0,
            };
            thumbnail.write_png(&mut buf)?;

            let path = self
                .ctx
                .thumbnail_storage
                .add_thumbnail(file.encoded_cd(), size.clone(), &buf.into_inner())
                .await?;
            dtos.push(ThumbnailDto::new(
                path,
                file.encoded_cd(),
                size,
                mime_type.to_string(),
            ))
        }

        Ok(dtos)
    }
}
