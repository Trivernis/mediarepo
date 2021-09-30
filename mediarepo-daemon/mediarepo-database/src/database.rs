use sea_orm::{DatabaseConnection, Database};
use mediarepo_core::error::{RepoDatabaseResult};

pub struct RepoDatabase {
    connection: DatabaseConnection,
}

impl RepoDatabase {
    /// Creates a new repo database from an existing connection
    pub(crate) fn new(connection: DatabaseConnection) -> Self {
        Self {connection}
    }

    /// Creates a new Repo Database Connection
    pub(crate) async fn connect<S: AsRef<str>>(uri: S) -> RepoDatabaseResult<Self> {
        let connection = Database::connect(uri.as_ref()).await?;

        Ok(Self::new(connection))
    }
}