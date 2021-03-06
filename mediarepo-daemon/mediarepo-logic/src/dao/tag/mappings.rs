use sea_orm::prelude::*;
use sea_orm::sea_query::Query;
use sea_orm::ActiveValue::Set;
use sea_orm::{DatabaseTransaction, TransactionTrait};

use mediarepo_core::error::RepoResult;
use mediarepo_database::entities::{content_descriptor_tag, namespace, tag};

use crate::dao::tag::TagDao;

impl TagDao {
    #[tracing::instrument(level = "debug", skip(self))]
    pub async fn upsert_mappings(&self, cd_ids: Vec<i64>, tag_ids: Vec<i64>) -> RepoResult<()> {
        let trx = self.ctx.db.begin().await?;

        let existing_mappings = get_existing_mappings(&trx, &cd_ids, &tag_ids).await?;

        let active_models: Vec<content_descriptor_tag::ActiveModel> = cd_ids
            .into_iter()
            .flat_map(|cd_id: i64| {
                tag_ids
                    .iter()
                    .filter(|tag_id| !existing_mappings.contains(&(cd_id, **tag_id)))
                    .map(move |tag_id| content_descriptor_tag::ActiveModel {
                        cd_id: Set(cd_id),
                        tag_id: Set(*tag_id),
                    })
                    .collect::<Vec<content_descriptor_tag::ActiveModel>>()
            })
            .collect();

        if !active_models.is_empty() {
            content_descriptor_tag::Entity::insert_many(active_models)
                .exec(&trx)
                .await?;

            trx.commit().await?;
        }

        Ok(())
    }

    #[tracing::instrument(level = "debug", skip(self))]
    pub async fn remove_mappings(&self, cd_ids: Vec<i64>, tag_ids: Vec<i64>) -> RepoResult<()> {
        let trx = self.ctx.db.begin().await?;
        content_descriptor_tag::Entity::delete_many()
            .filter(content_descriptor_tag::Column::CdId.is_in(cd_ids))
            .filter(content_descriptor_tag::Column::TagId.is_in(tag_ids))
            .exec(&trx)
            .await?;
        delete_orphans(&trx).await?;

        trx.commit().await?;

        Ok(())
    }
}

async fn get_existing_mappings(
    trx: &DatabaseTransaction,
    cd_ids: &[i64],
    tag_ids: &[i64],
) -> RepoResult<Vec<(i64, i64)>> {
    let existing_mappings: Vec<(i64, i64)> = content_descriptor_tag::Entity::find()
        .filter(content_descriptor_tag::Column::CdId.is_in(cd_ids.to_vec()))
        .filter(content_descriptor_tag::Column::TagId.is_in(tag_ids.to_vec()))
        .all(trx)
        .await?
        .into_iter()
        .map(|model: content_descriptor_tag::Model| (model.cd_id, model.tag_id))
        .collect();
    Ok(existing_mappings)
}

/// Deletes orphaned tag entries and namespaces from the database
async fn delete_orphans(trx: &DatabaseTransaction) -> RepoResult<()> {
    tag::Entity::delete_many()
        .filter(
            tag::Column::Id.not_in_subquery(
                Query::select()
                    .column(content_descriptor_tag::Column::TagId)
                    .from(content_descriptor_tag::Entity)
                    .group_by_col(content_descriptor_tag::Column::TagId)
                    .to_owned(),
            ),
        )
        .exec(trx)
        .await?;

    namespace::Entity::delete_many()
        .filter(
            namespace::Column::Id.not_in_subquery(
                Query::select()
                    .column(tag::Column::NamespaceId)
                    .from(tag::Entity)
                    .group_by_col(tag::Column::NamespaceId)
                    .to_owned(),
            ),
        )
        .exec(trx)
        .await?;

    Ok(())
}
