use crate::dto::KeyType::{
    FileChangeTime, FileCreatedTime, FileImportedTime, FileName, FileSize, FileType, Namespace,
    NumTags,
};
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

    pub fn key_type(&self) -> Option<KeyType> {
        KeyType::from_number(self.model.key_type)
    }

    pub fn ascending(&self) -> bool {
        self.model.ascending
    }

    pub fn value(&self) -> Option<&String> {
        self.model.value.as_ref()
    }
}

#[derive(Clone, Copy, Debug, PartialOrd, PartialEq)]
pub enum KeyType {
    Namespace = 0,
    FileName = 1,
    FileSize = 2,
    FileImportedTime = 3,
    FileCreatedTime = 4,
    FileChangeTime = 5,
    FileType = 6,
    NumTags = 7,
}

impl KeyType {
    pub fn from_number(number: i32) -> Option<KeyType> {
        match number {
            0 => Some(Namespace),
            1 => Some(FileName),
            2 => Some(FileSize),
            3 => Some(FileImportedTime),
            4 => Some(FileCreatedTime),
            5 => Some(FileChangeTime),
            6 => Some(FileType),
            7 => Some(NumTags),
            _ => None,
        }
    }

    pub fn to_number(&self) -> i32 {
        self.clone() as i32
    }
}

#[derive(Clone, Debug)]
pub struct AddSortingPresetDto {
    pub keys: Vec<AddSortKeyDto>,
}

#[derive(Clone, Debug)]
pub struct AddSortKeyDto {
    pub key_type: KeyType,
    pub ascending: bool,
    pub value: Option<String>,
}
