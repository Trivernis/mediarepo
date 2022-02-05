use sea_orm::{ActiveValue, DatabaseConnection};

use mediarepo_core::fs::file_hash_store::FileHashStore;
use mediarepo_core::fs::thumbnail_store::ThumbnailStore;

use crate::dao::file::FileDao;
use crate::dao::job::JobDao;
use crate::dao::sorting_preset::SortingPresetDao;
use crate::dao::tag::TagDao;

pub mod file;
pub mod job;
pub mod repo;
pub mod sorting_preset;
pub mod tag;

#[macro_export]
macro_rules! dao_provider {
    ($name:ident) => {
        use crate::dao::{DaoContext, DaoProvider};

        pub struct $name {
            ctx: DaoContext,
        }

        impl DaoProvider for $name {
            fn dao_ctx(&self) -> DaoContext {
                self.ctx.clone()
            }
        }

        impl $name {
            pub fn new(ctx: DaoContext) -> Self {
                Self { ctx }
            }
        }
    };
}

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

    fn sorting_preset(&self) -> SortingPresetDao {
        SortingPresetDao::new(self.dao_ctx())
    }
}

fn opt_to_active_val<T: Into<sea_orm::Value>>(opt: Option<T>) -> ActiveValue<T> {
    opt.map(|v| ActiveValue::Set(v))
        .unwrap_or(ActiveValue::NotSet)
}
