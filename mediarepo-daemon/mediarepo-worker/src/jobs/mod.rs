mod calculate_sizes;
mod vacuum;

pub use calculate_sizes::*;
use std::marker::PhantomData;
use std::sync::Arc;
pub use vacuum::*;

use async_trait::async_trait;
use mediarepo_core::error::RepoResult;
use mediarepo_core::typemap_rev::TypeMapKey;
use mediarepo_logic::dao::job::JobDao;
use mediarepo_logic::dao::repo::Repo;
use tokio::sync::RwLock;

type EmptyStatus = Arc<RwLock<()>>;

#[async_trait]
pub trait Job: Clone + Send + Sync {
    type JobState: Send + Sync;

    fn state(&self) -> Arc<RwLock<Self::JobState>>;

    async fn run(&self, repo: Arc<Repo>) -> RepoResult<()>;

    #[tracing::instrument(level = "debug", skip_all)]
    async fn save_state(&self, _job_dao: JobDao) -> RepoResult<()> {
        Ok(())
    }
}

pub struct JobTypeKey<T: Job>(PhantomData<T>);

impl<T: 'static> TypeMapKey for JobTypeKey<T>
where
    T: Job,
{
    type Value = Arc<RwLock<T::JobState>>;
}
