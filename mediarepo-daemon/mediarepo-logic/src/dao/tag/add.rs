use crate::dao::tag::{map_tag_dto, TagDao};
use crate::dto::{AddTagDto, NamespaceDto, TagDto};
use mediarepo_core::error::RepoResult;
use mediarepo_database::entities::{namespace, tag};
use sea_orm::prelude::*;
use sea_orm::ActiveValue::Set;
use sea_orm::{Condition, ConnectionTrait, DatabaseTransaction};
use std::collections::HashMap;
use std::iter::FromIterator;

impl TagDao {
    #[tracing::instrument(level = "debug", skip(self))]
    pub async fn add_all(&self, mut tags: Vec<AddTagDto>) -> RepoResult<Vec<TagDto>> {
        let namespaces = tags.iter().filter_map(|t| t.namespace.clone()).collect();
        let trx = self.ctx.db.begin().await?;
        let existing_tags = tags_by_name(&trx, tags.clone()).await?;

        if existing_tags.len() == tags.len() {
            return Ok(existing_tags);
        }
        let existing_tag_map: HashMap<String, TagDto> =
            HashMap::from_iter(existing_tags.into_iter().map(|t| (t.normalized_name(), t)));

        let namespace_map = add_or_get_all_namespaces(&trx, namespaces).await?;
        tags.retain(|dto| !existing_tag_map.contains_key(&dto.normalized_name()));
        let tag_models: Vec<tag::ActiveModel> = tags
            .iter()
            .map(|t| tag::ActiveModel {
                name: Set(t.name.to_owned()),
                namespace_id: Set(t
                    .namespace
                    .as_ref()
                    .and_then(|n| namespace_map.get(n))
                    .map(|n| n.id())),
                ..Default::default()
            })
            .collect();
        tag::Entity::insert_many(tag_models).exec(&trx).await?;
        let mut tag_dtos = tags_by_name(&trx, tags).await?;
        trx.commit().await?;
        tag_dtos.append(&mut existing_tag_map.into_iter().map(|(_, dto)| dto).collect());

        Ok(tag_dtos)
    }
}

async fn add_or_get_all_namespaces(
    trx: &DatabaseTransaction,
    mut namespaces: Vec<String>,
) -> RepoResult<HashMap<String, NamespaceDto>> {
    let existing_namespaces = namespaces_by_name(trx, namespaces.clone()).await?;
    let mut namespace_map = HashMap::from_iter(
        existing_namespaces
            .into_iter()
            .map(|nsp| (nsp.name().to_owned(), nsp)),
    );
    if namespaces.len() == namespace_map.len() {
        return Ok(namespace_map);
    }
    namespaces.retain(|nsp| !namespace_map.contains_key(nsp));
    let namespace_models: Vec<namespace::ActiveModel> = namespaces
        .iter()
        .map(|nsp| namespace::ActiveModel {
            name: Set(nsp.to_owned()),
            ..Default::default()
        })
        .collect();
    namespace::Entity::insert_many(namespace_models)
        .exec(trx)
        .await?;
    let additional_namespaces = namespaces_by_name(trx, namespaces.clone()).await?;

    for nsp in additional_namespaces {
        namespace_map.insert(nsp.name().to_owned(), nsp);
    }

    Ok(namespace_map)
}

async fn namespaces_by_name(
    trx: &DatabaseTransaction,
    names: Vec<String>,
) -> RepoResult<Vec<NamespaceDto>> {
    let namespaces: Vec<NamespaceDto> = namespace::Entity::find()
        .filter(namespace::Column::Name.is_in(names))
        .all(trx)
        .await?
        .into_iter()
        .map(NamespaceDto::new)
        .collect();

    Ok(namespaces)
}

async fn tags_by_name(trx: &DatabaseTransaction, tags: Vec<AddTagDto>) -> RepoResult<Vec<TagDto>> {
    let condition = tags
        .into_iter()
        .map(build_tag_condition)
        .fold(Condition::any(), Condition::add);
    let tags = tag::Entity::find()
        .find_also_related(namespace::Entity)
        .filter(condition)
        .all(trx)
        .await?
        .into_iter()
        .map(map_tag_dto)
        .collect();

    Ok(tags)
}

fn build_tag_condition(tag: AddTagDto) -> Condition {
    if let Some(namespace) = tag.namespace {
        Condition::all()
            .add(tag::Column::Name.eq(tag.name))
            .add(namespace::Column::Name.eq(namespace))
    } else {
        Condition::all().add(tag::Column::Name.eq(tag.name))
    }
}
