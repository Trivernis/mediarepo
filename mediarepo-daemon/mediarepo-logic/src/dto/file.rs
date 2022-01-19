use mediarepo_database::entities::file;
use mediarepo_database::entities::file_metadata;
use mediarepo_database::entities::content_descriptor;
use crate::dto::FileMetadataDto;

#[derive(Clone, Debug)]
pub struct FileDto {
    model: file::Model,
    content_descriptor: content_descriptor::Model,
    metadata: Option<FileMetadataDto>,
}

impl FileDto {
    pub(crate) fn new(model: file::Model, content_descriptor: content_descriptor::Model, metadata: Option<file_metadata::Model>) -> Self {
        Self {
            model,
            content_descriptor,
            metadata: metadata.map(FileMetadataDto::new)
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

    pub fn metadata(&self) -> Option<&FileMetadataDto> {
        self.metadata.as_ref()
    }
}