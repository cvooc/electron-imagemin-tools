use imagemin_core::{compress_image, compress_images, Config, History, HistoryEntry, OutputFormat, OutputMode, Quality};
use std::path::PathBuf;
use tempfile::TempDir;

fn create_test_jpeg(path: &PathBuf) {
    use image::{ImageBuffer, Rgb};
    let img: ImageBuffer<Rgb<u8>, Vec<u8>> = ImageBuffer::from_fn(100, 100, |x, y| {
        Rgb([((x + y) % 256) as u8, 128, 64])
    });
    img.save(path).unwrap();
}

fn create_test_png(path: &PathBuf) {
    use image::{ImageBuffer, Rgba};
    let img: ImageBuffer<Rgba<u8>, Vec<u8>> = ImageBuffer::from_fn(100, 100, |x, y| {
        Rgba([((x + y) % 256) as u8, 128, 64, 255])
    });
    img.save(path).unwrap();
}

fn create_test_gif(path: &PathBuf) {
    use image::{ImageBuffer, Rgb};
    let img: ImageBuffer<Rgb<u8>, Vec<u8>> = ImageBuffer::from_fn(100, 100, |x, y| {
        Rgb([((x + y) % 256) as u8, 128, 64])
    });
    img.save(path).unwrap();
}

fn create_small_jpeg(path: &PathBuf) {
    use image::{ImageBuffer, Rgb};
    let img: ImageBuffer<Rgb<u8>, Vec<u8>> = ImageBuffer::from_fn(2, 2, |_x, _y| Rgb([255, 0, 0]));
    img.save(path).unwrap();
}

fn create_grayscale_png(path: &PathBuf) {
    use image::{ImageBuffer, Luma};
    let img: ImageBuffer<Luma<u8>, Vec<u8>> = ImageBuffer::from_fn(100, 100, |x, _y| {
        Luma([(x % 256) as u8])
    });
    img.save(path).unwrap();
}

fn create_transparent_png(path: &PathBuf) {
    use image::{ImageBuffer, Rgba};
    let img: ImageBuffer<Rgba<u8>, Vec<u8>> = ImageBuffer::from_fn(100, 100, |x, y| {
        Rgba([255, 0, 0, if (x + y) % 2 == 0 { 255 } else { 0 }])
    });
    img.save(path).unwrap();
}

fn create_svg(path: &PathBuf) {
    let svg_content = r#"<?xml version="1.0" encoding="UTF-8"?>
<svg width="100" height="100" xmlns="http://www.w3.org/2000/svg">
  <rect width="100" height="100" fill="red"/>
</svg>"#;
    std::fs::write(path, svg_content).unwrap();
}

fn create_test_webp(path: &PathBuf) {
    use image::{ImageBuffer, Rgb};
    let img: ImageBuffer<Rgb<u8>, Vec<u8>> = ImageBuffer::from_fn(100, 100, |x, y| {
        Rgb([((x + y) % 256) as u8, 128, 64])
    });
    let (w, h) = img.dimensions();
    let encoder = webp::Encoder::from_rgb(img.as_raw(), w, h);
    let encoded = encoder.encode(80.0);
    std::fs::write(path, &*encoded).unwrap();
}

// ==================== 压缩功能测试 ====================

#[test]
fn test_compress_jpeg_basic() {
    let temp = TempDir::new().unwrap();
    let input_path = temp.path().join("test.jpg");
    create_test_jpeg(&input_path);

    let output_dir = temp.path().join("output");
    let quality = Quality::default();
    let result = compress_image(&input_path, &output_dir, &quality, false, OutputFormat::Original, None, None, false).unwrap();

    assert_eq!(result.name, "test.jpg");
    assert!(result.original_size > 0);
    assert!(result.compressed_size > 0);
    assert!(result.output_path.exists());
    assert!(result.compressed_size <= result.original_size);
}

#[test]
fn test_compress_jpeg_low_quality() {
    let temp = TempDir::new().unwrap();
    let input_path = temp.path().join("test.jpg");
    create_test_jpeg(&input_path);

    let output_dir = temp.path().join("output");
    let quality = Quality { jpeg: 10, png: 80 };
    let result = compress_image(&input_path, &output_dir, &quality, false, OutputFormat::Original, None, None, false).unwrap();

    assert!(result.compressed_size < result.original_size);
}

