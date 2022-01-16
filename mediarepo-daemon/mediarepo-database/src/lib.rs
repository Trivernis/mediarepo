use mediarepo_core::error::RepoDatabaseResult;
use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use sqlx::migrate::MigrateDatabase;
use std::time::Duration;

pub mod entities;
pub mod queries;

/// Connects to the database, runs migrations and returns the RepoDatabase wrapper type
pub async fn get_database<S: AsRef<str>>(uri: S) -> RepoDatabaseResult<DatabaseConnection> {
    migrate(uri.as_ref()).await?;
    let mut opt = ConnectOptions::new(uri.as_ref().to_string());
    opt.connect_timeout(Duration::from_secs(10))
        .idle_timeout(Duration::from_secs(10));

    let conn = Database::connect(opt).await?;

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
