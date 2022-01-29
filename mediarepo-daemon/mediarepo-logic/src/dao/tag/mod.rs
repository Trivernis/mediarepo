use sea_orm::prelude::*;
use sea_orm::JoinType;
use sea_orm::QuerySelect;

use mediarepo_core::error::RepoResult;
use mediarepo_database::entities::{content_descriptor, content_descriptor_tag, namespace, tag};

use crate::dao::{DaoContext, DaoProvider};
use crate::dto::{NamespaceDto, TagDto};

pub mod mappings;

pub struct TagDao {
    ctx: DaoContext,
}

impl DaoProvider for TagDao {
    fn dao_ctx(&self) -> DaoContext {
        self.ctx.clone()
    }
}

impl TagDao {
    pub fn new(ctx: DaoContext) -> Self {
        Self { ctx }
    }

    #[tracing::instrument(level = "debug", skip(self))]
    pub async fn all(&self) -> RepoResult<Vec<TagDto>> {
        let tags = tag::Entity::find()
            .find_also_related(namespace::Entity)
            .all(&self.ctx.db)
            .await?
            .into_iter()
            .map(map_tag_dto)
            .collect();

        Ok(tags)
    }

    #[tracing::instrument(level = "debug", skip(self))]
    pub async fn all_namespaces(&self) -> RepoResult<Vec<NamespaceDto>> {
        let namespaces = namespace::Entity::find()
            .all(&self.ctx.db)
            .await?
            .into_iter()
            .map(NamespaceDto::new)
            .collect();

        Ok(namespaces)
    }

    #[tracing::instrument(level = "debug", skip(self))]
    pub async fn tags_for_cd(&self, cd_id: i64) -> RepoResult<Vec<TagDto>> {
        let tags = tag::Entity::find()
            .find_also_related(namespace::Entity)
            .join(
                JoinType::LeftJoin,
                content_descriptor_tag::Relation::Tag.def().rev(),
            )
            .join(
                JoinType::InnerJoin,
                content_descriptor_tag::Relation::ContentDescriptorId.def(),
            )
            .filter(content_descriptor::Column::Id.eq(cd_id))
            .all(&self.ctx.db)
            .await?
            .into_iter()
            .map(map_tag_dto)
            .collect();

        Ok(tags)
    }
}

fn map_tag_dto(result: (tag::Model, Option<namespace::Model>)) -> TagDto {
    TagDto::new(result.0, result.1)
}