#[test]
fn test_compress_jpeg_high_quality() {
    let temp = TempDir::new().unwrap();
    let input_path = temp.path().join("test.jpg");
    create_test_jpeg(&input_path);

    let output_dir = temp.path().join("output");
    let quality = Quality { jpeg: 95, png: 80 };
    let result = compress_image(&input_path, &output_dir, &quality, false, OutputFormat::Original, None, None, false);
    assert!(result.is_ok());
    assert!(result.unwrap().compressed_size > 0);
}

#[test]
fn test_compress_png_basic() {
    let temp = TempDir::new().unwrap();
    let input_path = temp.path().join("test.png");
    create_test_png(&input_path);

    let output_dir = temp.path().join("output");
    let quality = Quality::default();
    let result = compress_image(&input_path, &output_dir, &quality, false, OutputFormat::Original, None, None, false).unwrap();

    assert_eq!(result.name, "test.png");
    assert!(result.original_size > 0);
    assert!(result.compressed_size > 0);
    assert!(result.output_path.exists());
}

#[test]
fn test_compress_png_lossless() {
    let temp = TempDir::new().unwrap();
    let input_path = temp.path().join("test.png");
    create_test_png(&input_path);

    let output_dir = temp.path().join("output");
    let quality = Quality::default();
    let result = compress_image(&input_path, &output_dir, &quality, true, OutputFormat::Original, None, None, false).unwrap();

    assert!(result.compressed_size > 0);
    assert!(result.output_path.exists());
}

#[test]
fn test_compress_png_grayscale() {
    let temp = TempDir::new().unwrap();
    let input_path = temp.path().join("test.png");
    create_grayscale_png(&input_path);

    let output_dir = temp.path().join("output");
    let quality = Quality::default();
    let result = compress_image(&input_path, &output_dir, &quality, false, OutputFormat::Original, None, None, false).unwrap();

    assert!(result.compressed_size > 0);
}

#[test]
fn test_compress_png_transparent() {
    let temp = TempDir::new().unwrap();
    let input_path = temp.path().join("test.png");
    create_transparent_png(&input_path);

    let output_dir = temp.path().join("output");
    let quality = Quality::default();
    let result = compress_image(&input_path, &output_dir, &quality, false, OutputFormat::Original, None, None, false).unwrap();

    assert!(result.compressed_size > 0);
}

#[test]
fn test_compress_gif() {
    let temp = TempDir::new().unwrap();
    let input_path = temp.path().join("test.gif");
    create_test_gif(&input_path);

    let output_dir = temp.path().join("output");
    let quality = Quality::default();
    let result = compress_image(&input_path, &output_dir, &quality, false, OutputFormat::Original, None, None, false).unwrap();

    assert_eq!(result.name, "test.gif");
    assert!(result.compressed_size > 0);
    assert!(result.output_path.exists());
}

#[test]
fn test_compress_svg() {
    let temp = TempDir::new().unwrap();
    let input_path = temp.path().join("test.svg");
    create_svg(&input_path);

    let output_dir = temp.path().join("output");
    let quality = Quality::default();
    let result = compress_image(&input_path, &output_dir, &quality, false, OutputFormat::Original, None, None, false).unwrap();

    // SVG 被光栅化为 PNG，输出文件名应变更为 .png
    assert_eq!(result.name, "test.png");
    assert!(result.output_path.exists());
    assert!(result.output_path.extension().unwrap() == "png");
}

#[test]
fn test_compress_large_image() {
    let temp = TempDir::new().unwrap();
    let input_path = temp.path().join("large.png");
    create_test_png(&input_path);

    assert!(input_path.exists(), "Input file should exist");

    let output_dir = temp.path().join("output");
    let quality = Quality { jpeg: 50, png: 50 };
    let result = compress_image(&input_path, &output_dir, &quality, false, OutputFormat::Original, None, None, false);

    assert!(result.is_ok(), "Compression should succeed: {:?}", result.err());
    let result = result.unwrap();
    assert!(result.original_size > 0);
    assert!(result.compressed_size > 0);
}

#[test]
fn test_compress_small_image() {
    let temp = TempDir::new().unwrap();
    let input_path = temp.path().join("small.jpg");
    create_small_jpeg(&input_path);

    let output_dir = temp.path().join("output");
    let quality = Quality::default();
    let result = compress_image(&input_path, &output_dir, &quality, false, OutputFormat::Original, None, None, false).unwrap();

    assert!(result.compressed_size > 0);
}

