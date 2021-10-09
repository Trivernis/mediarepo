use std::io::Error;
use serde::Serialize;

pub type AppResult<T> = Result<T, AppError>;

#[derive(Debug, Serialize)]
pub enum AppError {
  Msg(String)
}

impl From<std::io::Error> for AppError {
  fn from(e: Error) -> Self {
    Self::Msg(e.to_string())
  }
}

impl From<toml::de::Error> for AppError {
  fn from(e: toml::de::Error) -> Self {
    Self::Msg(format!("Failed to deserialize toml: {:?}", e))
  }
}

impl From<toml::ser::Error> for AppError {
  fn from(e: toml::ser::Error) -> Self {
    Self::Msg(format!("Failed to serialize to toml: {:?}", e))
  }
}
