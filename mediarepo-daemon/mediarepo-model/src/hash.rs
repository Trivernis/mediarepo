use crate::file::File;
use mediarepo_core::error::RepoResult;
use mediarepo_database::entities::file;
use mediarepo_database::entities::hash;
use sea_orm::prelude::*;
use sea_orm::{DatabaseConnection, Set};
use std::fmt::Debug;

pub struct Hash {
    db: DatabaseConnection,
    model: hash::Model,
}

impl Hash {
    #[tracing::instrument(level = "trace")]
    pub(crate) fn new(db: DatabaseConnection, model: hash::Model) -> Self {
        Self { db, model }
    }

    /// Searches for the hash by id
    #[tracing::instrument(level = "debug", skip(db))]
    pub async fn by_id(db: DatabaseConnection, id: i64) -> RepoResult<Option<Self>> {
        let hash = hash::Entity::find_by_id(id)
            .one(&db)
            .await?
            .map(|model| Self::new(db, model));

        Ok(hash)
    }

    /// Returns the hash by value
    #[tracing::instrument(level = "debug", skip(db))]
    pub async fn by_value<S: AsRef<str> + Debug>(
        db: DatabaseConnection,
        value: S,
    ) -> RepoResult<Option<Self>> {
        let hash = hash::Entity::find()
            .filter(hash::Column::Value.eq(value.as_ref()))
            .one(&db)
            .await?
            .map(|model| Self::new(db, model));

        Ok(hash)
    }

    /// Adds a new hash to the database
    #[tracing::instrument(level = "debug", skip(db))]
    pub async fn add(db: DatabaseConnection, value: String) -> RepoResult<Self> {
        let active_model = hash::ActiveModel {
            value: Set(value),
            ..Default::default()
        };
        let active_model: hash::ActiveModel = active_model.insert(&db).await?;
        let hash = Self::by_id(db, active_model.id.unwrap())
            .await?
            .expect("Inserted value does not exist");

        Ok(hash)
    }

    pub fn id(&self) -> i64 {
        self.model.id
    }

    pub fn value(&self) -> &String {
        &self.model.value
    }

    /// Returns the file associated with the hash
    #[tracing::instrument(level = "debug", skip(self))]
    pub async fn file(&self) -> RepoResult<Option<File>> {
        let file = self
            .model
            .find_related(file::Entity)
            .one(&self.db)
            .await?
            .map(|file_model| File::new(self.db.clone(), file_model, self.model.clone()));

        Ok(file)
    }
}