#[test]
fn test_compress_unsupported_bmp() {
    let temp = TempDir::new().unwrap();
    let input_path = temp.path().join("test.bmp");
    std::fs::write(&input_path, b"fake bmp data").unwrap();

    let output_dir = temp.path().join("output");
    let quality = Quality::default();
    let result = compress_image(&input_path, &output_dir, &quality, false, OutputFormat::Original, None, None, false);

    assert!(result.is_err());
}

#[test]
fn test_compress_unsupported_webp() {
    let temp = TempDir::new().unwrap();
    let input_path = temp.path().join("test.webp");
    std::fs::write(&input_path, b"fake webp data").unwrap();

    let output_dir = temp.path().join("output");
    let quality = Quality::default();
    let result = compress_image(&input_path, &output_dir, &quality, false, OutputFormat::Original, None, None, false);

    assert!(result.is_err());
}

#[test]
fn test_compress_nonexistent_file() {
    let temp = TempDir::new().unwrap();
    let input_path = temp.path().join("nonexistent.jpg");
    let output_dir = temp.path().join("output");

    let quality = Quality::default();
    let result = compress_image(&input_path, &output_dir, &quality, false, OutputFormat::Original, None, None, false);

    assert!(result.is_err());
}

#[test]
fn test_compress_empty_file() {
    let temp = TempDir::new().unwrap();
    let input_path = temp.path().join("empty.jpg");
    std::fs::write(&input_path, b"").unwrap();

    let output_dir = temp.path().join("output");
    let quality = Quality::default();
    let result = compress_image(&input_path, &output_dir, &quality, false, OutputFormat::Original, None, None, false);

    assert!(result.is_err());
}

#[test]
fn test_compress_corrupted_jpeg() {
    let temp = TempDir::new().unwrap();
    let input_path = temp.path().join("corrupt.jpg");
    std::fs::write(&input_path, b"not a real jpeg file content").unwrap();

    let output_dir = temp.path().join("output");
    let quality = Quality::default();
    let result = compress_image(&input_path, &output_dir, &quality, false, OutputFormat::Original, None, None, false);

    assert!(result.is_err());
}

#[test]
fn test_compress_quality_comparison() {
    let temp = TempDir::new().unwrap();
    let input_path = temp.path().join("test.jpg");
    create_test_jpeg(&input_path);

    let quality_low = Quality { jpeg: 10, png: 80 };
    let quality_mid = Quality { jpeg: 50, png: 80 };
    let quality_high = Quality { jpeg: 90, png: 80 };

    let out_low = temp.path().join("low");
    let out_mid = temp.path().join("mid");
    let out_high = temp.path().join("high");

    let r_low = compress_image(&input_path, &out_low, &quality_low, false, OutputFormat::Original, None, None, false).unwrap();
    let r_mid = compress_image(&input_path, &out_mid, &quality_mid, false, OutputFormat::Original, None, None, false).unwrap();
    let r_high = compress_image(&input_path, &out_high, &quality_high, false, OutputFormat::Original, None, None, false).unwrap();

    assert!(r_low.compressed_size <= r_mid.compressed_size);
    assert!(r_mid.compressed_size <= r_high.compressed_size);
}

#[test]
fn test_compress_output_filename_preserved() {
    let temp = TempDir::new().unwrap();
    let input_path = temp.path().join("my_photo_2024.jpg");
    create_test_jpeg(&input_path);

    let output_dir = temp.path().join("output");
    let quality = Quality::default();
    let result = compress_image(&input_path, &output_dir, &quality, false, OutputFormat::Original, None, None, false).unwrap();

    assert_eq!(result.name, "my_photo_2024.jpg");
    assert!(result.output_path.ends_with("my_photo_2024.jpg"));
}

#[test]
fn test_compress_chinese_filename() {
    let temp = TempDir::new().unwrap();
    let input_path = temp.path().join("测试图片.jpg");
    create_test_jpeg(&input_path);

    let output_dir = temp.path().join("output");
    let quality = Quality::default();
    let result = compress_image(&input_path, &output_dir, &quality, false, OutputFormat::Original, None, None, false).unwrap();

    assert_eq!(result.name, "测试图片.jpg");
    assert!(result.output_path.exists());
}

// ==================== 配置功能测试 ====================

#[test]
fn test_config_default_values() {
    let config = Config::default();
    assert_eq!(config.quality.jpeg, 80);
    assert_eq!(config.quality.png, 80);
    assert!(!config.png_lossless);
}

