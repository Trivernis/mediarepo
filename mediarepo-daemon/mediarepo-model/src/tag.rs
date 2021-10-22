use crate::namespace::Namespace;
use mediarepo_core::error::RepoResult;
use mediarepo_database::entities::namespace;
use mediarepo_database::entities::tag;
use sea_orm::prelude::*;
use sea_orm::{Condition, DatabaseConnection, Set};

#[derive(Clone)]
pub struct Tag {
    db: DatabaseConnection,
    model: tag::Model,
    namespace: Option<namespace::Model>,
}

impl Tag {
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
    pub async fn all(db: DatabaseConnection) -> RepoResult<Vec<Self>> {
        let tags: Vec<Self> = tag::Entity::find()
            .inner_join(namespace::Entity)
            .select_also(namespace::Entity)
            .all(&db)
            .await?
            .into_iter()
            .map(|(tag, namespace)| Self::new(db.clone(), tag, namespace))
            .collect();

        Ok(tags)
    }

    /// Returns the tag by id
    pub async fn by_id(db: DatabaseConnection, id: i64) -> RepoResult<Option<Self>> {
        let tag = tag::Entity::find_by_id(id)
            .find_also_related(namespace::Entity)
            .one(&db)
            .await?
            .map(|(model, namespace)| Self::new(db, model, namespace));

        Ok(tag)
    }

    /// Returns one tag by name and namespace
    pub async fn by_name<S1: ToString>(
        db: DatabaseConnection,
        name: S1,
        namespace: Option<String>,
    ) -> RepoResult<Option<Self>> {
        let mut entries = Self::all_by_name(db, vec![(namespace, name.to_string())]).await?;

        Ok(entries.pop())
    }

    /// Retrieves the namespaced tags by name and namespace
    pub async fn all_by_name(
        db: DatabaseConnection,
        namespaces_with_names: Vec<(Option<String>, String)>,
    ) -> RepoResult<Vec<Self>> {
        let mut or_condition = Condition::any();

        for (namespace, name) in namespaces_with_names {
            let mut all_condition = Condition::all().add(tag::Column::Name.eq(name));

            all_condition = if let Some(namespace) = namespace {
                all_condition.add(namespace::Column::Name.eq(namespace))
            } else {
                all_condition.add(tag::Column::NamespaceId.eq(Option::<i64>::None))
            };
            or_condition = or_condition.add(all_condition);
        }

        let tags: Vec<Self> = tag::Entity::find()
            .find_also_related(namespace::Entity)
            .filter(or_condition)
            .all(&db)
            .await?
            .into_iter()
            .map(|(t, n)| Self::new(db.clone(), t, n))
            .collect();

        Ok(tags)
    }

    /// Adds a new tag to the database
    pub async fn add<S: ToString>(
        db: DatabaseConnection,
        name: S,
        namespace_id: Option<i64>,
    ) -> RepoResult<Self> {
        let active_model = tag::ActiveModel {
            name: Set(name.to_string()),
            namespace_id: Set(namespace_id),
            ..Default::default()
        };
        let active_model = active_model.insert(&db).await?;
        let tag = Self::by_id(db, active_model.id.unwrap()).await?.unwrap();

        Ok(tag)
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
}
