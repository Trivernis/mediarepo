use std::fmt::{Debug, Formatter};

use sea_orm::DbErr;
use thiserror::Error;
use tokio_graceful_shutdown::GracefulShutdownError;

pub type RepoResult<T> = Result<T, RepoError>;
pub type RepoDatabaseResult<T> = Result<T, RepoDatabaseError>;

#[derive(Error, Debug)]
pub enum RepoError {
    #[error(transparent)]
    Db(#[from] RepoDatabaseError),

    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error(transparent)]
    Multibase(#[from] multibase::Error),

    #[error("Config Error: {0}")]
    TomlDe(#[from] toml::de::Error),

    #[error("Config Error: {0}")]
    TomlSer(#[from] toml::ser::Error),

    #[error(transparent)]
    IPC(#[from] mediarepo_api::bromine::error::Error),

    #[error(transparent)]
    Raw(StringError),

    #[error(transparent)]
    Thumbnailer(#[from] thumbnailer::error::ThumbError),

    #[error("no free tcp port available")]
    PortUnavailable,

    #[error("failed to decode data {0}")]
    Decode(#[from] data_encoding::DecodeError),

    #[error("failed to read repo.toml configuration file {0}")]
    Config(#[from] config::ConfigError),

    #[error("the database file is corrupted {0}")]
    Corrupted(String),

    #[error("bincode de-/serialization failed {0}")]
    Bincode(#[from] bincode::Error),

    #[error("graceful shutdown failed {0}")]
    Shutdown(#[from] GracefulShutdownError),
}

#[derive(Error, Debug)]
pub enum RepoDatabaseError {
    #[error(transparent)]
    SeaOrmDb(#[from] sea_orm::DbErr),

    #[error(transparent)]
    SeaOrmColumn(#[from] sea_orm::error::ColumnFromStrErr),

    #[error(transparent)]
    Sqlx(#[from] sqlx::error::Error),

    #[error(transparent)]
    SqlxMigrateError(#[from] sqlx::migrate::MigrateError),

    #[error("An invalid handle {0} was used")]
    InvalidHandle(i64),
}

#[derive(Debug)]
pub struct StringError(String);

impl std::fmt::Display for StringError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.0, f)
    }
}

impl std::error::Error for StringError {}

impl From<sea_orm::DbErr> for RepoError {
    fn from(other: DbErr) -> Self {
        Self::Db(RepoDatabaseError::from(other))
    }
}

impl From<&str> for RepoError {
    fn from(s: &str) -> Self {
        Self::Raw(StringError(s.to_string()))
    }
}

impl From<RepoError> for mediarepo_api::bromine::error::Error {
    fn from(e: RepoError) -> mediarepo_api::bromine::error::Error {
        mediarepo_api::bromine::error::Error::Message(format!("{:?}", e))
    }
}
