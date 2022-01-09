use serde::{Deserialize, Serialize};
use tracing::Level;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct LoggingSettings {
    pub level: LogLevel,
    pub trace_sql: bool,
    pub trace_api_calls: bool,
}

impl Default for LoggingSettings {
    fn default() -> Self {
        Self {
            level: LogLevel::Info,
            trace_sql: false,
            trace_api_calls: false,
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum LogLevel {
    Off,
    Error,
    Warn,
    Info,
    Debug,
    Trace,
}

impl Into<Option<Level>> for LogLevel {
    fn into(self) -> Option<Level> {
        match self {
            LogLevel::Off => None,
            LogLevel::Error => Some(Level::ERROR),
            LogLevel::Warn => Some(Level::WARN),
            LogLevel::Info => Some(Level::INFO),
            LogLevel::Debug => Some(Level::DEBUG),
            LogLevel::Trace => Some(Level::TRACE),
        }
    }
}
