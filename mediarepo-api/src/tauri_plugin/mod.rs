use tauri::plugin::Plugin;
use tauri::{AppHandle, Builder, Invoke, Manager, Runtime};

use state::ApiState;

use crate::tauri_plugin::state::BufferState;

mod commands;
pub mod custom_schemes;
pub mod error;
mod state;

use commands::*;

pub fn register_plugin<R: Runtime>(builder: Builder<R>) -> Builder<R> {
    let repo_plugin = MediarepoPlugin::new();

    custom_schemes::register_custom_uri_schemes(builder.plugin(repo_plugin))
}

pub struct MediarepoPlugin<R: Runtime> {
    invoke_handler: Box<dyn Fn(Invoke<R>) + Send + Sync>,
}

impl<R: Runtime> MediarepoPlugin<R> {
    pub fn new() -> Self {
        Self {
            invoke_handler: Box::new(tauri::generate_handler![
                get_all_files,
                find_files,
                read_file_by_hash,
                get_file_thumbnails,
                read_thumbnail
            ]),
        }
    }
}

impl<R: Runtime> Plugin<R> for MediarepoPlugin<R> {
    fn name(&self) -> &'static str {
        "mediarepo"
    }

    fn initialize(
        &mut self,
        app: &AppHandle<R>,
        _config: serde_json::value::Value,
    ) -> tauri::plugin::Result<()> {
        let api_state = ApiState::new();
        app.manage(api_state);

        let buffer_state = BufferState::default();
        app.manage(buffer_state);

        Ok(())
    }

    fn extend_api(&mut self, message: Invoke<R>) {
        (self.invoke_handler)(message)
    }
}
