use sea_orm::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "content_descriptors")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub descriptor: Vec<u8>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl Related<super::file::Entity> for Entity {
    fn to() -> RelationDef {
        super::file::Relation::ContentDescriptorId.def().rev()
    }
}

impl Related<super::tag::Entity> for Entity {
    fn to() -> RelationDef {
        super::content_descriptor_tag::Relation::Tag.def()
    }

    fn via() -> Option<RelationDef> {
        Some(
            super::content_descriptor_tag::Relation::ContentDescriptorId
                .def()
                .rev(),
        )
    }
}

impl Related<super::source::Entity> for Entity {
    fn to() -> RelationDef {
        super::content_descriptor_source::Relation::Source.def()
    }

    fn via() -> Option<RelationDef> {
        Some(
            super::content_descriptor_source::Relation::ContentDescriptorId
                .def()
                .rev(),
        )
    }
}

impl ActiveModelBehavior for ActiveModel {}
