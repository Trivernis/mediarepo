use crate::TypeMap;
use mediarepo_core::bromine::prelude::*;
use mediarepo_core::error::RepoResult;
use mediarepo_core::mediarepo_api::types::jobs::{JobType, RunJobRequest};
use mediarepo_core::type_keys::{RepoPathKey, SettingsKey, SizeMetadataKey};
use mediarepo_logic::dao::DaoProvider;
use mediarepo_worker::jobs::{CalculateSizesJob, VacuumJob};

use crate::utils::{get_job_dispatcher_from_context, get_repo_from_context};

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
        let dispatcher = get_job_dispatcher_from_context(ctx).await;

        if !run_request.sync {
            // early response to indicate that the job will be run
            ctx.emit_to(Self::name(), "run_job", ()).await?;
        }

        match run_request.job_type {
            JobType::MigrateContentDescriptors => job_dao.migrate_content_descriptors().await?,
            JobType::CalculateSizes => calculate_all_sizes(ctx).await?,
            JobType::CheckIntegrity => job_dao.check_integrity().await?,
            JobType::Vacuum => {
                dispatcher.dispatch(VacuumJob::default()).await;
            }
            JobType::GenerateThumbnails => job_dao.generate_missing_thumbnails().await?,
        }

        Ok(Response::empty())
    }
}

async fn calculate_all_sizes(ctx: &Context) -> RepoResult<()> {
    let (repo_path, settings) = {
        let data = ctx.data.read().await;
        (
            data.get::<RepoPathKey>().unwrap().clone(),
            data.get::<SettingsKey>().unwrap().clone(),
        )
    };
    let job = CalculateSizesJob::new(repo_path, settings);
    let dispatcher = get_job_dispatcher_from_context(ctx).await;
    let state = dispatcher.dispatch(job).await;
    let mut rx = {
        let state = state.read().await;
        state.sizes_channel.subscribe()
    };

    while let Ok((size_type, size)) = rx.recv().await {
        let mut data = ctx.data.write().await;
        let size_map = data.get_mut::<SizeMetadataKey>().unwrap();
        size_map.insert(size_type, size);
    }

    Ok(())
}
