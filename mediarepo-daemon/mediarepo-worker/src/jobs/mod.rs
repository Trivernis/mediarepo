mod calculate_sizes;
mod check_integrity;
mod generate_missing_thumbnails;
mod migrate_content_descriptors;
mod vacuum;

pub use calculate_sizes::*;
pub use check_integrity::*;
pub use generate_missing_thumbnails::*;
pub use migrate_content_descriptors::*;
use std::marker::PhantomData;
use std::sync::Arc;
pub use vacuum::*;

use crate::handle::JobHandle;
use async_trait::async_trait;
use mediarepo_core::bincode;
use mediarepo_core::error::{RepoError, RepoResult};
use mediarepo_core::trait_bound_typemap::TypeMapKey;
use mediarepo_database::entities::job_state::JobType;
use mediarepo_logic::dao::job::JobDao;
use mediarepo_logic::dao::repo::Repo;
use mediarepo_logic::dto::{JobStateDto, UpsertJobStateDto};
use serde::de::DeserializeOwned;
use serde::Serialize;
use tokio::sync::RwLock;

type EmptyStatus = Arc<RwLock<()>>;

#[async_trait]
pub trait Job: Clone + Send + Sync {
    type JobStatus: Send + Sync;
    type Result: Send + Sync;

    fn status(&self) -> Arc<RwLock<Self::JobStatus>>;

    async fn load_state(&self, _job_dao: JobDao) -> RepoResult<()> {
        Ok(())
    }

    async fn run(&self, repo: Arc<Repo>) -> RepoResult<Self::Result>;

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

pub fn deserialize_state<T: DeserializeOwned>(dto: JobStateDto) -> RepoResult<T> {
    bincode::deserialize(dto.value()).map_err(RepoError::from)
}

pub fn serialize_state<T: Serialize>(
    job_type: JobType,
    state: &T,
) -> RepoResult<UpsertJobStateDto> {
    let dto = UpsertJobStateDto {
        value: bincode::serialize(state)?,
        job_type,
    };

    Ok(dto)
}
