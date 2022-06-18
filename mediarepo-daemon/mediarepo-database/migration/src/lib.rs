pub use sea_orm_migration::prelude::*;

mod m20220101_000001_create_table;
mod m20220611_122851_tag_implications;
mod utils;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_create_table::Migration),
            Box::new(m20220611_122851_tag_implications::Migration),
        ]
    }
}
