use crate::dao::tag::TagDao;
use mediarepo_core::error::RepoResult;
use mediarepo_database::entities::{content_descriptor_tag, namespace, tag};
use sea_orm::prelude::*;
use sea_orm::JoinType;
use sea_orm::{FromQueryResult, QuerySelect};
use std::collections::HashMap;
use std::iter::FromIterator;

#[derive(Debug, FromQueryResult)]
struct CDIDNamespaceTag {
    cd_id: i64,
    namespace: String,
    tag: String,
}

impl TagDao {
    #[tracing::instrument(level = "debug", skip(self, cdids))]
    pub async fn cdids_with_namespaced_tags(
        &self,
        cdids: Vec<i64>,
    ) -> RepoResult<HashMap<i64, HashMap<String, Vec<String>>>> {
        let cd_namespace_tags: Vec<CDIDNamespaceTag> = content_descriptor_tag::Entity::find()
            .select_only()
            .column(content_descriptor_tag::Column::CdId)
            .column_as(tag::Column::Name, "tag")
            .column_as(namespace::Column::Name, "namespace")
            .join(
                JoinType::InnerJoin,
                content_descriptor_tag::Relation::Tag.def(),
            )
            .join(JoinType::Join, namespace::Relation::Tag.def().rev())
            .filter(content_descriptor_tag::Column::CdId.is_in(cdids))
            .into_model::<CDIDNamespaceTag>()
            .all(&self.ctx.db)
            .await?;

        let mut cd_id_namespaces: HashMap<i64, HashMap<String, Vec<String>>> = HashMap::new();
        for cnt in cd_namespace_tags {
            if let Some(entry) = cd_id_namespaces.get_mut(&cnt.cd_id) {
                if let Some(nsp_entry) = entry.get_mut(&cnt.namespace) {
                    nsp_entry.push(cnt.tag);
                } else {
                    entry.insert(cnt.namespace, vec![cnt.tag]);
                }
            } else {
                cd_id_namespaces.insert(
                    cnt.cd_id,
                    HashMap::from_iter(vec![(cnt.namespace, vec![cnt.tag])].into_iter()),
                );
            }
        }

        Ok(cd_id_namespaces)
    }
}
