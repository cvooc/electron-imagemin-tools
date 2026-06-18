# AGENTS.md

## 项目概述

基于 Electron 9 + imagemin 的桌面批量图片压缩工具。UI 使用 MDUI + jQuery，界面语言为中文。源码无构建流程，直接加载原始 JS。

## 命令

- `npm start` — 本地启动 Electron 应用
- `npm run build` — 使用 electron-packager 打包（输出到 `build/`）

项目未配置测试、lint、typecheck 或 CI。

## 架构

- `main.js` — Electron 主进程：窗口创建、IPC 处理、imagemin 压缩逻辑
- `www/index.html` — 渲染进程入口（通过 `file://` 协议加载）
- `www/js/index.js` — 渲染进程逻辑：拖放、文件选择、IPC 通信、UI 更新
- `www/mdui/` — 内置的 MDUI（Material Design）CSS/JS
- `www/css/main.css` — 应用样式
- `public/icon/` — electron-packager 使用的应用图标
- `examples/image/` — 示例图片

## IPC 通信流程

渲染进程 → 主进程：`req-comp-files`（文件信息数组 + 质量参数）
主进程 → 渲染进程：`rsp-comp-files`（压缩结果 + 输出目录）或 `rsp-comp-files-error`

窗口控制：`close-main-window`、`min-main-window`

## 关键细节

- BrowserWindow 设置了 `nodeIntegration: true`，渲染进程拥有完整 Node.js 访问权限
- 压缩输出目录：`~/retrocode_io/imagemin/<时间戳>/`
- 支持格式：JPEG、PNG、GIF、SVG（BMP 和 WebP 插件存在但已注释掉）
- 质量设置存储在 `localStorage` 中，键名：`jpg-quality`、`pngQ-quality`、`webpQ-quality`
- PNG 质量范围在渲染进程中被限制为 21–100（pngquant 要求）
- `main.js` 中扩展了 `Date.prototype.format`，用于生成输出目录名
- 构建脚本硬编码了 Electron 9.0.3 版本，并使用淘宝镜像下载 Electron 二进制文件

## 代码风格

- 无模块打包器或转译器，主进程和渲染进程均直接使用 `require()`
- 渲染进程使用 jQuery 操作 DOM
- `var`/`const` 混用，注释为中文
- 发版时需更新 `www/js/index.js` 中的 `version` 常量
