use chrono::NaiveDateTime;
use sea_orm::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "namespaces")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub job_type: JobType,
    pub name: Option<String>,
    pub next_run: Option<NaiveDateTime>,
    pub interval: Option<i64>,
}

#[derive(Clone, Debug, PartialEq, EnumIter, DeriveActiveEnum)]
#[sea_orm(rs_type = "u32", db_type = "Integer")]
pub enum JobType {
    #[sea_orm(num_value = 10)]
    MigrateCDs,
    #[sea_orm(num_value = 20)]
    CalculateSizes,
    #[sea_orm(num_value = 30)]
    GenerateThumbs,
    #[sea_orm(num_value = 40)]
    CheckIntegrity,
    #[sea_orm(num_value = 50)]
    Vacuum,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::job_state::Entity")]
    JobState,
}

impl Related<super::job_state::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::JobState.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
