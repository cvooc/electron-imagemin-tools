# AGENTS.md

## 项目概述

基于 Rust + iced 框架开发的本地图片压缩工具，是旧 Electron 9 + imagemin 方案的重构替代版，目标是显著减小打包体积（从约 200 MB 降至约 26 MB）。UI 使用 iced 原生渲染，界面语言为中文。

## 命令

- `cargo run --release` — 本地运行 GUI 应用
- `cargo test` / `cargo test -p imagemin-core` — 运行测试
- `cargo build --release` — 构建发布版本
- `.\test.ps1` — Windows 测试脚本
- `.\build.ps1` — Windows 构建脚本，输出到 `dist/cvooc-imagemin-compressor.exe`

项目未配置 lint、typecheck 或 CI。

## 架构

```
├── Cargo.toml              # 工作空间配置
├── crates/
│   ├── core/               # 核心压缩逻辑
│   │   ├── src/lib.rs      # 模块导出
│   │   ├── src/compress.rs # 压缩算法 (mozjpeg + imagequant + oxipng + gif + svg + webp)
│   │   ├── src/config.rs   # 配置管理
│   │   └── tests/          # 集成测试
│   └── ui/                 # iced GUI 界面
│       ├── src/main.rs     # 入口（字体、窗口设置）
│       ├── src/app.rs      # 应用状态与消息处理
│       └── src/views/      # UI 组件
├── assets/                 # 图标、字体等资源
└── dist/                   # 发布文件
```

## 通信流程

iced 采用 Elm 架构：
- `app.rs` 中的 `Message` 枚举定义所有用户事件和异步结果。
- `update()` 处理消息，`view()` 根据 `AppState` 渲染对应界面。
- 文件选择使用 `rfd::AsyncFileDialog`。
- 压缩在 `tokio` 异步任务中调用 `imagemin_core::compress_images`。

## 关键细节

- 主窗口无边框（`decorations: false`），标题栏支持拖动。
- 压缩输出目录模式由 `Config.output_mode` 控制：
  - `Timestamped`：`~/retrocode_io/imagemin/<时间戳>/`（默认）
  - `SameDir`：与输入文件同目录
  - `Custom`：用户自定义目录
- 配置文件路径：`~/.config/cvooc-imagemin-compressor/config.toml`
- 支持格式：JPEG、PNG、GIF、SVG、WebP
- 质量范围：JPEG 0–100，PNG 0–100（imagequant 内部最低质量为 0）
- 配置保存时机：应用启动加载，设置页修改后立即保存
- 字体处理：`assets/` 目录嵌入字体文件，同时以 "Microsoft YaHei" 作为系统 fallback

## 代码风格

- Rust 2021 edition，使用 `cargo fmt` 格式化。
- 核心错误处理使用 `thiserror` 定义的 `CompressError`。
- UI 消息使用 iced 的 `Command::perform` 发起异步任务。
- 注释为中文。
- 发版时需更新 `Cargo.toml` workspace `version`。
