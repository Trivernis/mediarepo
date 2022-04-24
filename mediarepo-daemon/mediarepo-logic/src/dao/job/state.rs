use crate::dao::job::JobDao;
use crate::dto::{JobStateDto, UpsertJobStateDto};
use mediarepo_core::error::RepoResult;
use mediarepo_database::entities::job_state;
use mediarepo_database::entities::job_state::JobType;
use sea_orm::prelude::*;
use sea_orm::ActiveValue::Set;
use sea_orm::{Condition, TransactionTrait};

impl JobDao {
    /// Returns all job states for a given job id
    pub async fn state_for_job_type(&self, job_type: JobType) -> RepoResult<Option<JobStateDto>> {
        let state = job_state::Entity::find()
            .filter(job_state::Column::JobType.eq(job_type))
            .one(&self.ctx.db)
            .await?
            .map(JobStateDto::new);

        Ok(state)
    }

    pub async fn upsert_state(&self, state: UpsertJobStateDto) -> RepoResult<()> {
        self.upsert_multiple_states(vec![state]).await
    }

    pub async fn upsert_multiple_states(&self, states: Vec<UpsertJobStateDto>) -> RepoResult<()> {
        let trx = self.ctx.db.begin().await?;

        job_state::Entity::delete_many()
            .filter(build_state_filters(&states))
            .exec(&trx)
            .await?;
        job_state::Entity::insert_many(build_active_state_models(states))
            .exec(&trx)
            .await?;

        trx.commit().await?;

        Ok(())
    }
}

fn build_state_filters(states: &[UpsertJobStateDto]) -> Condition {
    states
        .iter()
        .map(|s| Condition::all().add(job_state::Column::JobType.eq(s.job_type)))
        .fold(Condition::any(), |acc, cond| acc.add(cond))
}

fn build_active_state_models(states: Vec<UpsertJobStateDto>) -> Vec<job_state::ActiveModel> {
    states
        .into_iter()
        .map(|s| job_state::ActiveModel {
            job_type: Set(s.job_type),
            value: Set(s.value),
        })
        .collect()
}
