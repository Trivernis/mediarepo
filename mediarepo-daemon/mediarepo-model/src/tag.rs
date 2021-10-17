use crate::namespace::Namespace;
use mediarepo_core::error::RepoResult;
use mediarepo_database::entities::namespace;
use mediarepo_database::entities::tag;
use sea_orm::prelude::*;
use sea_orm::QuerySelect;
use sea_orm::{DatabaseConnection, JoinType, Set};

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
            .find_also_related(namespace::Entity)
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

    /// Retrieves the unnamespaced tag by name
    pub async fn by_name<S: AsRef<str>>(
        db: DatabaseConnection,
        name: S,
    ) -> RepoResult<Option<Self>> {
        let tag = tag::Entity::find()
            .filter(tag::Column::Name.eq(name.as_ref()))
            .filter(tag::Column::NamespaceId.eq(Option::<i64>::None))
            .one(&db)
            .await?
            .map(|t| Tag::new(db, t, None));

        Ok(tag)
    }

    /// Retrieves the namespaced tag by name and namespace
    pub async fn by_name_and_namespace<S1: AsRef<str>, S2: AsRef<str>>(
        db: DatabaseConnection,
        name: S1,
        namespace: S2,
    ) -> RepoResult<Option<Self>> {
        let tag = tag::Entity::find()
            .find_also_related(namespace::Entity)
            .join(JoinType::InnerJoin, namespace::Relation::Tag.def())
            .filter(namespace::Column::Name.eq(namespace.as_ref()))
            .filter(tag::Column::Name.eq(name.as_ref()))
            .one(&db)
            .await?
            .map(|(t, n)| Self::new(db.clone(), t, n));

        Ok(tag)
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
