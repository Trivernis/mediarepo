use sea_orm::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "sort_keys")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub key_type: i32,
    pub ascending: bool,
    pub value: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl Related<super::sorting_preset::Entity> for Entity {
    fn to() -> RelationDef {
        super::sorting_preset_key::Relation::SortingPreset.def()
    }

    fn via() -> Option<RelationDef> {
        Some(super::sorting_preset_key::Relation::SortingKey.def().rev())
    }
}

impl ActiveModelBehavior for ActiveModel {}
