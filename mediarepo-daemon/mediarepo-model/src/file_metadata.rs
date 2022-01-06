use std::fmt::Debug;

use chrono::{Local, NaiveDateTime};
use sea_orm::prelude::*;
use sea_orm::{DatabaseConnection, Set};

use mediarepo_core::error::RepoResult;
use mediarepo_database::entities::file_metadata;

#[derive(Clone)]
pub struct FileMetadata {
    db: DatabaseConnection,
    model: file_metadata::Model,
}

impl FileMetadata {
    #[tracing::instrument(level = "trace")]
    pub(crate) fn new(db: DatabaseConnection, model: file_metadata::Model) -> Self {
        Self { db, model }
    }

    /// Fetches the file by id
    #[tracing::instrument(level = "debug", skip(db))]
    pub async fn by_id(db: DatabaseConnection, id: i64) -> RepoResult<Option<Self>> {
        let file_metadata = file_metadata::Entity::find_by_id(id)
            .one(&db)
            .await?
            .map(|m| FileMetadata::new(db, m));

        Ok(file_metadata)
    }

    /// Adds a file with its hash to the database
    #[tracing::instrument(level = "debug", skip(db))]
    pub(crate) async fn add(
        db: DatabaseConnection,
        file_id: i64,
        size: i64,
        creation_time: NaiveDateTime,
        change_time: NaiveDateTime,
    ) -> RepoResult<Self> {
        let file = file_metadata::ActiveModel {
            file_id: Set(file_id),
            size: Set(size),
            import_time: Set(Local::now().naive_local()),
            creation_time: Set(creation_time),
            change_time: Set(change_time),
            ..Default::default()
        };
        let model = file.insert(&db).await?;

        Ok(Self::new(db, model))
    }

    pub fn file_id(&self) -> i64 {
        self.model.file_id
    }

    pub fn size(&self) -> i64 {
        self.model.size
    }

    pub fn name(&self) -> &Option<String> {
        &self.model.name
    }

    pub fn comment(&self) -> &Option<String> {
        &self.model.comment
    }

    pub fn import_time(&self) -> &NaiveDateTime {
        &self.model.import_time
    }

    pub fn creation_time(&self) -> &NaiveDateTime {
        &self.model.creation_time
    }

    pub fn change_time(&self) -> &NaiveDateTime {
        &self.model.change_time
    }

    /// Changes the name of the file
    #[tracing::instrument(level = "debug", skip(self))]
    pub async fn set_name<S: ToString + Debug>(&mut self, name: S) -> RepoResult<()> {
        let mut active_model = self.get_active_model();
        active_model.name = Set(Some(name.to_string()));
        self.model = active_model.update(&self.db).await?;

        Ok(())
    }

    /// Changes the comment of the file
    #[tracing::instrument(level = "debug", skip(self))]
    pub async fn set_comment<S: ToString + Debug>(&mut self, comment: S) -> RepoResult<()> {
        let mut active_file = self.get_active_model();
        active_file.comment = Set(Some(comment.to_string()));
        self.model = active_file.update(&self.db).await?;

        Ok(())
    }

    /// Returns the active model of the file with only the id set
    fn get_active_model(&self) -> file_metadata::ActiveModel {
        file_metadata::ActiveModel {
            file_id: Set(self.file_id()),
            ..Default::default()
        }
    }
}
