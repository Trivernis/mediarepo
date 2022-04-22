use crate::dao::tag::{map_tag_dto, TagDao};
use crate::dto::TagDto;
use mediarepo_core::error::RepoResult;
use mediarepo_core::itertools::Itertools;
use mediarepo_database::entities::{content_descriptor, content_descriptor_tag, namespace, tag};
use sea_orm::prelude::*;
use std::collections::HashMap;

impl TagDao {
    #[tracing::instrument(level = "debug", skip(self, cds))]
    pub async fn all_for_cds_map(
        &self,
        cds: Vec<Vec<u8>>,
    ) -> RepoResult<HashMap<Vec<u8>, Vec<TagDto>>> {
        let mut cd_tag_map = cds
            .iter()
            .cloned()
            .map(|cd| (cd, Vec::new()))
            .collect::<HashMap<Vec<u8>, Vec<TagDto>>>();

        let tag_cd_entries = tags_for_cds(&self.ctx.db, cds).await?;

        let tag_ids: Vec<i64> = tag_cd_entries
            .iter()
            .map(|(t, _)| t.tag_id)
            .unique()
            .collect();

        let tags = tags_for_tag_ids(&self.ctx.db, tag_ids).await?;

        let tag_id_map = tags
            .into_iter()
            .map(|t| (t.id(), t))
            .collect::<HashMap<i64, TagDto>>();
        let existing_cds_with_tags = create_cd_tag_map(tag_cd_entries, tag_id_map);
        cd_tag_map.extend(existing_cds_with_tags.into_iter());

        Ok(cd_tag_map)
    }
}

async fn tags_for_cds(
    db: &DatabaseConnection,
    cds: Vec<Vec<u8>>,
) -> RepoResult<
    Vec<(
        content_descriptor_tag::Model,
        Option<content_descriptor::Model>,
    )>,
> {
    let tag_cd_entries = content_descriptor_tag::Entity::find()
        .find_also_related(content_descriptor::Entity)
        .filter(content_descriptor::Column::Descriptor.is_in(cds))
        .all(db)
        .await?;

    Ok(tag_cd_entries)
}

async fn tags_for_tag_ids(db: &DatabaseConnection, ids: Vec<i64>) -> RepoResult<Vec<TagDto>> {
    let tags = tag::Entity::find()
        .find_also_related(namespace::Entity)
        .filter(tag::Column::Id.is_in(ids))
        .all(db)
        .await?
        .into_iter()
        .map(map_tag_dto)
        .collect();

    Ok(tags)
}

fn create_cd_tag_map(
    tag_cd_entries: Vec<(
        content_descriptor_tag::Model,
        Option<content_descriptor::Model>,
    )>,
    tag_id_map: HashMap<i64, TagDto>,
) -> HashMap<Vec<u8>, Vec<TagDto>> {
    tag_cd_entries
        .into_iter()
        .filter_map(|(t, cd)| Some((cd?, tag_id_map.get(&t.tag_id)?.clone())))
        .sorted_by_key(|(cd, _)| cd.id)
        .group_by(|(cd, _)| cd.descriptor.to_owned())
        .into_iter()
        .map(|(key, group)| (key, group.map(|(_, t)| t).collect::<Vec<TagDto>>()))
        .collect::<HashMap<Vec<u8>, Vec<TagDto>>>()
}
