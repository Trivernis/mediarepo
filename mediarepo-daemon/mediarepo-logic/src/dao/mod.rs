pub mod file;
pub mod job;
pub mod repo;
pub mod tag;

use crate::dao::file::FileDao;
use crate::dao::job::JobDao;
use crate::dao::tag::TagDao;
use mediarepo_core::fs::file_hash_store::FileHashStore;
use mediarepo_core::fs::thumbnail_store::ThumbnailStore;
use sea_orm::{ActiveValue, DatabaseConnection};

#[derive(Clone)]
pub struct DaoContext {
    pub db: DatabaseConnection,
    pub main_storage: FileHashStore,
    pub thumbnail_storage: ThumbnailStore,
}

pub trait DaoProvider {
    fn dao_ctx(&self) -> DaoContext;

    fn file(&self) -> FileDao {
        FileDao::new(self.dao_ctx())
    }

    fn tag(&self) -> TagDao {
        TagDao::new(self.dao_ctx())
    }

    fn job(&self) -> JobDao {
        JobDao::new(self.dao_ctx())
    }
}

fn opt_to_active_val<T: Into<sea_orm::Value>>(opt: Option<T>) -> ActiveValue<T> {
    opt.map(|v| ActiveValue::Set(v))
        .unwrap_or(ActiveValue::NotSet)
}
