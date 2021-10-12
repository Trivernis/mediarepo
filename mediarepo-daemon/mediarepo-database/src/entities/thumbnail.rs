use sea_orm::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "thumbnails")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub file_id: i64,
    pub storage_id: i64,
    pub hash_id: i64,
    pub height: i32,
    pub width: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::file::Entity",
        from = "Column::FileId",
        to = "super::file::Column::Id"
    )]
    File,

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

impl Related<super::file::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::File.def()
    }
}

impl Related<super::storage::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Storage.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
