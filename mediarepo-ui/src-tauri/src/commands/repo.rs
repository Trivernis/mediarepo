use std::path::PathBuf;
use rmp_ipc::IPCBuilder;
use serde::{Serialize, Deserialize};
use crate::context::Context;
use crate::error::{AppError, AppResult};
use tokio::fs;
use crate::settings::save_settings;
use rmp_ipc::context::Context as IPCContext;
use tauri::Window;
use crate::ipc::build_ipc_context;
use std::mem;

static REPO_CONFIG_FILE: &str = "repo.toml";

#[derive(Serialize, Deserialize, Clone)]
pub struct Repository {
  name: String,
  path: Option<String>,
  address: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RepoConfig {
  pub listen_address: String,
  pub database_path: String,
  pub default_file_store: String,
}

#[tauri::command]
pub async fn get_repositories(context: tauri::State<'_, Context>) -> AppResult<Vec<Repository>> {
  let settings = context.settings.read().await;

  Ok(settings.repositories.values().cloned().collect())
}

#[tauri::command]
pub async fn get_active_repository(context: tauri::State<'_, Context>) -> AppResult<Option<Repository>> {
  let repo = context.active_repository.read().await;
  Ok(repo.clone())
}

#[tauri::command]
pub async fn add_repository(name: String, path: String, context: tauri::State<'_, Context>) -> AppResult<Vec<Repository>> {
  let repo_path = path.clone();
  let path = PathBuf::from(path);
  let RepoConfig { listen_address, .. } = read_repo_config(path.join(REPO_CONFIG_FILE)).await?;

  let repo = Repository {
    name,
    path: Some(repo_path),
    address: listen_address,
  };

  let mut repositories = Vec::new();
  {
    let mut settings = context.settings.write().await;
    settings.repositories.insert(repo.name.clone(), repo);
    save_settings(&settings)?;
    repositories.append(&mut settings.repositories.values().cloned().collect());
  }

  Ok(repositories)
}

#[tauri::command]
pub async fn select_repository(window: Window, name: String, context: tauri::State<'_, Context>) -> AppResult<()> {
  let settings = context.settings.read().await;
  let repo = settings.repositories.get(&name).ok_or(AppError::new(format!("Repository '{}' not found", name)))?;
  let ipc = connect(window, &repo.address).await?;
  let mut ipc_ctx = context.ipc.write().await;
  let old_ipc = mem::replace(&mut *ipc_ctx, Some(ipc));

  if let Some(old_ctx) = old_ipc {
    old_ctx.stop().await?;
  }
  let mut active_repo = context.active_repository.write().await;
  *active_repo = Some(repo.clone());

  Ok(())
}

async fn read_repo_config(path: PathBuf) -> AppResult<RepoConfig> {
  let toml_str = fs::read_to_string(path).await?;
  let config = toml::from_str(&toml_str)?;

  Ok(config)
}

/// Connects to the IPC Server
async fn connect(window: Window, address: &str) -> AppResult<IPCContext> {
  build_ipc_context(window, address).await
}
