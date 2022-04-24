use chrono::NaiveDateTime;

use mediarepo_core::content_descriptor::encode_content_descriptor;
use mediarepo_core::mediarepo_api::types::files::FileStatus as ApiFileStatus;
use mediarepo_database::entities::content_descriptor;
use mediarepo_database::entities::file;
use mediarepo_database::entities::file_metadata;

use crate::dto::FileMetadataDto;

#[derive(Clone, Debug)]
pub struct FileDto {
    model: file::Model,
    content_descriptor: content_descriptor::Model,
    metadata: Option<FileMetadataDto>,
}

impl FileDto {
    pub(crate) fn new(
        model: file::Model,
        content_descriptor: content_descriptor::Model,
        metadata: Option<file_metadata::Model>,
    ) -> Self {
        Self {
            model,
            content_descriptor,
            metadata: metadata.map(FileMetadataDto::new),
        }
    }

    pub fn id(&self) -> i64 {
        self.model.id
    }

    pub fn cd_id(&self) -> i64 {
        self.model.cd_id
    }

    pub fn cd(&self) -> &[u8] {
        &self.content_descriptor.descriptor
    }

    pub fn encoded_cd(&self) -> String {
        encode_content_descriptor(&self.content_descriptor.descriptor)
    }

    pub fn status(&self) -> FileStatus {
        match self.model.status {
            10 => FileStatus::Imported,
            20 => FileStatus::Archived,
            30 => FileStatus::Deleted,
            _ => FileStatus::Imported,
        }
    }

    pub fn mime_type(&self) -> &String {
        &self.model.mime_type
    }

    pub fn metadata(&self) -> Option<&FileMetadataDto> {
        self.metadata.as_ref()
    }

    pub fn into_metadata(self) -> Option<FileMetadataDto> {
        self.metadata
    }
}

#[derive(Clone, Debug)]
pub struct AddFileDto {
    pub content: Vec<u8>,
    pub mime_type: String,
    pub creation_time: NaiveDateTime,
    pub change_time: NaiveDateTime,
    pub name: Option<String>,
}

#[derive(Clone, Debug, Default)]
pub struct UpdateFileDto {
    pub id: i64,
    pub cd_id: Option<i64>,
    pub mime_type: Option<String>,
    pub status: Option<FileStatus>,
}

#[derive(Copy, Clone, Debug)]
pub enum FileStatus {
    Imported = 10,
    Archived = 20,
    Deleted = 30,
}

impl From<ApiFileStatus> for FileStatus {
    fn from(s: ApiFileStatus) -> Self {
        match s {
            ApiFileStatus::Imported => Self::Imported,
            ApiFileStatus::Archived => Self::Archived,
            ApiFileStatus::Deleted => Self::Deleted,
        }
    }
}
