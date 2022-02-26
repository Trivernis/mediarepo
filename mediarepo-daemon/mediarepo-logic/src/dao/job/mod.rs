use crate::dao_provider;
use crate::dto::JobDto;
use chrono::Local;
use mediarepo_core::error::RepoResult;
use mediarepo_database::entities::job;
use sea_orm::prelude::*;

pub mod generate_missing_thumbnails;
pub mod migrate_content_descriptors;
pub mod sqlite_operations;
pub mod state;

dao_provider!(JobDao);

impl JobDao {
    /// Returns a list of all jobs that are scheduled (have a next_run date)
    pub async fn scheduled_for_now(&self) -> RepoResult<Vec<JobDto>> {
        let jobs = job::Entity::find()
            .filter(job::Column::NextRun.is_not_null())
            .filter(job::Column::NextRun.lt(Local::now().naive_local()))
            .all(&self.ctx.db)
            .await?
            .into_iter()
            .map(JobDto::new)
            .collect();

        Ok(jobs)
    }
}
