mod calculate_sizes;
mod check_integrity;
mod generate_missing_thumbnails;
mod vacuum;

pub use calculate_sizes::*;
pub use check_integrity::*;
pub use generate_missing_thumbnails::*;
use std::marker::PhantomData;
use std::sync::Arc;
pub use vacuum::*;

use crate::handle::JobHandle;
use async_trait::async_trait;
use mediarepo_core::error::RepoResult;
use mediarepo_core::trait_bound_typemap::TypeMapKey;
use mediarepo_logic::dao::job::JobDao;
use mediarepo_logic::dao::repo::Repo;
use tokio::sync::RwLock;

type EmptyStatus = Arc<RwLock<()>>;

#[async_trait]
pub trait Job: Clone + Send + Sync {
    type JobStatus: Send + Sync;
    type Result: Send + Sync;

    fn status(&self) -> Arc<RwLock<Self::JobStatus>>;

    async fn run(&self, repo: Arc<Repo>) -> RepoResult<Self::Result>;

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
    type Value = JobHandle<T::JobStatus, T::Result>;
}
