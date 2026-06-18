use imagemin_core::{compress_image, Config, Quality};
use std::path::PathBuf;

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
    let img: ImageBuffer<Rgb<u8>, Vec<u8>> = ImageBuffer::from_fn(2, 2, |_x, _y| {
        Rgb([255, 0, 0])
    });
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
    let temp_dir = std::env::temp_dir().join("test_jpeg_basic");
    std::fs::create_dir_all(&temp_dir).unwrap();

    let input_path = temp_dir.join("test.jpg");
    create_test_jpeg(&input_path);

    let output_dir = temp_dir.join("output");
    std::fs::create_dir_all(&output_dir).unwrap();

    let quality = Quality::default();
    let result = compress_image(&input_path, &output_dir, &quality).unwrap();

    assert_eq!(result.name, "test.jpg");
    assert!(result.original_size > 0);
    assert!(result.compressed_size > 0);
    assert!(result.output_path.exists());
    assert!(result.compressed_size <= result.original_size);

    std::fs::remove_dir_all(&temp_dir).unwrap();
}

#[test]
fn test_compress_jpeg_low_quality() {
    let temp_dir = std::env::temp_dir().join("test_jpeg_low");
    std::fs::create_dir_all(&temp_dir).unwrap();

    let input_path = temp_dir.join("test.jpg");
    create_test_jpeg(&input_path);

    let output_dir = temp_dir.join("output");
    std::fs::create_dir_all(&output_dir).unwrap();

    let quality = Quality { jpeg: 10, png: 80 };
    let result = compress_image(&input_path, &output_dir, &quality).unwrap();

    assert!(result.compressed_size < result.original_size);

    std::fs::remove_dir_all(&temp_dir).unwrap();
}

#[test]
fn test_compress_jpeg_high_quality() {
    let temp_dir = std::env::temp_dir().join("test_jpeg_high");
    std::fs::create_dir_all(&temp_dir).unwrap();

    let input_path = temp_dir.join("test.jpg");
    create_test_jpeg(&input_path);

    let output_dir = temp_dir.join("output");
    std::fs::create_dir_all(&output_dir).unwrap();

    let quality = Quality { jpeg: 95, png: 80 };
    let result = compress_image(&input_path, &output_dir, &quality);
    assert!(result.is_ok());
    assert!(result.unwrap().compressed_size > 0);

    std::fs::remove_dir_all(&temp_dir).unwrap();
}

#[test]
fn test_compress_png_basic() {
    let temp_dir = std::env::temp_dir().join("test_png_basic");
    std::fs::create_dir_all(&temp_dir).unwrap();

    let input_path = temp_dir.join("test.png");
    create_test_png(&input_path);

    let output_dir = temp_dir.join("output");
    std::fs::create_dir_all(&output_dir).unwrap();

    let quality = Quality::default();
    let result = compress_image(&input_path, &output_dir, &quality).unwrap();

    assert_eq!(result.name, "test.png");
    assert!(result.original_size > 0);
    assert!(result.compressed_size > 0);
    assert!(result.output_path.exists());

    std::fs::remove_dir_all(&temp_dir).unwrap();
}

#[test]
fn test_compress_png_grayscale() {
    let temp_dir = std::env::temp_dir().join("test_png_gray");
    std::fs::create_dir_all(&temp_dir).unwrap();

    let input_path = temp_dir.join("test.png");
    create_grayscale_png(&input_path);

    let output_dir = temp_dir.join("output");
    std::fs::create_dir_all(&output_dir).unwrap();

    let quality = Quality::default();
    let result = compress_image(&input_path, &output_dir, &quality).unwrap();

    assert!(result.compressed_size > 0);

    std::fs::remove_dir_all(&temp_dir).unwrap();
}

#[test]
fn test_compress_png_transparent() {
    let temp_dir = std::env::temp_dir().join("test_png_trans");
    std::fs::create_dir_all(&temp_dir).unwrap();

    let input_path = temp_dir.join("test.png");
    create_transparent_png(&input_path);

    let output_dir = temp_dir.join("output");
    std::fs::create_dir_all(&output_dir).unwrap();

    let quality = Quality::default();
    let result = compress_image(&input_path, &output_dir, &quality).unwrap();

    assert!(result.compressed_size > 0);

    std::fs::remove_dir_all(&temp_dir).unwrap();
}

#[test]
fn test_compress_gif_keeps_size() {
    let temp_dir = std::env::temp_dir().join("test_gif");
    std::fs::create_dir_all(&temp_dir).unwrap();

    let input_path = temp_dir.join("test.gif");
    create_test_gif(&input_path);

    let output_dir = temp_dir.join("output");
    std::fs::create_dir_all(&output_dir).unwrap();

    let quality = Quality::default();
    let result = compress_image(&input_path, &output_dir, &quality).unwrap();

    assert_eq!(result.original_size, result.compressed_size);

    std::fs::remove_dir_all(&temp_dir).unwrap();
}

