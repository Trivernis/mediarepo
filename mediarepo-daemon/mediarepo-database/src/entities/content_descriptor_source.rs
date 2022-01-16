use sea_orm::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "cd_source_mappings")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub cd_id: i64,
    #[sea_orm(primary_key)]
    pub source_id: i64,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::content_descriptor::Entity",
        from = "Column::CdId",
        to = "super::content_descriptor::Column::Id"
    )]
    ContentDescriptorId,
    #[sea_orm(
        belongs_to = "super::source::Entity",
        from = "Column::SourceId",
        to = "super::source::Column::Id"
    )]
    Source,
}

impl Related<super::content_descriptor::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ContentDescriptorId.def()
    }
}

impl Related<super::source::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Source.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
