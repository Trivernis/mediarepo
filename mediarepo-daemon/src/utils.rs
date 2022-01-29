use std::path::PathBuf;

use tokio::fs;

use mediarepo_core::error::RepoResult;
use mediarepo_core::settings::{PathSettings, Settings};
use mediarepo_core::settings::v1::SettingsV1;
use mediarepo_logic::dao::repo::Repo;

/// Loads the settings from a toml path
pub fn load_settings(root_path: &PathBuf) -> RepoResult<Settings> {
    let contents = std::fs::read_to_string(root_path.join("repo.toml"))?;

    if let Ok(settings_v1) = SettingsV1::from_toml_string(&contents) {
        let settings = Settings::from_v1(settings_v1)?;
        settings.save(root_path)?;

        Ok(settings)
    } else {
        Settings::read(root_path)
    }
}

pub async fn get_repo(root_path: &PathBuf, path_settings: &PathSettings) -> RepoResult<Repo> {
    Repo::connect(
        format!(
            "sqlite://{}",
            path_settings.db_file_path(root_path).to_string_lossy()
        ),
        path_settings.files_dir(root_path),
        path_settings.thumbs_dir(root_path),
    )
    .await
}

pub async fn create_paths_for_repo(root: &PathBuf, settings: &PathSettings) -> RepoResult<()> {
    if !root.exists() {
        fs::create_dir_all(&root).await?;
    }
    let db_path = settings.database_dir(root);
    if !db_path.exists() {
        fs::create_dir_all(db_path).await?;
    }
    let files_path = settings.files_dir(root);
    if !files_path.exists() {
        fs::create_dir_all(files_path).await?;
    }
    let thumbnail_path = settings.thumbs_dir(root);
    if !thumbnail_path.exists() {
        fs::create_dir_all(thumbnail_path).await?;
    }

    Ok(())
}
