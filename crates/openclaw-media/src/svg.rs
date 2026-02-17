use openclaw_errors::{OpenClawError, Result};
use tracing::debug;

/// SVG to PNG converter using resvg
pub struct SvgProcessor;

impl SvgProcessor {
    /// Convert SVG to PNG
    pub fn to_png(svg_data: &[u8], width: u32, height: u32) -> Result<Vec<u8>> {
        let svg_str = String::from_utf8(svg_data.to_vec())
            .map_err(|e| OpenClawError::Media(format!("Invalid UTF-8 in SVG: {}", e)))?;
        
        let opts = resvg::usvg::Options::default();
        
        let tree = resvg::usvg::Tree::from_str(&svg_str, &opts)
            .map_err(|e| OpenClawError::Media(format!("Failed to parse SVG: {}", e)))?;
        
        let size = tree.size();
        
        let width = if width == 0 { size.width() } else { width as f32 };
        let height = if height == 0 { size.height() } else { height as f32 };
        
        let pixmap_size = resvg::tiny_skia::IntSize::from_wh(width as u32, height as u32)
            .ok_or_else(|| OpenClawError::Media("Invalid size".to_string()))?;
        
        let mut pixmap = resvg::tiny_skia::Pixmap::new(pixmap_size.width(), pixmap_size.height())
            .ok_or_else(|| OpenClawError::Media("Failed to create pixmap".to_string()))?;
        
        let transform = resvg::tiny_skia::Transform::from_scale(
            width / size.width(),
            height / size.height(),
        );
        
        resvg::render(&tree, transform, &mut pixmap.as_mut());
        
        // Convert to PNG
        let mut png_data = Vec::new();
        let encoder = image::codecs::png::PngEncoder::new(&mut png_data);
        encoder.encode(
            pixmap.data(),
            pixmap.width(),
            pixmap.height(),
            image::ExtendedColorType::Rgba8,
        ).map_err(|e| OpenClawError::Media(format!("PNG encode error: {}", e)))?;
        
        Ok(png_data)
    }
    
    /// Get SVG dimensions
    pub fn dimensions(svg_data: &[u8]) -> Result<(u32, u32)> {
        let svg_str = String::from_utf8(svg_data.to_vec())
            .map_err(|e| OpenClawError::Media(format!("Invalid UTF-8: {}", e)))?;
        
        let opts = resvg::usvg::Options::default();
        let tree = resvg::usvg::Tree::from_str(&svg_str, &opts)
            .map_err(|e| OpenClawError::Media(format!("Parse error: {}", e)))?;
        
        let size = tree.size();
        Ok((size.width() as u32, size.height() as u32))
    }
}