mod constants;
mod utils;

use crate::constants::{DEFAULT_STORAGE_NAME, SETTINGS_PATH, THUMBNAIL_STORAGE_NAME};
use crate::utils::{create_paths_for_repo, get_repo, load_settings};
use log::LevelFilter;
use mediarepo_core::error::RepoResult;
use mediarepo_core::settings::Settings;
use mediarepo_core::type_keys::SettingsKey;
use mediarepo_core::utils::parse_tags_file;
use mediarepo_model::file::File as RepoFile;
use mediarepo_model::repo::Repo;
use mediarepo_model::type_keys::RepoKey;
use mediarepo_socket::get_builder;
use pretty_env_logger::env_logger::WriteStyle;
use std::env;
use std::path::PathBuf;
use std::str::FromStr;
use std::sync::Arc;
use structopt::StructOpt;
use tokio::fs;
use tokio::runtime;
use tokio::runtime::Runtime;
use tracing_flame::FlameLayer;
use tracing_subscriber::{fmt, prelude::*};

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

    /// Imports file from a folder (by glob pattern) into the repository
    Import {
        /// The path to the folder where the files are located
        #[structopt()]
        folder_path: String,
    },

    /// Starts the event server for the selected repository
    Start,
}

fn main() -> RepoResult<()> {
    let opt: Opt = Opt::from_args();
    let mut _guard = None;
    if opt.profile {
        _guard = Some(init_tracing_flame());
    } else {
        build_logger();
    }

    match opt.cmd.clone() {
        SubCommand::Init { force } => get_single_thread_runtime().block_on(init(opt, force)),
        SubCommand::Start => get_multi_thread_runtime().block_on(start_server(opt)),
        SubCommand::Import { folder_path } => {
            get_single_thread_runtime().block_on(import(opt, folder_path))
        }
    }?;

    Ok(())
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

fn build_logger() {
    pretty_env_logger::formatted_timed_builder()
        .filter(
            None,
            env::var("RUST_LOG")
                .ok()
                .and_then(|level| LevelFilter::from_str(&level).ok())
                .unwrap_or(LevelFilter::Info),
        )
        .write_style(WriteStyle::Always)
        .filter_module("sqlx", log::LevelFilter::Warn)
        .filter_module("tokio", log::LevelFilter::Info)
        .init();
}

fn init_tracing_flame() -> impl Drop {
    let fmt_layer = fmt::Layer::default();
    let (flame_layer, _guard) = FlameLayer::with_file("./tracing.folded").unwrap();
    tracing_subscriber::registry()
        .with(fmt_layer)
        .with(flame_layer)
        .init();
    _guard
}

async fn init_repo(opt: &Opt) -> RepoResult<(Settings, Repo)> {
    let settings = load_settings(&opt.repo.join(SETTINGS_PATH)).await?;
    let mut repo = get_repo(&opt.repo.join(&settings.database_path).to_str().unwrap()).await?;
    let main_storage_path = opt.repo.join(&settings.default_file_store);
    let thumb_storage_path = opt.repo.join(&settings.thumbnail_store);
    repo.set_main_storage(main_storage_path.to_str().unwrap())
        .await?;
    repo.set_thumbnail_storage(thumb_storage_path.to_str().unwrap())
        .await?;
    Ok((settings, repo))
}

/// Starts the server
async fn start_server(opt: Opt) -> RepoResult<()> {
    let (settings, repo) = init_repo(&opt).await?;

    get_builder(&settings.listen_address)
        .insert::<SettingsKey>(settings)
        .insert::<RepoKey>(Arc::new(repo))
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
    let thumb_storage_path = opt.repo.join(&settings.thumbnail_store);
    repo.add_storage(THUMBNAIL_STORAGE_NAME, thumb_storage_path.to_str().unwrap())
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

    let paths: Vec<PathBuf> = glob::glob(&path)
        .unwrap()
        .into_iter()
        .filter_map(|r| r.ok())
        .filter(|e| e.is_file())
        .collect();

    for path in paths {
        if let Err(e) = import_single_image(path, &repo).await {
            log::error!("Import failed: {:?}", e);
        }
    }

    Ok(())
}

/// Creates thumbnails of all sizes
async fn import_single_image(path: PathBuf, repo: &Repo) -> RepoResult<()> {
    log::info!("Importing file");
    let file = repo.add_file_by_path(path.clone()).await?;
    log::info!("Creating thumbnails");
    repo.create_thumbnails_for_file(file.clone()).await?;
    let tags_path = PathBuf::from(format!("{}{}", path.to_str().unwrap(), ".txt"));
    add_tags_from_tags_file(tags_path, repo, file).await?;

    Ok(())
}

async fn add_tags_from_tags_file(
    tags_path: PathBuf,
    repo: &Repo,
    file: RepoFile,
) -> RepoResult<()> {
    log::info!("Adding tags");
    if tags_path.exists() {
        let tags = parse_tags_file(tags_path).await?;
        let mut tag_ids = Vec::new();

        for (namespace, name) in tags {
            let tag = if let Some(namespace) = namespace {
                log::info!("Adding namespaced tag '{}:{}'", namespace, name);
                repo.add_or_find_namespaced_tag(name, namespace).await?
            } else {
                log::info!("Adding unnamespaced tag '{}'", name);
                repo.add_or_find_unnamespaced_tag(name).await?
            };
            tag_ids.push(tag.id());
        }
        log::info!("Mapping {} tags to the file", tag_ids.len());
        file.add_tags(tag_ids).await?;
    } else {
        log::info!("No tags file '{:?}' found", tags_path);
    }
    Ok(())
}
