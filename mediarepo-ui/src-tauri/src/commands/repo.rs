use std::path::PathBuf;
use rmp_ipc::IPCBuilder;
use serde::{Serialize, Deserialize};
use crate::context::Context;
use crate::error::{AppError, AppResult};
use tokio::fs;
use crate::settings::save_settings;
use rmp_ipc::context::Context as IPCContext;

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
pub async fn select_repository(name: String, context: tauri::State<'_, Context>) -> AppResult<()> {
  let settings = context.settings.read().await;
  let repo = settings.repositories.get(&name).ok_or(AppError::new(format!("Repository '{}' not found", name)))?;
  let ipc = connect(&repo.address).await?;
  let mut ipc_ctx = context.ipc.write().await;
  *ipc_ctx = Some(ipc);

  Ok(())
}

async fn read_repo_config(path: PathBuf) -> AppResult<RepoConfig> {
  let toml_str = fs::read_to_string(path).await?;
  let config = toml::from_str(&toml_str)?;

  Ok(config)
}

/// Connects to the IPC Server
async fn connect(address: &str) -> AppResult<IPCContext> {
  let ctx = IPCBuilder::new().address(address).build_client().await?;

  Ok(ctx)
}