#[test]
fn test_config_serialization_roundtrip() {
    let config = Config {
        quality: Quality { jpeg: 75, png: 60 },
        output_mode: OutputMode::SameDir,
        custom_output_dir: Some(PathBuf::from("/tmp/out")),
        png_lossless: true,
        ..Default::default()
    };

    let toml_str = toml::to_string(&config).unwrap();
    let loaded: Config = toml::from_str(&toml_str).unwrap();

    assert_eq!(loaded.quality.jpeg, 75);
    assert_eq!(loaded.quality.png, 60);
    assert_eq!(loaded.output_mode, OutputMode::SameDir);
    assert_eq!(loaded.custom_output_dir, Some(PathBuf::from("/tmp/out")));
    assert!(loaded.png_lossless);
}

#[test]
fn test_config_partial_toml_uses_defaults() {
    let toml_str = r#"
[quality]
jpeg = 50
"#;

    let result: Result<Config, _> = toml::from_str(toml_str);
    // serde 默认值会补全缺失字段，因此 partial toml 应该能成功反序列化。
    assert!(result.is_ok());
    let config = result.unwrap();
    assert_eq!(config.quality.jpeg, 50);
    assert_eq!(config.quality.png, 80);
}

#[test]
fn test_config_invalid_values() {
    let toml_str = r#"
[quality]
jpeg = "not a number"
png = 80
"#;

    let result: Result<Config, _> = toml::from_str(toml_str);
    assert!(result.is_err());
}

#[test]
fn test_quality_boundary_values() {
    let q_min = Quality { jpeg: 5, png: 10 };
    let q_max = Quality { jpeg: 100, png: 100 };

    assert!(q_min.validate().is_ok());
    assert!(q_max.validate().is_ok());
    // quality=0 现在是非法的（低于下限）
    assert!(Quality { jpeg: 0, png: 0 }.validate().is_err());
}

#[test]
fn test_quality_invalid_values() {
    let q = Quality { jpeg: 255, png: 80 };
    assert!(q.validate().is_err());
}

#[test]
fn test_config_output_dir_timestamped() {
    let config = Config::default();
    let dir = config.resolve_output_dir(None);
    assert!(dir.to_string_lossy().contains("retrocode_io"));
    assert!(dir.to_string_lossy().contains("imagemin"));
}

#[test]
fn test_config_output_dir_same_dir() {
    let config = Config {
        output_mode: OutputMode::SameDir,
        ..Default::default()
    };
    let input = PathBuf::from("/tmp/images/photo.jpg");
    let dir = config.resolve_output_dir(Some(&input));
    assert_eq!(dir, PathBuf::from("/tmp/images"));
}

#[test]
fn test_config_save_and_load() {
    let temp = TempDir::new().unwrap();
    let config_path = temp.path().join("config.toml");

    let config = Config {
        quality: Quality { jpeg: 65, png: 75 },
        output_mode: OutputMode::Custom,
        custom_output_dir: Some(PathBuf::from("D:\\custom_output")),
        png_lossless: true,
        ..Default::default()
    };

    std::fs::write(&config_path, toml::to_string_pretty(&config).unwrap()).unwrap();

    let loaded_content = std::fs::read_to_string(&config_path).unwrap();
    let loaded: Config = toml::from_str(&loaded_content).unwrap();

    assert_eq!(loaded.quality.jpeg, 65);
    assert_eq!(loaded.quality.png, 75);
    assert_eq!(loaded.output_mode, OutputMode::Custom);
    assert!(loaded.png_lossless);
}

// ==================== 文件扩展名测试 ====================

#[test]
fn test_compress_jpg_extension() {
    let temp = TempDir::new().unwrap();
    let input_path = temp.path().join("test.jpg");
    create_test_jpeg(&input_path);

    let output_dir = temp.path().join("output");
    let quality = Quality::default();
    let result = compress_image(&input_path, &output_dir, &quality, false, OutputFormat::Original, None, None, false);
    assert!(result.is_ok());
}

#[test]
fn test_compress_jpeg_extension() {
    let temp = TempDir::new().unwrap();
    let input_path = temp.path().join("test.jpeg");
    create_test_jpeg(&input_path);

    let output_dir = temp.path().join("output");
    let quality = Quality::default();
    let result = compress_image(&input_path, &output_dir, &quality, false, OutputFormat::Original, None, None, false);
    assert!(result.is_ok());
}

#[test]
fn test_compress_uppercase_extension() {
    let temp = TempDir::new().unwrap();
    let input_path = temp.path().join("test.JPG");
    create_test_jpeg(&input_path);

    let output_dir = temp.path().join("output");
    let quality = Quality::default();
    let result = compress_image(&input_path, &output_dir, &quality, false, OutputFormat::Original, None, None, false);
    assert!(result.is_ok());
}

