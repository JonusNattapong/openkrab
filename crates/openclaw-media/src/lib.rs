use openclaw_core::MediaType;
use openclaw_errors::{OpenClawError, Result};
use image::{DynamicImage, ImageFormat, ImageReader};
use std::io::Cursor;
use std::path::Path;
use tracing::info;

pub mod image_processor;
pub mod svg;

pub use image_processor::*;
pub use svg::*;

/// Media processor for images
pub struct MediaProcessor {
    max_file_size: usize,
}

impl MediaProcessor {
    pub fn new() -> Self {
        Self {
            max_file_size: 50 * 1024 * 1024, // 50MB
        }
    }

    /// Load image from bytes
    pub fn load_image(&self, data: &[u8]) -> Result<DynamicImage> {
        if data.len() > self.max_file_size {
            return Err(OpenClawError::Media(format!(
                "File too large: {} bytes",
                data.len()
            )));
        }

        image::load_from_memory(data)
            .map_err(|e| OpenClawError::Media(format!("Failed to load image: {}", e)))
    }

    /// Resize image
    pub fn resize(&self, img: &DynamicImage, width: u32, height: u32) -> DynamicImage {
        img.resize(width, height, image::imageops::FilterType::Lanczos3)
    }

    /// Create thumbnail
    pub fn thumbnail(&self, img: &DynamicImage, max_size: u32) -> DynamicImage {
        img.thumbnail(max_size, max_size)
    }

    /// Compress image to JPEG
    pub fn compress(&self, img: &DynamicImage, _quality: u8) -> Result<Vec<u8>> {
        let mut buffer = Cursor::new(Vec::new());
        img.write_to(&mut buffer, ImageFormat::Jpeg)
            .map_err(|e| OpenClawError::Media(format!("Failed to compress: {}", e)))?;
        Ok(buffer.into_inner())
    }

    /// Get image dimensions
    pub fn dimensions(&self, img: &DynamicImage) -> (u32, u32) {
        img.dimensions()
    }

    /// Process image
    pub async fn process_image(&self, data: Vec<u8>, target_type: MediaType) -> Result<Vec<u8>> {
        let img = self.load_image(&data)?;

        match target_type {
            MediaType::Image => self.compress(&img, 85),
            MediaType::Thumbnail => {
                let thumb = self.thumbnail(&img, 200);
                self.compress(&thumb, 75)
            }
            _ => Err(OpenClawError::Media("Unsupported type".to_string())),
        }
    }
}

impl Default for MediaProcessor {
    fn default() -> Self {
        Self::new()
    }
}

/// Detect image format from magic bytes
pub fn detect_format(data: &[u8]) -> Option<ImageFormat> {
    match data.get(0..8) {
        Some(&[0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A, ..]) => Some(ImageFormat::Png),
        Some(&[0xFF, 0xD8, 0xFF, ..]) => Some(ImageFormat::Jpeg),
        Some(&[0x47, 0x49, 0x46, 0x38, ..]) => Some(ImageFormat::Gif),
        _ => None,
    }
}

/// Convert to base64
pub fn to_base64(data: &[u8]) -> String {
    base64::Engine::encode(&base64::engine::general_purpose::STANDARD, data)
}