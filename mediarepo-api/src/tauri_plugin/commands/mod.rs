use tauri::State;

pub use file::*;

use crate::tauri_plugin::state::{ApiState, BufferState, OnceBuffer};

pub mod file;

pub type ApiAccess<'a> = State<'a, ApiState>;
pub type BufferAccess<'a> = State<'a, BufferState>;

/// Adds a once-buffer to the buffer store
fn add_once_buffer(buffer_state: BufferAccess, key: String, mime: String, buf: Vec<u8>) -> String {
    let uri = format!("once://{}", key);
    let once_buffer = OnceBuffer::new(mime, buf);
    let mut once_buffers = buffer_state.buffer.lock();
    once_buffers.insert(key, once_buffer);

    uri
}
