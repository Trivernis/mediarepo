#![cfg_attr(
all(not(debug_assertions), target_os = "windows"),
windows_subsystem = "windows"
)]

use tauri::{LogicalSize, Size};
use tracing_subscriber::EnvFilter;
use tracing_subscriber::fmt::format::FmtSpan;

fn main() {
    tracing_subscriber::fmt::SubscriberBuilder::default()
        .with_env_filter(EnvFilter::from_default_env())
        .with_writer(std::io::stdout)
        .with_span_events(FmtSpan::NEW | FmtSpan::CLOSE)
        .compact()
        .init();
    mediarepo_api::tauri_plugin::register_plugin(tauri::Builder::default())
        .on_page_load(|window, _| {
            window.set_title(format!("mediarepo {}", env!("CARGO_PKG_VERSION")).as_str()).expect("failed to set window title");
            window.set_min_size(Some(Size::Logical(LogicalSize { width: 1000.0, height: 750.0 }))).expect("failed to set minimal size");
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
