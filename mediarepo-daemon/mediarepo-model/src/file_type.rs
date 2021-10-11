use mime::Mime;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, PartialOrd, PartialEq)]
pub enum FileType {
    Other = -1,
    Unknown = 0,
    Image = 1,
    Video = 2,
    Audio = 3,
}

impl From<Mime> for FileType {
    fn from(mime_type: Mime) -> Self {
        match mime_type.type_() {
            mime::IMAGE => Self::Image,
            mime::VIDEO => Self::Video,
            mime::AUDIO => Self::Audio,
            _ => Self::Other,
        }
    }
}
