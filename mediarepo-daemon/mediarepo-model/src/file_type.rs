use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Clone, Debug, Serialize, Deserialize, PartialOrd, PartialEq)]
pub enum FileType {
    Other = -1,
    Unknown = 0,
    Image = 1,
    Video = 2,
    Audio = 3,
}

impl From<&PathBuf> for FileType {
    fn from(path: &PathBuf) -> Self {
        let mime = mime_guess::from_path(path).first();
        if let Some(mime) = mime {
            match mime.type_() {
                mime::IMAGE => Self::Image,
                mime::VIDEO => Self::Video,
                mime::AUDIO => Self::Audio,
                _ => Self::Other,
            }
        } else {
            Self::Unknown
        }
    }
}
