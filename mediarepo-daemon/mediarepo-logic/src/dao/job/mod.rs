use crate::dao::{DaoContext, DaoProvider};
use mediarepo_core::content_descriptor::{
    convert_v1_descriptor_to_v2, encode_content_descriptor, is_v1_content_descriptor,
};
use mediarepo_core::error::RepoResult;
use mediarepo_database::entities::content_descriptor;
use sea_orm::prelude::*;
use sea_orm::ActiveValue::Set;
use sea_orm::ConnectionTrait;

pub struct JobDao {
    ctx: DaoContext,
}

impl DaoProvider for JobDao {
    fn dao_ctx(&self) -> DaoContext {
        self.ctx.clone()
    }
}

impl JobDao {
    pub fn new(ctx: DaoContext) -> JobDao {
        Self { ctx }
    }

    pub async fn migrate_content_descriptors(&self) -> RepoResult<()> {
        let cds: Vec<content_descriptor::Model> =
            content_descriptor::Entity::find().all(&self.ctx.db).await?;

        tracing::info!("Converting content descriptors to v2 format...");
        let mut converted_count = 0;

        for mut cd in cds {
            if is_v1_content_descriptor(&cd.descriptor) {
                let trx = self.ctx.db.begin().await?;
                let src_cd = cd.descriptor;
                let dst_cd = convert_v1_descriptor_to_v2(&src_cd)?;

                let active_model = content_descriptor::ActiveModel {
                    id: Set(cd.id),
                    descriptor: Set(dst_cd.clone()),
                };
                self.ctx.main_storage.rename_file(&src_cd, &dst_cd).await?;
                self.ctx
                    .thumbnail_storage
                    .rename_parent(
                        encode_content_descriptor(&src_cd),
                        encode_content_descriptor(&dst_cd),
                    )
                    .await?;
                trx.commit().await?;
                converted_count += 1;
            }
        }
        tracing::info!("Converted {} descriptors", converted_count);

        Ok(())
    }
}
