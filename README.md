# cvooc-imagemin-compressor

基于 Rust 和 iced 框架开发的本地图片压缩工具，替代 Electron 版本，显著减小打包体积。

## 功能

- 支持 JPG/PNG/GIF/SVG/WebP 格式
- **AVIF 输出**：可将任意格式转为 AVIF
- **格式转换**：支持输出为 JPEG、PNG、WebP、AVIF，或保持原格式
- **图片尺寸调整**：可设置最大宽/高，等比缩小
- **元数据剥离**：可移除 EXIF、XMP、ICC 等元数据
- 批量压缩（UI 层真正并行，rayon 核心并行）
- 可调节压缩质量（JPEG 0-100，PNG 0-100）
- PNG 可选纯无损优化或有损量化（默认有损，灰度/索引图自动保留原格式）
- 三种输出目录模式：时间戳子目录、与输入文件同目录、自定义目录
- **暗色模式**：支持跟随系统 / 浅色 / 深色三种主题
- **压缩历史**：自动记录每次压缩结果，可回溯查看
- **压缩预览**：结果页点击预览可对比原图和压缩后图片
- 拖放文件 / **拖放文件夹**支持（递归扫描图片）
- 快捷键：Esc 返回、Ctrl+O 选文件、Ctrl+R 重新压缩
- Toast 通知（压缩完成显示节省量）
- 确认对话框（清空列表前确认）
- 压缩失败在结果表格中显示原因
- 无边框窗口，可拖动
- 中文界面

## 体积对比

| 版本 | 大小 |
|------|------|
| Electron 版本 | ~200 MB |
| **Rust 版本** | **~9 MB** |

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
│   │   │   ├── compress.rs # 压缩算法 (mozjpeg + imagequant + oxipng + gif + svg + webp + avif)
│   │   │   ├── config.rs   # 配置管理
│   │   │   └── history.rs  # 压缩历史记录
│   │   └── tests/
│   │       └── integration.rs  # 集成测试
│   └── ui/                 # iced UI 界面
│       ├── Cargo.toml
│       ├── build.rs        # 构建脚本（嵌入图标）
│       ├── icon.rc         # Windows 资源文件
│       └── src/
│           ├── main.rs     # 入口（系统字体加载与 fallback）
│           ├── app.rs      # 应用状态、消息流、异步压缩调度
│           └── views/      # UI 组件
│               ├── mod.rs
│               ├── header.rs       # 标题栏
│               ├── drop_zone.rs    # 拖放区域
│               ├── progress.rs     # 进度条
│               ├── result_table.rs # 结果表格
│               ├── settings.rs     # 设置页
│               ├── history.rs      # 压缩历史
│               ├── modal.rs        # 确认对话框
│               ├── stack.rs        # Stack 叠放布局
│               └── toast.rs        # Toast 通知
└── dist/
    └── cvooc-imagemin-compressor.exe  # 发布文件
```

## 技术栈

| 组件 | 技术 |
|------|------|
| UI 框架 | iced 0.12 |
| JPEG 压缩 | mozjpeg（渐进式编码） |
| PNG 压缩 | imagequant（有损索引色） + oxipng（无损优化） |
| GIF 压缩 | imagequant 减少调色板（单帧）/ 保留动画 |
| SVG 处理 | resvg + tiny-skia 光栅化为 PNG |
| WebP 处理 | image crate 解码后转 JPEG/PNG |
| AVIF 编码 | ravif |
| 并行处理 | rayon |
| 异步任务 | tokio |
| 配置管理 | serde + toml |
| 历史记录 | serde_json |
| 文件对话框 | rfd |
| 系统主题 | dark-light |
| 系统调用 | open |
| 图标嵌入 | embed-resource |

## 输出目录

- **时间戳子目录**（默认）：`~/retrocode_io/imagemin/<时间戳>/`
- **与输入文件同目录**：每个文件输出到各自所在的目录
- **自定义目录**：用户在设置页选择

## 配置文件

配置保存在: `~/.config/cvooc-imagemin-compressor/config.toml`

```toml
[quality]
jpeg = 80
png = 80

output_mode = "timestamped"   # timestamped / same_dir / custom
custom_output_dir = "D:/compressed"
png_lossless = false
theme = "system"               # system / light / dark
output_format = "original"     # original / jpeg / png / webp / avif
strip_metadata = false

# 图片尺寸限制（不填 = 不限制）
# max_width = 4096
# max_height = 4096
```

历史记录保存在: `~/.config/cvooc-imagemin-compressor/history.json`

## 许可证

CC0-1.0
