use std::path::PathBuf;

use structopt::StructOpt;
use tokio::fs;
use tokio::runtime;
use tokio::runtime::Runtime;

use mediarepo_core::error::RepoResult;
use mediarepo_core::futures;
use mediarepo_core::settings::Settings;
use mediarepo_core::utils::parse_tags_file;
use mediarepo_model::file::{File as RepoFile, File};
use mediarepo_model::repo::Repo;
use mediarepo_socket::start_tcp_server;
use num_integer::Integer;
use std::env;

use crate::constants::{
    DEFAULT_STORAGE_NAME, DEFAULT_STORAGE_PATH, SETTINGS_PATH, THUMBNAIL_STORAGE_NAME,
    THUMBNAIL_STORAGE_PATH,
};
use crate::utils::{create_paths_for_repo, get_repo, load_settings};

mod constants;
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

    /// Imports file from a folder (by glob pattern) into the repository
    Import {
        /// The path to the folder where the files are located
        #[structopt()]
        folder_path: String,

        /// If imported files should be deleted after import
        #[structopt(long)]
        delete: bool,
    },

    /// Starts the event server for the selected repository
    Start,
}

fn main() -> RepoResult<()> {
    let mut opt: Opt = Opt::from_args();
    opt.repo = env::current_dir()
        .unwrap()
        .join(opt.repo)
        .canonicalize()
        .unwrap();
    let mut _guard = None;
    if opt.profile {
        _guard = Some(logging::init_tracing_flame());
    } else {
        logging::init_tracing();
    }

    match opt.cmd.clone() {
        SubCommand::Init { force } => get_single_thread_runtime().block_on(init(opt, force)),
        SubCommand::Start => get_multi_thread_runtime().block_on(start_server(opt)),
        SubCommand::Import {
            folder_path,
            delete,
        } => get_single_thread_runtime().block_on(import(opt, folder_path, delete)),
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

async fn init_repo(opt: &Opt) -> RepoResult<(Settings, Repo)> {
    let settings = load_settings(&opt.repo.join(SETTINGS_PATH)).await?;
    let mut repo = get_repo(&opt.repo.join(&settings.database_path).to_str().unwrap()).await?;

    repo.set_main_storage(&settings.default_file_store).await?;
    repo.set_thumbnail_storage(opt.repo.join(&settings.thumbnail_store))
        .await?;
    Ok((settings, repo))
}

/// Starts the server
async fn start_server(opt: Opt) -> RepoResult<()> {
    let (settings, repo) = init_repo(&opt).await?;
    let mut handles = Vec::new();

    #[cfg(unix)]
    {
        let socket_path = opt.repo.join("repo.sock");
        let handle =
            mediarepo_socket::create_unix_socket(socket_path, settings.clone(), repo.clone())?;
        handles.push(handle);
    }

    let (address, tcp_handle) = start_tcp_server(
        settings.listen_address.clone(),
        settings.port_range,
        settings,
        repo,
    )?;
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
    create_paths_for_repo(
        &opt.repo,
        &settings,
        DEFAULT_STORAGE_PATH,
        THUMBNAIL_STORAGE_PATH,
    )
    .await?;
    let db_path = opt.repo.join(&settings.database_path);
    if db_path.exists() {
        panic!("Database already exists in location. Use --force with init to delete everything and start a new repository");
    }
    log::debug!("Creating repo");
    let repo = get_repo(&db_path.to_str().unwrap()).await?;
    let storage_path = opt.repo.join(DEFAULT_STORAGE_PATH).canonicalize().unwrap();
    log::debug!("Adding storage");
    repo.add_storage(DEFAULT_STORAGE_NAME, storage_path.to_str().unwrap())
        .await?;
    let thumb_storage_path = opt
        .repo
        .join(THUMBNAIL_STORAGE_PATH)
        .canonicalize()
        .unwrap();
    repo.add_storage(THUMBNAIL_STORAGE_NAME, thumb_storage_path.to_str().unwrap())
        .await?;
    let settings_string = settings.to_toml_string()?;
    log::debug!("Writing settings");
    fs::write(opt.repo.join(SETTINGS_PATH), &settings_string.into_bytes()).await?;
    log::info!("Repository initialized");

    Ok(())
}

/// Imports files from a source into the database
async fn import(opt: Opt, path: String, delete_files: bool) -> RepoResult<()> {
    let (_s, repo) = init_repo(&opt).await?;
    log::info!("Importing");

    let paths: Vec<PathBuf> = glob::glob(&path)
        .unwrap()
        .into_iter()
        .filter_map(|r| r.ok())
        .filter(|e| e.is_file())
        .collect();

    for path in paths {
        if let Err(e) = import_single_image(&path, &repo).await {
            log::error!("Import failed: {:?}", e);
            if delete_files {
                log::info!("Deleting file {:?}", path);
                let _ = fs::remove_file(&path).await;
            }
        } else {
            if delete_files {
                log::info!("Deleting file {:?}", path);
                let _ = fs::remove_file(&path).await;
            }
        }
    }
    log::info!("Creating thumbnails...");
    let mut files = repo.files().await?;

    for _ in 0..(files.len().div_ceil(&64)) {
        futures::future::join_all(
            (0..64)
                .filter_map(|_| files.pop())
                .map(|f| create_file_thumbnails(&repo, f)),
        )
        .await
        .into_iter()
        .filter_map(|r| r.err())
        .for_each(|e| log::error!("Failed to create thumbnail: {:?}", e));
    }

    Ok(())
}

/// Creates thumbnails of all sizes
async fn import_single_image(path: &PathBuf, repo: &Repo) -> RepoResult<()> {
    log::info!("Importing file");
    let file = repo.add_file_by_path(path.clone()).await?;
    log::info!("Adding tags");
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
        let mut tags = parse_tags_file(tags_path).await?;
        log::info!("Found {} tags in the tag file", tags.len());
        let resolved_tags = repo.tags_by_names(tags.clone()).await?;

        tags.retain(|tag| {
            resolved_tags
                .iter()
                .find(|t| if let (Some(ns1), Some(ns2)) = (t.namespace(), &tag.0) {
                    *ns1.name() == *ns2
                } else { t.namespace().is_none() && tag.0.is_none() } && *t.name() == *tag.1)
                .is_none()
        });
        let mut tag_ids: Vec<i64> = resolved_tags.into_iter().map(|t| t.id()).collect();
        log::info!("Existing tag_ids count is {}", tag_ids.len());
        log::info!("{} tags need to be created", tags.len());

        for (namespace, name) in tags {
            let tag = if let Some(namespace) = namespace {
                log::info!("Adding namespaced tag '{}:{}'", namespace, name);
                repo.add_namespaced_tag(name, namespace).await?
            } else {
                log::info!("Adding unnamespaced tag '{}'", name);
                repo.add_unnamespaced_tag(name).await?
            };
            tag_ids.push(tag.id());
        }
        log::info!("Mapping {} tags to the file", tag_ids.len());
        if !tag_ids.is_empty() {
            file.add_tags(tag_ids).await?;
        }
    } else {
        log::info!("No tags file '{:?}' found", tags_path);
    }
    Ok(())
}

#[tracing::instrument(skip(repo, file))]
async fn create_file_thumbnails(repo: &Repo, file: File) -> RepoResult<()> {
    let file_thumbnails = repo.get_file_thumbnails(file.hash().to_owned()).await?;

    if file_thumbnails.is_empty() {
        repo.create_thumbnails_for_file(&file).await?;
    }
    Ok(())
}
