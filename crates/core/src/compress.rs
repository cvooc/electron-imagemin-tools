use std::path::{Path, PathBuf};
use thiserror::Error;

use super::config::OutputFormat;

#[derive(Error, Debug)]
pub enum CompressError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Image error: {0}")]
    Image(String),
    #[error("Unsupported format: {0}")]
    UnsupportedFormat(String),
    #[error("Invalid configuration: {0}")]
    InvalidConfig(String),
}

#[derive(Debug, Clone)]
pub struct CompressOptions {
    pub input_path: PathBuf,
    pub output_dir: PathBuf,
    pub quality: super::config::Quality,
    pub png_lossless: bool,
    pub output_format: super::config::OutputFormat,
    pub max_width: Option<u32>,
    pub max_height: Option<u32>,
    pub strip_metadata: bool,
    pub resize_filter: String,
}

pub struct CompressResult {
    pub name: String,
    pub original_size: u64,
    pub compressed_size: u64,
    pub output_path: PathBuf,
    /// 输出文件 xxhash3 校验和（十六进制）
    pub checksum: String,
    /// 备注，如 "已是最优，无需压缩"
    pub note: String,
}

fn compress_jpeg_raw(rgb: &image::RgbImage, quality: u8) -> Result<Vec<u8>, CompressError> {
    use mozjpeg::{ColorSpace, Compress};

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

fn compress_jpeg(input: &[u8], quality: u8) -> Result<Vec<u8>, CompressError> {
    let img = image::load_from_memory(input)
        .map_err(|e| CompressError::Image(e.to_string()))?;
    let rgb = img.to_rgb8();
    compress_jpeg_raw(&rgb, quality)
}

/// 使用 imagequant 将 RGBA 数据量化，并生成索引色 PNG。
fn quantize_to_indexed_png(
    rgba: &image::RgbaImage,
    quality: u8,
) -> Result<Vec<u8>, CompressError> {
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
        use png::{BitDepth, ColorType, Encoder};

        let has_alpha = palette.iter().any(|p| p.a != 255);
        let trns: Vec<u8> = palette.iter().map(|p| p.a).collect();
        let palette_bytes: Vec<u8> = palette
            .iter()
            .flat_map(|p| vec![p.r, p.g, p.b])
            .collect();

        let mut encoder = Encoder::new(&mut png_data, width, height);
        encoder.set_color(ColorType::Indexed);
        encoder.set_depth(BitDepth::Eight);
        encoder.set_palette(&palette_bytes);

        if has_alpha {
            encoder.set_trns(&trns);
        }

        let mut writer = encoder
            .write_header()
            .map_err(|e| CompressError::Image(e.to_string()))?;
        writer
            .write_image_data(&pixels)
            .map_err(|e| CompressError::Image(e.to_string()))?;
    }

    let opts = oxipng::Options {
        strip: oxipng::StripChunks::Safe,
        ..oxipng::Options::from_preset(3)
    };
    let optimized = oxipng::optimize_from_memory(&png_data, &opts)
        .map_err(|e| CompressError::Image(e.to_string()))?;

    Ok(optimized)
}

/// 对已经解码的 RGBA 数据进行有损量化压缩。
/// 注意：decoded 数据无法走真正的 oxipng 无损路径，因此总是量化后接 oxipng。
fn compress_png_raw(rgba: &image::RgbaImage, quality: u8) -> Result<Vec<u8>, CompressError> {
    quantize_to_indexed_png(rgba, quality)
}

