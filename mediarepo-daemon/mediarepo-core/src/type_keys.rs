use crate::settings::Settings;
use std::path::PathBuf;
use typemap_rev::TypeMapKey;

pub struct SettingsKey;

impl TypeMapKey for SettingsKey {
    type Value = Settings;
}

pub struct RepoPathKey;

impl TypeMapKey for RepoPathKey {
    type Value = PathBuf;
}
