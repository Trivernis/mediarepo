use tauri::State;

pub use daemon::*;
pub use file::*;
pub use job::*;
pub use repo::*;
pub use tag::*;

use crate::tauri_plugin::state::{ApiState, AppState, BufferState};

pub mod daemon;
pub mod file;
pub mod job;
pub mod repo;
pub mod tag;

pub type ApiAccess<'a> = State<'a, ApiState>;
pub type AppAccess<'a> = State<'a, AppState>;
pub type BufferAccess<'a> = State<'a, BufferState>;
