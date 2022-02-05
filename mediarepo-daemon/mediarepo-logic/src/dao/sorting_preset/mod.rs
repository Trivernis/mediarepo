pub mod add;

use crate::dao_provider;
use crate::dto::{SortKeyDto, SortingPresetDto};
use mediarepo_core::error::RepoResult;
use mediarepo_database::entities::{sort_key, sorting_preset, sorting_preset_key};
use sea_orm::prelude::*;
use sea_orm::{JoinType, QueryOrder, QuerySelect};

dao_provider!(SortingPresetDao);

impl SortingPresetDao {
    #[tracing::instrument(level = "debug", skip(self))]
    pub async fn all(&self) -> RepoResult<Vec<SortingPresetDto>> {
        let presets = sorting_preset::Entity::find()
            .find_with_related(sort_key::Entity)
            .join(
                JoinType::InnerJoin,
                sorting_preset_key::Relation::SortingKey.def(),
            )
            .order_by_asc(sorting_preset_key::Column::KeyIndex)
            .all(&self.ctx.db)
            .await?
            .into_iter()
            .map(map_sorting_preset_dto)
            .collect();

        Ok(presets)
    }

    #[tracing::instrument(level = "debug", skip(self))]
    pub async fn delete(&self, id: i32) -> RepoResult<()> {
        sorting_preset::Entity::delete_many()
            .filter(sorting_preset::Column::Id.eq(id))
            .exec(&self.ctx.db)
            .await?;

        Ok(())
    }
}

fn map_sorting_preset_dto(
    entry: (sorting_preset::Model, Vec<sort_key::Model>),
) -> SortingPresetDto {
    SortingPresetDto::new(entry.0, entry.1.into_iter().map(SortKeyDto::new).collect())
}
