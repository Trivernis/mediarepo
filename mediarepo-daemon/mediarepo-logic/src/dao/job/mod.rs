pub mod migrate_content_descriptors;
pub mod sqlite_operations;

use crate::dao::{DaoContext, DaoProvider};

pub struct JobDao {
    ctx: DaoContext,
}

impl DaoProvider for JobDao {
    fn dao_ctx(&self) -> DaoContext {
        self.ctx.clone()
    }
}

impl JobDao {
    pub fn new(ctx: DaoContext) -> JobDao {
        Self { ctx }
    }
}
