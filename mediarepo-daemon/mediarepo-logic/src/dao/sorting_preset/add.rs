use crate::dao::sorting_preset::SortingPresetDao;
use crate::dto::{AddSortKeyDto, AddSortingPresetDto, SortKeyDto, SortingPresetDto};
use mediarepo_core::error::RepoResult;
use mediarepo_database::entities::{sort_key, sorting_preset, sorting_preset_key};
use sea_orm::prelude::*;
use sea_orm::ActiveValue::Set;
use sea_orm::{
    Condition, DatabaseTransaction, DbBackend, FromQueryResult, JoinType, QuerySelect, Statement,
    TransactionTrait,
};

#[allow(unused_imports)]
use sea_orm::TryGetableMany; // otherwise intellijrust hates on me

impl SortingPresetDao {
    #[tracing::instrument(level = "debug", skip(self))]
    pub async fn add(&self, preset: AddSortingPresetDto) -> RepoResult<SortingPresetDto> {
        let trx = self.ctx.db.begin().await?;
        let keys = add_keys(&trx, preset.keys).await?;
        let key_ids = keys
            .iter()
            .enumerate()
            .map(|(idx, key)| (idx, key.id()))
            .collect::<Vec<(usize, i32)>>();
        let condition = key_ids
            .iter()
            .cloned()
            .map(create_mapping_condition)
            .fold(Condition::any(), |acc, cond| acc.add(cond));
        let existing_preset: Option<sorting_preset::Model> = sorting_preset::Entity::find()
            .join(
                JoinType::InnerJoin,
                sorting_preset_key::Relation::SortingPreset.def().rev(),
            )
            .filter(condition)
            .one(&trx)
            .await?;
        if let Some(model) = existing_preset {
            trx.commit().await?;
            return Ok(SortingPresetDto::new(model, keys));
        }

        // sea_orm currently doesn't support all-default-value inserts.
        // TODOD: Replace after the change for default inserts has been merged
        let preset_model = sorting_preset::Model::find_by_statement(Statement::from_string(
            DbBackend::Sqlite,
            "INSERT INTO sorting_presets DEFAULT VALUES RETURNING *;".to_string(),
        ))
        .one(&trx)
        .await?
        .expect("failed to insert new sorting preset");

        let mapping_models = key_ids
            .into_iter()
            .map(|(idx, key)| sorting_preset_key::ActiveModel {
                preset_id: Set(preset_model.id),
                key_id: Set(key),
                key_index: Set(idx as i32),
            })
            .collect::<Vec<sorting_preset_key::ActiveModel>>();

        if !mapping_models.is_empty() {
            sorting_preset_key::Entity::insert_many(mapping_models)
                .exec(&trx)
                .await?;
        }
        trx.commit().await?;

        Ok(SortingPresetDto::new(preset_model, keys))
    }
}

async fn add_keys(
    trx: &DatabaseTransaction,
    keys: Vec<AddSortKeyDto>,
) -> RepoResult<Vec<SortKeyDto>> {
    let mut key_dtos = find_sort_keys(trx, &keys).await?;
    let mut insert_keys = keys.clone();

    key_dtos.iter().for_each(|key| {
        insert_keys.retain(|k| {
            k.ascending != key.ascending()
                || k.key_type != key.key_type().unwrap()
                || !compare_opts_eq(key.value(), k.value.as_ref())
        })
    });

    if !insert_keys.is_empty() {
        let active_models: Vec<sort_key::ActiveModel> = insert_keys
            .iter()
            .cloned()
            .map(|key| sort_key::ActiveModel {
                key_type: Set(key.key_type.to_number()),
                ascending: Set(key.ascending),
                value: Set(key.value),
                ..Default::default()
            })
            .collect();
        sort_key::Entity::insert_many(active_models)
            .exec(trx)
            .await?;
        let mut new_keys = find_sort_keys(trx, &insert_keys).await?;
        key_dtos.append(&mut new_keys);
    }

    let keys_original_order = keys
        .into_iter()
        .filter_map(|k| {
            key_dtos
                .iter()
                .find(|key| {
                    k.ascending == key.ascending()
                        && k.key_type == key.key_type().unwrap()
                        && compare_opts_eq(key.value(), k.value.as_ref())
                })
                .cloned()
        })
        .collect::<Vec<SortKeyDto>>();

    Ok(keys_original_order)
}

async fn find_sort_keys(
    trx: &DatabaseTransaction,
    keys: &[AddSortKeyDto],
) -> RepoResult<Vec<SortKeyDto>> {
    if keys.is_empty() {
        return Ok(vec![]);
    }
    let condition = keys
        .iter()
        .cloned()
        .map(create_sort_key_condition)
        .fold(Condition::any(), |acc, cond| acc.add(cond));
    let keys = sort_key::Entity::find()
        .filter(condition)
        .all(trx)
        .await?
        .into_iter()
        .map(SortKeyDto::new)
        .collect();

    Ok(keys)
}

fn create_sort_key_condition(key: AddSortKeyDto) -> Condition {
    let mut condition = Condition::all()
        .add(sort_key::Column::KeyType.eq(key.key_type.to_number()))
        .add(sort_key::Column::Ascending.eq(key.ascending));

    if let Some(value) = key.value {
        condition = condition.add(sort_key::Column::Value.eq(value))
    } else {
        condition = condition.add(sort_key::Column::Value.is_null())
    }

    condition
}

fn create_mapping_condition(entry: (usize, i32)) -> Condition {
    Condition::all()
        .add(sorting_preset_key::Column::KeyId.eq(entry.1))
        .add(sorting_preset_key::Column::KeyIndex.eq(entry.0 as i32))
}

fn compare_opts_eq<T: Eq>(opt1: Option<T>, opt2: Option<T>) -> bool {
    if let (Some(opt1), Some(opt2)) = (&opt1, &opt2) {
        opt1 == opt2
    } else {
        opt1.is_none() && opt2.is_none()
    }
}
