# cvooc-imagemin-compressor

基于 Rust 和 iced 框架开发的本地图片压缩工具，替代 Electron 版本，显著减小打包体积。

## 功能

- 支持 JPG/PNG/GIF/SVG 格式
- 批量压缩（使用 rayon 并行处理）
- 可调节压缩质量 (JPEG 0-100, PNG 21-100)
- 拖放文件支持
- 点击选择文件
- 无边框窗口，可拖动
- 中文界面
- 压缩完成后打开输出目录

## 体积对比

| 版本 | 大小 |
|------|------|
| Electron 版本 | ~200 MB |
| **Rust 版本** | **~26 MB** |

## 构建

```bash
# 运行测试
.\test.ps1

# 构建发布版本
.\build.ps1
```

或手动执行：

```bash
cargo test
cargo build --release
```

## 运行

```bash
cargo run --release
```

## 项目结构

```
cvooc-imagemin-compressor/
├── Cargo.toml              # 工作空间配置
├── build.ps1               # 构建脚本
├── test.ps1                # 测试脚本
├── assets/
│   ├── icon.ico            # 应用图标
│   └── test.png            # 测试图片
├── crates/
│   ├── core/               # 核心压缩逻辑
│   │   ├── Cargo.toml
│   │   ├── src/
│   │   │   ├── lib.rs      # 模块导出
│   │   │   ├── compress.rs # 压缩算法 (mozjpeg + imagequant + oxipng)
│   │   │   └── config.rs   # 配置管理
│   │   └── tests/
│   │       └── integration.rs  # 集成测试 (31 个)
│   └── ui/                 # iced UI 界面
│       ├── Cargo.toml
│       ├── build.rs        # 构建脚本（嵌入图标）
│       ├── icon.rc         # Windows 资源文件
│       └── src/
│           ├── main.rs     # 入口
│           ├── app.rs      # 应用状态和消息处理
│           └── views/      # UI 组件
│               ├── mod.rs
│               ├── header.rs      # 标题栏
│               ├── drop_zone.rs   # 拖放区域
│               ├── progress.rs    # 进度显示
│               ├── result_table.rs # 结果表格
│               └── settings.rs    # 设置对话框
└── dist/
    └── cvooc-imagemin-compressor.exe  # 发布文件
```

## 技术栈

| 组件 | 技术 |
|------|------|
| UI 框架 | iced 0.12 |
| JPEG 压缩 | mozjpeg（渐进式编码） |
| PNG 压缩 | imagequant（有损） + oxipng（无损优化） |
| 并行处理 | rayon |
| 配置管理 | serde + toml |
| 文件对话框 | rfd |
| 系统调用 | open |
| 图标嵌入 | embed-resource |

## 测试用例

### 单元测试 (6 个)

| 测试 | 描述 |
|------|------|
| `test_compress_jpeg_invalid` | 无效 JPEG 数据返回错误 |
| `test_compress_png_invalid` | 无效 PNG 数据返回错误 |
| `test_unsupported_format` | 不支持的格式返回错误 |
| `test_default_config` | 默认配置值正确 |
| `test_config_serialization` | 配置序列化/反序列化 |
| `test_output_dir` | 输出目录路径正确 |

### 集成测试 (31 个)

#### JPEG 压缩测试 (5 个)

| 测试 | 描述 |
|------|------|
| `test_compress_jpeg_basic` | 基本 JPEG 压缩 |
| `test_compress_jpeg_low_quality` | 低质量压缩（更小文件） |
| `test_compress_jpeg_high_quality` | 高质量压缩 |
| `test_compress_jpg_extension` | .jpg 扩展名支持 |
| `test_compress_jpeg_extension` | .jpeg 扩展名支持 |

#### PNG 压缩测试 (4 个)

| 测试 | 描述 |
|------|------|
| `test_compress_png_basic` | 基本 PNG 压缩 |
| `test_compress_png_grayscale` | 灰度 PNG 压缩 |
| `test_compress_png_transparent` | 透明 PNG 压缩 |
| `test_compress_large_image` | 大图片压缩 |

#### GIF/SVG 测试 (2 个)

| 测试 | 描述 |
|------|------|
| `test_compress_gif_keeps_size` | GIF 保持原样 |
| `test_compress_svg_keeps_content` | SVG 内容不变 |

#### 错误处理测试 (5 个)

| 测试 | 描述 |
|------|------|
| `test_compress_unsupported_bmp` | 不支持 BMP 格式 |
| `test_compress_unsupported_webp` | 不支持 WebP 格式 |
| `test_compress_nonexistent_file` | 不存在的文件 |
| `test_compress_empty_file` | 空文件 |
| `test_compress_corrupted_jpeg` | 损坏的 JPEG 文件 |

#### 配置测试 (6 个)

| 测试 | 描述 |
|------|------|
| `test_config_default_values` | 默认配置值 |
| `test_config_serialization_roundtrip` | 配置序列化往返 |
| `test_config_partial_toml` | 不完整的 TOML |
| `test_config_invalid_values` | 无效配置值 |
| `test_quality_boundary_values` | 质量边界值 |
| `test_config_output_dir` | 输出目录路径 |
| `test_config_save_and_load` | 配置保存和加载 |

#### 文件扩展名测试 (4 个)

| 测试 | 描述 |
|------|------|
| `test_compress_uppercase_extension` | 大写扩展名 (.JPG) |
| `test_compress_mixed_case_extension` | 混合大小写 (.Png) |
| `test_compress_output_filename_preserved` | 输出文件名保持 |
| `test_compress_chinese_filename` | 中文文件名 |

#### 批量处理测试 (2 个)

| 测试 | 描述 |
|------|------|
| `test_batch_compress_multiple_files` | 批量压缩多文件 |
| `test_compress_to_same_directory` | 压缩到同目录 |

#### 性能测试 (2 个)

| 测试 | 描述 |
|------|------|
| `test_compress_quality_comparison` | 不同质量压缩效果对比 |
| `test_compress_small_image` | 小图片压缩 |

## 输出目录

压缩后的文件保存在: `~/retrocode_io/imagemin/<时间戳>/`

## 配置文件

配置保存在: `~/.config/cvooc-imagemin-compressor/config.toml`

```toml
[quality]
jpeg = 80
png = 80
```

## 许可证

CC0-1.0
