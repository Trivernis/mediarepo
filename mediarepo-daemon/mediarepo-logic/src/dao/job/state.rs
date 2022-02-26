use crate::dao::job::JobDao;
use crate::dto::{JobStateDto, UpsertJobStateDto};
use mediarepo_core::error::RepoResult;
use mediarepo_database::entities::job_state;
use sea_orm::prelude::*;
use sea_orm::ActiveValue::Set;
use sea_orm::{Condition, TransactionTrait};

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

fn build_state_filters(states: &Vec<UpsertJobStateDto>) -> Condition {
    states
        .iter()
        .map(|s| {
            Condition::all()
                .add(job_state::Column::JobId.eq(s.job_id))
                .add(job_state::Column::Key.eq(s.key.to_owned()))
        })
        .fold(Condition::any(), |acc, cond| acc.add(cond))
}

fn build_active_state_models(states: Vec<UpsertJobStateDto>) -> Vec<job_state::ActiveModel> {
    states
        .into_iter()
        .map(|s| job_state::ActiveModel {
            job_id: Set(s.job_id),
            key: Set(s.key),
            value: Set(s.value),
        })
        .collect()
}
