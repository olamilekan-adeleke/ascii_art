use image::ImageError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ImageProcessingError {
    #[error("Image path cannot be null or empty")]
    NullPath,

    #[error("Invalid image format. Allowed types are: jpg, jpeg, png, bmp")]
    InvalidFormat,

    #[error("Failed to load image: {0}")]
    LoadError(#[from] ImageError),

    #[error("Trying to get Pixels Out of Bounds")]
    OutOfBounds,
}
