use sea_orm::prelude::*;
use sea_orm::TryFromU64;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "namespaces")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub job_type: JobType,
    pub value: Vec<u8>,
}

#[derive(Clone, Copy, Debug, PartialEq, EnumIter, DeriveActiveEnum)]
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

impl TryFromU64 for JobType {
    fn try_from_u64(n: u64) -> Result<Self, DbErr> {
        let value = match n {
            10 => Self::MigrateCDs,
            20 => Self::CalculateSizes,
            30 => Self::GenerateThumbs,
            40 => Self::CheckIntegrity,
            50 => Self::Vacuum,
            _ => return Err(DbErr::Custom(String::from("Invalid job type"))),
        };

        Ok(value)
    }
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
