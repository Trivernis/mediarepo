use sea_orm::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "files")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub status: i32,
    pub mime_type: String,
    pub cd_id: i64,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::content_descriptor::Entity",
        from = "Column::CdId",
        to = "super::content_descriptor::Column::Id"
    )]
    ContentDescriptorId,
}

impl Related<super::content_descriptor::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ContentDescriptorId.def()
    }
}

impl Related<super::file_metadata::Entity> for Entity {
    fn to() -> RelationDef {
        super::file_metadata::Relation::File.def().rev()
    }
}

impl ActiveModelBehavior for ActiveModel {}
