#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use crate::commands::repo::{get_repositories, add_repository};
use crate::context::Context;
use crate::settings::load_settings;

mod commands;
pub mod context;
pub mod error;
mod settings;

fn main() {
  let settings = load_settings().expect("Failed to load settings");
  let context = Context::new(settings);

  tauri::Builder::default()
    .manage(context)
    .invoke_handler(tauri::generate_handler![get_repositories, add_repository])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
