#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use tracing_subscriber::fmt::format::FmtSpan;
use tracing_subscriber::EnvFilter;

fn main() {
  tracing_subscriber::fmt::SubscriberBuilder::default()
    .with_env_filter(EnvFilter::from_default_env())
    .with_writer(std::io::stdout)
    .with_span_events(FmtSpan::NEW | FmtSpan::CLOSE)
    .compact()
    .init();
  mediarepo_api::tauri_plugin::register_plugin(tauri::Builder::default())
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
