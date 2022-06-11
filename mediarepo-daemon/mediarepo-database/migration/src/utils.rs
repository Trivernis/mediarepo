#[macro_export]
macro_rules! drop_tables {
    ($man:expr, $($tbl:expr),*) => {
        use sea_orm_migration::prelude::*;
        $(
            $man.drop_table(TableDropStatement::new().table($tbl).to_owned()).await?;
        )*
    }
}