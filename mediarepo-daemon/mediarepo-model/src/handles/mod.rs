pub mod tag_handle;

use async_trait::async_trait;
use mediarepo_core::error::RepoResult;
use sea_orm::DatabaseConnection;

#[async_trait]
pub trait EntityHandle {
    type Model;

    /// Returns the ID that is stored in the handle
    fn id(&self) -> i64;

    /// Returns the model associated with the handle
    async fn model(&self, db: DatabaseConnection) -> RepoResult<Self::Model>;
}
