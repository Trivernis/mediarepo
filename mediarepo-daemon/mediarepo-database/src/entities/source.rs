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

impl Related<super::hash::Entity> for Entity {
    fn to() -> RelationDef {
        super::hash_source::Relation::Hash.def()
    }

    fn via() -> Option<RelationDef> {
        Some(super::hash_source::Relation::Source.def().rev())
    }
}

impl ActiveModelBehavior for ActiveModel {}
