use std::fmt::{Display, Formatter};

pub type DaemonResult<T> = Result<T, DaemonError>;

#[derive(Debug)]
pub struct DaemonError {
    pub message: String,
}

impl Display for DaemonError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.message.fmt(f)
    }
}

impl std::error::Error for DaemonError {}

impl From<std::io::Error> for DaemonError {
    fn from(e: std::io::Error) -> Self {
        Self {
            message: e.to_string(),
        }
    }
}

impl From<String> for DaemonError {
    fn from(s: String) -> Self {
        Self { message: s }
    }
}
