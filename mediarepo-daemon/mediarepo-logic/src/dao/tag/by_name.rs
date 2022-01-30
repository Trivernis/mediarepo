use crate::dao::tag::{map_tag_dto, TagDao};
use crate::dto::TagDto;
use mediarepo_core::error::RepoResult;
use mediarepo_database::entities::{namespace, tag};
use sea_orm::prelude::*;
use sea_orm::sea_query::Expr;
use sea_orm::Condition;

#[derive(Clone, Debug)]
pub struct TagByNameQuery {
    pub namespace: Option<String>,
    pub name: String,
}

impl TagDao {
    /// Filters all tags by names
    /// wildcards are supported
    #[tracing::instrument(level = "debug", skip(self))]
    pub async fn all_by_name(&self, names: Vec<TagByNameQuery>) -> RepoResult<Vec<TagDto>> {
        let mut condition_count = 0;
        let condition = names
            .into_iter()
            .filter_map(name_query_to_condition)
            .inspect(|_| condition_count += 1)
            .fold(Condition::any(), Condition::add);
        if condition_count == 0 {
            return Ok(vec![]);
        }

        let tags = tag::Entity::find()
            .find_also_related(namespace::Entity)
            .filter(condition)
            .all(&self.ctx.db)
            .await?
            .into_iter()
            .map(map_tag_dto)
            .collect();

        Ok(tags)
    }
}

fn name_query_to_condition(query: TagByNameQuery) -> Option<Condition> {
    let TagByNameQuery { namespace, name } = query;
    let mut condition = Condition::all();

    if !name.ends_with('*') {
        condition = condition.add(tag::Column::Name.eq(name))
    } else if name.len() > 1 {
        condition =
            condition.add(tag::Column::Name.like(&*format!("{}%", name.trim_end_matches("*"))))
    } else if namespace.is_none() {
        return None;
    }

    condition = if let Some(namespace) = namespace {
        condition.add(namespace::Column::Name.eq(namespace))
    } else {
        condition.add(Expr::tbl(tag::Entity, tag::Column::NamespaceId).is_null())
    };

    Some(condition)
}
