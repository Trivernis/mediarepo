mod constants;
mod utils;

use crate::constants::{DEFAULT_STORAGE_NAME, SETTINGS_PATH};
use crate::utils::{create_paths_for_repo, get_repo, load_settings};
use mediarepo_core::error::RepoResult;
use mediarepo_core::settings::Settings;
use mediarepo_core::type_keys::SettingsKey;
use mediarepo_model::repo::Repo;
use mediarepo_model::type_keys::RepoKey;
use mediarepo_socket::get_builder;
use std::path::PathBuf;
use structopt::StructOpt;
use tokio::fs;

#[derive(Debug, StructOpt)]
#[structopt(name = "mediarepo", about = "A multimedia repository")]
struct Opt {
    /// The path to the repository. Defaults to the current working directory
    #[structopt(long, short, parse(from_os_str), default_value = ".")]
    repo: PathBuf,

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

    /// Imports file from a folder (by glob pattern) into the repository
    Import {
        /// The path to the folder where the files are located
        #[structopt()]
        folder_path: String,
    },

    /// Starts the event server for the selected repository
    Start,
}

#[tokio::main]
async fn main() -> RepoResult<()> {
    build_logger();
    let opt: Opt = Opt::from_args();
    match opt.cmd.clone() {
        SubCommand::Init { force } => init(opt, force).await,
        SubCommand::Start => start_server(opt).await,
        SubCommand::Import { folder_path } => import(opt, folder_path).await,
    }?;

    Ok(())
}

fn build_logger() {
    env_logger::builder()
        .filter_module("sqlx", log::LevelFilter::Warn)
        .filter_module("tokio", log::LevelFilter::Info)
        .filter_module("tracing", log::LevelFilter::Warn)
        .init();
}

async fn init_repo(opt: &Opt) -> RepoResult<(Settings, Repo)> {
    let settings = load_settings(&opt.repo.join(SETTINGS_PATH)).await?;
    let mut repo = get_repo(&opt.repo.join(&settings.database_path).to_str().unwrap()).await?;
    let main_storage_path = opt.repo.join(&settings.default_file_store);
    repo.set_main_storage(main_storage_path.to_str().unwrap())
        .await?;
    Ok((settings, repo))
}

/// Starts the server
async fn start_server(opt: Opt) -> RepoResult<()> {
    let (settings, repo) = init_repo(&opt).await?;

    get_builder(&settings.listen_address)
        .insert::<SettingsKey>(settings)
        .insert::<RepoKey>(repo)
        .build_server()
        .await?;

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
    create_paths_for_repo(&opt.repo, &settings).await?;
    let db_path = opt.repo.join(&settings.database_path);
    if db_path.exists() {
        panic!("Database already exists in location. Use --force with init to delete everything and start a new repository");
    }
    log::debug!("Creating repo");
    let repo = get_repo(&db_path.to_str().unwrap()).await?;
    let storage_path = opt.repo.join(&settings.default_file_store);
    log::debug!("Adding storage");
    repo.add_storage(DEFAULT_STORAGE_NAME, storage_path.to_str().unwrap())
        .await?;
    let settings_string = settings.to_toml_string()?;
    log::debug!("Writing settings");
    fs::write(opt.repo.join(SETTINGS_PATH), &settings_string.into_bytes()).await?;
    log::info!("Repository initialized");

    Ok(())
}

/// Imports files from a source into the database
async fn import(opt: Opt, path: String) -> RepoResult<()> {
    let (_s, repo) = init_repo(&opt).await?;
    log::info!("Importing");

    for entry in glob::glob(&path).unwrap() {
        if let Ok(path) = entry {
            if path.is_file() {
                log::debug!("Importing {:?}", path);
                if let Err(e) = repo.add_file_by_path(path).await {
                    log::error!("Failed to import: {:?}", e);
                }
            }
        }
    }

    Ok(())
}
