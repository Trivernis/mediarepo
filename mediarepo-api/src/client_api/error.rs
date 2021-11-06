use thiserror::Error;

pub type ApiResult<T> = Result<T, ApiError>;

#[derive(Debug, Error)]
pub enum ApiError {
    #[error(transparent)]
    IPC(#[from] rmp_ipc::error::Error),

    #[error("The servers api version (version {server:?}) is incompatible with the api client {client:?}")]
    VersionMismatch { server: String, client: String },
}
