use crate::TypeMap;
use mediarepo_core::bromine::prelude::*;
use mediarepo_core::error::RepoResult;
use mediarepo_core::mediarepo_api::types::jobs::{JobType, RunJobRequest};
use mediarepo_core::type_keys::{RepoPathKey, SettingsKey, SizeMetadataKey};
use mediarepo_worker::handle::JobState;
use mediarepo_worker::job_dispatcher::JobDispatcher;
use mediarepo_worker::jobs::{
    CalculateSizesJob, CheckIntegrityJob, GenerateMissingThumbsJob, Job, MigrateCDsJob, VacuumJob,
};

use crate::utils::get_job_dispatcher_from_context;

pub struct JobsNamespace;

impl NamespaceProvider for JobsNamespace {
    fn name() -> &'static str {
        "jobs"
    }

    fn register(handler: &mut EventHandler) {
        events!(handler,
            "run_job" => Self::run_job,
            "is_job_running" => Self::is_job_running
        );
    }
}

impl JobsNamespace {
    #[tracing::instrument(skip_all)]
    pub async fn run_job(ctx: &Context, event: Event) -> IPCResult<Response> {
        let run_request = event.payload::<RunJobRequest>()?;
        let dispatcher = get_job_dispatcher_from_context(ctx).await;

        if !run_request.sync {
            // early response to indicate that the job will be run
            ctx.emit_to(Self::name(), "run_job", ()).await?;
        }

        match run_request.job_type {
            JobType::MigrateContentDescriptors => {
                dispatch_job(&dispatcher, MigrateCDsJob::default(), run_request.sync).await?
            }
            JobType::CalculateSizes => calculate_all_sizes(ctx).await?,
            JobType::CheckIntegrity => {
                dispatch_job(&dispatcher, CheckIntegrityJob::default(), run_request.sync).await?
            }
            JobType::Vacuum => {
                dispatch_job(&dispatcher, VacuumJob::default(), run_request.sync).await?
            }
            JobType::GenerateThumbnails => {
                dispatch_job(
                    &dispatcher,
                    GenerateMissingThumbsJob::default(),
                    run_request.sync,
                )
                .await?
            }
        }

        Ok(Response::empty())
    }

    #[tracing::instrument(skip_all)]
    pub async fn is_job_running(ctx: &Context, event: Event) -> IPCResult<Response> {
        let job_type = event.payload::<JobType>()?;
        let dispatcher = get_job_dispatcher_from_context(ctx).await;

        let running = match job_type {
            JobType::MigrateContentDescriptors => {
                is_job_running::<MigrateCDsJob>(&dispatcher).await
            }
            JobType::CalculateSizes => is_job_running::<CalculateSizesJob>(&dispatcher).await,
            JobType::GenerateThumbnails => {
                is_job_running::<GenerateMissingThumbsJob>(&dispatcher).await
            }
            JobType::CheckIntegrity => is_job_running::<CheckIntegrityJob>(&dispatcher).await,
            JobType::Vacuum => is_job_running::<VacuumJob>(&dispatcher).await,
        };

        Response::payload(ctx, running)
    }
}

async fn dispatch_job<J: 'static + Job>(
    dispatcher: &JobDispatcher,
    job: J,
    sync: bool,
) -> RepoResult<()> {
    let mut handle = if let Some(handle) = dispatcher.get_handle::<J>().await {
        if handle.state().await == JobState::Running {
            handle
        } else {
            dispatcher.dispatch(job).await
        }
    } else {
        dispatcher.dispatch(job).await
    };
    if sync {
        if let Some(result) = handle.take_result().await {
            result?;
        }
    }
    Ok(())
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
    let handle = dispatcher.dispatch(job).await;
    let mut rx = {
        let status = handle.status().read().await;
        status.sizes_channel.subscribe()
    };

    while let Ok((size_type, size)) = rx.recv().await {
        let mut data = ctx.data.write().await;
        let size_map = data.get_mut::<SizeMetadataKey>().unwrap();
        size_map.insert(size_type, size);
    }

    Ok(())
}

async fn is_job_running<T: 'static + Job>(dispatcher: &JobDispatcher) -> bool {
    if let Some(handle) = dispatcher.get_handle::<T>().await {
        let state = handle.state().await;
        state == JobState::Running
    } else {
        false
    }
}
