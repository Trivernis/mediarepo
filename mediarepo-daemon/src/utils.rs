use mediarepo_core::error::RepoResult;
use mediarepo_core::settings::Settings;
use mediarepo_model::repo::Repo;
use std::path::PathBuf;
use tokio::fs;

/// Loads the settings from a toml path
pub async fn load_settings(path: &PathBuf) -> RepoResult<Settings> {
    let contents = fs::read_to_string(path).await?;
    Settings::from_toml_string(&contents)
}

pub async fn get_repo(db_path: &str) -> RepoResult<Repo> {
    Repo::connect(format!("sqlite://{}", db_path)).await
}

pub async fn create_paths_for_repo(root: &PathBuf, settings: &Settings) -> RepoResult<()> {
    if !root.exists() {
        fs::create_dir_all(&root).await?;
    }
    let db_path = root.join(&settings.database_path);
    if !db_path.exists() {
        fs::create_dir_all(db_path.parent().unwrap()).await?;
    }
    let storage_path = root.join(&settings.default_file_store);
    if !storage_path.exists() {
        fs::create_dir_all(storage_path).await?;
    }

    Ok(())
}
