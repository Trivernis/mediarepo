#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use crate::commands::emit_info;
use crate::commands::files::*;
use crate::commands::repo::{
  add_repository, get_active_repository, get_repositories, select_repository,
};
use crate::commands::tags::*;
use crate::context::Context;
use crate::settings::load_settings;
use tauri::http::ResponseBuilder;
use tauri::Manager;

mod commands;
pub mod context;
pub mod error;
mod ipc;
mod settings;

fn main() {
  let settings = load_settings().expect("Failed to load settings");
  let context = Context::new(settings);

  tauri::Builder::default()
    .manage(context)
    .register_uri_scheme_protocol("once", |app, request| {
      let context = app.state::<Context>();
      let resource_key = request.uri().trim_start_matches("once://");
      let buffer = {
        let mut buffers = context.once_buffers.lock().unwrap();
        buffers.remove(resource_key)
      };
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
    .invoke_handler(tauri::generate_handler![
      get_repositories,
      add_repository,
      select_repository,
      get_active_repository,
      emit_info,
      get_all_files,
      read_file_by_hash,
      get_thumbnails,
      read_thumbnail,
      get_tags_for_file,
      find_files,
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
