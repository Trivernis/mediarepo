use crate::storage::Storage;
use mediarepo_core::error::RepoResult;
use mediarepo_database::get_database;
use sea_orm::DatabaseConnection;

pub struct Repo {
    db: DatabaseConnection,
}

impl Repo {
    pub(crate) fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    /// Connects to the database with the given uri
    pub async fn connect<S: AsRef<str>>(uri: S) -> RepoResult<Self> {
        let db = get_database(uri).await?;
        Ok(Self::new(db))
    }

    /// Adds a storage to the repository
    pub async fn add_storage<S1: ToString, S2: ToString>(
        &self,
        name: S1,
        path: S2,
    ) -> RepoResult<Storage> {
        Storage::create(self.db.clone(), name, path).await
    }
}
