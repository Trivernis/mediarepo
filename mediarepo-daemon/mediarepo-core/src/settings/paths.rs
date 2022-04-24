use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PathSettings {
    pub(crate) database_directory: String,
    pub(crate) files_directory: String,
    pub(crate) thumbnail_directory: String,
}

impl Default for PathSettings {
    fn default() -> Self {
        Self {
            database_directory: String::from("db"),
            files_directory: String::from("files"),
            thumbnail_directory: String::from("thumbnails"),
        }
    }
}

impl PathSettings {
    #[inline]
    pub fn database_dir(&self, root: &Path) -> PathBuf {
        root.join(&self.database_directory)
    }

    #[inline]
    pub fn files_dir(&self, root: &Path) -> PathBuf {
        root.join(&self.files_directory)
    }

    #[inline]
    pub fn thumbs_dir(&self, root: &Path) -> PathBuf {
        root.join(&self.thumbnail_directory)
    }

    #[inline]
    pub fn db_file_path(&self, root: &Path) -> PathBuf {
        self.database_dir(root).join("repo.db")
    }

    #[inline]
    pub fn frontend_state_file_path(&self, root: &Path) -> PathBuf {
        self.database_dir(root).join("frontend-state.json")
    }
}
