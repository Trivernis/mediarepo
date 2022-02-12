use crate::client_api::ApiClient;
use crate::tauri_plugin::error::{PluginError, PluginResult};
use crate::tauri_plugin::state::{ApiState, AppState, AsyncTask, BufferState};
use crate::types::identifier::FileIdentifier;
use std::borrow::Cow;
use std::collections::HashMap;
use std::sync::Arc;
use tauri::http::{Request, Response, ResponseBuilder};
use tauri::{AppHandle, Builder, Manager, Runtime};
use tokio::runtime::{Builder as TokioRuntimeBuilder, Runtime as TokioRuntime};
use url::Url;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub fn register_custom_uri_schemes<R: Runtime>(builder: Builder<R>) -> Builder<R> {
    let runtime =
        Arc::new(build_uri_runtime().expect("Failed to build async runtime for custom schemes"));
    builder
        .register_uri_scheme_protocol("once", once_scheme)
        .register_uri_scheme_protocol("content", {
            let runtime = Arc::clone(&runtime);
            move |a, r| runtime.block_on(content_scheme(a, r))
        })
        .register_uri_scheme_protocol("thumb", move |a, r| runtime.block_on(thumb_scheme(a, r)))
}

fn build_uri_runtime() -> PluginResult<TokioRuntime> {
    let runtime = TokioRuntimeBuilder::new_current_thread()
        .thread_name("custom-scheme")
        .enable_all()
        .build()?;

    Ok(runtime)
}

#[tracing::instrument(level = "debug", skip_all)]
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

#[tracing::instrument(level = "debug", skip_all)]
async fn content_scheme<R: Runtime>(app: &AppHandle<R>, request: &Request) -> Result<Response> {
    let api_state = app.state::<ApiState>();
    let buf_state = app.state::<BufferState>();
    let hash = request.uri().trim_start_matches("content://");

    if let Some(buffer) = buf_state.get_entry(hash) {
        tracing::debug!("Fetching content from cache");
        ResponseBuilder::new()
            .status(200)
            .mimetype(&buffer.mime)
            .body(buffer.buf)
    } else {
        tracing::debug!("Fetching content from daemon");
        let api = api_state.api().await?;
        let file = api
            .file
            .get_file(FileIdentifier::CD(hash.to_string()))
            .await?;
        let mime = file.mime_type;
        let bytes = api
            .file
            .read_file(FileIdentifier::CD(hash.to_string()))
            .await?;
        tracing::debug!("Received {} content bytes", bytes.len());
        buf_state.add_entry(hash.to_string(), mime.clone(), bytes.clone());
        ResponseBuilder::new()
            .mimetype(&mime)
            .status(200)
            .body(bytes)
    }
}

#[tracing::instrument(level = "debug", skip_all)]
async fn thumb_scheme<R: Runtime>(app: &AppHandle<R>, request: &Request) -> Result<Response> {
    let api_state = app.state::<ApiState>();
    let buf_state = app.state::<BufferState>();
    let app_state = app.state::<AppState>();

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
        tracing::debug!("Fetching content from cache");
        ResponseBuilder::new()
            .status(200)
            .mimetype(&buffer.mime)
            .body(buffer.buf)
    } else {
        tracing::debug!("Content not loaded. Singnaling retry.");
        let api = api_state.api().await?;
        let buf_state = buf_state.inner().clone();

        app_state
            .add_async_task(build_fetch_thumbnail_task(
                buf_state,
                api,
                hash.to_string(),
                request.uri().to_string(),
                width,
                height,
            ))
            .await;

        ResponseBuilder::new()
            .mimetype("text/plain")
            .status(301)
            .header("Retry-After", "1")
            .body("Content loading. Retry in 1s.".as_bytes().to_vec())
    }
}

fn build_fetch_thumbnail_task(
    buf_state: BufferState,
    api: ApiClient,
    hash: String,
    request_uri: String,
    width: u32,
    height: u32,
) -> AsyncTask {
    AsyncTask::new(async move {
        tracing::debug!("Fetching content from daemon");
        let (thumb, bytes) = api
            .file
            .get_thumbnail_of_size(
                FileIdentifier::CD(hash),
                ((height as f32 * 0.5) as u32, (width as f32 * 0.5) as u32),
                ((height as f32 * 1.5) as u32, (width as f32 * 1.5) as u32),
            )
            .await?;
        tracing::debug!("Received {} content bytes", bytes.len());
        buf_state.add_entry(request_uri, thumb.mime_type.clone(), bytes.clone());

        Ok(())
    })
}
