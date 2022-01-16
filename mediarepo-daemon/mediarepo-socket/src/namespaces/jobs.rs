use crate::utils::{calculate_size, get_repo_from_context};
use mediarepo_core::bromine::prelude::*;
use mediarepo_core::error::RepoResult;
use mediarepo_core::mediarepo_api::types::jobs::{JobType, RunJobRequest};
use mediarepo_core::mediarepo_api::types::repo::SizeType;
use mediarepo_core::type_keys::SizeMetadataKey;

pub struct JobsNamespace;

impl NamespaceProvider for JobsNamespace {
    fn name() -> &'static str {
        "jobs"
    }

    fn register(handler: &mut EventHandler) {
        events!(handler,
            "run_job" => Self::run_job
        )
    }
}

impl JobsNamespace {
    #[tracing::instrument(skip_all)]
    pub async fn run_job(ctx: &Context, event: Event) -> IPCResult<()> {
        let run_request = event.payload::<RunJobRequest>()?;
        let repo = get_repo_from_context(ctx).await;

        match run_request.job_type {
            JobType::MigrateContentDescriptors => repo.migrate().await?,
            JobType::CalculateSizes => calculate_all_sizes(ctx).await?,
            JobType::CheckIntegrity => {}
        }

        ctx.emit_to(Self::name(), "run_job", ()).await?;

        Ok(())
    }
}

async fn calculate_all_sizes(ctx: &Context) -> RepoResult<()> {
    let size_types = vec![
        SizeType::Total,
        SizeType::FileFolder,
        SizeType::ThumbFolder,
        SizeType::DatabaseFile,
    ];
    for size_type in size_types {
        let size = calculate_size(&size_type, ctx).await?;
        let mut data = ctx.data.write().await;
        let size_map = data.get_mut::<SizeMetadataKey>().unwrap();
        size_map.insert(size_type, size);
    }

    Ok(())
}
