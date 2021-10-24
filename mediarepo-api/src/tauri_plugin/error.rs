use crate::client_api::error::ApiError;
use serde::Serialize;

pub type PluginResult<T> = Result<T, PluginError>;

#[derive(Clone, Serialize)]
pub struct PluginError {
    message: String,
}

impl From<&str> for PluginError {
    fn from(s: &str) -> Self {
        Self {
            message: s.to_string(),
        }
    }
}

impl From<ApiError> for PluginError {
    fn from(e: ApiError) -> Self {
        Self {
            message: format!("ApiError: {:?}", e),
        }
    }
}
