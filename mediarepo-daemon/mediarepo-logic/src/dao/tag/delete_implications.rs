use mediarepo_core::error::RepoResult;
use mediarepo_database::entities::tag_implication;
use sea_orm::{prelude::*, Condition};

use crate::dto::TagImplicationDto;

use super::TagDao;

impl TagDao {
    pub async fn delete_implications(
        &self,
        implications: Vec<TagImplicationDto>,
    ) -> RepoResult<()> {
        let filter_condition = implications
            .into_iter()
            .map(|i| {
                Condition::all()
                    .add(tag_implication::Column::TagId.eq(i.tag_id()))
                    .add(tag_implication::Column::ImpliedTagId.eq(i.implied_tag_id()))
            })
            .fold(Condition::any(), |acc, val| acc.add(val));
        tag_implication::Entity::delete_many()
            .filter(filter_condition)
            .exec(&self.ctx.db)
            .await?;
        Ok(())
    }
}
