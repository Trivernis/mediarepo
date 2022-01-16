use sea_orm::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "cd_tag_mappings")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub cd_id: i64,
    #[sea_orm(primary_key)]
    pub tag_id: i64,
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
        belongs_to = "super::tag::Entity",
        from = "Column::TagId",
        to = "super::tag::Column::Id"
    )]
    Tag,
}

impl Related<super::content_descriptor::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ContentDescriptorId.def()
    }
}

impl Related<super::tag::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Tag.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
