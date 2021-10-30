use crate::tauri_plugin::state::BufferState;
use tauri::http::ResponseBuilder;
use tauri::{Builder, Manager, Runtime};

pub fn register_custom_uri_schemes<R: Runtime>(builder: Builder<R>) -> Builder<R> {
    builder.register_uri_scheme_protocol("once", |app, request| {
        let buf_state = app.state::<BufferState>();
        let resource_key = request.uri().trim_start_matches("once://");

        let buffer = buf_state.get_entry(resource_key);
        buf_state.clear_expired();
        if let Some(buffer) = buffer {
            ResponseBuilder::new()
                .mimetype(&buffer.mime)
                .status(200)
                .body(buffer.buf)
        } else {
            ResponseBuilder::new()
                .mimetype("text/plain")
                .status(404)
                .body("Resource not found".as_bytes().to_vec())
        }
    })
}
