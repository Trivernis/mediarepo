use sea_orm::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "tags")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub namespace_id: Option<i64>,
    pub name: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::namespace::Entity",
        from = "Column::NamespaceId",
        to = "super::namespace::Column::Id"
    )]
    Namespace,
}

impl Related<super::content_descriptor::Entity> for Entity {
    fn to() -> RelationDef {
        super::content_descriptor_tag::Relation::ContentDescriptorId.def()
    }

    fn via() -> Option<RelationDef> {
        Some(super::content_descriptor_tag::Relation::Tag.def().rev())
    }
}

impl Related<super::namespace::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Namespace.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
