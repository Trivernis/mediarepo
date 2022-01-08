use crate::file::File;
use mediarepo_core::content_descriptor::convert_v1_descriptor_to_v2;
use mediarepo_core::error::RepoResult;
use mediarepo_database::entities::content_descriptor;
use mediarepo_database::entities::file;
use sea_orm::prelude::*;
use sea_orm::{DatabaseConnection, Set};
use std::fmt::Debug;

pub struct ContentDescriptor {
    db: DatabaseConnection,
    model: content_descriptor::Model,
}

impl ContentDescriptor {
    #[tracing::instrument(level = "trace")]
    pub(crate) fn new(db: DatabaseConnection, model: content_descriptor::Model) -> Self {
        Self { db, model }
    }

    pub async fn all(db: DatabaseConnection) -> RepoResult<Vec<Self>> {
        let descriptors = content_descriptor::Entity::find()
            .all(&db)
            .await?
            .into_iter()
            .map(|model| Self::new(db.clone(), model))
            .collect();

        Ok(descriptors)
    }

    /// Searches for the hash by id
    #[tracing::instrument(level = "debug", skip(db))]
    pub async fn by_id(db: DatabaseConnection, id: i64) -> RepoResult<Option<Self>> {
        let hash = content_descriptor::Entity::find_by_id(id)
            .one(&db)
            .await?
            .map(|model| Self::new(db, model));

        Ok(hash)
    }

    /// Returns the hash by value
    #[tracing::instrument(level = "debug", skip(db))]
    pub async fn by_value<D: AsRef<[u8]> + Debug>(
        db: DatabaseConnection,
        descriptor: D,
    ) -> RepoResult<Option<Self>> {
        let cid = content_descriptor::Entity::find()
            .filter(content_descriptor::Column::Descriptor.eq(descriptor.as_ref()))
            .one(&db)
            .await?
            .map(|model| Self::new(db, model));

        Ok(cid)
    }

    /// Adds a new hash to the database
    #[tracing::instrument(level = "debug", skip(db))]
    pub async fn add(db: DatabaseConnection, descriptor: Vec<u8>) -> RepoResult<Self> {
        let active_model = content_descriptor::ActiveModel {
            descriptor: Set(descriptor),
            ..Default::default()
        };
        let model = active_model.insert(&db).await?;

        Ok(Self::new(db, model))
    }

    pub fn id(&self) -> i64 {
        self.model.id
    }

    pub fn descriptor(&self) -> &[u8] {
        &self.model.descriptor[..]
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

    pub async fn convert_v1_to_v2(&mut self) -> RepoResult<()> {
        let descriptor = convert_v1_descriptor_to_v2(&self.model.descriptor)?;
        let active_model = content_descriptor::ActiveModel {
            id: Set(self.id()),
            descriptor: Set(descriptor),
        };
        self.model = active_model.update(&self.db).await?;

        Ok(())
    }
}