#[test]
fn test_compress_svg_keeps_content() {
    let temp_dir = std::env::temp_dir().join("test_svg");
    std::fs::create_dir_all(&temp_dir).unwrap();

    let input_path = temp_dir.join("test.svg");
    create_svg(&input_path);

    let output_dir = temp_dir.join("output");
    std::fs::create_dir_all(&output_dir).unwrap();

    let quality = Quality::default();
    let result = compress_image(&input_path, &output_dir, &quality).unwrap();

    assert_eq!(result.original_size, result.compressed_size);

    let original = std::fs::read_to_string(&input_path).unwrap();
    let compressed = std::fs::read_to_string(&result.output_path).unwrap();
    assert_eq!(original, compressed);

    std::fs::remove_dir_all(&temp_dir).unwrap();
}

#[test]
fn test_compress_large_image() {
    let temp_dir = std::env::temp_dir().join("test_large_img");
    let _ = std::fs::remove_dir_all(&temp_dir);
    std::fs::create_dir_all(&temp_dir).unwrap();

    let input_path = temp_dir.join("large.png");
    create_test_png(&input_path);

    assert!(input_path.exists(), "Input file should exist");

    let output_dir = temp_dir.join("output");
    std::fs::create_dir_all(&output_dir).unwrap();

    let quality = Quality { jpeg: 50, png: 50 };
    let result = compress_image(&input_path, &output_dir, &quality);

    assert!(result.is_ok(), "Compression should succeed: {:?}", result.err());
    let result = result.unwrap();
    assert!(result.original_size > 0);
    assert!(result.compressed_size > 0);

    let _ = std::fs::remove_dir_all(&temp_dir);
}

#[test]
fn test_compress_small_image() {
    let temp_dir = std::env::temp_dir().join("test_small");
    std::fs::create_dir_all(&temp_dir).unwrap();

    let input_path = temp_dir.join("small.jpg");
    create_small_jpeg(&input_path);

    let output_dir = temp_dir.join("output");
    std::fs::create_dir_all(&output_dir).unwrap();

    let quality = Quality::default();
    let result = compress_image(&input_path, &output_dir, &quality).unwrap();

    assert!(result.compressed_size > 0);

    std::fs::remove_dir_all(&temp_dir).unwrap();
}

#[test]
fn test_compress_unsupported_bmp() {
    let temp_dir = std::env::temp_dir().join("test_bmp");
    std::fs::create_dir_all(&temp_dir).unwrap();

    let input_path = temp_dir.join("test.bmp");
    std::fs::write(&input_path, b"fake bmp data").unwrap();

    let output_dir = temp_dir.join("output");
    std::fs::create_dir_all(&output_dir).unwrap();

    let quality = Quality::default();
    let result = compress_image(&input_path, &output_dir, &quality);

    assert!(result.is_err());

    std::fs::remove_dir_all(&temp_dir).unwrap();
}

#[test]
fn test_compress_unsupported_webp() {
    let temp_dir = std::env::temp_dir().join("test_webp");
    std::fs::create_dir_all(&temp_dir).unwrap();

    let input_path = temp_dir.join("test.webp");
    std::fs::write(&input_path, b"fake webp data").unwrap();

    let output_dir = temp_dir.join("output");
    std::fs::create_dir_all(&output_dir).unwrap();

    let quality = Quality::default();
    let result = compress_image(&input_path, &output_dir, &quality);

    assert!(result.is_err());

    std::fs::remove_dir_all(&temp_dir).unwrap();
}

#[test]
fn test_compress_nonexistent_file() {
    let temp_dir = std::env::temp_dir().join("test_nonexist");
    std::fs::create_dir_all(&temp_dir).unwrap();

    let input_path = temp_dir.join("nonexistent.jpg");
    let output_dir = temp_dir.join("output");
    std::fs::create_dir_all(&output_dir).unwrap();

    let quality = Quality::default();
    let result = compress_image(&input_path, &output_dir, &quality);

    assert!(result.is_err());

    std::fs::remove_dir_all(&temp_dir).unwrap();
}

#[test]
fn test_compress_empty_file() {
    let temp_dir = std::env::temp_dir().join("test_empty");
    std::fs::create_dir_all(&temp_dir).unwrap();

    let input_path = temp_dir.join("empty.jpg");
    std::fs::write(&input_path, b"").unwrap();

    let output_dir = temp_dir.join("output");
    std::fs::create_dir_all(&output_dir).unwrap();

    let quality = Quality::default();
    let result = compress_image(&input_path, &output_dir, &quality);

    assert!(result.is_err());

    std::fs::remove_dir_all(&temp_dir).unwrap();
}

