use chrono::NaiveDateTime;
use mediarepo_model::file::File;
use mediarepo_model::file_type::FileType;
use serde::Serialize;

#[derive(Clone, Debug, Serialize)]
pub struct FileResponse {
    pub name: Option<String>,
    pub comment: Option<String>,
    pub hash: String,
    pub file_type: FileType,
    pub creation_time: NaiveDateTime,
    pub change_time: NaiveDateTime,
    pub import_time: NaiveDateTime,
}

impl From<File> for FileResponse {
    fn from(file: File) -> Self {
        FileResponse {
            hash: file.hash().to_owned(),
            file_type: file.file_type(),
            name: file.name().to_owned(),
            creation_time: file.creation_time().to_owned(),
            change_time: file.change_time().to_owned(),
            import_time: file.import_time().to_owned(),
            comment: file.comment().to_owned(),
        }
    }
}
