use crate::dao::job::JobDao;
use crate::dto::JobStateDto;
use mediarepo_core::error::RepoResult;
use mediarepo_database::entities::job_state;
use sea_orm::prelude::*;

impl JobDao {
    /// Returns all job states for a given job id
    pub async fn states_for_job_id(&self, job_id: i64) -> RepoResult<Vec<JobStateDto>> {
        let states = job_state::Entity::find()
            .filter(job_state::Column::JobId.eq(job_id))
            .all(&self.ctx.db)
            .await?
            .into_iter()
            .map(JobStateDto::new)
            .collect();

        Ok(states)
    }
}