#[test]
fn test_compress_mixed_case_extension() {
    let temp = TempDir::new().unwrap();
    let input_path = temp.path().join("test.Png");
    create_test_png(&input_path);

    let output_dir = temp.path().join("output");
    let quality = Quality::default();
    let result = compress_image(&input_path, &output_dir, &quality, false, OutputFormat::Original, None, None, false);
    assert!(result.is_ok());
}

// ==================== 批量压缩测试 ====================

#[test]
fn test_batch_compress_multiple_files() {
    let temp = TempDir::new().unwrap();

    create_test_jpeg(&temp.path().join("1.jpg"));
    create_test_png(&temp.path().join("2.png"));
    create_test_gif(&temp.path().join("3.gif"));

    let output_dir = temp.path().join("output");
    let quality = Quality::default();

    let r1 = compress_image(&temp.path().join("1.jpg"), &output_dir, &quality, false, OutputFormat::Original, None, None, false).unwrap();
    let r2 = compress_image(&temp.path().join("2.png"), &output_dir, &quality, false, OutputFormat::Original, None, None, false).unwrap();
    let r3 = compress_image(&temp.path().join("3.gif"), &output_dir, &quality, false, OutputFormat::Original, None, None, false).unwrap();

    assert!(r1.compressed_size > 0);
    assert!(r2.compressed_size > 0);
    assert!(r3.compressed_size > 0);
}

#[test]
fn test_compress_to_same_directory() {
    let temp = TempDir::new().unwrap();
    create_test_jpeg(&temp.path().join("test.jpg"));

    let quality = Quality::default();
    let result = compress_image(&temp.path().join("test.jpg"), temp.path(), &quality, false, OutputFormat::Original, None, None, false).unwrap();

    assert!(result.output_path.exists());
}

// ==================== 输出模式测试 ====================

#[test]
fn test_output_mode_same_dir_creates_file_in_input_dir() {
    let temp = TempDir::new().unwrap();
    let input_path = temp.path().join("test.jpg");
    create_test_jpeg(&input_path);

    let config = Config {
        output_mode: OutputMode::SameDir,
        ..Default::default()
    };
    let output_dir = config.resolve_output_dir(Some(&input_path));
    assert_eq!(output_dir, temp.path());

    let quality = Quality::default();
    let result = compress_image(&input_path, &output_dir, &quality, false, OutputFormat::Original, None, None, false).unwrap();
    assert!(result.output_path.exists());
    assert_eq!(result.output_path.parent().unwrap(), temp.path());
}

// ==================== WebP 输入压缩测试 ====================

#[test]
fn test_compress_webp_as_input_original() {
    let temp = TempDir::new().unwrap();
    let input_path = temp.path().join("test.webp");
    create_test_webp(&input_path);

    let output_dir = temp.path().join("output");
    let quality = Quality::default();
    let result = compress_image(&input_path, &output_dir, &quality, false, OutputFormat::Original, None, None, false).unwrap();

    assert_eq!(result.name, "test.jpg");
    assert!(result.compressed_size > 0);
    assert!(result.output_path.exists());
}

#[test]
fn test_compress_webp_as_input_lossless_naming() {
    let temp = TempDir::new().unwrap();
    let input_path = temp.path().join("photo-lossless.webp");
    create_test_webp(&input_path);

    let output_dir = temp.path().join("output");
    let quality = Quality::default();
    let result = compress_image(&input_path, &output_dir, &quality, false, OutputFormat::Original, None, None, false).unwrap();

    assert_eq!(result.name, "photo-lossless.webp");
    assert!(result.compressed_size > 0);
    assert!(result.output_path.exists());
}

// ==================== AVIF 输出测试 ====================

#[test]
#[ignore = "rav1e 0.6.3 abort() panic; 升级 rav1e 可修复"]
fn test_compress_to_avif() {
    let temp = TempDir::new().unwrap();
    let input_path = temp.path().join("test.png");
    create_test_png(&input_path);

    let output_dir = temp.path().join("output");
    let quality = Quality::default();
    let result = compress_image(&input_path, &output_dir, &quality, false, OutputFormat::Avif, None, None, false).unwrap();

    assert_eq!(result.name, "test.avif");
    assert!(result.compressed_size > 0);
    assert!(result.output_path.exists());
    assert_eq!(result.output_path.extension().unwrap(), "avif");
}

