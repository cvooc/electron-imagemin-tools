use imagemin_core::{compress_image, Config, OutputFormat, OutputMode, Quality};
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
    let q_min = Quality { jpeg: 0, png: 0 };
    let q_max = Quality { jpeg: 100, png: 100 };

    assert!(q_min.validate().is_ok());
    assert!(q_max.validate().is_ok());
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
