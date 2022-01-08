use std::fmt::Debug;

use mediarepo_core::error::RepoResult;
use mediarepo_database::entities::content_descriptor;
use mediarepo_database::entities::content_descriptor_tag;
use mediarepo_database::entities::namespace;
use mediarepo_database::entities::tag;
use sea_orm::prelude::*;
use sea_orm::query::ConnectionTrait;
use sea_orm::sea_query::Expr;
use sea_orm::{Condition, DatabaseBackend, DatabaseConnection, JoinType, Set, Statement};
use sea_orm::{InsertResult, QuerySelect};

use crate::namespace::Namespace;

#[derive(Clone)]
pub struct Tag {
    db: DatabaseConnection,
    model: tag::Model,
    namespace: Option<namespace::Model>,
}

impl Tag {
    #[tracing::instrument(level = "trace")]
    pub(crate) fn new(
        db: DatabaseConnection,
        model: tag::Model,
        namespace: Option<namespace::Model>,
    ) -> Self {
        Self {
            db,
            model,
            namespace,
        }
    }

    /// Returns all tags stored in the database
    #[tracing::instrument(level = "debug", skip(db))]
    pub async fn all(db: DatabaseConnection) -> RepoResult<Vec<Self>> {
        let tags: Vec<Self> = tag::Entity::find()
            .left_join(namespace::Entity)
            .select_also(namespace::Entity)
            .all(&db)
            .await?
            .into_iter()
            .map(|(tag, namespace)| Self::new(db.clone(), tag, namespace))
            .collect();

        Ok(tags)
    }

    /// Returns the tag by id
    #[tracing::instrument(level = "debug", skip(db))]
    pub async fn by_id(db: DatabaseConnection, id: i64) -> RepoResult<Option<Self>> {
        let tag = tag::Entity::find_by_id(id)
            .find_also_related(namespace::Entity)
            .one(&db)
            .await?
            .map(|(model, namespace)| Self::new(db, model, namespace));

        Ok(tag)
    }

    /// Returns one tag by name and namespace
    #[tracing::instrument(level = "debug", skip(db))]
    pub async fn by_name<S1: ToString + Debug>(
        db: DatabaseConnection,
        name: S1,
        namespace: Option<String>,
    ) -> RepoResult<Option<Self>> {
        let mut entries = Self::all_by_name(db, vec![(namespace, name.to_string())]).await?;

        Ok(entries.pop())
    }

    /// Retrieves the namespaced tags by name and namespace
    #[tracing::instrument(level = "debug", skip(db))]
    pub async fn all_by_name(
        db: DatabaseConnection,
        namespaces_with_names: Vec<(Option<String>, String)>,
    ) -> RepoResult<Vec<Self>> {
        if namespaces_with_names.is_empty() {
            return Ok(vec![]);
        }
        let mut or_condition = Condition::any();

        for (namespace, name) in namespaces_with_names {
            let mut all_condition = Condition::all();
            if !name.ends_with('*') {
                all_condition = all_condition.add(tag::Column::Name.eq(name))
            } else if name.len() > 1 {
                all_condition = all_condition
                    .add(tag::Column::Name.like(&*format!("{}%", name.trim_end_matches("*"))))
            } else if namespace.is_none() {
                continue; // would result in an empty condition otherwise
            }

            all_condition = if let Some(namespace) = namespace {
                all_condition.add(namespace::Column::Name.eq(namespace))
            } else {
                all_condition.add(Expr::tbl(tag::Entity, tag::Column::NamespaceId).is_null())
            };
            or_condition = or_condition.add(all_condition);
        }

        let tags: Vec<Self> = tag::Entity::find()
            .find_also_related(namespace::Entity)
            .filter(or_condition)
            .group_by(tag::Column::Id)
            .all(&db)
            .await?
            .into_iter()
            .map(|(t, n)| Self::new(db.clone(), t, n))
            .collect();

        Ok(tags)
    }

    /// Returns all tags that are assigned to any of the passed hashes
    #[tracing::instrument(level = "debug", skip_all)]
    pub async fn for_cd_list(db: DatabaseConnection, cds: Vec<Vec<u8>>) -> RepoResult<Vec<Self>> {
        let tags: Vec<Self> = tag::Entity::find()
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
            .all(&db)
            .await?
            .into_iter()
            .map(|(t, n)| Self::new(db.clone(), t, n))
            .collect();

        Ok(tags)
    }

    pub async fn add_all(
        db: DatabaseConnection,
        namespaces_with_names: Vec<(Option<i64>, String)>,
    ) -> RepoResult<Vec<Self>> {
        if namespaces_with_names.is_empty() {
            return Ok(vec![]);
        }
        let models: Vec<tag::ActiveModel> = namespaces_with_names
            .into_iter()
            .map(|(namespace_id, name)| tag::ActiveModel {
                name: Set(name),
                namespace_id: Set(namespace_id),
                ..Default::default()
            })
            .collect();
        let txn = db.begin().await?;
        let last_id: i64 = txn
            .query_one(Statement::from_string(
                DatabaseBackend::Sqlite,
                r#"SELECT MAX(id) as "max_id" FROM tags"#.to_owned(),
            ))
            .await?
            .and_then(|res| res.try_get("", "max_id").ok())
            .unwrap_or(-1);

        let result: InsertResult<tag::ActiveModel> =
            tag::Entity::insert_many(models).exec(&txn).await?;
        let tags: Vec<Self> = tag::Entity::find()
            .find_also_related(namespace::Entity)
            .filter(tag::Column::Id.between(last_id, result.last_insert_id + 1))
            .all(&txn)
            .await?
            .into_iter()
            .map(|(t, n)| Self::new(db.clone(), t, n))
            .collect();
        txn.commit().await?;

        Ok(tags)
    }

    /// Adds a new tag to the database
    #[tracing::instrument(level = "debug", skip(db))]
    pub async fn add<S: ToString + Debug>(
        db: DatabaseConnection,
        name: S,
        namespace_id: Option<i64>,
    ) -> RepoResult<Self> {
        let active_model = tag::ActiveModel {
            name: Set(name.to_string()),
            namespace_id: Set(namespace_id),
            ..Default::default()
        };
        let model: tag::Model = active_model.insert(&db).await?;
        let namespace = model.find_related(namespace::Entity).one(&db).await?;

        Ok(Self::new(db, model, namespace))
    }

    /// The ID of the tag
    pub fn id(&self) -> i64 {
        self.model.id
    }

    /// The name of the tag
    pub fn name(&self) -> &String {
        &self.model.name
    }

    /// The namespace of the tag
    pub fn namespace(&self) -> Option<Namespace> {
        self.namespace
            .clone()
            .map(|n| Namespace::new(self.db.clone(), n))
    }

    /// Returns the normalized name of the tag (namespace:tag)
    pub fn normalized_name(&self) -> String {
        if let Some(namespace) = &self.namespace {
            format!("{}:{}", namespace.name, self.model.name)
        } else {
            self.model.name.to_owned()
        }
    }
}