#[test]
fn test_compress_corrupted_jpeg() {
    let temp_dir = std::env::temp_dir().join("test_corrupt");
    std::fs::create_dir_all(&temp_dir).unwrap();

    let input_path = temp_dir.join("corrupt.jpg");
    std::fs::write(&input_path, b"not a real jpeg file content").unwrap();

    let output_dir = temp_dir.join("output");
    std::fs::create_dir_all(&output_dir).unwrap();

    let quality = Quality::default();
    let result = compress_image(&input_path, &output_dir, &quality);

    assert!(result.is_err());

    std::fs::remove_dir_all(&temp_dir).unwrap();
}

#[test]
fn test_compress_quality_comparison() {
    let temp_dir = std::env::temp_dir().join("test_quality_cmp");
    std::fs::create_dir_all(&temp_dir).unwrap();

    let input_path = temp_dir.join("test.jpg");
    create_test_jpeg(&input_path);

    let quality_low = Quality { jpeg: 10, png: 80 };
    let quality_mid = Quality { jpeg: 50, png: 80 };
    let quality_high = Quality { jpeg: 90, png: 80 };

    let out_low = temp_dir.join("low");
    let out_mid = temp_dir.join("mid");
    let out_high = temp_dir.join("high");
    std::fs::create_dir_all(&out_low).unwrap();
    std::fs::create_dir_all(&out_mid).unwrap();
    std::fs::create_dir_all(&out_high).unwrap();

    let r_low = compress_image(&input_path, &out_low, &quality_low).unwrap();
    let r_mid = compress_image(&input_path, &out_mid, &quality_mid).unwrap();
    let r_high = compress_image(&input_path, &out_high, &quality_high).unwrap();

    assert!(r_low.compressed_size <= r_mid.compressed_size);
    assert!(r_mid.compressed_size <= r_high.compressed_size);

    std::fs::remove_dir_all(&temp_dir).unwrap();
}

#[test]
fn test_compress_output_filename_preserved() {
    let temp_dir = std::env::temp_dir().join("test_filename");
    std::fs::create_dir_all(&temp_dir).unwrap();

    let input_path = temp_dir.join("my_photo_2024.jpg");
    create_test_jpeg(&input_path);

    let output_dir = temp_dir.join("output");
    std::fs::create_dir_all(&output_dir).unwrap();

    let quality = Quality::default();
    let result = compress_image(&input_path, &output_dir, &quality).unwrap();

    assert_eq!(result.name, "my_photo_2024.jpg");
    assert!(result.output_path.ends_with("my_photo_2024.jpg"));

    std::fs::remove_dir_all(&temp_dir).unwrap();
}

#[test]
fn test_compress_chinese_filename() {
    let temp_dir = std::env::temp_dir().join("test_chinese");
    std::fs::create_dir_all(&temp_dir).unwrap();

    let input_path = temp_dir.join("测试图片.jpg");
    create_test_jpeg(&input_path);

    let output_dir = temp_dir.join("output");
    std::fs::create_dir_all(&output_dir).unwrap();

    let quality = Quality::default();
    let result = compress_image(&input_path, &output_dir, &quality).unwrap();

    assert_eq!(result.name, "测试图片.jpg");
    assert!(result.output_path.exists());

    std::fs::remove_dir_all(&temp_dir).unwrap();
}

// ==================== 配置功能测试 ====================

#[test]
fn test_config_default_values() {
    let config = Config::default();
    assert_eq!(config.quality.jpeg, 80);
    assert_eq!(config.quality.png, 80);
}

#[test]
fn test_config_serialization_roundtrip() {
    let config = Config {
        quality: Quality { jpeg: 75, png: 60 },
    };

    let toml_str = toml::to_string(&config).unwrap();
    let loaded: Config = toml::from_str(&toml_str).unwrap();

    assert_eq!(loaded.quality.jpeg, 75);
    assert_eq!(loaded.quality.png, 60);
}

#[test]
fn test_config_partial_toml() {
    let toml_str = r#"
[quality]
jpeg = 50
"#;

    let result: Result<Config, _> = toml::from_str(toml_str);
    assert!(result.is_err());
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
    let q_min = Quality { jpeg: 0, png: 21 };
    let q_max = Quality { jpeg: 100, png: 100 };

    assert_eq!(q_min.jpeg, 0);
    assert_eq!(q_min.png, 21);
    assert_eq!(q_max.jpeg, 100);
    assert_eq!(q_max.png, 100);
}

#[test]
fn test_config_output_dir() {
    let dir = Config::output_dir();
    assert!(dir.to_string_lossy().contains("retrocode_io"));
    assert!(dir.to_string_lossy().contains("imagemin"));
}

