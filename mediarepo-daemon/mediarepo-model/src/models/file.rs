use crate::models::file_type::FileType;
use crate::models::storage::Storage;
use chrono::NaiveDateTime;
use mediarepo_core::error::RepoResult;
use mediarepo_database::entities::file;
use mediarepo_database::entities::file::ActiveModel as ActiveFile;
use mediarepo_database::entities::file::Model as FileModel;
use mediarepo_database::entities::hash;
use mediarepo_database::entities::hash::Model as HashModel;
use sea_orm::prelude::*;
use sea_orm::{DatabaseConnection, Set};

pub struct File {
    db: DatabaseConnection,
    model: FileModel,
    hash: HashModel,
}

impl File {
    fn new(db: DatabaseConnection, model: FileModel, hash: HashModel) -> Self {
        Self { db, model, hash }
    }

    /// Fetches the file by id
    pub async fn by_id(db: DatabaseConnection, id: u64) -> RepoResult<Option<Self>> {
        if let Some((model, Some(hash))) = file::Entity::find_by_id(id)
            .find_also_related(hash::Entity)
            .one(&db)
            .await?
        {
            let file = File::new(db, model, hash);
            Ok(Some(file))
        } else {
            Ok(None)
        }
    }

    /// Returns the unique identifier of the file
    pub fn id(&self) -> u64 {
        self.model.id
    }

    /// Returns the hash of the file (content identifier)
    pub fn hash(&self) -> &String {
        &self.hash.value
    }

    /// Returns the type of the file
    pub fn file_type(&self) -> FileType {
        match self.model.file_type {
            1 => FileType::Image,
            2 => FileType::Video,
            3 => FileType::Audio,
            _ => FileType::Unknown,
        }
    }

    /// Returns the optional name of the file
    pub fn name(&self) -> &Option<String> {
        &self.model.name
    }

    /// Returns the comment of the file
    pub fn comment(&self) -> &Option<String> {
        &self.model.comment
    }

    /// Returns the import time of the file
    pub fn import_time(&self) -> &NaiveDateTime {
        &self.model.import_time
    }

    /// Returns the datetime when the file was created
    pub fn creation_time(&self) -> &NaiveDateTime {
        &self.model.creation_time
    }

    /// Returns the last time the file was changed
    pub fn change_time(&self) -> &NaiveDateTime {
        &self.model.change_time
    }

    /// Returns the storage where the file is stored
    pub async fn storage(&self) -> RepoResult<Storage> {
        let storage = Storage::by_id(self.db.clone(), self.model.storage_id)
            .await?
            .expect("The FK storage_id doesn't exist?!");

        Ok(storage)
    }

    /// Changes the name of the file
    pub async fn set_name<S: ToString>(&mut self, name: S) -> RepoResult<()> {
        let mut active_file = self.get_active_model();
        active_file.name = Set(Some(name.to_string()));
        let active_file = active_file.update(&self.db).await?;
        self.model.name = active_file.name.unwrap();

        Ok(())
    }

    /// Changes the comment of the file
    pub async fn set_comment<S: ToString>(&mut self, comment: S) -> RepoResult<()> {
        let mut active_file = self.get_active_model();
        active_file.comment = Set(Some(comment.to_string()));
        let active_file = active_file.update(&self.db).await?;
        self.model.comment = active_file.comment.unwrap();

        Ok(())
    }

    /// Changes the type of the file
    pub async fn set_file_type(&mut self, file_type: FileType) -> RepoResult<()> {
        let mut active_file = self.get_active_model();
        active_file.file_type = Set(file_type as u32);
        let active_file = active_file.update(&self.db).await?;
        self.model.file_type = active_file.file_type.unwrap();

        Ok(())
    }

    /// Returns the active model of the file with only the id set
    fn get_active_model(&self) -> ActiveFile {
        ActiveFile {
            id: Set(self.id()),
            ..Default::default()
        }
    }
}
