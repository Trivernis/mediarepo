use mediarepo_core::error::RepoDatabaseResult;
use sea_orm::{Database, DatabaseConnection};
use sqlx::migrate::MigrateDatabase;

pub mod entities;
pub mod queries;

/// Connects to the database, runs migrations and returns the RepoDatabase wrapper type
pub async fn get_database<S: AsRef<str>>(uri: S) -> RepoDatabaseResult<DatabaseConnection> {
    migrate(uri.as_ref()).await?;
    let conn = Database::connect(uri.as_ref()).await?;

    Ok(conn)
}

async fn migrate(uri: &str) -> RepoDatabaseResult<()> {
    use sqlx::Connection;
    if !sqlx::Sqlite::database_exists(uri).await? {
        sqlx::Sqlite::create_database(uri).await?;
    }
    let mut conn = sqlx::SqliteConnection::connect(uri).await?;
    sqlx::migrate!().run(&mut conn).await?;

    Ok(())
}
