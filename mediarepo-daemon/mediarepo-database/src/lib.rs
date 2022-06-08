use std::time::Duration;

use migration::MigratorTrait;
use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use sqlx::migrate::MigrateDatabase;

use mediarepo_core::error::RepoDatabaseResult;

pub mod entities;
pub mod queries;

/// Connects to the database, runs migrations and returns the RepoDatabase wrapper type
pub async fn get_database<S: AsRef<str>>(uri: S) -> RepoDatabaseResult<DatabaseConnection> {
    if !sqlx::Sqlite::database_exists(uri.as_ref()).await? {
        sqlx::Sqlite::create_database(uri.as_ref()).await?;
    }
    let mut opt = ConnectOptions::new(uri.as_ref().to_string());
    opt.connect_timeout(Duration::from_secs(10))
        .idle_timeout(Duration::from_secs(10))
        .sqlx_logging(true);

    let conn = Database::connect(opt).await?;
    migration::Migrator::up(&conn, None).await?;

    Ok(conn)
}
