use std::env;
use std::path::PathBuf;
use std::time::Duration;

use structopt::StructOpt;
use tokio::fs;
use tokio::io::AsyncWriteExt;
use tokio::runtime;
use tokio::runtime::Runtime;

use mediarepo_core::error::RepoResult;
use mediarepo_core::fs::drop_file::DropFile;
use mediarepo_core::settings::{PathSettings, Settings};
use mediarepo_core::tokio_graceful_shutdown::{SubsystemHandle, Toplevel};
use mediarepo_logic::dao::repo::Repo;
use mediarepo_socket::start_tcp_server;

use crate::utils::{create_paths_for_repo, get_repo, load_settings};

mod logging;
mod utils;

#[derive(Debug, StructOpt)]
#[structopt(name = "mediarepo", about = "A multimedia repository")]
struct Opt {
    /// The path to the repository. Defaults to the current working directory
    #[structopt(long, short, parse(from_os_str), default_value = ".")]
    repo: PathBuf,

    #[structopt(long, short)]
    profile: bool,

    /// The subcommand to invoke
    #[structopt(subcommand)]
    cmd: SubCommand,
}

#[derive(Clone, Debug, StructOpt)]
enum SubCommand {
    /// Initializes an empty repository
    Init {
        /// Forces the creation of a new repository. This will delete everything in the repository
        /// path to create an empty repository.
        #[structopt(short, long)]
        force: bool,
    },

    /// Starts the event server for the selected repository
    Start,
}

fn main() -> RepoResult<()> {
    let mut opt: Opt = Opt::from_args();
    opt.repo = env::current_dir().unwrap().join(opt.repo);

    let settings = if opt.repo.exists() {
        opt.repo = opt.repo.canonicalize().unwrap();

        match load_settings(&opt.repo) {
            Ok(s) => s,
            Err(e) => {
                log::warn!("failed to read settings {}", e);
                Settings::default()
            }
        }
    } else {
        Settings::default()
    };
    clean_old_connection_files(&opt.repo)?;

    let mut guards = Vec::new();
    if opt.profile {
        guards.push(logging::init_tracing_flame());
    } else {
        guards.append(&mut logging::init_tracing(&opt.repo, &settings.logging));
    }

    let result = match opt.cmd.clone() {
        SubCommand::Init { force } => get_single_thread_runtime().block_on(init(opt, force)),
        SubCommand::Start => get_multi_thread_runtime().block_on(start_server(opt, settings)),
    };

    match result {
        Ok(_) => Ok(()),
        Err(e) => {
            tracing::error!("a critical error occurred when running the daemon: {}", e);

            Err(e)
        }
    }
}

fn get_single_thread_runtime() -> Runtime {
    log::info!("Using current thread runtime");
    runtime::Builder::new_current_thread()
        .enable_all()
        .max_blocking_threads(1)
        .build()
        .unwrap()
}

fn get_multi_thread_runtime() -> Runtime {
    log::info!("Using multi thread runtime");
    runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap()
}

async fn init_repo(opt: &Opt, paths: &PathSettings) -> RepoResult<Repo> {
    let repo = get_repo(&opt.repo, paths).await?;

    Ok(repo)
}

/// Starts the server
async fn start_server(opt: Opt, settings: Settings) -> RepoResult<()> {
    let repo = init_repo(&opt, &settings.paths).await?;
    let mut top_level = Toplevel::new();

    #[cfg(unix)]
    {
        if settings.server.unix_socket.enabled {
            let settings = settings.clone();
            let repo_path = opt.repo.clone();
            let repo = repo.clone();

            top_level = top_level.start("mediarepo-unix-socket", |subsystem| {
                Box::pin(async move {
                    start_and_await_unix_socket(subsystem, repo_path, settings, repo).await?;
                    Ok(())
                })
            })
        }
    }

    if settings.server.tcp.enabled {
        top_level = top_level.start("mediarepo-tcp", move |subsystem| {
            Box::pin(async move {
                start_and_await_tcp_server(subsystem, opt.repo, settings, repo).await?;

                Ok(())
            })
        })
    }
    if let Err(e) = top_level
        .catch_signals()
        .handle_shutdown_requests(Duration::from_millis(1000))
        .await
    {
        tracing::error!("an error occurred when running the servers {}", e);
    }

    tracing::warn!(
        r"All servers quit.
        Either they were requested to stop, a fatal error occurred or no servers are enabled in the config.
        Stopping daemon..."
    );

    Ok(())
}

async fn start_and_await_tcp_server(
    subsystem: SubsystemHandle,
    repo_path: PathBuf,
    settings: Settings,
    repo: Repo,
) -> RepoResult<()> {
    let (address, handle) = start_tcp_server(subsystem.clone(), repo_path.clone(), settings, repo)?;
    let (mut file, _guard) = DropFile::new(repo_path.join("repo.tcp")).await?;
    file.write_all(&address.into_bytes()).await?;

    tokio::select! {
        _ = subsystem.on_shutdown_requested() => {
            tracing::info!("shutdown requested")
        },
        result = handle => {
            if let Err(e) = result {
                tracing::error!("the tcp server shut down with an error {}", e);
                subsystem.request_shutdown();
            }
        }
    }

    Ok(())
}

#[cfg(unix)]
async fn start_and_await_unix_socket(
    subsystem: SubsystemHandle,
    repo_path: PathBuf,
    settings: Settings,
    repo: Repo,
) -> RepoResult<()> {
    let socket_path = repo_path.join("repo.sock");
    let handle = mediarepo_socket::create_unix_socket(
        subsystem.clone(),
        socket_path,
        repo_path.clone(),
        settings,
        repo,
    )?;
    let _guard = DropFile::from_path(repo_path.join("repo.sock"));

    tokio::select! {
        _ = subsystem.on_shutdown_requested() => {
            tracing::info!("shutdown requested")
        },
        result = handle => {
            if let Err(e) = result {
                tracing::error!("the unix socket shut down with an error {}", e);
                subsystem.request_shutdown();
            }
        }
    }

    Ok(())
}

/// Initializes an empty repository
async fn init(opt: Opt, force: bool) -> RepoResult<()> {
    log::info!("Initializing repository at {:?}", opt.repo);

    if force {
        log::debug!("Removing old repository");
        fs::remove_dir_all(&opt.repo).await?;
    }
    let settings = Settings::default();

    log::debug!("Creating paths");
    create_paths_for_repo(&opt.repo, &settings.paths).await?;

    if settings.paths.db_file_path(&opt.repo).exists() {
        panic!("Database already exists in location. Use --force with init to delete everything and start a new repository");
    }
    log::debug!("Creating repo");
    let _repo = get_repo(&opt.repo, &settings.paths).await?;

    log::debug!("Writing settings");
    settings.save(&opt.repo)?;

    log::info!("Repository initialized");

    Ok(())
}

fn clean_old_connection_files(root: &PathBuf) -> RepoResult<()> {
    let paths = ["repo.tcp", "repo.sock"];

    for path in paths {
        let path = root.join(path);

        if path.exists() {
            std::fs::remove_file(&path)?;
        }
    }

    Ok(())
}
