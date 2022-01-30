use crate::dao::job::JobDao;
use mediarepo_core::error::RepoError::Corrupted;
use mediarepo_core::error::RepoResult;
use sea_orm::DatabaseBackend::Sqlite;
use sea_orm::{ConnectionTrait, FromQueryResult, Statement};

#[derive(Debug, FromQueryResult)]
struct IntegrityCheckResult {
    integrity_check: String,
}

impl JobDao {
    #[tracing::instrument(level = "debug", skip(self))]
    pub async fn check_integrity(&self) -> RepoResult<()> {
        let check_result: Option<IntegrityCheckResult> = IntegrityCheckResult::find_by_statement(
            Statement::from_string(Sqlite, String::from("PRAGMA integrity_check;")),
        )
        .one(&self.ctx.db)
        .await?;
        tracing::debug!("check result = {:?}", check_result);

        check_result
            .ok_or_else(|| Corrupted(String::from("no check result")))
            .and_then(map_check_result)
    }

    #[tracing::instrument(level = "debug", skip(self))]
    pub async fn vacuum(&self) -> RepoResult<()> {
        self.ctx
            .db
            .execute(Statement::from_string(Sqlite, String::from("VACUUM;")))
            .await?;

        Ok(())
    }
}

fn map_check_result(result: IntegrityCheckResult) -> RepoResult<()> {
    if result.integrity_check == "ok" {
        Ok(())
    } else {
        Err(Corrupted(result.integrity_check))
    }
}