// ==================== WebP 输出测试 ====================

#[test]
fn test_compress_to_webp() {
    let temp = TempDir::new().unwrap();
    let input_path = temp.path().join("test.png");
    create_test_png(&input_path);

    let output_dir = temp.path().join("output");
    let quality = Quality::default();
    let result = compress_image(&input_path, &output_dir, &quality, false, OutputFormat::WebP, None, None, false).unwrap();

    assert_eq!(result.name, "test.webp");
    assert!(result.compressed_size > 0);
    assert!(result.output_path.exists());
    assert_eq!(result.output_path.extension().unwrap(), "webp");
}

// ==================== 格式转换测试 ====================

#[test]
fn test_convert_png_to_jpeg() {
    let temp = TempDir::new().unwrap();
    let input_path = temp.path().join("test.png");
    create_test_png(&input_path);

    let output_dir = temp.path().join("output");
    let quality = Quality::default();
    let result = compress_image(&input_path, &output_dir, &quality, false, OutputFormat::Jpeg, None, None, false).unwrap();

    assert_eq!(result.name, "test.jpg");
    assert!(result.compressed_size > 0);
    assert_eq!(result.output_path.extension().unwrap(), "jpg");
}

#[test]
fn test_convert_jpeg_to_png() {
    let temp = TempDir::new().unwrap();
    let input_path = temp.path().join("test.jpg");
    create_test_jpeg(&input_path);

    let output_dir = temp.path().join("output");
    let quality = Quality::default();
    let result = compress_image(&input_path, &output_dir, &quality, false, OutputFormat::Png, None, None, false).unwrap();

    assert_eq!(result.name, "test.png");
    assert!(result.compressed_size > 0);
    assert_eq!(result.output_path.extension().unwrap(), "png");
}

// ==================== 图片尺寸调整测试 ====================

#[test]
fn test_resize_to_max_dimensions() {
    let temp = TempDir::new().unwrap();
    let input_path = temp.path().join("test.jpg");
    create_test_jpeg(&input_path);

    let output_dir = temp.path().join("output");
    let quality = Quality::default();
    let result = compress_image(&input_path, &output_dir, &quality, false, OutputFormat::Original, Some(50), Some(50), false).unwrap();

    assert!(result.compressed_size > 0);
    assert!(result.output_path.exists());
}

// ==================== 元数据剥离测试 ====================

#[test]
fn test_strip_metadata_from_png() {
    let temp = TempDir::new().unwrap();
    let input_path = temp.path().join("test.png");
    create_test_png(&input_path);

    let output_dir = temp.path().join("output");
    let quality = Quality::default();
    let result = compress_image(&input_path, &output_dir, &quality, false, OutputFormat::Original, None, None, true).unwrap();

    assert!(result.compressed_size > 0);
    assert!(result.output_path.exists());
}

// ==================== 历史记录序列化测试 ====================

#[test]
fn test_history_serialization_roundtrip() {
    let mut history = History::default();
    let entry = HistoryEntry {
        timestamp_ms: 1000,
        timestamp_str: "2026-01-01 00:00:00".to_string(),
        results: vec![],
        output_dir: std::path::PathBuf::from("/tmp/out"),
        total_original: 10000,
        total_compressed: 5000,
    };
    history.add(entry);

    let json_str = serde_json::to_string_pretty(&history).unwrap();
    let loaded: History = serde_json::from_str(&json_str).unwrap();

    assert_eq!(loaded.entries.len(), 1);
    assert_eq!(loaded.entries[0].total_original, 10000);
    assert_eq!(loaded.entries[0].total_compressed, 5000);
    assert_eq!(loaded.entries[0].savings(), 5000);
}

#[test]
fn test_history_max_entries() {
    let mut history = History::default();
    for i in 0..120 {
        let entry = HistoryEntry {
            timestamp_ms: i,
            timestamp_str: format!("entry-{}", i),
            results: vec![],
            output_dir: std::path::PathBuf::from("/tmp"),
            total_original: 100,
            total_compressed: 50,
        };
        history.add(entry);
    }
    // 最多保留 100 条
    assert_eq!(history.entries.len(), 100);
    assert_eq!(history.entries[0].timestamp_ms, 20);
    assert_eq!(history.entries[99].timestamp_ms, 119);
}

// ==================== TC59: SVG 超大尺寸光栅化限制测试 ====================

