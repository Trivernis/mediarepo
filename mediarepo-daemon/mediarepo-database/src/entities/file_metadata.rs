use chrono::NaiveDateTime;
use sea_orm::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "file_metadata")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub file_id: i64,
    pub name: Option<String>,
    pub comment: Option<String>,
    pub size: i64,
    pub import_time: NaiveDateTime,
    pub creation_time: NaiveDateTime,
    pub change_time: NaiveDateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::file::Entity",
        from = "Column::FileId",
        to = "super::file::Column::Id"
    )]
    File,
}

impl Related<super::file::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::File.def()
    }
}
impl ActiveModelBehavior for ActiveModel {}
