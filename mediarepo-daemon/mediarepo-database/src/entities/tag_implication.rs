use sea_orm::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "tag_implications")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub tag_id: i64,
    pub implied_tag_id: i64,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::tag::Entity",
        from = "Column::TagId",
        to = "super::tag::Column::Id"
    )]
    Tag,
    #[sea_orm(
        belongs_to = "super::tag::Entity",
        from = "Column::TagId",
        to = "super::tag::Column::Id"
    )]
    ImpliedTag,
}

pub struct TagToImpliedLink;
pub struct ImpliedToTagLink;

impl Linked for TagToImpliedLink {
    type FromEntity = super::tag::Entity;
    type ToEntity = super::tag::Entity;

    fn link(&self) -> Vec<sea_orm::LinkDef> {
        vec![Relation::Tag.def().rev(), Relation::ImpliedTag.def()]
    }
}

impl Linked for ImpliedToTagLink {
    type FromEntity = super::tag::Entity;
    type ToEntity = super::tag::Entity;

    fn link(&self) -> Vec<sea_orm::LinkDef> {
        vec![Relation::ImpliedTag.def().rev(), Relation::Tag.def()]
    }
}

impl ActiveModelBehavior for ActiveModel {}
