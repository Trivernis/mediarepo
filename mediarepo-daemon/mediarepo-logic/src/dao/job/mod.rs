use crate::dao_provider;

pub mod migrate_content_descriptors;
pub mod sqlite_operations;

dao_provider!(JobDao);
