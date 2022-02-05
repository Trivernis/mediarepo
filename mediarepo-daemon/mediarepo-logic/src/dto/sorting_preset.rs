use mediarepo_database::entities::sort_key;
use mediarepo_database::entities::sorting_preset;

#[derive(Clone, Debug)]
pub struct SortingPresetDto {
    model: sorting_preset::Model,
    keys: Vec<SortKeyDto>,
}

impl SortingPresetDto {
    pub fn new(model: sorting_preset::Model, keys: Vec<SortKeyDto>) -> Self {
        Self { model, keys }
    }

    pub fn id(&self) -> i32 {
        self.model.id
    }

    pub fn keys(&self) -> &Vec<SortKeyDto> {
        &self.keys
    }
}

#[derive(Clone, Debug)]
pub struct SortKeyDto {
    model: sort_key::Model,
}

impl SortKeyDto {
    pub fn new(model: sort_key::Model) -> Self {
        Self { model }
    }

    pub fn id(&self) -> i32 {
        self.model.id
    }

    pub fn key_type(&self) -> i32 {
        self.model.key_type
    }

    pub fn ascending(&self) -> bool {
        self.model.ascending
    }

    pub fn value(&self) -> Option<&String> {
        self.model.value.as_ref()
    }
}
