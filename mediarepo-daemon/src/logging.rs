use std::fs;
use std::path::PathBuf;

use console_subscriber::ConsoleLayer;
use rolling_file::RollingConditionBasic;
use tracing::Level;
use tracing_appender::non_blocking::{NonBlocking, WorkerGuard};
use tracing_flame::FlameLayer;
use tracing_log::LogTracer;
use tracing_subscriber::{
    fmt::{self},
    Layer, Registry,
};
use tracing_subscriber::filter::{self, Targets};
use tracing_subscriber::fmt::format::FmtSpan;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

use mediarepo_core::settings::LoggingSettings;

#[allow(dyn_drop)]
pub type DropGuard = Box<dyn Drop>;

pub fn init_tracing(repo_path: &PathBuf, log_cfg: &LoggingSettings) -> Vec<DropGuard> {
    LogTracer::init().expect("failed to subscribe to log entries");
    let log_path = repo_path.join("logs");
    let mut guards = Vec::new();

    if !log_path.exists() {
        fs::create_dir(&log_path).expect("failed to create directory for log files");
    }
    let (stdout_writer, guard) = tracing_appender::non_blocking(std::io::stdout());
    guards.push(Box::new(guard) as DropGuard);

    let stdout_layer = fmt::layer()
        .with_thread_names(false)
        .with_target(true)
        .with_writer(stdout_writer)
        .with_span_events(FmtSpan::NEW | FmtSpan::CLOSE)
        .with_filter(
            std::env::var("RUST_LOG")
                .unwrap_or(String::from("info,sqlx=warn"))
                .parse::<filter::Targets>()
                .unwrap_or(
                    filter::Targets::new()
                        .with_default(Level::INFO)
                        .with_target("sqlx", Level::WARN),
                ),
        );

    let (sql_writer, guard) = get_sql_log_writer(&log_path);
    guards.push(Box::new(guard) as DropGuard);

    let sql_layer = fmt::layer()
        .with_writer(sql_writer)
        .pretty()
        .with_ansi(false)
        .with_span_events(FmtSpan::NONE)
        .with_filter(get_sql_targets(log_cfg.trace_sql));

    let (bromine_writer, guard) = get_bromine_log_writer(&log_path);
    guards.push(Box::new(guard) as DropGuard);

    let bromine_layer = fmt::layer()
        .with_writer(bromine_writer)
        .pretty()
        .with_ansi(false)
        .with_span_events(FmtSpan::NEW | FmtSpan::CLOSE)
        .with_filter(get_bromine_targets(log_cfg.trace_api_calls));

    let (app_log_writer, guard) = get_application_log_writer(&log_path);
    guards.push(Box::new(guard) as DropGuard);

    let app_log_layer = fmt::layer()
        .with_writer(app_log_writer)
        .pretty()
        .with_ansi(false)
        .with_span_events(FmtSpan::NEW | FmtSpan::CLOSE)
        .with_filter(get_app_targets(log_cfg.level.clone().into()));

    let registry = Registry::default()
        .with(stdout_layer)
        .with(sql_layer)
        .with(bromine_layer)
        .with(app_log_layer);

    let tokio_console_enabled = std::env::var("TOKIO_CONSOLE")
        .map(|v| v.eq_ignore_ascii_case("true"))
        .unwrap_or(false);

    if tokio_console_enabled {
        let console_layer = ConsoleLayer::builder().with_default_env().spawn();
        let registry = registry.with(console_layer);
        tracing::subscriber::set_global_default(registry).expect("Failed to initialize tracing");
    } else {
        tracing::subscriber::set_global_default(registry).expect("Failed to initialize tracing");
    }

    guards
}

fn get_sql_log_writer(log_path: &PathBuf) -> (NonBlocking, WorkerGuard) {
    tracing_appender::non_blocking(
        rolling_file::BasicRollingFileAppender::new(
            log_path.join("sql.log"),
            RollingConditionBasic::new().max_size(1024 * 1024),
            3,
        )
        .expect("failed to create sql log file"),
    )
}

fn get_bromine_log_writer(log_path: &PathBuf) -> (NonBlocking, WorkerGuard) {
    tracing_appender::non_blocking(
        rolling_file::BasicRollingFileAppender::new(
            log_path.join("bromine.log"),
            RollingConditionBasic::new().max_size(1024 * 1024 * 10),
            2,
        )
        .expect("failed to create bromine log file"),
    )
}

fn get_application_log_writer(log_path: &PathBuf) -> (NonBlocking, WorkerGuard) {
    tracing_appender::non_blocking(
        rolling_file::BasicRollingFileAppender::new(
            log_path.join("repo.log"),
            RollingConditionBasic::new().max_size(1024 * 1024 * 10),
            3,
        )
        .expect("failed to create repo log file"),
    )
}

fn get_app_targets(level: Option<Level>) -> Targets {
    filter::Targets::new()
        .with_target("bromine", Level::WARN)
        .with_target("sqlx", Level::WARN)
        .with_target("sea_orm", Level::WARN)
        .with_target("tokio", Level::WARN)
        .with_target("console_subscriber", Level::ERROR)
        .with_target("h2", Level::WARN)
        .with_default(level)
}

fn get_sql_targets(trace_sql: bool) -> Targets {
    if trace_sql {
        filter::Targets::new()
            .with_target("sqlx", Level::WARN)
            .with_target("sea_orm", Level::TRACE)
            .with_target("mediarepo_database", Level::TRACE)
    } else {
        filter::Targets::new().with_default(None)
    }
}

fn get_bromine_targets(trace_bromine: bool) -> Targets {
    if trace_bromine {
        filter::Targets::new().with_target("bromine", Level::DEBUG)
    } else {
        filter::Targets::new().with_default(None)
    }
}

pub fn init_tracing_flame() -> DropGuard {
    let fmt_layer = fmt::Layer::default();
    let (flame_layer, guard) = FlameLayer::with_file("./tracing.folded").unwrap();
    tracing_subscriber::registry()
        .with(fmt_layer)
        .with(flame_layer)
        .init();

    Box::new(guard)
}