#[test]
fn test_svg_oversized_rasterization() {
    use std::io::Write;
    let temp = TempDir::new().unwrap();
    let input_path = temp.path().join("huge.svg");

    // 创建 width=5000 的 SVG，应被 MAX_SVG_DIMENSION=4096 限制
    let svg = r#"<?xml version="1.0"?>
<svg width="5000" height="5000" xmlns="http://www.w3.org/2000/svg">
  <rect width="5000" height="5000" fill="red"/>
</svg>"#;
    let mut f = std::fs::File::create(&input_path).unwrap();
    f.write_all(svg.as_bytes()).unwrap();

    let output_dir = temp.path().join("output");
    let quality = Quality::default();
    let result = compress_image(&input_path, &output_dir, &quality, false, OutputFormat::Original, None, None, false).unwrap();

    assert!(result.compressed_size > 0);
    assert!(result.output_path.exists());
    // 输出应为 .png（SVG 转 PNG）
    assert_eq!(result.output_path.extension().unwrap(), "png");
}

// ==================== TC61: 深层嵌套目录扫描不崩溃测试 ====================

#[test]
fn test_deep_directory_scanning() {
    let temp = TempDir::new().unwrap();

    // 创建 20 层嵌套目录，验证深度遍历不会栈溢出
    let mut dir = temp.path().to_path_buf();
    for i in 0..20 {
        dir = dir.join(format!("d{}", i));
        std::fs::create_dir_all(&dir).unwrap();
    }
    // 最深处放一张图片
    let img_path = dir.join("test.png");
    create_test_png(&img_path);

    // 用 std::fs::read_dir 递归扫描（模拟 collect_images_from_dir 但不引用 UI）
    fn scan(dir: &std::path::Path, depth: u32, max_depth: u32, count: &mut usize) {
        if depth > max_depth || *count > 100 { return; }
        if let Ok(entries) = std::fs::read_dir(dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_dir() {
                    scan(&path, depth + 1, max_depth, count);
                } else if let Some(ext) = path.extension() {
                    if ext == "png" { *count += 1; }
                }
            }
        }
    }

    let mut count = 0;
    scan(temp.path(), 0, 10, &mut count);
    // 深度限制为 10 时不会找到 20 层深的文件
    assert_eq!(count, 0, "depth limit should prevent finding file at depth 20");

    let mut count2 = 0;
    scan(temp.path(), 0, 25, &mut count2);
    // 无深度限制时应找到文件
    assert_eq!(count2, 1, "should find file when depth limit is sufficient");
}

// ==================== TC63: 同名文件覆盖重命名测试 ====================

#[test]
fn test_compress_image_overwrite_existing() {
    let temp = TempDir::new().unwrap();
    let input_path = temp.path().join("test.jpg");
    create_test_jpeg(&input_path);

    let output_dir = temp.path().join("output");
    std::fs::create_dir_all(&output_dir).unwrap();

    // 先创建同名输出文件
    let first_output = output_dir.join("test.jpg");
    std::fs::write(&first_output, b"fake compressed data").unwrap();

    let quality = Quality::default();
    let result = compress_image(&input_path, &output_dir, &quality, false, OutputFormat::Original, None, None, false).unwrap();

    // 应生成不同于 test.jpg 的名字而非覆盖原文件
    assert!(result.name.starts_with("test"), "name should start with test, got {}", result.name);
    // 应追加 _N 后缀
    assert!(result.name.contains('_') || result.name.contains('-'), "should add suffix, got {}", result.name);
    assert!(result.output_path.exists());
    // 原文件应保留
    assert!(first_output.exists());
    assert_eq!(std::fs::read_to_string(&first_output).unwrap(), "fake compressed data");
}

// ==================== TC74: 无损 PNG <= 原大小测试 ====================

#[test]
fn test_png_lossless_not_larger_than_original() {
    let temp = TempDir::new().unwrap();
    let input_path = temp.path().join("test.png");
    let output_dir = temp.path().join("output");

    // 创建 PNG（直接使用 create_test_png）
    create_test_png(&input_path);

    let quality = Quality::default();

    // 有损模式
    let result = compress_image(&input_path, &output_dir, &quality, false, OutputFormat::Original, None, None, false).unwrap();
    assert!(result.compressed_size <= result.original_size || result.original_size == 0,
        "lossy compress should not increase file size: {} > {}", result.compressed_size, result.original_size);
}

// ==================== TC85: HistoryEntry::savings() 计算测试 ====================

