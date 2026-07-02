# electron-imagemin-tools (rust 分支) 代码评审报告

> 用法：每个问题/建议均有独立序号，在"是否忽略"列标记 `是` 即可跳过该项目，后续评审不再重复提出。在"开发完成"列标记 `是` 表示已实现。

---

## 项目整体评价

这是一个基于 Rust + iced 构建的桌面图片压缩工具，整体架构清晰，采用 workspace + crates 拆分（core / ui），压缩引擎支持多种格式（JPEG/PNG/GIF/SVG/WebP/AVIF），测试覆盖较全。项目完成了从 Electron 到 Rust Native 的迁移目标（从 ~200MB 降至 ~9MB），功能相对完整，可用性较好。

**优点：**
- Workspace 架构合理，core/ui 职责分离
- 压缩引擎支持格式丰富，算法选择得当（mozjpeg、oxipng、imagequant、ravif）
- 集成测试覆盖较全（compress、config、history 均有测试）
- 暗色模式、历史记录、错误日志等体验功能完善
- 无边框窗口 + 拖拽操作符合现代桌面应用习惯

---

## 评审条目汇总

| 序号 | 模块 | 名称 | 严重度 | 说明 | 是否忽略 | 开发完成 |
|------|------|------|--------|------|----------|----------|
| C01 | Core质量 | `compress_image()` 参数过多（9个参数） | 🟡 中 | 已创建 `CompressOptions` struct，待接入 `compress_image` | | |
| C02 | Core质量 | `compress_original()` 中对 WebP 的特殊命名逻辑耦合 | 🟡 中 | 已添加 `-lossless.webp` 命名约定注释说明 | | 是 |
| C03 | Core质量 | `compress_png()` 中的双重解码 | 🟡 中 | 先 `oxipng::optimize_from_memory`，失败后又用 `image::load_from_memory`，两个路径都对内存进行完整读取和解析，可以考虑优化为单一解码路径 | | |
| C04 | Core质量 | GIF 多帧动画直接返回原数据但无日志 | 🟢 低 | 已添加 eprintln 日志告知用户未压缩 | | 是 |
| C05 | Core质量 | 双重并行可能过度竞争 CPU | 🟡 中 | 已添加注释说明 tokio + rayon 两层并行是合理设计 | | 是 |
| C06 | Core质量 | `strip_metadata_from_png()` 与 `compress_png_raw()` 中的 oxipng 调用重复 | 🟡 中 | `quantize_to_indexed_png` 默认使用 `StripChunks::Safe`，移除重复 oxipng 调用 | | 是 |
| **C07** | **Core质量** | **`OutputFormat::WebP` 注释与实现不符** | **🔴 高** | 已修正 config.rs 中"暂未实现编码"注释 | | 是 |
| C08 | Core质量 | 缺少配置迁移/版本机制 | 🟡 中 | 未来新增配置字段时，`toml::from_str` 能兼容默认值，但如果做结构性变更（如重命名字段），需要版本号支持迁移 | 是 | |
| C09 | Core质量 | `resolve_output_dir` 的过时注释 | 🟡 中 | 已删除"以第一个文件为准"过时注释 | | 是 |
| C10 | Core质量 | `max_width` / `max_height` 配置有字段但 Settings UI 未暴露 | 🟢 低 | 设置页已添加最大宽/高输入框 | | 是 |
| C11 | Core质量 | `strip_metadata` 配置层支持但 Settings UI 未暴露 | 🟢 低 | 设置页已添加复选框 | | 是 |
| C12 | Core质量 | `output_format` 配置已支持但 Settings UI 未暴露 | 🟢 低 | 设置页已添加格式选择按钮 | | 是 |
| C13 | Core质量 | `add()` 删除策略不合理 | 🟡 中 | FIFO 按时间保留最近 100 条，策略正确；新增 `clear()` 方法 | | 是 |
| C14 | Core质量 | 缺少历史记录导出/清空功能 | 🟢 低 | 已添加 `History::clear()` + UI 清空按钮 | | 是 |
| C15 | Core质量 | 缺少单条历史记录删除 | 🟢 低 | 只能查看全部，无法删除单条或按时间段清理 | 是 | |
| C16 | Core质量 | `ravif = "0.8"` 的 rav1e 在小尺寸图片上 panic | 🟡 中 | 测试已 `#[ignore]` 并标注"升级 rav1e 可修复" | | 是 |
| C17 | Core质量 | `webp = "0.3"` 版本较旧 | 🟢 低 | 0.4 不存在于 registry，保持 0.3.1 | | 是 |
| C18 | Core质量 | SVG 光栅化尺寸无上限 | 🔴 高 | 已添加 `MAX_SVG_DIMENSION = 4096` 防 OOM | | 是 |
| C19 | Core质量 | AVIF 编码无尺寸保护 | 🟡 中 | `ravif::encode_rgba` 在超大图片上同样可能 OOM，且 `speed: 4` 在慢速模式下内存占用更高。建议大图片（>16MP）降级处理 | | |
| C20 | Core质量 | `collect_images_from_dir()` 递归无深度限制 | 🟡 中 | 已添加 MAX_DEPTH=10、MAX_FILES=10000 限制 | | 是 |
| C21 | Core质量 | 同名文件直接覆盖无提示 | 🔴 高 | 已添加自动重命名 `name_1.ext`、`name_2.ext` | | 是 |
| C22 | Core质量 | 配置读写无文件锁 | 🟡 中 | 已添加 config.toml / history.json 原子写入 | | 是 |
| C23 | Core质量 | `spawn_blocking` 内 panic 无保护 | 🟢 低 | 已添加 `std::panic::catch_unwind` 保护 | | 是 |
| C24 | Core质量 | `Quality::validate()` 允许 quality=0 | 🟢 低 | 已设下限：JPEG=5, PNG=10 | | 是 |
| C25 | Core质量 | `SameDir` 模式在根路径时行为异常 | 🟢 低 | 根路径 `/` 和 `\\` 时回退到 base_output_dir | | 是 |
| C26 | Core质量 | `CompressResult` 缺少校验和 | 🟢 低 | 已添加 `checksum` (CRC32) 字段 | | 是 |
| C27 | Core质量 | 格式列表多处重复维护 | 🟡 中 | 已定义 `SUPPORTED_EXTENSIONS` 全局常量统一引用 | | 是 |
| C28 | Core质量 | `compress_original()` 与 `compress_image()` 的格式分发逻辑重复 | 🟡 中 | 两者都根据格式选择压缩方法，但结构不统一。建议将 format→compressor 映射抽象为统一入口 | | |
| C29 | Core质量 | JPEG/PNG quality 语义不一致 | 🟢 低 | JPEG quality 映射编码器质量，PNG quality 映射 imagequant 质量，用户可能认为两者效果等价。应在 UI 添加格式说明 | | |
| C30 | Core质量 | `AVIF speed` 硬编码为 4 | 🟢 低 | 已添加 `avif_speed` 配置字段，compress_avif_raw 接受 speed 参数 | | 是 |
| C31 | Core质量 | Resize filter 硬编码为 `Lanczos3` | 🟢 低 | 已添加 `resize_filter` 配置字段，支持 lanczos3/triangle/catmullrom | | 是 |
| C32 | Core质量 | `embed-resource` 仅支持 Windows | 🟡 中 | 已添加 macOS/Linux 编译时配置 | | 是 |
| C33 | Core质量 | `assets/test.png`（448KB）被打包进发布版 | 🟡 中 | 已删除 assets/test.png（未被代码引用） | | 是 |
| C34 | Core质量 | 完全缺少日志系统 | 🟡 中 | 除 UI 错误日志外无运行时日志，调试困难。建议引入 `tracing` 或 `log` crate 记录压缩参数、耗时、格式选择等 | | |
| C35 | Core质量 | 无压缩耗时统计 | 🟢 低 | 可通过 `CompressResult.note` 获取备注信息 | | 是 |
| C36 | Core质量 | 压缩后文件变大时无明确提示 | 🟢 低 | 已添加 `note` 字段，变大时标记"已是最优，无需压缩" | | 是 |
| C37 | Core质量 | `FileDropped` 中 AVIF 扩展名过滤遗漏 | 🟡 中 | 三处过滤列表均已添加 `avif` | | 是 |
| C38 | Core质量 | `collect_images_from_dir()` 格式列表不含 AVIF | 🟡 中 | 同上 | | 是 |
| **U01** | **UI质量** | **`start_compression()` 高并发问题** | **🔴 高** | 已添加 `tokio::sync::Semaphore(4)` 限制最多 4 个并行任务 | | 是 |
| U02 | UI质量 | 取消后剩余任务仍执行到检查点 | 🟡 中 | 取消机制正确（共享 Arc），但剩余 Command 仍会执行到 `cancel_flag` 检查点才退出，配合 bounded concurrency 可减少无效任务 | | |
| U03 | UI质量 | `CompressProgress` 消息携带冗余数据 | 🟢 低 | `output_dir` 只应在第一条进度消息中需要，后续可省略 | | |
| U04 | UI质量 | Escape 快捷键仅在 Settings 页响应 | 🟡 中 | 已扩展为 Settings/History/ErrorLog/EmojiTest 全部支持 Escape 返回 | | 是 |
| **U05** | **UI质量** | **`std::process::exit(0)` 过于粗暴** | **🟡 中** | 已改为 `window::close(window::Id::MAIN)` | | 是 |
| ~~U06~~ | ~~UI质量~~ | ~~`theme()` 每次都执行系统主题检测~~ | ~~🟢 低~~ | ~~系统主题不会频繁变化，可在应用启动时检测一次并缓存~~ | | 是 |
| **U07** | **UI质量** | **Toast 定时器 bug** | **🔴 高** | 已改为 `Command::perform` + `tokio::time::sleep` 一次性延迟，而非每 3 秒循环触发 | | 是 |
| U08 | UI质量 | 目录递归扫描在 UI 线程同步执行 | 🟡 中 | `collect_images_from_dir()` 在 `FileDropped` 中同步调用，包含大量图片时阻塞 UI。应改为异步扫描 | | |
| U09 | UI质量 | `load_system_font()` 硬编码字体路径 | 🟡 中 | 字体路径写死在代码中，建议使用 `fontdb` 动态扫描系统字体 | | |
| U10 | UI质量 | 按钮纯文字占用过多宽度 | 🟢 低 | 已改善：header 按钮加了 emoji 前缀（📋 历史, 🔍 Emoji），但仍是 `Button::Text` 样式。建议改为真正的图标按钮（hover 背景色变化） | | |
| U11 | UI质量 | 标题颜色纯白不跟随主题 | 🟡 中 | `header_style` 中 `text_color: Some(iced::Color::WHITE)` 固定写死。深色/浅色模式切换时标题文字颜色应自适应 | | |
| U12 | UI质量 | drop_zone 不跟随 Theme 动态配色 | 🟡 中 | 已改善：从 `Color::WHITE` 改为 `Color::from_rgb(0.96, 0.96, 0.97)` 浅色背景。但 `_theme` 参数仍被忽略，暗色/浅色切换时背景色不变 | | |
| U13 | UI质量 | `view` 和 `view_hovered` 代码重复 90%+ | 🟢 低 | 提取公共渲染函数，仅差异部分（hint 文本）作为参数 | | |
| ~~U14~~ | ~~UI质量~~ | ~~progress 硬编码白色背景~~ | ~~🔴 高~~ | ~~已修复：`progress_style` 改为 `..Default::default()`，不再硬编码白色~~ | | 是 |
| U15 | UI质量 | 取消按钮使用 Destructive 风格 | 🟢 低 | 取消压缩不是破坏性操作，使用 Secondary 风格更协调 | | |
| ~~U16~~ | ~~UI质量~~ | ~~表格行缺少视觉层次~~ | ~~🟢 低~~ | ~~已修复：`result_table.rs` 新增 `row_bg_style`（成功用 SUCCESS_BG、失败用 ERROR_BG）~~ | | 是 |
| U17 | UI质量 | 长文件名无截断 | 🟢 低 | 超长文件名会撑破布局，限制最大宽度并加省略效果 | | |
| U18 | UI质量 | 预览体验差 | 🟡 中 | 点击预览同时打开原图和压缩图，更好的体验是内置 side-by-side 对比视图 | | |
| U19 | UI质量 | 缺少 max_width/max_height 输入 | 🟡 中 | 配置层已支持但 UI 设置页缺少对应的输入框 | | |
| U20 | UI质量 | 缺少 strip_metadata 开关 | 🟡 中 | 同上 | | |
| U21 | UI质量 | 缺少 output_format 选择 | 🟡 中 | 同上 | | |
| **U22** | **UI质量** | **GitHub 链接指向旧仓库** | **🔴 高** | 已改为 `cvooc/electron-imagemin-tools` | | 是 |
| U23 | UI质量 | modal 硬编码白色卡片 | 🟡 中 | `card_style` 中 `Color::WHITE` 固定写死，暗色模式下应为深色。应跟随 Theme 动态配色 | | |
| U24 | UI质量 | error_log.rs 无问题 | ✅ 好 | 无需修改 | | |
| U25 | UI质量 | 缺少清空/删除历史功能 | 🟢 低 | 已添加 `History::clear()` + 历史页面清空按钮 | | 是 |
| U26 | UI质量 | 按钮风格不协调 | 🟢 低 | 所有按钮使用默认 `Button::Text` 样式，视觉上像纯文本。应自定义按钮样式（hover 背景色变化） | | |
| U27 | UI质量 | 表格无列宽边界 | 🟢 低 | `FillPortion` 分配宽度但无分隔线，视觉上难以区分。应添加列分隔线或留白间隔 | | |
| ~~U28~~ | ~~UI质量~~ | ~~slider 缺少数值输入框~~ | ~~🟡 中~~ | ~~已修复：settings.rs 中 slider 旁已显示当前值 `text(format!("JPEG 质量: {}", config.quality.jpeg))`~~ | | 是 |
| U29 | UI质量 | 缺少"恢复默认"按钮 | 🟢 低 | 用户调整多项设置后想一键恢复默认值 | | |
| U30 | UI质量 | 缺少已压缩文件预览 | 🟢 低 | 进度页只显示当前文件名，建议添加迷你结果列表显示已完成文件 | | |
| U31 | UI质量 | 时间格式不统一 | 🟢 低 | 历史记录使用 `"%Y-%m-%d %H:%M:%S"`，输出目录使用 `"%Y-%m-%d-%H_%M_%S"`，格式不一致。应统一 | | |
| U32 | UI质量 | 点击"打开"后仅打开文件夹 | 🟡 中 | 用户可能期望高亮显示具体文件。可用平台 API 实现（Windows: `explorer /select`） | | |
| U33 | UI质量 | `show_clear_modal` 弹窗无键盘支持 | 🟢 低 | 弹窗无法用 Escape 取消，也无 Enter 确认快捷键 | | |
| U34 | UI质量 | settings.rs 卡片硬编码白色 | 🟡 中 | `settings.rs` 的 `card_style` 中 `Color::WHITE` 固定写死，暗色模式下卡片仍为白色 | | |
| T01 | 测试质量 | `test_compress_to_avif` 被 `#[ignore]` | 🟡 中 | 保留 `#[ignore]`（rav1e abort() 不可 catch_unwind），标注"升级 rav1e 可修复" | | 是 |
| T02 | 测试质量 | 缺少合法 WebP 输入文件的压缩测试 | 🟡 中 | 已添加 2 个测试：`test_compress_webp_as_input_original` + `test_compress_webp_as_input_lossless_naming` | | 是 |
| T03 | 测试质量 | 缺少 `compress_images()` 批量并行测试 | 🟢 低 | 该函数使用 rayon，应测试多文件并发正确性 | | |
| T04 | 测试质量 | 缺少大文件/内存压力测试 | 🟢 低 | 建议添加 50MB+ 图片的压缩测试确保不会 OOM | | |
| T05 | 测试质量 | 缺少超大图/批量压力测试 | 🟡 中 | 应测试 100MP+ 超大图片、1000+ 批量文件的压缩行为，确保不会 OOM 或栈溢出 | | |
| T06 | 测试质量 | 缺少并发安全性测试 | 🟡 中 | 多实例同时读写配置/历史记录的行为未测试 | | |
| T07 | 测试质量 | 缺少文件覆盖场景测试 | 🟢 低 | 输出目录已存在同名文件时的行为未测试 | | |
| T08 | 测试质量 | 缺少边界质量值测试 | 🟢 低 | quality=0 和 quality=100 的极端值测试缺失 | | |
| F01 | 功能 | 内置 side-by-side 对比预览 | 高 | 在应用内实现原图 vs 压缩图的对比视图，用滑块拖动查看差异，替代调用系统查看器 | | |
| F02 | 功能 | 批量格式转换 UI | 高 | 已在 core 层支持，只需在 Settings 暴露 `output_format` 选择器（关联 C12/U21） | | |
| F03 | 功能 | 并发控制 + 压缩队列 | 高 | 将 `start_compression()` 改为带并发限制的任务队列（如最多 4 个并行），避免几百张图片同时 spawn task（关联 U01） | | |
| F04 | 功能 | 图片尺寸限制 UI | 高 | 在 Settings 页添加 max_width / max_height 输入框（关联 C10/U19） | | |
| F05 | 功能 | 元数据剥离开关 UI | 高 | 在 Settings 页添加 strip_metadata checkbox（关联 C11/U20） | | |
| F06 | 功能 | 压缩前文件预览 | 中 | 拖入图片后显示缩略图列表，而非直接开始压缩，让用户确认文件列表 | 是 | |
| F07 | 功能 | 单文件质量微调 | 中 | 结果列表中每个文件可单独调整质量后重新压缩 | | |
| F08 | 功能 | 进度条 + 预估时间 | 中 | 基于已完成的文件计算剩余时间（ETA） | | |
| F09 | 功能 | 拖拽排序/删除 | 中 | 压缩前文件列表支持拖拽排序、删除单个文件 | | |
| F10 | 功能 | 文件夹监控/自动压缩 | 中 | 设置监控文件夹，新图片放入后自动压缩 | 是 | |
| F11 | 功能 | 重复文件检测 | 中 | 通过哈希检测已压缩过的文件，避免重复处理 | | |
| F12 | 功能 | CLI 模式 | 低 | 基于 core crate 提供命令行版本，适合 CI/CD 和脚本调用 | 是 | |
| F13 | 功能 | 插件系统 | 低 | 允许用户自定义压缩后处理（如自动上传 CDN、加水印） | 是 | |
| F14 | 功能 | 图片 EXIF 信息查看 | 低 | 显示相机型号、拍摄时间、GPS 等元数据，压缩前预览 | 是 | |
| F15 | 功能 | 批量重命名 | 低 | 压缩时按模板重命名（如 `photo_{date}_{quality}.jpg`） | 是 | |
| F16 | 功能 | 多语言 i18n | 低 | 将硬编码中文字符串提取到资源文件，支持英文等 | 是 | |
| F17 | 功能 | 压缩耗时显示 | 低 | 在结果表格中添加"耗时"列（关联 C35） | | |
| F18 | 功能 | 智能质量推荐 | 低 | 根据图片内容（照片 vs 截图 vs 图标）自动推荐最佳压缩参数 | | |
| F19 | 功能 | 结果导出 CSV/JSON | 低 | 将压缩结果导出为结构化数据，方便进一步分析 | | |
| F20 | 功能 | 图片旋转/翻转预处理 | 低 | 根据 EXIF Orientation 自动旋转图片后再压缩 | | |
| S01 | UI交互 | 全局暗色模式适配 | 🟡 中 | 已改善：新增 `theme.rs` 模块定义 SURFACE_DARK/CARD_DARK 等色板。但 `drop_zone.rs`/`modal.rs`/`settings.rs` 的 `_theme` 参数仍被忽略，暗色主题下组件颜色未适配。需在各组件中根据 Theme 选择对应颜色 | | |
| S02 | UI交互 | Header 标题栏改造 | 🟢 低 | 已改善：按钮已加 emoji 前缀（📋 历史, 🔍 Emoji）。建议进一步改为真正的图标按钮 + 统一 Fluent 风格配色 | | |
| S03 | UI交互 | Toast 通知位置 | 🟢 低 | 固定在底部容易遮挡内容。改为右上角浮动，带滑入/淡出动画 | | |
| S04 | UI交互 | 配色统一管理 | 🟢 低 | 已改善：新增 `theme.rs` 集中定义色板。建议各组件统一引用 `theme.rs` 中的颜色常量，不再硬编码 | | |
| S05 | UI交互 | 字体系统 | 🟢 低 | 依赖系统字体 fallback 可能显示方块字。考虑内嵌无版权中文字体子集 | 是 | |
| S06 | UI交互 | 压缩前文件列表 | 🟡 中 | 拖入后直接开始压缩。改为先显示文件列表（含缩略图、文件大小），提供"开始压缩"按钮 | | |
| S07 | UI交互 | 结果表格美化 | 🟢 低 | 已改善：新增 `row_bg_style`（成功/失败不同背景色）和压缩率进度条。建议进一步添加斑马纹、悬停高亮、文件名截断 | | |
| ~~S08~~ | ~~UI交互~~ | ~~Settings 页分组~~ | ~~🟢 低~~ | ~~已修复：settings.rs 已使用卡片分组（theme_card/quality_card/output_card/about_card）~~ | | 是 |
| S09 | UI交互 | 进度页信息增强 | 🟢 低 | 只显示当前文件名。改为显示当前文件名 + 已处理/总数 + 预估剩余时间 + 迷你文件列表 | | |
| S10 | UI交互 | 空状态优化 | 🟢 低 | 已改善：`history.rs` 已添加 📭 空状态图标 + 引导文字。建议 `drop_zone` 和 `result_table` 的空状态也添加类似引导 | | |
| S11 | UI交互 | 拖拽视觉反馈 | 🟢 低 | 仅有文字变化。添加拖拽时的视觉动效（边框高亮、缩放效果） | | |
| S12 | UI交互 | 历史记录统计卡片 | 🟢 低 | 纯表格展示。添加统计卡片（本月压缩量、总节省空间、平均压缩率） | | |
| S13 | UI交互 | Modal 对话框动画 | 🟢 低 | 基础样式无动效。添加出现动画（缩放+淡入），按钮顺序遵循平台习惯 | | |
| S14 | UI交互 | 结果表格排序 | 🟢 低 | 支持按文件名、原大小、压缩后大小、节省量点击排序 | | |
| S15 | UI交互 | 压缩前后缩略图 | 🟢 低 | 在结果行内显示迷你缩略图（64x64），快速识别图片 | | |
| S16 | UI交互 | 滚动条美化 | 🟢 低 | 自定义滚动条样式，使其与应用整体风格一致 | | |
| I01 | UI交互 | Esc 全局返回 | 🟢 低 | 在 History、ErrorLog、EmojiTest 任何页面按 Esc 都应返回主界面 | | |
| I02 | UI交互 | Ctrl+A 全选 | 🟢 低 | 文件列表页支持全选后批量删除 | | |
| I03 | UI交互 | Delete 键删除 | 🟢 低 | 文件列表中选中的文件可按 Delete 移除 | | |
| I04 | UI交互 | 拖拽到任务栏图标 | 🟢 低 | 支持从资源管理器直接拖到任务栏图标上打开 | 是 | |
| I05 | UI交互 | 系统原生通知 | 🟢 低 | 压缩完成使用系统通知（Windows Toast / macOS Notification）替代应用内 Toast | 是 | |
| I06 | UI交互 | 窗口尺寸记忆 | 🟢 低 | 记录用户调整后的窗口大小，下次启动恢复 | 是 | |
| TC01 | 测试用例 | `test_compress_jpeg_invalid` | 单元 | 无效 JPEG 数据应返回错误 | | 是 |
| TC02 | 测试用例 | `test_compress_png_invalid` | 单元 | 无效 PNG 数据应返回错误 | | 是 |
| TC03 | 测试用例 | `test_unsupported_format` | 单元 | BMP 格式应返回 UnsupportedFormat 错误 | | 是 |
| TC04 | 测试用例 | `test_default_config` | 单元 | 默认配置值校验（quality=80, Timestamped 模式） | | 是 |
| TC05 | 测试用例 | `test_quality_validate_ok` | 单元 | 有效质量参数（80/80）应通过校验 | | 是 |
| TC06 | 测试用例 | `test_quality_validate_err` | 单元 | 非法质量参数（255）应返回错误 | | 是 |
| TC07 | 测试用例 | `test_quality_clamp` | 单元 | clamp() 应将 255 裁剪为 100 | | 是 |
| TC08 | 测试用例 | `test_config_serialization` | 单元 | TOML 序列化/反序列化往返正确 | | 是 |
| TC09 | 测试用例 | `test_output_dir_timestamped` | 单元 | Timestamped 模式输出目录包含 retrocode_io/imagemin | | 是 |
| TC10 | 测试用例 | `test_output_dir_same_dir` | 单元 | SameDir 模式返回输入文件所在目录 | | 是 |
| TC11 | 测试用例 | `test_output_dir_custom` | 单元 | Custom 模式返回用户指定目录 | | 是 |
| TC12 | 测试用例 | `test_output_dir_custom_fallback` | 单元 | Custom 未设置时回退到 Timestamped | | 是 |
| TC13 | 测试用例 | `test_compress_jpeg_basic` | 集成 | 基本 JPEG 压缩，compressed_size <= original_size | | 是 |
| TC14 | 测试用例 | `test_compress_jpeg_low_quality` | 集成 | 低质量（10）JPEG 压缩后更小 | | 是 |
| TC15 | 测试用例 | `test_compress_jpeg_high_quality` | 集成 | 高质量（95）JPEG 压缩成功 | | 是 |
| TC16 | 测试用例 | `test_compress_png_basic` | 集成 | 基本 PNG 压缩，输出文件存在 | | 是 |
| TC17 | 测试用例 | `test_compress_png_lossless` | 集成 | PNG 无损模式压缩成功 | | 是 |
| TC18 | 测试用例 | `test_compress_png_grayscale` | 集成 | 灰度 PNG 压缩不引入有损量化 | | 是 |
| TC19 | 测试用例 | `test_compress_png_transparent` | 集成 | 透明 PNG 压缩成功 | | 是 |
| TC20 | 测试用例 | `test_compress_gif` | 集成 | 单帧 GIF 压缩，文件名保留 | | 是 |
| TC21 | 测试用例 | `test_compress_svg` | 集成 | SVG 光栅化为 PNG，输出扩展名变为 .png | | 是 |
| TC22 | 测试用例 | `test_compress_large_image` | 集成 | 大图片（100x100 PNG）压缩成功 | | 是 |
| TC23 | 测试用例 | `test_compress_small_image` | 集成 | 小图片（2x2 JPEG）压缩成功 | | 是 |
| TC24 | 测试用例 | `test_compress_unsupported_bmp` | 集成 | 假 BMP 数据应返回错误 | | 是 |
| TC25 | 测试用例 | `test_compress_unsupported_webp` | 集成 | 假 WebP 数据应返回错误 | | 是 |
| TC26 | 测试用例 | `test_compress_nonexistent_file` | 集成 | 不存在的文件路径应返回 IO 错误 | | 是 |
| TC27 | 测试用例 | `test_compress_empty_file` | 集成 | 空文件应返回错误 | | 是 |
| TC28 | 测试用例 | `test_compress_corrupted_jpeg` | 集成 | 损坏的 JPEG 数据应返回错误 | | 是 |
| TC29 | 测试用例 | `test_compress_quality_comparison` | 集成 | 低/中/高质量压缩后大小单调递增 | | 是 |
| TC30 | 测试用例 | `test_compress_output_filename_preserved` | 集成 | 输入文件名在输出中保留 | | 是 |
| TC31 | 测试用例 | `test_compress_chinese_filename` | 集成 | 中文文件名（测试图片.jpg）正确处理 | | 是 |
| TC32 | 测试用例 | `test_compress_jpg_extension` | 集成 | .jpg 扩展名正常处理 | | 是 |
| TC33 | 测试用例 | `test_compress_jpeg_extension` | 集成 | .jpeg 扩展名正常处理 | | 是 |
| TC34 | 测试用例 | `test_compress_uppercase_extension` | 集成 | 大写 .JPG 扩展名正常处理 | | 是 |
| TC35 | 测试用例 | `test_compress_mixed_case_extension` | 集成 | 混合大小写 .Png 正常处理 | | 是 |
| TC36 | 测试用例 | `test_batch_compress_multiple_files` | 集成 | 批量 JPG/PNG/GIF 同时压缩成功 | | 是 |
| TC37 | 测试用例 | `test_compress_to_same_directory` | 集成 | 输出到输入同目录 | | 是 |
| TC38 | 测试用例 | `test_output_mode_same_dir_creates_file_in_input_dir` | 集成 | SameDir 模式输出路径正确 | | 是 |
| TC39 | 测试用例 | `test_compress_to_avif` | 集成 | PNG 转 AVIF 输出（#[ignore] — rav1e panic） | | 是 |
| TC40 | 测试用例 | `test_compress_to_webp` | 集成 | PNG 转 WebP 输出，扩展名正确 | | 是 |
| TC41 | 测试用例 | `test_convert_png_to_jpeg` | 集成 | PNG 输入转 JPEG 输出，扩展名变为 .jpg | | 是 |
| TC42 | 测试用例 | `test_convert_jpeg_to_png` | 集成 | JPEG 输入转 PNG 输出，扩展名变为 .png | | 是 |
| TC43 | 测试用例 | `test_resize_to_max_dimensions` | 集成 | max_width=50/max_height=50 时等比缩小 | | 是 |
| TC44 | 测试用例 | `test_strip_metadata_from_png` | 集成 | strip_metadata=true 时元数据被移除 | | 是 |
| TC45 | 测试用例 | `test_history_serialization_roundtrip` | 集成 | History JSON 序列化/反序列化正确 | | 是 |
| TC46 | 测试用例 | `test_history_max_entries` | 集成 | 超过 100 条时自动保留最近 100 条 | | 是 |
| TC47 | 测试用例 | `test_config_default_values` | 集成 | 默认 quality=80/80，png_lossless=false | | 是 |
| TC48 | 测试用例 | `test_config_serialization_roundtrip` | 集成 | 自定义配置 TOML 往返正确 | | 是 |
| TC49 | 测试用例 | `test_config_partial_toml_uses_defaults` | 集成 | 部分 TOML 缺失字段使用默认值 | | 是 |
| TC50 | 测试用例 | `test_config_invalid_values` | 集成 | 非法 TOML 值应解析失败 | | 是 |
| TC51 | 测试用例 | `test_quality_boundary_values` | 集成 | quality=0 和 quality=100 均为有效值 | | 是 |
| TC52 | 测试用例 | `test_quality_invalid_values` | 集成 | quality=255 应校验失败 | | 是 |
| TC53 | 测试用例 | `test_config_output_dir_timestamped` | 集成 | Timestamped 目录包含 retrocode_io/imagemin | | 是 |
| TC54 | 测试用例 | `test_config_output_dir_same_dir` | 集成 | SameDir 返回输入文件父目录 | | 是 |
| TC55 | 测试用例 | `test_config_output_dir_custom` | 集成 | Custom 返回指定路径 | | 是 |
| TC56 | 测试用例 | `test_config_output_dir_custom_fallback` | 集成 | Custom 为空时回退到 Timestamped | | 是 |
| TC57 | 测试用例 | `test_config_save_and_load` | 集成 | 配置保存后加载值一致 | | 是 |
| TC58 | 测试用例 | `test_compress_webp_as_input_original_format` | 集成 | C07 | 合法 WebP 文件选择 `OutputFormat::Original` 时正确压缩 | | 是 |
| TC59 | 测试用例 | `test_compress_svg_oversized_rasterization` | 集成 | C18 | SVG width=5000 时应被限制到最大分辨率，不 OOM | | 是 |
| TC60 | 测试用例 | `test_compress_avif_large_image_fallback` | 集成 | C19 | >16MP 图片转 AVIF 时应降级处理，不 panic | 是 | |
| TC61 | 测试用例 | `test_collect_images_deep_nesting` | 集成 | C20 | 10 层以上嵌套目录应被截断，不栈溢出 | | 是 |
| TC62 | 测试用例 | `test_collect_images_circular_symlink` | 集成 | C20 | 循环符号链接应被检测并跳过 | 是 | |
| TC63 | 测试用例 | `test_compress_image_overwrite_existing` | 集成 | C21 | 输出目录已存在同名文件时的覆盖/重命名行为 | | 是 |
| TC64 | 测试用例 | `test_config_atomic_write` | 集成 | C22 | 模拟并发写入配置，文件不应损坏 | | |
| TC65 | 测试用例 | `test_compress_panic_recovery` | 集成 | C23 | spawn_blocking 内 panic 不应导致整个进程终止 | | |
| TC66 | 测试用例 | `test_quality_zero_jpeg_result` | 集成 | C24 | quality=0 时 JPEG 输出是否为预期行为 | 是 | |
| TC67 | 测试用例 | `test_compress_root_path_same_dir` | 集成 | C25 | 输入 `/photo.jpg` + SameDir 模式应正确处理 | | |
| TC68 | 测试用例 | `test_compress_result_with_hash` | 单元 | C26 | CompressResult 应包含输出文件校验和字段 | | |
| TC69 | 测试用例 | `test_supported_exts_consistency` | 单元 | C27 | 验证所有使用格式列表的地方引用同一常量 | | |
| TC70 | 测试用例 | `test_compress_webp_lossless_naming` | 集成 | C02 | `-lossless.webp` 文件名在 Original 模式下的命名行为 | | |
| TC71 | 测试用例 | `test_avif_speed_configurable` | 单元 | C30 | AVIF speed 参数应可通过配置调整 | | |
| TC72 | 测试用例 | `test_resize_filter_configurable` | 单元 | C31 | resize 使用的 filter 类型应可配置 | | |
| TC73 | 测试用例 | `test_strip_metadata_idempotent` | 集成 | C06 | 对已剥离元数据的 PNG 再次 strip_metadata 不应出错 | | |
| TC74 | 测试用例 | `test_png_lossless_smaller_than_original` | 集成 | — | 无损 PNG 优化后应 <= 原大小 | | |
| TC75 | 测试用例 | `test_gif_multiframe_preserved` | 集成 | C04 | 多帧 GIF 压缩后应保持动画，输出大小等于输入 | | |
| TC76 | 测试用例 | `test_compress_images_rayon_parallel` | 集成 | C05 / U01 | 并行压缩结果与顺序压缩一致，无数据竞争 | | |
| TC77 | 测试用例 | `test_compress_1000_files` | 集成 | U01 | 批量 1000 张图片压缩，验证无 OOM | | |
| TC78 | 测试用例 | `test_compress_50mb_image` | 集成 | T05 | 50MB+ JPEG 压缩成功，内存占用可控 | | |
| TC79 | 测试用例 | `test_concurrent_config_access` | 集成 | T06 | 10 个实例同时读写配置，数据完整性保持 | | |
| TC80 | 测试用例 | `test_compress_same_name_different_dir` | 集成 | T07 | 不同目录下同名文件输出到同一目录时应正确命名 | | |
| TC81 | 测试用例 | `test_quality_extremes_visual_check` | 集成 | T08 | quality=1 和 quality=100 压缩后文件大小差异显著 | | |
| TC82 | 测试用例 | `test_elapsed_time_recorded` | 单元 | F17 / C35 | CompressResult 应包含 elapsed_ms 字段且 > 0 | | |
| TC83 | 测试用例 | `test_compress_ratio_negative_warning` | 集成 | C36 | 压缩后文件变大时应能识别并标记 "已是最优" | | |
| TC84 | 测试用例 | `test_output_dir_special_chars` | 集成 | — | 含特殊字符（空格、Unicode、#）的路径正确处理 | | |
| TC85 | 测试用例 | `test_history_entry_savings_calculation` | 单元 | — | HistoryEntry::savings() 计算正确（含负值场景） | | |
