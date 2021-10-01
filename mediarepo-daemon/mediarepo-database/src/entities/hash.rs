use sea_orm::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "hashes")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: u64,
    pub value: String
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
}

impl Related<super::tag::Entity> for Entity {
    fn to() -> RelationDef {
        super::hash_tag::Relation::Tag.def()
    }

    fn via() -> Option<RelationDef> {
        Some(super::hash_tag::Relation::Hash.def().rev())
    }
}

impl Related<super::source::Entity> for Entity {
    fn to() -> RelationDef {
        super::hash_source::Relation::Source.def()
    }

    fn via() -> Option<RelationDef> {
        Some(super::hash_source::Relation::Hash.def().rev())
    }
}

impl ActiveModelBehavior for ActiveModel {}