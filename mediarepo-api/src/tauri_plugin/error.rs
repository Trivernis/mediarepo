use crate::client_api::error::ApiError;
use crate::daemon_management::error::DaemonError;
use rmp_ipc::error::Error;
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
        let message = match e {
            ApiError::IPC(ipc_error) => match ipc_error {
                Error::Message(message) => message,
                Error::SendError => String::from("Failed to send event to daemon"),
                Error::ErrorEvent(e) => {
                    format!("Received error: {}", e.to_string())
                }
                e => {
                    format!("{:?}", e)
                }
            },
            ApiError::VersionMismatch => {String::from("The servers API version is not supported by the client. Please make sure both are up to date.")}
        };
        Self { message }
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

impl From<DaemonError> for PluginError {
    fn from(e: DaemonError) -> Self {
        Self { message: e.message }
    }
}
