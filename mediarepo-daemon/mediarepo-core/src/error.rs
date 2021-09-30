use thiserror::Error;

#[derive(Error, Debug)]
pub enum RepoError {
    #[error(transparent)]
    Db(#[from] sea_orm::error::DbErr),

    #[error(transparent)]
    Orm(#[from] sea_orm::error::ColumnFromStrErr)
}