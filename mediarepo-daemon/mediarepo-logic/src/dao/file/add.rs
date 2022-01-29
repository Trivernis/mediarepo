use crate::dto::{AddFileDto, FileDto};
use chrono::{Local, NaiveDateTime};
use mediarepo_core::error::RepoResult;
use mediarepo_database::entities::{content_descriptor, file, file_metadata};
use sea_orm::ActiveValue::Set;
use sea_orm::{ActiveModelTrait, ConnectionTrait, DatabaseTransaction};
use std::io::Cursor;

use crate::dao::file::FileDao;

impl FileDao {
    #[tracing::instrument(level = "debug", skip(self))]
    pub async fn add(&self, add_dto: AddFileDto) -> RepoResult<FileDto> {
        let trx = self.ctx.db.begin().await?;
        let file_size = add_dto.content.len();
        let cd_bin = self
            .ctx
            .main_storage
            .add_file(Cursor::new(add_dto.content), None)
            .await?;
        let cd_model = content_descriptor::ActiveModel {
            descriptor: Set(cd_bin),
            ..Default::default()
        };
        let cd = cd_model.insert(&trx).await?;

        let model = file::ActiveModel {
            cd_id: Set(cd.id),
            mime_type: Set(add_dto.mime_type),
            ..Default::default()
        };
        let file: file::Model = model.insert(&trx).await?;

        let metadata = add_file_metadata(
            &trx,
            file.id,
            file_size as i64,
            add_dto.creation_time,
            add_dto.change_time,
            add_dto.name,
        )
        .await?;

        trx.commit().await?;

        Ok(FileDto::new(file, cd, Some(metadata)))
    }
}

async fn add_file_metadata(
    trx: &DatabaseTransaction,
    file_id: i64,
    size: i64,
    creation_time: NaiveDateTime,
    change_time: NaiveDateTime,
    name: Option<String>,
) -> RepoResult<file_metadata::Model> {
    let metadata_model = file_metadata::ActiveModel {
        file_id: Set(file_id),
        size: Set(size),
        import_time: Set(Local::now().naive_local()),
        creation_time: Set(creation_time),
        change_time: Set(change_time),
        name: Set(name),
        ..Default::default()
    };

    let metadata = metadata_model.insert(trx).await?;

    Ok(metadata)
}
