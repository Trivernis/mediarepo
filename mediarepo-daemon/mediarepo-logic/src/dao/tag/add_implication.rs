use crate::dto::AddTagImplicationDto;
use mediarepo_core::error::RepoResult;
use mediarepo_database::entities::tag_implication;

use super::TagDao;
use sea_orm::{prelude::*, Set};

impl TagDao {
    pub async fn add_implications(&self, dtos: Vec<AddTagImplicationDto>) -> RepoResult<()> {
        let active_models = dtos
            .into_iter()
            .map(|dto| tag_implication::ActiveModel {
                tag_id: Set(dto.tag_id),
                implied_tag_id: Set(dto.implied_tag_id),
            })
            .collect::<Vec<tag_implication::ActiveModel>>();
        tag_implication::Entity::insert_many(active_models)
            .exec(&self.ctx.db)
            .await?;

        Ok(())
    }
}
