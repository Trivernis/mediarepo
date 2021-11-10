use rmp_ipc::error::Error;
use sea_orm::DbErr;
use std::fmt::{Display, Formatter};
use thiserror::Error;

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
    IPC(#[from] rmp_ipc::error::Error),

    #[error(transparent)]
    Raw(StringError),

    #[error(transparent)]
    Thumbnailer(#[from] thumbnailer::error::ThumbError),

    #[error("No free tcp port available")]
    PortUnavailable,
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

impl Display for StringError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
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

impl From<RepoError> for rmp_ipc::error::Error {
    fn from(e: RepoError) -> Error {
        rmp_ipc::error::Error::Message(format!("{:?}", e))
    }
}
