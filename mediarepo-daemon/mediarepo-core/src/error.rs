use sea_orm::DbErr;
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
}

impl From<sea_orm::DbErr> for RepoError {
    fn from(other: DbErr) -> Self {
        Self::Db(RepoDatabaseError::from(other))
    }
}
