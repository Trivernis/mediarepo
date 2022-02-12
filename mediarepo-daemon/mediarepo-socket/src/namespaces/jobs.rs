use mediarepo_core::bromine::prelude::*;
use mediarepo_core::error::RepoResult;
use mediarepo_core::mediarepo_api::types::jobs::{JobType, RunJobRequest};
use mediarepo_core::mediarepo_api::types::repo::SizeType;
use mediarepo_core::type_keys::SizeMetadataKey;
use mediarepo_logic::dao::DaoProvider;

use crate::utils::{calculate_size, get_repo_from_context};

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
    pub async fn run_job(ctx: &Context, event: Event) -> IPCResult<Response> {
        let run_request = event.payload::<RunJobRequest>()?;
        let job_dao = get_repo_from_context(ctx).await.job();

        if !run_request.sync {
            // early response to indicate that the job will be run
            ctx.emit_to(Self::name(), "run_job", ()).await?;
        }

        match run_request.job_type {
            JobType::MigrateContentDescriptors => job_dao.migrate_content_descriptors().await?,
            JobType::CalculateSizes => calculate_all_sizes(ctx).await?,
            JobType::CheckIntegrity => job_dao.check_integrity().await?,
            JobType::Vacuum => job_dao.vacuum().await?,
            JobType::GenerateThumbnails => job_dao.generate_missing_thumbnails().await?,
        }

        Ok(Response::empty())
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
