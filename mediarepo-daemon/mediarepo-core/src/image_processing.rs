use crate::error::RepoResult;
use image::imageops::FilterType;
use image::io::Reader as ImageReader;
use image::{DynamicImage, ImageOutputFormat};
use std::io::Cursor;
use std::path::PathBuf;
use tokio::fs::OpenOptions;
use tokio::io::{AsyncRead, AsyncReadExt, BufReader};

/// Represents the different sizes of a thumbnail
#[derive(Clone, Copy)]
pub enum ThumbnailSize {
    Small,
    Medium,
    Large,
}

impl ThumbnailSize {
    pub fn dimensions(&self) -> (u32, u32) {
        match self {
            ThumbnailSize::Small => (128, 128),
            ThumbnailSize::Medium => (256, 256),
            ThumbnailSize::Large => (512, 512),
        }
    }
}

/// Reads an image from a path
pub async fn read_image_from_path(path: &PathBuf) -> RepoResult<DynamicImage> {
    let file = OpenOptions::new().read(true).open(path).await?;
    let mut reader = BufReader::new(file);
    read_image(&mut reader).await
}

/// Reads an image from a reader
pub async fn read_image<R: AsyncRead + Unpin>(reader: &mut R) -> RepoResult<DynamicImage> {
    let mut buf = Vec::new();
    reader.read_to_end(&mut buf).await?;
    let image = ImageReader::new(Cursor::new(buf))
        .with_guessed_format()?
        .decode()?;

    Ok(image)
}

/// Returns the bytes of an image in the png format
pub fn get_image_bytes_png(image: DynamicImage) -> RepoResult<Vec<u8>> {
    let mut buf = Vec::new();
    image.write_to(&mut buf, ImageOutputFormat::Png)?;

    Ok(buf)
}

/// Creates a thumbnail with the defined thumbnail size
pub fn create_thumbnail(image: DynamicImage, size: ThumbnailSize) -> DynamicImage {
    let (height, width) = size.dimensions();
    image.resize(height, width, FilterType::Nearest)
}
