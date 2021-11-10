use crate::handles::EntityHandle;
use crate::tag::Tag;
use async_trait::async_trait;
use mediarepo_core::error::{RepoDatabaseError, RepoResult};
use sea_orm::DatabaseConnection;

pub struct TagHandle(pub(crate) i64);

#[async_trait]
impl EntityHandle for TagHandle {
    type Model = Tag;

    fn id(&self) -> i64 {
        self.0
    }

    async fn model(&self, db: DatabaseConnection) -> RepoResult<Self::Model> {
        let tag = Tag::by_id(db, self.0)
            .await?
            .ok_or_else(|| RepoDatabaseError::InvalidHandle(self.id()))?;

        Ok(tag)
    }
}
