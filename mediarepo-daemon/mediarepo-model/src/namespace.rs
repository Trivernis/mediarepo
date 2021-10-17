use mediarepo_core::error::RepoResult;
use mediarepo_database::entities::namespace;
use sea_orm::prelude::*;
use sea_orm::{DatabaseConnection, Set};

#[derive(Clone)]
pub struct Namespace {
    db: DatabaseConnection,
    model: namespace::Model,
}

impl Namespace {
    pub(crate) fn new(db: DatabaseConnection, model: namespace::Model) -> Self {
        Self { db, model }
    }

    /// Retrieves the namespace by id
    pub async fn by_id(db: DatabaseConnection, id: i64) -> RepoResult<Option<Self>> {
        let namespace = namespace::Entity::find_by_id(id)
            .one(&db)
            .await?
            .map(|model| Self::new(db, model));

        Ok(namespace)
    }

    /// Retrieves a namespace by its name
    pub async fn by_name<S: AsRef<str>>(
        db: DatabaseConnection,
        name: S,
    ) -> RepoResult<Option<Self>> {
        let namespace = namespace::Entity::find()
            .filter(namespace::Column::Name.eq(name.as_ref()))
            .one(&db)
            .await?
            .map(|model| Self::new(db, model));

        Ok(namespace)
    }

    /// Adds a namespace to the database
    pub async fn add<S: ToString>(db: DatabaseConnection, name: S) -> RepoResult<Self> {
        let active_model = namespace::ActiveModel {
            name: Set(name.to_string()),
            ..Default::default()
        };
        let active_model = active_model.insert(&db).await?;
        let namespace = Self::by_id(db, active_model.id.unwrap()).await?.unwrap();

        Ok(namespace)
    }

    /// The ID of the namespace
    pub fn id(&self) -> i64 {
        self.model.id
    }

    /// The name of the namespace
    pub fn name(&self) -> &String {
        &self.model.name
    }
}
