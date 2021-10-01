use sea_orm::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "hash_source_mappings")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub hash_id: u64,
    #[sea_orm(primary_key)]
    pub source_id: u64,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(belongs_to = "super::hash::Entity", from = "Column::HashId", to = "super::hash::Column::Id")]
    Hash,
    #[sea_orm(belongs_to = "super::source::Entity", from = "Column::SourceId", to = "super::source::Column::Id")]
    Source,
}

impl Related<super::hash::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Hash.def()
    }
}

impl Related<super::source::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Source.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}