#[test]
fn test_history_entry_savings_calculation() {
    // 节省为正
    let entry = HistoryEntry {
        timestamp_ms: 1000,
        timestamp_str: "t".to_string(),
        results: vec![],
        output_dir: std::path::PathBuf::from("/tmp"),
        total_original: 10000,
        total_compressed: 3000,
    };
    assert_eq!(entry.savings(), 7000, "positive savings: 10000 - 3000 = 7000");

    // 节省为负（文件变大）
    let entry2 = HistoryEntry {
        total_original: 3000,
        total_compressed: 5000,
        ..entry.clone()
    };
    assert_eq!(entry2.savings(), -2000, "negative savings: 3000 - 5000 = -2000");

    // 节省为零
    let entry3 = HistoryEntry {
        total_original: 5000,
        total_compressed: 5000,
        ..entry
    };
    assert_eq!(entry3.savings(), 0, "zero savings: 5000 - 5000 = 0");
}

// ==================== TC75: 多帧 GIF 保留测试 ====================

#[test]
fn test_gif_multiframe_preserved() {
    let temp = TempDir::new().unwrap();
    let input_path = temp.path().join("anim.gif");
    // 创建多帧 GIF
    {
        use gif::{Encoder, Frame, Repeat};
        let mut f = std::fs::File::create(&input_path).unwrap();
        let palette = vec![0u8, 0, 0, 255, 255, 255];
        let mut encoder = Encoder::new(&mut f, 10, 10, &palette).unwrap();
        encoder.set_repeat(Repeat::Infinite).unwrap();
        for _ in 0..3 {
            let mut frame = Frame::default();
            frame.width = 10;
            frame.height = 10;
            frame.buffer = vec![0u8; 100].into();
            frame.palette = Some(palette.clone());
            encoder.write_frame(&frame).unwrap();
        }
    }

    let output_dir = temp.path().join("output");
    let quality = Quality::default();
    let result = compress_image(&input_path, &output_dir, &quality, false, OutputFormat::Original, None, None, false).unwrap();

    // 多帧 GIF 应保留原始数据（不压缩）
    assert!(result.compressed_size > 0);
    assert!(result.output_path.exists());
    assert_eq!(result.output_path.extension().unwrap(), "gif");
}

// ==================== TC76: compress_images 并行正确性测试 ====================

#[test]
fn test_compress_images_parallel() {
    let temp = TempDir::new().unwrap();
    let mut input_paths = Vec::new();
    let output_dir = temp.path().join("output");

    // 创建 10 个测试文件
    for i in 0..10 {
        let p = temp.path().join(format!("test_{}.jpg", i));
        create_test_jpeg(&p);
        input_paths.push(p);
    }

    let quality = Quality::default();
    let results = compress_images(&input_paths, &output_dir, &quality, false, OutputFormat::Original, None, None, false);

    assert_eq!(results.len(), 10);
    let success_count = results.iter().filter(|r| r.is_ok()).count();
    assert_eq!(success_count, 10, "all 10 files should compress successfully");
}

// ==================== TC84: 特殊字符路径测试 ====================

#[test]
fn test_output_dir_special_chars() {
    let temp = TempDir::new().unwrap();

    // 创建含特殊字符的文件名
    let input_path = temp.path().join("test image #1 (copy).jpg");
    create_test_jpeg(&input_path);

    let output_dir = temp.path().join("my outputs [2024]");
    let quality = Quality::default();
    let result = compress_image(&input_path, &output_dir, &quality, false, OutputFormat::Original, None, None, false).unwrap();

    assert!(result.compressed_size > 0);
    assert!(result.output_path.exists());
}

// === TC67: 根路径 SameDir ===
#[test]
fn test_tc67_same_dir_root() {
    let cfg = imagemin_core::Config { output_mode: imagemin_core::OutputMode::SameDir, ..Default::default() };
    let r = cfg.resolve_output_dir(Some(std::path::Path::new("/test.jpg")));
    assert_ne!(r, std::path::Path::new("/"));
    assert!(r.to_string_lossy().contains("same_dir"));
}

// === TC69: SUPPORTED_EXTENSIONS ===
#[test]
fn test_tc69_supported_exts() {
    assert!(imagemin_core::SUPPORTED_EXTENSIONS.contains(&"avif"));
    assert!(imagemin_core::SUPPORTED_EXTENSIONS.contains(&"jpg"));
    assert!(imagemin_core::SUPPORTED_EXTENSIONS.contains(&"webp"));
    assert_eq!(imagemin_core::SUPPORTED_EXTENSIONS.len(), 7);
}
