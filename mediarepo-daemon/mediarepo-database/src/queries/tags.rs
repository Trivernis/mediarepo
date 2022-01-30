use std::collections::HashMap;
use std::fmt::Display;
use std::iter::FromIterator;

use sea_orm::{DatabaseConnection, Statement};
use sea_orm::DbBackend;
use sea_orm::FromQueryResult;

use mediarepo_core::error::RepoResult;

#[derive(Debug, FromQueryResult)]
struct CIDNamespaceTag {
    cd_id: i64,
    namespace: String,
    tag: String,
}

#[tracing::instrument(level = "debug", skip_all)]
pub async fn get_cids_with_namespaced_tags(
    db: &DatabaseConnection,
    hash_ids: Vec<i64>,
) -> RepoResult<HashMap<i64, HashMap<String, Vec<String>>>> {
    let hash_namespace_tags: Vec<CIDNamespaceTag> =
        CIDNamespaceTag::find_by_statement(Statement::from_sql_and_values(
            DbBackend::Sqlite,
            format!(
                r#"SELECT ctm.cd_id, n.name as namespace, t.name as tag
            FROM cd_tag_mappings ctm
                     INNER JOIN tags t on ctm.tag_id = t.id
                     JOIN namespaces n on t.namespace_id = n.id
            WHERE t.namespace_id IS NOT NULL
              AND ctm.cd_id IN ({}) ORDER BY t.namespace_id;"#,
                vec_to_query_list(hash_ids)
            )
            .as_str(),
            vec![],
        ))
        .all(db)
        .await?;
    let mut cd_id_namespaces: HashMap<i64, HashMap<String, Vec<String>>> = HashMap::new();
    for hnt in hash_namespace_tags {
        if let Some(entry) = cd_id_namespaces.get_mut(&hnt.cd_id) {
            if let Some(nsp_entry) = entry.get_mut(&hnt.namespace) {
                nsp_entry.push(hnt.tag);
            } else {
                entry.insert(hnt.namespace, vec![hnt.tag]);
            }
        } else {
            cd_id_namespaces.insert(
                hnt.cd_id,
                HashMap::from_iter(vec![(hnt.namespace, vec![hnt.tag])].into_iter()),
            );
        }
    }

    Ok(cd_id_namespaces)
}

#[derive(Debug, FromQueryResult)]
struct CIDTagCount {
    cd_id: i64,
    tag_count: i32,
}

#[tracing::instrument(level = "debug", skip_all)]
pub async fn get_content_descriptors_with_tag_count(
    db: &DatabaseConnection,
    cd_ids: Vec<i64>,
) -> RepoResult<HashMap<i64, u32>> {
    if cd_ids.is_empty() {
        return Ok(HashMap::new());
    }
    let hash_tag_counts: Vec<CIDTagCount> =
        CIDTagCount::find_by_statement(Statement::from_sql_and_values(
            DbBackend::Sqlite,
            format!(
                r#"
        SELECT ctm.cd_id, COUNT(ctm.tag_id) AS "tag_count" from cd_tag_mappings ctm
        WHERE ctm.cd_id IN ({})
        GROUP BY cd_id
    "#,
                vec_to_query_list(cd_ids)
            )
            .as_str(),
            vec![],
        ))
        .all(db)
        .await?;

    let mappings = hash_tag_counts
        .into_iter()
        .map(
            |CIDTagCount {
                 cd_id: hash_id,
                 tag_count,
             }| (hash_id, tag_count as u32),
        )
        .collect::<HashMap<i64, u32>>();

    Ok(mappings)
}

fn vec_to_query_list<D: Display>(input: Vec<D>) -> String {
    let mut entries = input
        .into_iter()
        .fold(String::new(), |acc, val| format!("{}{},", acc, val));
    if entries.len() > 0 {
        entries.remove(entries.len() - 1);
    }

    entries
}
