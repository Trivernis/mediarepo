use chrono::NaiveDateTime;
use sea_orm::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "files")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub file_type: u32,
    pub name: Option<String>,
    pub comment: Option<String>,
    pub mime_type: Option<String>,
    pub size: Option<i64>,
    pub storage_id: i64,
    pub hash_id: i64,
    pub import_time: NaiveDateTime,
    pub creation_time: NaiveDateTime,
    pub change_time: NaiveDateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::hash::Entity",
        from = "Column::HashId",
        to = "super::hash::Column::Id"
    )]
    Hash,

    #[sea_orm(
        belongs_to = "super::storage::Entity",
        from = "Column::StorageId",
        to = "super::storage::Column::Id"
    )]
    Storage,
}

impl Related<super::hash::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Hash.def()
    }
}

impl Related<super::storage::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Storage.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
