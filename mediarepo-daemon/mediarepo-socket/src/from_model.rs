use mediarepo_api::types::files::{FileMetadataResponse, ThumbnailMetadataResponse};
use mediarepo_api::types::tags::TagResponse;
use mediarepo_model::file::File;
use mediarepo_model::tag::Tag;
use mediarepo_model::thumbnail::Thumbnail;

pub trait FromModel<M> {
    fn from_model(model: M) -> Self;
}

impl FromModel<File> for FileMetadataResponse {
    fn from_model(file: File) -> Self {
        Self {
            id: file.id(),
            name: file.name().to_owned(),
            comment: file.comment().to_owned(),
            hash: file.hash().to_owned(),
            file_type: file.file_type() as u32,
            mime_type: file.mime_type().to_owned(),
            creation_time: file.creation_time().to_owned(),
            change_time: file.change_time().to_owned(),
            import_time: file.import_time().to_owned(),
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
