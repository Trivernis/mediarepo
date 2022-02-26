use crate::state_data::StateData;
use async_trait::async_trait;
use mediarepo_core::error::RepoResult;
use mediarepo_logic::dao::repo::Repo;
use mediarepo_logic::dto::JobDto;

#[async_trait]
pub trait ScheduledJob {
    fn new(dto: JobDto) -> Self;

    async fn set_state(&self, state: StateData) -> RepoResult<()>;

    async fn run(&self, repo: Repo) -> RepoResult<()>;

    fn execution_state(&self) -> JobExecutionState;
}

#[derive(Clone, Copy, Debug, Ord, PartialOrd, Eq, PartialEq)]
pub enum JobExecutionState {
    Scheduled,
    Running,
    Finished,
}
