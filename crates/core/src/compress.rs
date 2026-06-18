use std::path::{Path, PathBuf};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum CompressError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Image error: {0}")]
    Image(String),
    #[error("Unsupported format: {0}")]
    UnsupportedFormat(String),
}

#[derive(Debug, Clone)]
pub struct CompressResult {
    pub name: String,
    pub original_size: u64,
    pub compressed_size: u64,
    pub output_path: PathBuf,
}

fn compress_jpeg(input: &[u8], quality: u8) -> Result<Vec<u8>, CompressError> {
    use mozjpeg::{ColorSpace, Compress};

    let img = image::load_from_memory(input)
        .map_err(|e| CompressError::Image(e.to_string()))?;
    let rgb = img.to_rgb8();
    let (width, height) = rgb.dimensions();

    let mut compress = Compress::new(ColorSpace::JCS_RGB);
    compress.set_size(width as usize, height as usize);
    compress.set_quality(quality as f32);
    compress.set_optimize_coding(true);
    compress.set_progressive_mode();

    let mut comp = compress
        .start_compress(Vec::new())
        .map_err(|e| CompressError::Image(e.to_string()))?;

    let raw = rgb.as_raw();
    let stride = (width * 3) as usize;

    for row in 0..height {
        let start = (row * width * 3) as usize;
        let end = start + stride;
        comp.write_scanlines(&raw[start..end])
            .map_err(|e| CompressError::Image(e.to_string()))?;
    }

    let data = comp
        .finish()
        .map_err(|e| CompressError::Image(e.to_string()))?;
    Ok(data)
}

fn compress_png(input: &[u8], quality: u8) -> Result<Vec<u8>, CompressError> {
    let img = image::load_from_memory(input)
        .map_err(|e| CompressError::Image(e.to_string()))?;
    let rgba = img.to_rgba8();
    let (width, height) = rgba.dimensions();

    let raw_data: Vec<rgb::RGBA<u8>> = rgba
        .pixels()
        .map(|p| rgb::RGBA::new(p[0], p[1], p[2], p[3]))
        .collect();

    let mut liq = imagequant::new();
    let _ = liq.set_quality(0, quality);

    let mut img = liq
        .new_image(raw_data, width as usize, height as usize, 0.0)
        .map_err(|e| CompressError::Image(e.to_string()))?;

    let mut result = liq
        .quantize(&mut img)
        .map_err(|e| CompressError::Image(e.to_string()))?;

    let (palette, pixels) = result
        .remapped(&mut img)
        .map_err(|e| CompressError::Image(e.to_string()))?;

    let mut png_data = Vec::new();
    {
        use png::Encoder;

        let mut encoder = Encoder::new(&mut png_data, width, height);
        encoder.set_color(png::ColorType::Rgba);
        encoder.set_depth(png::BitDepth::Eight);

        let mut writer = encoder
            .write_header()
            .map_err(|e| CompressError::Image(e.to_string()))?;

        let raw_data: Vec<u8> = pixels
            .iter()
            .flat_map(|&idx| {
                let p = palette[idx as usize];
                vec![p.r, p.g, p.b, p.a]
            })
            .collect();

        writer
            .write_image_data(&raw_data)
            .map_err(|e| CompressError::Image(e.to_string()))?;
    }

    let opts = oxipng::Options::from_preset(3);
    let optimized = oxipng::optimize_from_memory(&png_data, &opts)
        .map_err(|e| CompressError::Image(e.to_string()))?;

    Ok(optimized)
}

pub fn compress_image(
    input_path: &Path,
    output_dir: &Path,
    quality: &super::config::Quality,
) -> Result<CompressResult, CompressError> {
    let input = std::fs::read(input_path)?;
    let ext = input_path
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("")
        .to_lowercase();

    let filename = input_path
        .file_name()
        .unwrap()
        .to_string_lossy()
        .to_string();
    let output_path = output_dir.join(&filename);

    let compressed = match ext.as_str() {
        "jpg" | "jpeg" => compress_jpeg(&input, quality.jpeg)?,
        "png" => compress_png(&input, quality.png)?,
        "gif" | "svg" => {
            std::fs::copy(input_path, &output_path)?;
            return Ok(CompressResult {
                name: filename,
                original_size: input.len() as u64,
                compressed_size: input.len() as u64,
                output_path,
            });
        }
        _ => return Err(CompressError::UnsupportedFormat(ext)),
    };

    std::fs::write(&output_path, &compressed)?;

    Ok(CompressResult {
        name: filename,
        original_size: input.len() as u64,
        compressed_size: compressed.len() as u64,
        output_path,
    })
}

pub fn compress_images(
    input_paths: &[PathBuf],
    output_dir: &Path,
    quality: &super::config::Quality,
) -> Vec<Result<CompressResult, CompressError>> {
    use rayon::prelude::*;

    input_paths
        .par_iter()
        .map(|path| compress_image(path, output_dir, quality))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compress_jpeg_invalid() {
        let result = compress_jpeg(b"not an image", 80);
        assert!(result.is_err());
    }

    #[test]
    fn test_compress_png_invalid() {
        let result = compress_png(b"not a png", 80);
        assert!(result.is_err());
    }

    #[test]
    fn test_unsupported_format() {
        let dir = std::env::temp_dir();
        let input = dir.join("test.bmp");
        std::fs::write(&input, b"test").unwrap();

        let quality = super::super::config::Quality::default();
        let result = compress_image(&input, &dir, &quality);
        assert!(result.is_err());
    }
}
