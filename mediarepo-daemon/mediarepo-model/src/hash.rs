use crate::file::File;
use crate::thumbnail::Thumbnail;
use mediarepo_core::error::RepoResult;
use mediarepo_database::entities::file;
use mediarepo_database::entities::hash;
use mediarepo_database::entities::thumbnail;
use sea_orm::prelude::*;
use sea_orm::{DatabaseConnection, Set};

pub struct Hash {
    db: DatabaseConnection,
    model: hash::Model,
}

impl Hash {
    pub(crate) fn new(db: DatabaseConnection, model: hash::Model) -> Self {
        Self { db, model }
    }

    /// Searches for the hash by id
    pub async fn by_id(db: DatabaseConnection, id: i64) -> RepoResult<Option<Self>> {
        let hash = hash::Entity::find_by_id(id)
            .one(&db)
            .await?
            .map(|model| Self::new(db, model));

        Ok(hash)
    }

    /// Adds a new hash to the database
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
    pub async fn file(&self) -> RepoResult<Option<File>> {
        let file = self
            .model
            .find_related(file::Entity)
            .one(&self.db)
            .await?
            .map(|file_model| File::new(self.db.clone(), file_model, self.model.clone()));

        Ok(file)
    }

    /// Returns the the thumbnail associated with the hash
    pub async fn thumbnail(&self) -> RepoResult<Option<Thumbnail>> {
        let thumbnail = self
            .model
            .find_related(thumbnail::Entity)
            .one(&self.db)
            .await?
            .map(|thumb_model| Thumbnail::new(self.db.clone(), thumb_model, self.model.clone()));

        Ok(thumbnail)
    }
}
