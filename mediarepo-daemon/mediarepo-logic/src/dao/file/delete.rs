use sea_orm::ConnectionTrait;
use sea_orm::prelude::*;

use mediarepo_core::error::{RepoResult};
use mediarepo_database::entities::{
    content_descriptor, content_descriptor_tag, file, file_metadata,
};

use crate::dao::file::{FileDao};
use crate::dto::FileDto;

impl FileDao {
    #[tracing::instrument(level = "debug", skip(self))]
    pub async fn delete(&self, file: FileDto) -> RepoResult<()> {
        let trx = self.ctx.db.begin().await?;

        file_metadata::Entity::delete_many()
            .filter(file_metadata::Column::FileId.eq(file.id()))
            .exec(&trx)
            .await?;
        file::Entity::delete_many()
            .filter(file::Column::Id.eq(file.id()))
            .exec(&trx)
            .await?;
        content_descriptor_tag::Entity::delete_many()
            .filter(content_descriptor_tag::Column::CdId.eq(file.cd_id()))
            .exec(&trx)
            .await?;
        content_descriptor::Entity::delete_many()
            .filter(content_descriptor::Column::Id.eq(file.cd_id()))
            .exec(&trx)
            .await?;

        self.ctx
            .thumbnail_storage
            .delete_parent(&file.encoded_cd())
            .await?;
        self.ctx.main_storage.delete_file(file.cd()).await?;
        trx.commit().await?;

        Ok(())
    }
}
