use mediarepo_core::error::RepoResult;
use mediarepo_core::file_hash_store::FileHashStore;
use mediarepo_database::entities::storage;
use mediarepo_database::entities::storage::ActiveModel as ActiveStorage;
use mediarepo_database::entities::storage::Model as StorageModel;
use sea_orm::prelude::*;
use sea_orm::{DatabaseConnection, Set, Unset};
use std::path::PathBuf;
use tokio::fs;
use tokio::io::{AsyncRead, BufReader};

#[derive(Clone)]
pub struct Storage {
    db: DatabaseConnection,
    model: StorageModel,
    store: FileHashStore,
}

impl Storage {
    fn new(db: DatabaseConnection, model: StorageModel) -> Self {
        let path = PathBuf::from(&model.path);
        Self {
            store: FileHashStore::new(path),
            db,
            model,
        }
    }

    /// Returns all available storages
    pub async fn all(db: DatabaseConnection) -> RepoResult<Vec<Self>> {
        let storages: Vec<storage::Model> = storage::Entity::find().all(&db).await?;
        let storages = storages
            .into_iter()
            .map(|s| Self::new(db.clone(), s))
            .collect();

        Ok(storages)
    }

    /// Returns the storage by id
    pub async fn by_id(db: DatabaseConnection, id: i64) -> RepoResult<Option<Self>> {
        if let Some(model) = storage::Entity::find_by_id(id).one(&db).await? {
            let storage = Self::new(db, model);
            Ok(Some(storage))
        } else {
            Ok(None)
        }
    }

    /// Returns the storage by path
    pub async fn by_path<S: ToString>(db: DatabaseConnection, path: S) -> RepoResult<Option<Self>> {
        if let Some(model) = storage::Entity::find()
            .filter(storage::Column::Path.eq(path.to_string()))
            .one(&db)
            .await?
        {
            let storage = Self::new(db, model);
            Ok(Some(storage))
        } else {
            Ok(None)
        }
    }

    /// Creates a new active storage and also creates the associated directory
    /// if it doesn't exist yet.
    pub async fn create<S1: ToString, S2: ToString>(
        db: DatabaseConnection,
        name: S1,
        path: S2,
    ) -> RepoResult<Self> {
        let path = path.to_string();
        let name = name.to_string();
        let path_buf = PathBuf::from(&path);

        if !path_buf.exists() {
            fs::create_dir(path_buf).await?;
        }
        let storage = ActiveStorage {
            id: Unset(None),
            name: Set(name),
            path: Set(path),
            ..Default::default()
        };
        let storage: ActiveStorage = storage.insert(&db).await?;
        let storage = Self::by_id(db, storage.id.unwrap())
            .await?
            .expect("Inserted storage doesn't exist?!");

        Ok(storage)
    }

    /// Returns the unique identifier of this storage
    pub fn id(&self) -> i64 {
        self.model.id
    }

    /// Returns the name of the storage
    pub fn name(&self) -> &String {
        &self.model.name
    }

    /// Returns the path of the storage
    pub fn path(&self) -> &String {
        &self.model.path
    }

    /// Sets a new name for the storage
    pub async fn set_name<S: ToString>(&self, name: S) -> RepoResult<()> {
        let mut active_storage: ActiveStorage = self.get_active_model();
        active_storage.name = Set(name.to_string());
        active_storage.update(&self.db).await?;

        Ok(())
    }

    /// Sets a new path for the storage. This will only update the database record
    /// so if the physical part of the storage is already created it needs to be migrated first
    pub async fn set_path<S: ToString>(&mut self, path: S) -> RepoResult<()> {
        let mut active_storage: ActiveStorage = self.get_active_model();
        active_storage.path = Set(path.to_string());
        let storage: ActiveStorage = active_storage.update(&self.db).await?;
        self.model.path = storage.path.unwrap();

        Ok(())
    }

    /// Checks if the storage exists on the harddrive
    pub fn exists(&self) -> bool {
        let path = PathBuf::from(&self.path());

        path.exists()
    }

    /// Adds a file to the store
    pub async fn add_file<R: AsyncRead + Unpin>(&self, reader: R) -> RepoResult<String> {
        self.store.add_file(reader, None).await
    }

    /// Returns the buf reader to the given hash
    pub async fn get_file_reader<S: ToString>(
        &self,
        hash: S,
    ) -> RepoResult<BufReader<tokio::fs::File>> {
        let (_ext, reader) = self.store.get_file(hash.to_string()).await?;

        Ok(reader)
    }

    /// Returns the active model with only the ID filled so saves always perform an update
    fn get_active_model(&self) -> ActiveStorage {
        ActiveStorage {
            id: Set(self.model.id),
            ..Default::default()
        }
    }
}
