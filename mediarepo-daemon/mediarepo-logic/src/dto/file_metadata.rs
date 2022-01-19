use chrono::NaiveDateTime;
use mediarepo_database::entities::file_metadata;

#[derive(Clone, Debug)]
pub struct FileMetadataDto {
    model: file_metadata::Model,
}

impl FileMetadataDto {
    pub(crate) fn new(model: file_metadata::Model) -> Self {
        Self {model}
    }

    pub fn file_id(&self) -> i64 {
        self.model.file_id
    }

    pub fn name(&self) -> Option<&String> {
        self.model.name.as_ref()
    }

    pub fn comment(&self) -> Option<&String> {
        self.model.comment.as_ref()
    }

    pub fn size(&self) -> i64 {
        self.model.size
    }

    pub fn import_time(&self) -> NaiveDateTime {
        self.model.import_time
    }

    pub fn creation_time(&self) -> NaiveDateTime {
        self.model.creation_time
    }

    pub fn change_time(&self) -> NaiveDateTime {
        self.model.change_time
    }
}