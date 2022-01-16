use sea_orm::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "sources")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub url: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl Related<super::content_descriptor::Entity> for Entity {
    fn to() -> RelationDef {
        super::content_descriptor_source::Relation::ContentDescriptorId.def()
    }

    fn via() -> Option<RelationDef> {
        Some(
            super::content_descriptor_source::Relation::Source
                .def()
                .rev(),
        )
    }
}

impl ActiveModelBehavior for ActiveModel {}
