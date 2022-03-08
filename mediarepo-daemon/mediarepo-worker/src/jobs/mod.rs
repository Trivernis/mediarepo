mod vacuum;

pub use vacuum::*;

use crate::execution_state::JobExecutionState;
use crate::progress::{JobProgressUpdate, ProgressSender};
use crate::state_data::StateData;
use async_trait::async_trait;
use mediarepo_core::error::RepoResult;
use mediarepo_logic::dao::repo::Repo;
use tokio::sync::mpsc::Sender;

#[async_trait]
pub trait ScheduledJob {
    async fn set_state(&self, state: StateData) -> RepoResult<()>;

    async fn run(&self, sender: &ProgressSender, repo: Repo) -> RepoResult<()>;
}
