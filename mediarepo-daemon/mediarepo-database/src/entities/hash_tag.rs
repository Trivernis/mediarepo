use sea_orm::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "hash_tag_mappings")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub hash_id: u64,
    #[sea_orm(primary_key)]
    pub tag_id: u64,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(belongs_to = "super::hash::Entity", from = "Column::HashId", to = "super::hash::Column::Id")]
    Hash,
    #[sea_orm(belongs_to = "super::tag::Entity", from = "Column::TagId", to = "super::tag::Column::Id")]
    Tag,
}

impl Related<super::hash::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Hash.def()
    }
}

impl Related<super::tag::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Tag.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}