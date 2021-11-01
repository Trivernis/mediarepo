use parking_lot::lock_api::Mutex;
use tauri::State;

pub use daemon::*;
pub use file::*;
pub use repo::*;
pub use tag::*;

use crate::tauri_plugin::state::{ApiState, AppState, BufferState, VolatileBuffer};

pub mod daemon;
pub mod file;
pub mod repo;
pub mod tag;

pub type ApiAccess<'a> = State<'a, ApiState>;
pub type BufferAccess<'a> = State<'a, BufferState>;
pub type AppAccess<'a> = State<'a, AppState>;

/// Adds a once-buffer to the buffer store
fn add_once_buffer(
    buffer_state: BufferAccess<'_>,
    key: String,
    mime: String,
    buf: Vec<u8>,
) -> String {
    let uri = format!("once://{}", key);
    let once_buffer = VolatileBuffer::new(mime, buf);
    let mut once_buffers = buffer_state.buffer.write();
    once_buffers.insert(key, Mutex::new(once_buffer));

    uri
}
