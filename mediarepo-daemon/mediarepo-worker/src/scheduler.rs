use crate::execution_state::JobExecutionState;
use crate::jobs::{ScheduledJob, VacuumJob};
use crate::jobs_table::JobsTable;
use crate::progress::{JobProgressUpdate, ProgressSender};
use crate::state_data::StateData;
use mediarepo_core::error::RepoResult;
use mediarepo_core::futures::select;
use mediarepo_core::settings::LogLevel::Debug;
use mediarepo_core::tokio_graceful_shutdown::SubsystemHandle;
use mediarepo_database::entities::job::JobType;
use mediarepo_logic::dao::repo::Repo;
use mediarepo_logic::dao::DaoProvider;
use mediarepo_logic::dto::JobDto;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::mpsc::{channel, Receiver, Sender};
use tokio::sync::RwLock;

#[derive(Clone)]
pub struct Scheduler {
    repo: Repo,
    state_data: Arc<RwLock<Vec<StateData>>>,
    jobs_table: JobsTable,
}

impl Scheduler {
    pub fn new(repo: Repo) -> Self {
        Self {
            repo,
            state_data: Default::default(),
            jobs_table: Default::default(),
        }
    }

    pub fn run(self, subsystem: SubsystemHandle) -> JobsTable {
        tokio::task::spawn({
            let subsystem = subsystem.clone();
            let scheduler = self.clone();
            async move {
                scheduler.loop_save_states(subsystem).await;
            }
        });

        let (tx, rx) = channel(32);

        tokio::task::spawn({
            let scheduler = self.clone();
            async move {
                scheduler.loop_schedule(subsystem, tx).await;
            }
        });

        let jobs_table = self.jobs_table.clone();

        tokio::task::spawn(async move { self.update_on_progress(rx).await });

        jobs_table
    }

    async fn loop_schedule(
        self,
        subsystem: SubsystemHandle,
        job_progress_sender: Sender<JobProgressUpdate>,
    ) {
        loop {
            if let Err(e) = self.schedule(&job_progress_sender).await {
                tracing::error!("failed to schedule jobs: {}", e);
            }
            tokio::select! {
                _ = tokio::time::sleep(Duration::from_secs(1)) => {},
                _ = subsystem.on_shutdown_requested() => {
                    break;
                }
            }
        }
    }

    async fn schedule(&self, job_progress_sender: &Sender<JobProgressUpdate>) -> RepoResult<()> {
        let mut scheduled_jobs = self.repo.job().scheduled_for_now().await?;
        let running_jobs = self.running_jobs().await;

        scheduled_jobs.retain(|j| !running_jobs.contains(&j.id()));

        for job in scheduled_jobs {
            let mut sender = job_progress_sender.clone();
            let mut progress = JobProgressUpdate::new(job.id());
            let scheduled_job = create_job(job);
            let _ = sender.send(progress.clone()).await;
            let repo = self.repo.clone();

            tokio::task::spawn(async move {
                progress.set_state(JobExecutionState::Running);
                let _ = sender.send(progress.clone()).await;

                let progress_sender = ProgressSender::new(progress.id(), sender);
                if let Err(e) = scheduled_job.run(&progress_sender, repo).await {
                    tracing::error!("error occurred during job execution: {}", e);
                }
                let sender = progress_sender.inner;
                progress.set_state(JobExecutionState::Finished);
                let _ = sender.send(progress).await;
            });
        }

        Ok(())
    }

    async fn loop_save_states(self, subsystem: SubsystemHandle) {
        loop {
            if let Err(e) = self.save_states().await {
                tracing::error!("failed to save job state {}", e);
            }

            tokio::select! {
                _ = tokio::time::sleep(Duration::from_secs(1)) => {},
                _ = subsystem.on_shutdown_requested() => {
                    let _ = self.save_states().await;
                    break;
                }
            }
        }
    }

    async fn save_states(&self) -> RepoResult<()> {
        let mut changed_states = Vec::new();
        {
            let states = self.state_data.read().await;
            for state in &*states {
                changed_states.append(&mut state.changed_states().await);
            }
        }
        self.repo
            .job()
            .upsert_multiple_states(changed_states)
            .await?;

        Ok(())
    }

    async fn update_on_progress(mut self, mut rx: Receiver<JobProgressUpdate>) {
        while let Some(progress) = rx.recv().await {
            let mut jobs_table = self.jobs_table.write().await;

            if let JobExecutionState::Finished = progress.state() {
                let mut state_data = self.state_data.write().await;
                state_data.retain(|s| s.job_id() != progress.id());
            }

            if let Some(entry) = jobs_table.get_mut(&progress.id()) {
                entry.last_update = Some(progress);
            }
        }
    }

    async fn running_jobs(&self) -> Vec<i64> {
        let jobs_table = self.jobs_table.read().await;
        jobs_table
            .values()
            .filter_map(|v| v.last_update.as_ref())
            .filter(|u| u.state() != JobExecutionState::Finished)
            .map(|u| u.id())
            .collect()
    }
}

fn create_job(dto: JobDto) -> Box<dyn ScheduledJob + Send + Sync> {
    match dto.job_type() {
        JobType::MigrateCDs => {
            todo!()
        }
        JobType::CalculateSizes => {
            todo!()
        }
        JobType::GenerateThumbs => {
            todo!()
        }
        JobType::CheckIntegrity => {
            todo!()
        }
        JobType::Vacuum => Box::new(VacuumJob),
    }
}
