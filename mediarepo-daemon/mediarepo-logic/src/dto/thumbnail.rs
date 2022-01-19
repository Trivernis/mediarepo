use mediarepo_core::fs::thumbnail_store::Dimensions;

#[derive(Clone, Debug)]
pub struct ThumbnailDto {
    parent_cd: String,
    size: Dimensions,
    mime_type: String,
}

impl ThumbnailDto {
    pub fn new(parent_cd: String, size: Dimensions, mime_type: String) -> Self {
        Self {parent_cd, size, mime_type}
    }

    pub fn parent_cd(&self) -> &String {
        &self.parent_cd
    }

    pub fn size(&self) -> &Dimensions {
        &self.size
    }

    pub fn mime_type(&self) -> &String {
        &self.mime_type
    }
}