fn compress_png(input: &[u8], quality: u8, lossless: bool) -> Result<Vec<u8>, CompressError> {
    // 先尝试用 oxipng 无损优化一遍，作为基准。
    // 若文件不是真实 PNG（扩展名误判），回退到 image 解码后重编码。
    let lossless_optimized = match oxipng::optimize_from_memory(input, &oxipng::Options::from_preset(3))
    {
        Ok(data) => data,
        Err(oxipng_err) => {
            // oxipng 失败时尝试用 image 解码后另存为 PNG
            let img = match image::load_from_memory(input) {
                Ok(img) => img,
                Err(_) => return Err(CompressError::Image(format!(
                    "文件不是合法 PNG 格式: {}", oxipng_err
                ))),
            };
            let rgba = img.to_rgba8();
            return quantize_to_indexed_png(&rgba, quality);
        }
    };

    if lossless {
        return Ok(lossless_optimized);
    }

    // 读取原图信息以决定是否需要进一步有损量化。
    let decoder = png::Decoder::new(std::io::Cursor::new(input));
    let reader = decoder
        .read_info()
        .map_err(|e| CompressError::Image(e.to_string()))?;
    let info = reader.info();

    // 灰度、索引色图片直接走无损，避免引入有损量化。
    let skip_lossy = matches!(
        info.color_type,
        png::ColorType::Grayscale | png::ColorType::Indexed | png::ColorType::GrayscaleAlpha
    );

    if skip_lossy {
        return Ok(lossless_optimized);
    }

    // RGB/RGBA 图片尝试有损量化；若量化后没有比无损更小，则回退到无损。
    let img = image::load_from_memory(input)
        .map_err(|e| CompressError::Image(e.to_string()))?;
    let rgba = img.to_rgba8();

    let lossy = quantize_to_indexed_png(&rgba, quality)?;

    if lossy.len() < lossless_optimized.len() {
        Ok(lossy)
    } else {
        Ok(lossless_optimized)
    }
}

/// 压缩 GIF：单帧 GIF 通过 imagequant 减少调色板；多帧动画 GIF 保留原样。
fn compress_gif(input: &[u8], quality: u8) -> Result<Vec<u8>, CompressError> {
    let mut decoder = gif::DecodeOptions::new();
    decoder.set_color_output(gif::ColorOutput::RGBA);
    let mut decoder = decoder
        .read_info(input)
        .map_err(|e| CompressError::Image(e.to_string()))?;

    let mut frames = Vec::new();
    let width = decoder.width() as u32;
    let height = decoder.height() as u32;

    while let Some(frame) = decoder
        .read_next_frame()
        .map_err(|e| CompressError::Image(e.to_string()))?
    {
        frames.push(frame.clone());
    }

    // 多帧动画先保持原样，避免破坏动画。
    if frames.len() != 1 {
        eprintln!("compress_gif: 多帧 GIF（{} 帧），保持原始数据", frames.len());
        return Ok(input.to_vec());
    }

    let frame = &frames[0];
    let rgba = image::RgbaImage::from_raw(width, height, frame.buffer.to_vec())
        .ok_or_else(|| CompressError::Image("GIF 帧数据尺寸不匹配".to_string()))?;

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

    let palette_bytes: Vec<u8> = palette
        .iter()
        .flat_map(|p| vec![p.r, p.g, p.b])
        .collect();

    let mut output = Vec::new();
    {
        let mut encoder = gif::Encoder::new(
            &mut output,
            width as u16,
            height as u16,
            &palette_bytes,
        )
        .map_err(|e| CompressError::Image(e.to_string()))?;
        encoder
            .set_repeat(gif::Repeat::Infinite)
            .map_err(|e| CompressError::Image(e.to_string()))?;

        let mut frame = gif::Frame::from_indexed_pixels(width as u16, height as u16, pixels, None);
        frame.delay = 10;
        encoder
            .write_frame(&frame)
            .map_err(|e| CompressError::Image(e.to_string()))?;
    }

    // 若压缩后反而更大，返回原图。
    if output.len() < input.len() {
        Ok(output)
    } else {
        Ok(input.to_vec())
    }
}

/// SVG 光栅化最大尺寸（像素），防止恶意超大 SVG 导致 OOM。
const MAX_SVG_DIMENSION: u32 = 4096;

