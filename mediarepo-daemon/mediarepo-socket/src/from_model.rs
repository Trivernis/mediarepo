use mediarepo_core::mediarepo_api::types::files::{
    FileBasicDataResponse, FileMetadataResponse, FileStatus, ThumbnailMetadataResponse,
};
use mediarepo_core::mediarepo_api::types::filtering::{
    SortDirection, SortKey, SortNamespace, SortingPreset,
};
use mediarepo_core::mediarepo_api::types::tags::{NamespaceResponse, TagResponse};
use mediarepo_logic::dto::{
    FileDto, FileMetadataDto, FileStatus as FileStatusModel, KeyType, NamespaceDto, SortKeyDto,
    SortingPresetDto, TagDto, ThumbnailDto,
};

pub trait FromModel<M> {
    fn from_model(model: M) -> Self;
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
            size: model.size() as u64,
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

impl FromModel<TagDto> for TagResponse {
    fn from_model(model: TagDto) -> Self {
        Self {
            id: model.id(),
            namespace: model.namespace().map(|n| n.name().to_owned()),
            name: model.name().to_owned(),
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

impl FromModel<NamespaceDto> for NamespaceResponse {
    fn from_model(model: NamespaceDto) -> Self {
        Self {
            id: model.id(),
            name: model.name().to_owned(),
        }
    }
}

impl FromModel<SortingPresetDto> for SortingPreset {
    fn from_model(model: SortingPresetDto) -> Self {
        SortingPreset {
            id: model.id(),
            keys: model
                .into_keys()
                .into_iter()
                .filter_map(map_sort_dto_to_key)
                .collect(),
        }
    }
}

fn map_sort_dto_to_key(dto: SortKeyDto) -> Option<SortKey> {
    let direction = map_direction(dto.ascending());

    match dto.key_type()? {
        KeyType::Namespace => Some(SortKey::Namespace(SortNamespace {
            name: dto.value()?.to_owned(),
            direction,
        })),
        KeyType::FileName => Some(SortKey::FileName(direction)),
        KeyType::FileSize => Some(SortKey::FileSize(direction)),
        KeyType::FileImportedTime => Some(SortKey::FileImportedTime(direction)),
        KeyType::FileCreatedTime => Some(SortKey::FileCreatedTime(direction)),
        KeyType::FileChangeTime => Some(SortKey::FileChangeTime(direction)),
        KeyType::FileType => Some(SortKey::FileType(direction)),
        KeyType::NumTags => Some(SortKey::NumTags(direction)),
    }
}

fn map_direction(ascending: bool) -> SortDirection {
    if ascending {
        SortDirection::Ascending
    } else {
        SortDirection::Descending
    }
}
