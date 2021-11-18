use mediarepo_core::error::RepoResult;
use sea_orm::DbBackend;
use sea_orm::FromQueryResult;
use sea_orm::{DatabaseConnection, Statement};
use std::collections::HashMap;
use std::fmt::Display;
use std::iter::FromIterator;

#[derive(Debug, FromQueryResult)]
struct HashNamespaceTags {
    hash_id: i64,
    namespace: String,
    tag: String,
}

#[tracing::instrument(level = "debug", skip_all)]
pub async fn get_hashes_with_namespaced_tags(
    db: &DatabaseConnection,
    hash_ids: Vec<i64>,
) -> RepoResult<HashMap<i64, HashMap<String, Vec<String>>>> {
    let hash_namespace_tags: Vec<HashNamespaceTags> =
        HashNamespaceTags::find_by_statement(Statement::from_sql_and_values(
            DbBackend::Sqlite,
            format!(
                r#"SELECT htm.hash_id, n.name as namespace, t.name as tag
            FROM hash_tag_mappings htm
                     INNER JOIN tags t on htm.tag_id = t.id
                     JOIN namespaces n on t.namespace_id = n.id
            WHERE t.namespace_id IS NOT NULL
              AND htm.hash_id IN ({}) ORDER BY t.namespace_id;"#,
                vec_to_query_list(hash_ids)
            )
            .as_str(),
            vec![],
        ))
        .all(db)
        .await?;
    let mut hash_namespaces: HashMap<i64, HashMap<String, Vec<String>>> = HashMap::new();
    for hnt in hash_namespace_tags {
        if let Some(entry) = hash_namespaces.get_mut(&hnt.hash_id) {
            if let Some(nsp_entry) = entry.get_mut(&hnt.namespace) {
                nsp_entry.push(hnt.tag);
            } else {
                entry.insert(hnt.namespace, vec![hnt.tag]);
            }
        } else {
            hash_namespaces.insert(
                hnt.hash_id,
                HashMap::from_iter(vec![(hnt.namespace, vec![hnt.tag])].into_iter()),
            );
        }
    }

    Ok(hash_namespaces)
}

#[derive(Debug, FromQueryResult)]
struct HashTagCount {
    hash_id: i64,
    tag_count: i32,
}

#[tracing::instrument(level = "debug", skip_all)]
pub async fn get_hashes_with_tag_count(
    db: &DatabaseConnection,
    hash_ids: Vec<i64>,
) -> RepoResult<HashMap<i64, u32>> {
    if hash_ids.is_empty() {
        return Ok(HashMap::new());
    }
    let hash_tag_counts: Vec<HashTagCount> =
        HashTagCount::find_by_statement(Statement::from_sql_and_values(
            DbBackend::Sqlite,
            format!(
                r#"
        SELECT htm.hash_id, COUNT(htm.tag_id) AS "tag_count" from hash_tag_mappings htm
        WHERE htm.hash_id IN ({})
        GROUP BY hash_id
    "#,
                vec_to_query_list(hash_ids)
            )
            .as_str(),
            vec![],
        ))
        .all(db)
        .await?;

    let mappings = hash_tag_counts
        .into_iter()
        .map(|HashTagCount { hash_id, tag_count }| (hash_id, tag_count as u32))
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