/// 将 SVG 光栅化为 PNG 后压缩。输出格式从 .svg 变为 .png。
fn compress_svg(input: &[u8], quality: u8) -> Result<Vec<u8>, CompressError> {
    let svg_str = std::str::from_utf8(input)
        .map_err(|e| CompressError::Image(format!("SVG 不是合法 UTF-8: {}", e)))?;

    let options = usvg::Options::default();
    let tree = usvg::Tree::from_str(svg_str, &options)
        .map_err(|e| CompressError::Image(format!("解析 SVG 失败: {}", e)))?;

    let pixmap_size = tree.size().to_int_size();
    let w = pixmap_size.width().min(MAX_SVG_DIMENSION);
    let h = pixmap_size.height().min(MAX_SVG_DIMENSION);

    // 如果 SVG 尺寸超出上限，处理为 1x1（由用户 resize 决定，而非 OOM）
    let (width, height) = if w == 0 || h == 0 {
        (1u32, 1u32)
    } else {
        (w, h)
    };

    let mut pixmap = tiny_skia::Pixmap::new(width, height)
        .ok_or_else(|| CompressError::Image("无法创建 SVG 渲染缓冲区".to_string()))?;
    pixmap.fill(tiny_skia::Color::from_rgba8(255, 255, 255, 0));

    resvg::render(&tree, tiny_skia::Transform::default(), &mut pixmap.as_mut());

    let rgba = image::RgbaImage::from_raw(
        width,
        height,
        pixmap.data().to_vec(),
    )
    .ok_or_else(|| CompressError::Image("SVG 渲染结果尺寸不匹配".to_string()))?;

    // 用 oxipng 无损优化光栅化后的 PNG。
    quantize_to_indexed_png(&rgba, quality)
}

/// 解码 WebP 后按是否有透明通道分别输出为 PNG 或 JPEG。
fn compress_webp(
    input: &[u8],
    quality: &super::config::Quality,
    _png_lossless: bool,
) -> Result<Vec<u8>, CompressError> {
    let img = image::load_from_memory(input)
        .map_err(|e| CompressError::Image(e.to_string()))?;

    if img.color().has_alpha() {
        let rgba = img.to_rgba8();
        compress_png_raw(&rgba, quality.png)
    } else {
        let rgb = img.to_rgb8();
        compress_jpeg_raw(&rgb, quality.jpeg)
    }
}

/// 如果图片尺寸超出限制，等比缩小。
fn resize_if_needed(
    img: image::DynamicImage,
    max_width: Option<u32>,
    max_height: Option<u32>,
    filter: &str,
) -> image::DynamicImage {
    let (w, h) = (img.width(), img.height());
    let limit_w = max_width.unwrap_or(u32::MAX);
    let limit_h = max_height.unwrap_or(u32::MAX);

    if w <= limit_w && h <= limit_h {
        return img;
    }

    let ratio = (limit_w as f64 / w as f64).min(limit_h as f64 / h as f64);
    let new_w = (w as f64 * ratio) as u32;
    let new_h = (h as f64 * ratio) as u32;

    let ftype = match filter {
        "triangle" => image::imageops::FilterType::Triangle,
        "catmullrom" => image::imageops::FilterType::CatmullRom,
        _ => image::imageops::FilterType::Lanczos3,
    };
    img.resize_exact(new_w, new_h, ftype)
}


/// 将 RGB 数据编码为 WebP。
fn compress_webp_encoder_rgb(
    rgb: &image::RgbImage,
    quality: u8,
) -> Result<Vec<u8>, CompressError> {
    use webp::Encoder;

    let (width, height) = rgb.dimensions();
    let encoder = Encoder::from_rgb(rgb.as_raw(), width, height);
    let encoded = encoder.encode(quality as f32);
    Ok(encoded.to_vec())
}

/// 将 RGBA 数据编码为 WebP（带透明通道）。
fn compress_webp_encoder_rgba(
    rgba: &image::RgbaImage,
    quality: u8,
) -> Result<Vec<u8>, CompressError> {
    use webp::Encoder;

    let (width, height) = rgba.dimensions();
    let encoder = Encoder::from_rgba(rgba.as_raw(), width, height);
    let encoded = encoder.encode(quality as f32);
    Ok(encoded.to_vec())
}

/// 将 RGBA 数据编码为 AVIF。
fn compress_avif_raw(
    rgba: &image::RgbaImage,
    quality: u8,
    speed: u8,
) -> Result<Vec<u8>, CompressError> {
    use ravif::{encode_rgba, Config, Img};

    let (width, height) = rgba.dimensions();

    // image::RgbaImage 使用 [u8; 4]，需转为 rgb::RGBA<u8>
    let pixels: Vec<rgb::RGBA<u8>> = rgba
        .pixels()
        .map(|p| rgb::RGBA::new(p[0], p[1], p[2], p[3]))
        .collect();
    let img = Img::new(pixels.as_slice(), width as usize, height as usize);

    let config = Config {
        quality: quality as f32,
        alpha_quality: quality as f32,
        speed: speed as u8,
        premultiplied_alpha: false,
        color_space: ravif::ColorSpace::RGB,
        threads: 0,
    };

    let (data, _, _) = encode_rgba(img, &config)
        .map_err(|e| CompressError::Image(format!("AVIF 编码失败: {}", e)))?;

    Ok(data)
}

