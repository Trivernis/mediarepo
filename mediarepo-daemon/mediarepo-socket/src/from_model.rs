use mediarepo_core::mediarepo_api::types::files::{
    FileBasicDataResponse, FileMetadataResponse, FileStatus, ThumbnailMetadataResponse,
};
use mediarepo_core::mediarepo_api::types::tags::{NamespaceResponse, TagResponse};
use mediarepo_logic::dto::{
    FileDto, FileMetadataDto, FileStatus as FileStatusModel, TagDto, ThumbnailDto,
};
use mediarepo_logic::file::File;
use mediarepo_logic::file_metadata::FileMetadata;
use mediarepo_logic::namespace::Namespace;
use mediarepo_logic::tag::Tag;
use mediarepo_logic::thumbnail::Thumbnail;

pub trait FromModel<M> {
    fn from_model(model: M) -> Self;
}

impl FromModel<FileMetadata> for FileMetadataResponse {
    fn from_model(metadata: FileMetadata) -> Self {
        Self {
            file_id: metadata.file_id(),
            name: metadata.name().to_owned(),
            comment: metadata.comment().to_owned(),
            creation_time: metadata.creation_time().to_owned(),
            change_time: metadata.change_time().to_owned(),
            import_time: metadata.import_time().to_owned(),
        }
    }
}

impl FromModel<FileMetadataDto> for FileMetadataResponse {
    fn from_model(model: FileMetadataDto) -> Self {
        Self {
            file_id: model.file_id(),
            name: model.name().cloned(),
            comment: model.comment().cloned(),
            creation_time: model.creation_time().to_owned(),
            change_time: model.change_time().to_owned(),
            import_time: model.import_time().to_owned(),
        }
    }
}

impl FromModel<File> for FileBasicDataResponse {
    fn from_model(file: File) -> Self {
        FileBasicDataResponse {
            id: file.id(),
            status: FileStatus::from_model(file.status()),
            cd: file.encoded_cd(),
            mime_type: file.mime_type().to_owned(),
        }
    }
}

impl FromModel<FileDto> for FileBasicDataResponse {
    fn from_model(model: FileDto) -> Self {
        FileBasicDataResponse {
            id: model.id(),
            status: FileStatus::from_model(model.status()),
            cd: model.encoded_cd(),
            mime_type: model.mime_type().to_owned(),
        }
    }
}

impl FromModel<FileStatusModel> for FileStatus {
    fn from_model(status: FileStatusModel) -> Self {
        match status {
            FileStatusModel::Imported => FileStatus::Imported,
            FileStatusModel::Archived => FileStatus::Archived,
            FileStatusModel::Deleted => FileStatus::Deleted,
        }
    }
}

impl FromModel<Tag> for TagResponse {
    fn from_model(model: Tag) -> Self {
        Self {
            id: model.id(),
            namespace: model.namespace().map(|n| n.name().to_owned()),
            name: model.name().to_owned(),
        }
    }
}

impl FromModel<TagDto> for TagResponse {
    fn from_model(model: TagDto) -> Self {
        Self {
            id: model.id(),
            namespace: model.namespace().map(|n| n.name().to_owned()),
            name: model.name().to_owned(),
        }
    }
}

impl FromModel<Thumbnail> for ThumbnailMetadataResponse {
    fn from_model(model: Thumbnail) -> Self {
        Self {
            file_hash: model.file_hash,
            height: model.size.height,
            width: model.size.width,
            mime_type: model.mime_type.to_owned(),
        }
    }
}

impl FromModel<ThumbnailDto> for ThumbnailMetadataResponse {
    fn from_model(model: ThumbnailDto) -> Self {
        Self {
            file_hash: model.parent_cd().to_owned(),
            height: model.size().height,
            width: model.size().width,
            mime_type: model.mime_type().to_owned(),
        }
    }
}

impl FromModel<Namespace> for NamespaceResponse {
    fn from_model(model: Namespace) -> Self {
        Self {
            id: model.id(),
            name: model.name().to_owned(),
        }
    }
}
