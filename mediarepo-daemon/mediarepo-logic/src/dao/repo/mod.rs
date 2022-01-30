use std::fmt::Debug;

use std::path::PathBuf;

use sea_orm::DatabaseConnection;

use mediarepo_core::error::RepoResult;
use mediarepo_core::fs::file_hash_store::FileHashStore;
use mediarepo_core::fs::thumbnail_store::ThumbnailStore;

use crate::dao::{DaoContext, DaoProvider};
use mediarepo_database::get_database;
use mediarepo_database::queries::analysis::{get_all_counts, Counts};

#[derive(Clone)]
pub struct Repo {
    db: DatabaseConnection,
    main_storage: FileHashStore,
    thumbnail_storage: ThumbnailStore,
}

impl DaoProvider for Repo {
    fn dao_ctx(&self) -> DaoContext {
        DaoContext {
            db: self.db.clone(),
            main_storage: self.main_storage.clone(),
            thumbnail_storage: self.thumbnail_storage.clone(),
        }
    }
}

impl Repo {
    pub(crate) fn new(
        db: DatabaseConnection,
        file_store_path: PathBuf,
        thumb_store_path: PathBuf,
    ) -> Self {
        Self {
            db,
            main_storage: FileHashStore::new(file_store_path),
            thumbnail_storage: ThumbnailStore::new(thumb_store_path),
        }
    }

    /// Connects to the database with the given uri
    #[tracing::instrument(level = "debug")]
    pub async fn connect<S: AsRef<str> + Debug>(
        uri: S,
        file_store_path: PathBuf,
        thumb_store_path: PathBuf,
    ) -> RepoResult<Self> {
        let db = get_database(uri).await?;
        Ok(Self::new(db, file_store_path, thumb_store_path))
    }

    /// Returns the database of the repo for raw sql queries
    pub fn db(&self) -> &DatabaseConnection {
        &self.db
    }

    /// Returns the size of the main storage
    #[inline]
    #[tracing::instrument(level = "debug", skip(self))]
    pub async fn get_main_store_size(&self) -> RepoResult<u64> {
        self.main_storage.get_size().await
    }

    /// Returns the size of the thumbnail storage
    #[inline]
    #[tracing::instrument(level = "debug", skip(self))]
    pub async fn get_thumb_store_size(&self) -> RepoResult<u64> {
        self.thumbnail_storage.get_size().await
    }

    /// Returns all entity counts
    #[inline]
    #[tracing::instrument(level = "debug", skip(self))]
    pub async fn get_counts(&self) -> RepoResult<Counts> {
        get_all_counts(&self.db).await
    }
}
