use mediarepo_core::error::RepoResult;
use sea_orm::DbBackend;
use sea_orm::FromQueryResult;
use sea_orm::{DatabaseConnection, Statement};
use std::collections::HashMap;
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
) -> RepoResult<HashMap<i64, HashMap<String, String>>> {
    let hash_namespace_tags: Vec<HashNamespaceTags> =
        HashNamespaceTags::find_by_statement(Statement::from_sql_and_values(
            DbBackend::Sqlite,
            format!(
                r#"SELECT htm.hash_id, n.name as namespace, t.name as tag
            FROM hash_tag_mappings htm
                     INNER JOIN tags t on htm.tag_id = t.id
                     JOIN namespaces n on t.namespace_id = n.id
            WHERE t.namespace_id IS NOT NULL
              AND htm.hash_id IN ({});"#,
                hash_ids
                    .into_iter()
                    .fold(String::new(), |acc, val| format!("{}{},", acc, val))
                    .trim_end_matches(",")
            )
            .as_str(),
            vec![],
        ))
        .all(&db)
        .await?;
    let mut hash_namespaces: HashMap<i64, HashMap<String, String>> = HashMap::new();
    for hnt in hash_namespace_tags {
        if let Some(entry) = hash_namespaces.get_mut(&hnt.hash_id) {
            entry.insert(hnt.namespace, hnt.tag);
        } else {
            hash_namespaces.insert(
                hnt.hash_id,
                HashMap::from_iter(vec![(hnt.namespace, hnt.tag)].into_iter()),
            );
        }
    }

    Ok(hash_namespaces)
}
