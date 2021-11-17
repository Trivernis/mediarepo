use crate::tauri_plugin::error::{PluginError, PluginResult};
use crate::tauri_plugin::state::{ApiState, BufferState};
use crate::types::identifier::FileIdentifier;
use std::borrow::Cow;
use std::collections::HashMap;
use tauri::http::{Request, Response, ResponseBuilder};
use tauri::{AppHandle, Builder, Manager, Runtime};
use tokio::runtime::{Builder as TokioRuntimeBuilder, Runtime as TokioRuntime};
use url::Url;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub fn register_custom_uri_schemes<R: Runtime>(builder: Builder<R>) -> Builder<R> {
    builder
        .register_uri_scheme_protocol("once", once_scheme)
        .register_uri_scheme_protocol("content", |a, r| {
            build_uri_runtime()?.block_on(content_scheme(a, r))
        })
        .register_uri_scheme_protocol("thumb", |a, r| {
            build_uri_runtime()?.block_on(thumb_scheme(a, r))
        })
}

fn build_uri_runtime() -> PluginResult<TokioRuntime> {
    let runtime = TokioRuntimeBuilder::new_current_thread()
        .thread_name("custom-scheme")
        .enable_all()
        .max_blocking_threads(1)
        .build()?;

    Ok(runtime)
}

fn once_scheme<R: Runtime>(app: &AppHandle<R>, request: &Request) -> Result<Response> {
    let buf_state = app.state::<BufferState>();
    let resource_key = request.uri().trim_start_matches("once://");

    let buffer = buf_state.get_entry(resource_key);

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
}

async fn content_scheme<R: Runtime>(app: &AppHandle<R>, request: &Request) -> Result<Response> {
    let api_state = app.state::<ApiState>();
    let buf_state = app.state::<BufferState>();
    let hash = request.uri().trim_start_matches("content://");

    if let Some(buffer) = buf_state.get_entry(hash) {
        ResponseBuilder::new()
            .status(200)
            .mimetype(&buffer.mime)
            .body(buffer.buf)
    } else {
        let api = api_state.api().await?;
        let file = api
            .file
            .get_file(FileIdentifier::Hash(hash.to_string()))
            .await?;
        let mime = file.mime_type.unwrap_or("image/png".to_string());
        let bytes = api
            .file
            .read_file_by_hash(FileIdentifier::Hash(hash.to_string()))
            .await?;
        buf_state.add_entry(hash.to_string(), mime.clone(), bytes.clone());
        ResponseBuilder::new()
            .mimetype(&mime)
            .status(200)
            .body(bytes)
    }
}

async fn thumb_scheme<R: Runtime>(app: &AppHandle<R>, request: &Request) -> Result<Response> {
    let api_state = app.state::<ApiState>();
    let buf_state = app.state::<BufferState>();

    let url = Url::parse(request.uri())?;
    let hash = url
        .domain()
        .ok_or_else(|| PluginError::from("Missing Domain"))?;

    let query_pairs = url
        .query_pairs()
        .collect::<HashMap<Cow<'_, str>, Cow<'_, str>>>();

    let height = query_pairs
        .get("height")
        .and_then(|h| h.parse::<u32>().ok())
        .unwrap_or(250);

    let width = query_pairs
        .get("width")
        .and_then(|w| w.parse::<u32>().ok())
        .unwrap_or(250);

    if let Some(buffer) = buf_state.get_entry(request.uri()) {
        ResponseBuilder::new()
            .status(200)
            .mimetype(&buffer.mime)
            .body(buffer.buf)
    } else {
        let api = api_state.api().await?;
        let (thumb, bytes) = api
            .file
            .get_thumbnail_of_size(
                FileIdentifier::Hash(hash.to_string()),
                ((height as f32 * 0.8) as u32, (width as f32 * 0.8) as u32),
                ((height as f32 * 1.2) as u32, (width as f32 * 1.2) as u32),
            )
            .await?;
        buf_state.add_entry(
            request.uri().to_string(),
            thumb.mime_type.clone(),
            bytes.clone(),
        );

        ResponseBuilder::new()
            .mimetype(&thumb.mime_type)
            .status(200)
            .body(bytes)
    }
}
