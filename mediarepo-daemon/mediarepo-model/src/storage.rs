use crate::hash::Hash;
use mediarepo_core::error::RepoResult;
use mediarepo_core::fs::file_hash_store::FileHashStore;
use mediarepo_database::entities::storage;
use mediarepo_database::entities::storage::ActiveModel as ActiveStorage;
use mediarepo_database::entities::storage::Model as StorageModel;
use sea_orm::prelude::*;
use sea_orm::{DatabaseConnection, NotSet, Set};
use std::fmt::Debug;
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
    #[tracing::instrument(level = "trace")]
    fn new(db: DatabaseConnection, model: StorageModel) -> Self {
        let path = PathBuf::from(&model.path);
        Self {
            store: FileHashStore::new(path),
            db,
            model,
        }
    }

    /// Returns all available storages
    #[tracing::instrument(level = "debug", skip(db))]
    pub async fn all(db: DatabaseConnection) -> RepoResult<Vec<Self>> {
        let storages: Vec<storage::Model> = storage::Entity::find().all(&db).await?;
        let storages = storages
            .into_iter()
            .map(|s| Self::new(db.clone(), s))
            .collect();

        Ok(storages)
    }

    /// Returns the storage by id
    #[tracing::instrument(level = "debug", skip(db))]
    pub async fn by_id(db: DatabaseConnection, id: i64) -> RepoResult<Option<Self>> {
        if let Some(model) = storage::Entity::find_by_id(id).one(&db).await? {
            let storage = Self::new(db, model);
            Ok(Some(storage))
        } else {
            Ok(None)
        }
    }

    /// Returns the storage by name
    #[tracing::instrument(level = "debug", skip(db))]
    pub async fn by_name<S: AsRef<str> + Debug>(
        db: DatabaseConnection,
        name: S,
    ) -> RepoResult<Option<Self>> {
        if let Some(model) = storage::Entity::find()
            .filter(storage::Column::Name.eq(name.as_ref()))
            .one(&db)
            .await?
        {
            let storage = Self::new(db, model);
            Ok(Some(storage))
        } else {
            Ok(None)
        }
    }

    /// Returns the storage by path
    #[tracing::instrument(level = "debug", skip(db))]
    pub async fn by_path<S: ToString + Debug>(
        db: DatabaseConnection,
        path: S,
    ) -> RepoResult<Option<Self>> {
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
    #[tracing::instrument(level = "debug", skip(db))]
    pub async fn create<S1: ToString + Debug, S2: ToString + Debug>(
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
            id: NotSet,
            name: Set(name),
            path: Set(path),
            ..Default::default()
        };
        let model = storage.insert(&db).await?;

        Ok(Self::new(db, model))
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
    #[tracing::instrument(level = "debug", skip(self))]
    pub async fn set_name<S: ToString + Debug>(&self, name: S) -> RepoResult<()> {
        let mut active_storage: ActiveStorage = self.get_active_model();
        active_storage.name = Set(name.to_string());
        active_storage.update(&self.db).await?;

        Ok(())
    }

    /// Sets a new path for the storage. This will only update the database record
    /// so if the physical part of the storage is already created it needs to be migrated first
    #[tracing::instrument(level = "debug", skip(self))]
    pub async fn set_path<S: ToString + Debug>(&mut self, path: S) -> RepoResult<()> {
        let mut active_storage: ActiveStorage = self.get_active_model();
        active_storage.path = Set(path.to_string());
        let storage = active_storage.update(&self.db).await?;

        self.model = storage;

        Ok(())
    }

    /// Checks if the storage exists on the harddrive
    pub fn exists(&self) -> bool {
        let path = PathBuf::from(&self.path());

        path.exists()
    }

    /// Adds a thumbnail
    #[tracing::instrument(level = "debug", skip(self, reader))]
    pub async fn store_entry<R: AsyncRead + Unpin>(&self, reader: R) -> RepoResult<Hash> {
        let hash = self.store.add_file(reader, None).await?;
        if let Some(hash) = Hash::by_value(self.db.clone(), &hash).await? {
            Ok(hash)
        } else {
            Hash::add(self.db.clone(), hash).await
        }
    }

    /// Returns the buf reader to the given hash
    #[tracing::instrument(level = "debug", skip(self))]
    pub async fn get_file_reader<S: ToString + Debug>(
        &self,
        hash: S,
    ) -> RepoResult<BufReader<tokio::fs::File>> {
        let (_ext, reader) = self.store.get_file(hash.to_string()).await?;

        Ok(reader)
    }

    /// Returns the size of the storage
    #[inline]
    #[tracing::instrument(level = "debug", skip(self))]
    pub async fn get_size(&self) -> RepoResult<u64> {
        self.store.get_size().await
    }

    /// Returns the active model with only the ID filled so saves always perform an update
    fn get_active_model(&self) -> ActiveStorage {
        ActiveStorage {
            id: Set(self.model.id),
            ..Default::default()
        }
    }
}
