use std::path::PathBuf;
use serde::{Serialize, Deserialize};
use crate::context::Context;
use crate::error::AppResult;
use tokio::fs;
use crate::settings::save_settings;

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

  Ok(settings.repositories.clone())
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
    settings.repositories.push(repo);
    save_settings(&settings)?;
    repositories.append(&mut settings.repositories.clone());
  }

  Ok(repositories)
}

async fn read_repo_config(path: PathBuf) -> AppResult<RepoConfig> {
  let toml_str = fs::read_to_string(path).await?;
  let config = toml::from_str(&toml_str)?;

  Ok(config)
}
