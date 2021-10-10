use std::error::Error;
use std::fmt::{Display, Formatter};
use serde::Serialize;

pub type AppResult<T> = Result<T, AppError>;

#[derive(Debug, Serialize)]
pub struct AppError {
  message: String
}

impl AppError {
  pub fn new<S: ToString>(msg: S) -> Self {
    Self {
      message: msg.to_string()
    }
  }
}

impl Display for AppError {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    self.message.fmt(f)
  }
}

impl Error for AppError {}

impl From<std::io::Error> for AppError {
  fn from(e: std::io::Error) -> Self {
    Self::new(e)
  }
}

impl From<toml::de::Error> for AppError {
  fn from(e: toml::de::Error) -> Self {
    Self::new(format!("Failed to deserialize toml: {:?}", e))
  }
}

impl From<toml::ser::Error> for AppError {
  fn from(e: toml::ser::Error) -> Self {
    Self::new(format!("Failed to serialize to toml: {:?}", e))
  }
}

impl From<rmp_ipc::error::Error> for AppError {
  fn from(e: rmp_ipc::error::Error) -> Self {
    Self::new(format!("Daemon Error: {:?}", e))
  }
}

impl From<tauri::Error> for AppError {
  fn from(e: tauri::Error) -> Self {
    Self::new(format!("Tauri error: {:?}", e))
  }
}

impl From<AppError> for rmp_ipc::error::Error {
  fn from(e: AppError) -> Self {
    rmp_ipc::error::Error::Message(e.message)
  }
}
