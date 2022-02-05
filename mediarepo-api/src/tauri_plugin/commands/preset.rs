use crate::tauri_plugin::commands::ApiAccess;
use crate::tauri_plugin::error::PluginResult;
use crate::types::filtering::{SortingPreset, SortKey};

#[tauri::command]
pub async fn all_sorting_presets(api_state: ApiAccess<'_>) -> PluginResult<Vec<SortingPreset>> {
    let api = api_state.api().await?;
    let presets = api.preset.all_sorting_presets().await?;

    Ok(presets)
}

#[tauri::command]
pub async fn add_sorting_preset(api_state: ApiAccess<'_>, sort_keys: Vec<SortKey>) -> PluginResult<SortingPreset> {
    let api = api_state.api().await?;
    let preset = api.preset.add_sorting_preset(sort_keys).await?;

    Ok(preset)
}

#[tauri::command]
pub async fn delete_sorting_preset(api_state: ApiAccess<'_>, id: i32) -> PluginResult<()> {
    let api = api_state.api().await?;
    api.preset.delete_sorting_preset(id).await?;

    Ok(())
}