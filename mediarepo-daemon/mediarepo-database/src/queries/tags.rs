use crate::entities::hash;
use crate::entities::hash_tag;
use crate::entities::namespace;
use crate::entities::tag;
use sea_orm::prelude::*;
use sea_orm::sea_query::Query;
use sea_orm::{DatabaseConnection, JoinType};
use std::collections::HashMap;
/*
pub async fn get_hashes_with_namespaced_tags(
    db: DatabaseConnection,
    hash_ids: Vec<i64>,
) -> HashMap<i64, HashMap<String, String>> {
    Query::select()
        .expr(hash_tag::Column::HashId)
        .expr(tag::Column::Name)
        .expr(namespace::Column::Name)
        .from(tag::Entity)
        .join(
            JoinType::LeftJoin,
            hash_tag::Entity,
            hash_tag::Column::TagId.eq(tag::Column::Id),
        )
        .join(
            JoinType::InnerJoin,
            namespace::Entity,
            tag::Column::NamespaceId.eq(namespace::Column::Id),
        )
        .build(&db)
        .await?;
    let tags: Vec<(tag::Model, Option<namespace::Model>)> = tag::Entity::find()
        .find_also_related(namespace::Entity)
        .join(JoinType::LeftJoin, hash_tag::Relation::Tag.def().rev())
        .join(JoinType::InnerJoin, hash_tag::Relation::Hash.def())
        .filter(hash::Column::Id.eq(self.hash.id))
        .all(&self.db)
        .await?;
    let tags = tags
        .into_iter()
        .map(|(tag, namespace)| Tag::new(self.db.clone(), tag, namespace))
        .collect();
}
*/
