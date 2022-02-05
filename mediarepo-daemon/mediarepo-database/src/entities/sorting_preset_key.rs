use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "sorting_preset_keys")]
pub struct Model {
    #[sea_orm(primary_key)]
    preset_id: i32,

    #[sea_orm(primary_key)]
    key_id: i32,

    #[sea_orm(primary_key)]
    key_index: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::sorting_preset::Entity"
        from = "Column::PresetId",
        to = "super::sorting_preset::Column::Id"
    )]
    SortingPreset,

    #[sea_orm(
        belongs_to = "super::sort_key::Entity",
        from = "Column::KeyId",
        to = "super::sort_key::Column::Id"
    )]
    SortingKey,
}

impl Related<super::sorting_preset::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::SortingPreset.def()
    }
}

impl Related<super::sort_key::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::SortingKey.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
