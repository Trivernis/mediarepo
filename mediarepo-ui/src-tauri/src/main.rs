#![cfg_attr(
all(not(debug_assertions), target_os = "windows"),
windows_subsystem = "windows"
)]

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
            window.set_title(format!("mediarepo {}", env!("CARGO_PKG_VERSION")).as_str()).unwrap();
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
