use std::path::PathBuf;

use structopt::StructOpt;
use tokio::fs;
use tokio::runtime;
use tokio::runtime::Runtime;

use mediarepo_core::error::RepoResult;
use mediarepo_core::futures;
use mediarepo_core::settings::{PathSettings, Settings};
use mediarepo_model::repo::Repo;
use mediarepo_socket::start_tcp_server;
use std::env;

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
        load_settings(&opt.repo)?
    } else {
        Settings::default()
    };

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
    let mut handles = Vec::new();

    #[cfg(unix)]
    {
        let socket_path = opt.repo.join("repo.sock");
        let handle = mediarepo_socket::create_unix_socket(
            socket_path,
            opt.repo.clone(),
            settings.clone(),
            repo.clone(),
        )?;
        handles.push(handle);
    }

    let (address, tcp_handle) = start_tcp_server(opt.repo.clone(), settings, repo)?;
    handles.push(tcp_handle);
    fs::write(opt.repo.join("repo.tcp"), &address.into_bytes()).await?;
    futures::future::join_all(handles.into_iter()).await;

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
