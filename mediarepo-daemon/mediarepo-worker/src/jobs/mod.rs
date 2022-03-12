mod vacuum;

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
    type JobStatus: Send + Sync;

    fn status(&self) -> Arc<RwLock<Self::JobStatus>>;

    async fn run(&self, repo: Arc<Repo>) -> RepoResult<()>;

    async fn save_status(&self, job_dao: JobDao) -> RepoResult<()>;
}

pub struct JobTypeKey<T: Job>(PhantomData<T>);

impl<T: 'static> TypeMapKey for JobTypeKey<T>
where
    T: Job,
{
    type Value = Arc<RwLock<T::JobStatus>>;
}
