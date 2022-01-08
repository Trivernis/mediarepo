use crate::tauri_plugin::commands::ApiAccess;
use crate::tauri_plugin::error::PluginResult;
use crate::types::jobs::JobType;

#[tauri::command]
pub async fn run_job(api_state: ApiAccess<'_>, job_type: JobType) -> PluginResult<()> {
    let api = api_state.api().await?;
    api.job.run_job(job_type).await?;

    Ok(())
}