#[test]
fn test_config_save_and_load() {
    let temp_config_dir = std::env::temp_dir().join("imagemin_config_test");
    std::fs::create_dir_all(&temp_config_dir).unwrap();

    let config = Config {
        quality: Quality { jpeg: 65, png: 75 },
    };

    let toml_content = toml::to_string_pretty(&config).unwrap();
    let config_path = temp_config_dir.join("config.toml");
    std::fs::write(&config_path, &toml_content).unwrap();

    let loaded_content = std::fs::read_to_string(&config_path).unwrap();
    let loaded: Config = toml::from_str(&loaded_content).unwrap();

    assert_eq!(loaded.quality.jpeg, 65);
    assert_eq!(loaded.quality.png, 75);

    std::fs::remove_dir_all(&temp_config_dir).unwrap();
}

// ==================== 文件扩展名测试 ====================

#[test]
fn test_compress_jpg_extension() {
    let temp_dir = std::env::temp_dir().join("test_ext_jpg");
    std::fs::create_dir_all(&temp_dir).unwrap();

    let input_path = temp_dir.join("test.jpg");
    create_test_jpeg(&input_path);

    let output_dir = temp_dir.join("output");
    std::fs::create_dir_all(&output_dir).unwrap();

    let quality = Quality::default();
    let result = compress_image(&input_path, &output_dir, &quality);
    assert!(result.is_ok());

    std::fs::remove_dir_all(&temp_dir).unwrap();
}

#[test]
fn test_compress_jpeg_extension() {
    let temp_dir = std::env::temp_dir().join("test_ext_jpeg");
    std::fs::create_dir_all(&temp_dir).unwrap();

    let input_path = temp_dir.join("test.jpeg");
    create_test_jpeg(&input_path);

    let output_dir = temp_dir.join("output");
    std::fs::create_dir_all(&output_dir).unwrap();

    let quality = Quality::default();
    let result = compress_image(&input_path, &output_dir, &quality);
    assert!(result.is_ok());

    std::fs::remove_dir_all(&temp_dir).unwrap();
}

#[test]
fn test_compress_uppercase_extension() {
    let temp_dir = std::env::temp_dir().join("test_ext_upper");
    std::fs::create_dir_all(&temp_dir).unwrap();

    let input_path = temp_dir.join("test.JPG");
    create_test_jpeg(&input_path);

    let output_dir = temp_dir.join("output");
    std::fs::create_dir_all(&output_dir).unwrap();

    let quality = Quality::default();
    let result = compress_image(&input_path, &output_dir, &quality);
    assert!(result.is_ok());

    std::fs::remove_dir_all(&temp_dir).unwrap();
}

#[test]
fn test_compress_mixed_case_extension() {
    let temp_dir = std::env::temp_dir().join("test_ext_mixed");
    std::fs::create_dir_all(&temp_dir).unwrap();

    let input_path = temp_dir.join("test.Png");
    create_test_png(&input_path);

    let output_dir = temp_dir.join("output");
    std::fs::create_dir_all(&output_dir).unwrap();

    let quality = Quality::default();
    let result = compress_image(&input_path, &output_dir, &quality);
    assert!(result.is_ok());

    std::fs::remove_dir_all(&temp_dir).unwrap();
}

// ==================== 批量压缩测试 ====================

#[test]
fn test_batch_compress_multiple_files() {
    let temp_dir = std::env::temp_dir().join("test_batch");
    std::fs::create_dir_all(&temp_dir).unwrap();

    create_test_jpeg(&temp_dir.join("1.jpg"));
    create_test_png(&temp_dir.join("2.png"));
    create_test_gif(&temp_dir.join("3.gif"));

    let output_dir = temp_dir.join("output");
    std::fs::create_dir_all(&output_dir).unwrap();

    let quality = Quality::default();

    let r1 = compress_image(&temp_dir.join("1.jpg"), &output_dir, &quality).unwrap();
    let r2 = compress_image(&temp_dir.join("2.png"), &output_dir, &quality).unwrap();
    let r3 = compress_image(&temp_dir.join("3.gif"), &output_dir, &quality).unwrap();

    assert!(r1.compressed_size > 0);
    assert!(r2.compressed_size > 0);
    assert!(r3.compressed_size > 0);

    std::fs::remove_dir_all(&temp_dir).unwrap();
}

#[test]
fn test_compress_to_same_directory() {
    let temp_dir = std::env::temp_dir().join("test_same_dir");
    std::fs::create_dir_all(&temp_dir).unwrap();

    create_test_jpeg(&temp_dir.join("test.jpg"));

    let quality = Quality::default();
    let result = compress_image(&temp_dir.join("test.jpg"), &temp_dir, &quality).unwrap();

    assert!(result.output_path.exists());

    std::fs::remove_dir_all(&temp_dir).unwrap();
}
