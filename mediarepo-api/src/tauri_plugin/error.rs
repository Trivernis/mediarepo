use crate::client_api::error::ApiError;
use serde::Serialize;
use std::fmt::{Display, Formatter};

pub type PluginResult<T> = Result<T, PluginError>;

#[derive(Clone, Debug, Serialize)]
pub struct PluginError {
    message: String,
}

impl Display for PluginError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.message.fmt(f)
    }
}

impl std::error::Error for PluginError {}

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

impl From<std::io::Error> for PluginError {
    fn from(e: std::io::Error) -> Self {
        Self {
            message: e.to_string(),
        }
    }
}

impl From<toml::de::Error> for PluginError {
    fn from(e: toml::de::Error) -> Self {
        Self {
            message: format!("Deserialization failed: {:?}", e),
        }
    }
}

impl From<toml::ser::Error> for PluginError {
    fn from(e: toml::ser::Error) -> Self {
        Self {
            message: format!("Serialization failed: {:?}", e),
        }
    }
}
