use crate::settings::Settings;
use mediarepo_api::types::repo::SizeType;
use std::collections::HashMap;
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

pub struct SizeMetadataKey;

impl TypeMapKey for SizeMetadataKey {
    type Value = HashMap<SizeType, u64>;
}