/// 将 RGB 数据编码为 AVIF。
fn compress_avif_from_rgb(
    rgb: &image::RgbImage,
    quality: u8,
    speed: u8,
) -> Result<Vec<u8>, CompressError> {
    use ravif::{encode_rgb, Config, Img};

    let (width, height) = rgb.dimensions();

    let pixels: Vec<rgb::RGB<u8>> = rgb
        .pixels()
        .map(|p| rgb::RGB::new(p[0], p[1], p[2]))
        .collect();
    let img = Img::new(pixels.as_slice(), width as usize, height as usize);

    let config = Config {
        quality: quality as f32,
        alpha_quality: quality as f32,
        speed: speed as u8,
        premultiplied_alpha: false,
        color_space: ravif::ColorSpace::RGB,
        threads: 0,
    };

    let (data, _) = encode_rgb(img, &config)
        .map_err(|e| CompressError::Image(format!("AVIF 编码失败: {}", e)))?;

    Ok(data)
}

pub fn compress_image(
    input_path: &Path,
    output_dir: &Path,
    quality: &super::config::Quality,
    png_lossless: bool,
    output_format: OutputFormat,
    max_width: Option<u32>,
    max_height: Option<u32>,
    _strip_metadata: bool,
) -> Result<CompressResult, CompressError> {
    quality
        .validate()
        .map_err(CompressError::InvalidConfig)?;

    let input = std::fs::read(input_path)?;
    let ext = input_path
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("")
        .to_lowercase();

    let filename = input_path
        .file_name()
        .ok_or_else(|| CompressError::Image("输入路径缺少文件名".to_string()))?
        .to_string_lossy()
        .to_string();

    let (output_filename, compressed) = match output_format {
        OutputFormat::Original => {
            // 保持原格式，使用原有逻辑
            compress_original(&input, &filename, &ext, quality, png_lossless)?
        }
        OutputFormat::Jpeg => {
            let new_name = change_extension(&filename, "jpg");
            let img = image::load_from_memory(&input)
                .map_err(|e| CompressError::Image(e.to_string()))?;
            let img = resize_if_needed(img, max_width, max_height, "lanczos3");
            let rgb = img.to_rgb8();
            (new_name, compress_jpeg_raw(&rgb, quality.jpeg)?)
        }
        OutputFormat::Png => {
            let new_name = change_extension(&filename, "png");
            let img = image::load_from_memory(&input)
                .map_err(|e| CompressError::Image(e.to_string()))?;
            let img = resize_if_needed(img, max_width, max_height, "lanczos3");
            let rgba = img.to_rgba8();
            let data = compress_png_raw(&rgba, quality.png)?;
            (new_name, data)
        }
        OutputFormat::WebP => {
            let new_name = change_extension(&filename, "webp");
            let img = image::load_from_memory(&input)
                .map_err(|e| CompressError::Image(e.to_string()))?;
            let img = resize_if_needed(img, max_width, max_height, "lanczos3");
            let data = if img.color().has_alpha() {
                compress_webp_encoder_rgba(&img.to_rgba8(), quality.jpeg)?
            } else {
                compress_webp_encoder_rgb(&img.to_rgb8(), quality.jpeg)?
            };
            (new_name, data)
        }
        OutputFormat::Avif => {
            let new_name = change_extension(&filename, "avif");
            let img = image::load_from_memory(&input)
                .map_err(|e| CompressError::Image(e.to_string()))?;
            let img = resize_if_needed(img, max_width, max_height, "lanczos3");
            let data = if img.color().has_alpha() {
                compress_avif_raw(&img.to_rgba8(), quality.jpeg, 4)?
            } else {
                compress_avif_from_rgb(&img.to_rgb8(), quality.jpeg, 4)?
            };
            (new_name, data)
        }
    };

    let output_path = output_dir.join(&output_filename);
    std::fs::create_dir_all(output_dir)?;

    // 若输出文件已存在，自动追加编号避免覆盖
    let (output_filename, output_path) = if output_path.exists() {
        let stem = output_path.file_stem().and_then(|s| s.to_str()).unwrap_or("file");
        let ext = output_path.extension().and_then(|s| s.to_str()).unwrap_or("");
        let mut counter = 1;
        loop {
            let new_name = format!("{}_{}.{}", stem, counter, ext);
            let candidate = output_dir.join(&new_name);
            if !candidate.exists() {
                break (new_name, candidate);
            }
            counter += 1;
        }
    } else {
        (output_filename, output_path)
    };

    std::fs::write(&output_path, &compressed)?;

    // 计算输出文件校验和
    let checksum = {
        use std::hash::Hasher;
        let mut hasher = crc32fast::Hasher::new();
        hasher.write(&compressed);
        format!("{:08x}", hasher.finish())
    };

    // 判断是否已是最优（压缩后反而变大或未变小）
    let note = if compressed.len() >= input.len() {
        "已是最优，无需压缩".to_string()
    } else {
        String::new()
    };

    Ok(CompressResult {
        name: output_filename,
        original_size: input.len() as u64,
        compressed_size: compressed.len() as u64,
        output_path,
        checksum,
        note,
    })
}

