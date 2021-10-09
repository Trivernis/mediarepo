use crate::settings::Settings;
use typemap_rev::TypeMapKey;

pub struct SettingsKey;

impl TypeMapKey for SettingsKey {
    type Value = Settings;
}
