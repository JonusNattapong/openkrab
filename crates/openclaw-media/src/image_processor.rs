use image::DynamicImage;
use openclaw_errors::Result;
use tracing::debug;

/// Image processing operations
pub struct ImageOps;

impl ImageOps {
    /// Resize with aspect ratio preserved
    pub fn resize_contain(img: &DynamicImage, max_width: u32, max_height: u32) -> DynamicImage {
        let (w, h) = img.dimensions();
        
        let ratio_w = max_width as f32 / w as f32;
        let ratio_h = max_height as f32 / h as f32;
        let ratio = ratio_w.min(ratio_h).min(1.0);
        
        let new_w = (w as f32 * ratio) as u32;
        let new_h = (h as f32 * ratio) as u32;
        
        img.resize(new_w, new_h, image::imageops::FilterType::Lanczos3)
    }
    
    /// Crop image
    pub fn crop(img: &DynamicImage, x: u32, y: u32, width: u32, height: u32) -> DynamicImage {
        img.crop_imm(x, y, width, height)
    }
    
    /// Rotate image
    pub fn rotate(img: &DynamicImage, degrees: f32) -> DynamicImage {
        img.rotate(degrees)
    }
    
    /// Apply grayscale
    pub fn grayscale(img: &DynamicImage) -> DynamicImage {
        img.grayscale()
    }
    
    /// Blur image
    pub fn blur(img: &DynamicImage, sigma: f32) -> DynamicImage {
        img.blur(sigma)
    }
    
    /// Brighten image
    pub fn brighten(img: &DynamicImage, value: i32) -> DynamicImage {
        img.brighten(value)
    }
}