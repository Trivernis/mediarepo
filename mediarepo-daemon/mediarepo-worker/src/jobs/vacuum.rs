use crate::execution_state::{ExecutionStateSynchronizer, JobExecutionState};
use crate::jobs::ScheduledJob;
use crate::progress::{JobProgressUpdate, ProgressSender};
use crate::state_data::StateData;
use async_trait::async_trait;
use mediarepo_core::error::RepoResult;
use mediarepo_logic::dao::repo::Repo;
use mediarepo_logic::dao::DaoProvider;
use std::sync::Arc;
use tokio::sync::mpsc::Sender;
use tokio::sync::RwLock;

#[derive(Default, Clone)]
pub struct VacuumJob;

#[async_trait]
impl ScheduledJob for VacuumJob {
    async fn set_state(&self, _: StateData) -> RepoResult<()> {
        Ok(())
    }

    async fn run(&self, sender: &ProgressSender, repo: Repo) -> RepoResult<()> {
        sender.send_progress_percent(0.0);
        repo.job().vacuum().await?;
        sender.send_progress_percent(1.0);

        Ok(())
    }
}
