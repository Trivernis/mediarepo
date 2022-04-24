use sea_orm::DbBackend;
use sea_orm::FromQueryResult;
use sea_orm::{DatabaseConnection, Statement};

use mediarepo_core::error::{RepoError, RepoResult};

#[derive(Debug, FromQueryResult)]
pub struct Counts {
    pub file_count: i64,
    pub cd_count: i64,
    pub tag_count: i64,
    pub namespace_count: i64,
    pub source_count: i64,
    pub mapping_count: i64,
}

pub async fn get_all_counts(db: &DatabaseConnection) -> RepoResult<Counts> {
    let counts = Counts::find_by_statement(Statement::from_sql_and_values(
        DbBackend::Sqlite,
        r#"
    SELECT *
    FROM (SELECT COUNT(*) AS file_count FROM files),
         (SELECT COUNT(*) AS cd_count FROM content_descriptors),
         (SELECT COUNT(*) AS tag_count FROM tags),
         (SELECT COUNT(*) AS namespace_count FROM namespaces),
         (SELECT COUNT(*) AS source_count FROM sources),
         (SELECT COUNT(*) AS mapping_count FROM cd_tag_mappings)
    "#,
        vec![],
    ))
    .one(db)
    .await?
    .ok_or_else(|| RepoError::from("could not retrieve metadata from database"))?;

    Ok(counts)
}
