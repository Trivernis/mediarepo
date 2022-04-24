use std::collections::HashMap;
use std::fmt::Display;

use sea_orm::DbBackend;
use sea_orm::FromQueryResult;
use sea_orm::{DatabaseConnection, Statement};

use mediarepo_core::error::RepoResult;

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
    if !entries.is_empty() {
        entries.remove(entries.len() - 1);
    }

    entries
}
