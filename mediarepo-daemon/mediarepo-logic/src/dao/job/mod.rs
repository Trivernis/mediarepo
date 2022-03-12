use crate::dao_provider;

pub mod generate_missing_thumbnails;
pub mod migrate_content_descriptors;
pub mod sqlite_operations;
pub mod state;

dao_provider!(JobDao);
