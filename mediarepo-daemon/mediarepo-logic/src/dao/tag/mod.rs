use sea_orm::prelude::*;
use sea_orm::JoinType;
use sea_orm::QuerySelect;
use std::collections::HashMap;
use std::iter::FromIterator;

use mediarepo_core::error::RepoResult;

use mediarepo_core::utils::parse_namespace_and_tag;
use mediarepo_database::entities::{content_descriptor, content_descriptor_tag, namespace, tag};

use crate::dao::tag::by_name::TagByNameQuery;
use crate::dao_provider;
use crate::dto::{NamespaceDto, TagDto};

pub mod add;
pub mod all_for_cds_map;
pub mod by_name;
pub mod cdids_with_namespaced_tags;
pub mod mappings;

dao_provider!(TagDao);

impl TagDao {
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

    #[tracing::instrument(level = "debug", skip(self, cds))]
    pub async fn all_for_cds(&self, cds: Vec<Vec<u8>>) -> RepoResult<Vec<TagDto>> {
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
            .filter(content_descriptor::Column::Descriptor.is_in(cds))
            .group_by(tag::Column::Id)
            .all(&self.ctx.db)
            .await?
            .into_iter()
            .map(map_tag_dto)
            .collect();

        Ok(tags)
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

    /// Returns a map mapping tag names to ids
    #[tracing::instrument(level = "debug", skip(self))]
    pub async fn normalized_tags_to_ids(
        &self,
        names: Vec<String>,
    ) -> RepoResult<HashMap<String, i64>> {
        let queries = names
            .into_iter()
            .map(parse_namespace_and_tag)
            .map(|(namespace, name)| TagByNameQuery { namespace, name })
            .collect();
        let tags = self.all_by_name(queries).await?;
        let tag_map = HashMap::from_iter(
            tags.into_iter()
                .map(|tag| (tag.normalized_name(), tag.id())),
        );

        Ok(tag_map)
    }
}

fn map_tag_dto(result: (tag::Model, Option<namespace::Model>)) -> TagDto {
    TagDto::new(result.0, result.1)
}
