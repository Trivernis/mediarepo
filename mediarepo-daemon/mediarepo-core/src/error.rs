use thiserror::Error;

pub type RepoResult<T> = Result<T, RepoError>;
pub type RepoDatabaseResult<T> =  Result<T, RepoDatabaseError>;

#[derive(Error, Debug)]
pub enum RepoError {
    #[error(transparent)]
    Db(#[from] RepoDatabaseError)
}

#[derive(Error, Debug)]
pub enum RepoDatabaseError {
    #[error(transparent)]
    SeaOrmDb(#[from] sea_orm::error::DbErr),

    #[error(transparent)]
    SeaOrmColumn(#[from] sea_orm::error::ColumnFromStrErr),

    #[error(transparent)]
    Sqlx(#[from] sqlx::error::Error),

    #[error(transparent)]
    SqlxMigrateError(#[from] sqlx::migrate::MigrateError)
}