use sea_orm::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "sorting_presets")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl Related<super::sort_key::Entity> for Entity {
    fn to() -> RelationDef {
        super::sorting_preset_key::Relation::SortingKey.def()
    }

    fn via() -> Option<RelationDef> {
        Some(
            super::sorting_preset_key::Relation::SortingPreset
                .def()
                .rev(),
        )
    }
}

impl ActiveModelBehavior for ActiveModel {}
