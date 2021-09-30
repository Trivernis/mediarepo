use mediarepo_core::error::{RepoDatabaseResult};
use crate::database::RepoDatabase;

pub mod database;
pub mod entities;

/// Connects to the database, runs migrations and returns the RepoDatabase wrapper type
pub async fn get_database<S: AsRef<str>>(uri: S) -> RepoDatabaseResult<RepoDatabase> {
    migrate(uri.as_ref()).await?;
    RepoDatabase::connect(uri).await
}

async fn migrate(uri: &str) -> RepoDatabaseResult<()> {
    use sqlx::Connection;
    let mut conn = sqlx::SqliteConnection::connect(uri).await?;
    sqlx::migrate!().run(&mut conn).await?;

    Ok(())
}