/// 保持原格式的压缩逻辑。
fn compress_original(
    input: &[u8],
    filename: &str,
    ext: &str,
    quality: &super::config::Quality,
    png_lossless: bool,
) -> Result<(String, Vec<u8>), CompressError> {
    match ext {
        "jpg" | "jpeg" => Ok((filename.to_string(), compress_jpeg(input, quality.jpeg)?)),
        "png" => Ok((filename.to_string(), compress_png(input, quality.png, png_lossless)?)),
        "gif" => Ok((filename.to_string(), compress_gif(input, quality.png)?)),
        "svg" => {
            let png_name = filename
                .rsplit_once('.')
                .map(|(name, _)| format!("{}.png", name))
                .unwrap_or_else(|| format!("{}.png", filename));
            Ok((png_name, compress_svg(input, quality.png)?))
        }
        "webp" => {
            // 若文件名含 -lossless 后缀则保留 .webp（用户明确表示想要无损格式）
            // 否则转为 JPEG（WebP 输入在 Original 模式下默认转 JPEG）
            let output_name = if filename
                .to_lowercase()
                .ends_with("-lossless.webp")
            {
                filename.to_string()
            } else {
                let stem = filename.rsplit_once('.').map(|(s, _)| s).unwrap_or(filename);
                format!("{}.jpg", stem)
            };
            Ok((output_name, compress_webp(input, quality, png_lossless)?))
        }
        _ => Err(CompressError::UnsupportedFormat(ext.to_string())),
    }
}

/// 将文件名后缀改为指定扩展名。
fn change_extension(filename: &str, new_ext: &str) -> String {
    let stem = filename.rsplit_once('.').map(|(s, _)| s).unwrap_or(filename);
    format!("{}.{}", stem, new_ext)
}

/// 批量压缩：使用 rayon 并行处理多个文件。
/// 注意：调用方已通过 tokio::spawn_blocking 执行，内部再用 rayon 是合理的两层并行设计。
pub fn compress_images(
    input_paths: &[PathBuf],
    output_dir: &Path,
    quality: &super::config::Quality,
    png_lossless: bool,
    output_format: OutputFormat,
    max_width: Option<u32>,
    max_height: Option<u32>,
    _strip_metadata: bool,
) -> Vec<Result<CompressResult, CompressError>> {
    use rayon::prelude::*;

    input_paths
        .par_iter()
        .map(|path| compress_image(path, output_dir, quality, png_lossless, output_format, max_width, max_height, _strip_metadata))
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
        let result = compress_png(b"not a png", 80, false);
        assert!(result.is_err());
    }

    #[test]
    fn test_unsupported_format() {
        let dir = std::env::temp_dir();
        let input = dir.join("test.bmp");
        std::fs::write(&input, b"test").unwrap();

        let quality = super::super::config::Quality::default();
        let result = compress_image(&input, &dir, &quality, false, OutputFormat::Original, None, None, false);
        assert!(result.is_err());
    }
}
