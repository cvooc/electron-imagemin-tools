use iced::widget::{column, container};
use iced::{window, Application, Command, Element, Length, Subscription, Theme};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

use crate::views::{drop_zone, emoji_test, error_log, header, history, modal, progress, result_table, settings, stack, toast};
use crate::theme;
use imagemin_core::{Config, History, HistoryEntry, OutputMode, ThemeMode, SUPPORTED_EXTENSIONS};

#[derive(Debug, Clone)]
pub enum AppState {
    Idle,
    Compressing { completed: usize, total: usize },
    Completed,
    Settings,
    History,
    ErrorLog,
    EmojiTest,
}

/// 压缩失败日志条目
#[derive(Debug, Clone)]
pub struct LogEntry {
    pub timestamp: String,
    pub filename: String,
    pub error: String,
    pub input_path: String,
}

pub struct App {
    state: AppState,
    config: Config,
    files: Vec<PathBuf>,
    hovered_file: Option<PathBuf>,
    output_dir: Option<PathBuf>,
    results: Vec<result_table::Row>,
    toast: Option<toast::Toast>,
    /// 是否显示清空列表确认弹窗
    show_clear_modal: bool,
    /// 压缩历史记录
    history: History,
    /// 取消压缩标志
    cancel_flag: Option<Arc<AtomicBool>>,
    /// 压缩失败日志
    logs: Vec<LogEntry>,
}

#[derive(Debug, Clone)]
pub enum Message {
    Header(header::Message),
    DropZone(drop_zone::Message),
    ResultTable(result_table::Message),
    Settings(settings::Message),
    History(history::Message),
    ErrorLog(error_log::Message),
    EmojiTest(emoji_test::Message),
    FilesSelected(Vec<PathBuf>),
    FileHovered(PathBuf),
    FileDropped(PathBuf),
    FileHoveredLeft,
    CompressProgress(usize, usize, Option<result_table::Row>, Option<PathBuf>),
    /// 键盘快捷键
    KeyPressed(iced::keyboard::Key, iced::keyboard::Modifiers),
    /// Toast 超时自动关闭
    ToastTimeout,
    /// Modal 确认清空
    ConfirmClear,
    CancelClear,
    /// 取消压缩
    CancelCompression,
}

impl Application for App {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        let (config, config_err) = Config::load();

        let mut toast = None;
        if let Some(err) = config_err {
            toast = Some(toast::Toast::info(err));
        }

        (
            Self {
                state: AppState::Idle,
                config,
                files: Vec::new(),
                hovered_file: None,
                output_dir: None,
                results: Vec::new(),
                toast,
                show_clear_modal: false,
                history: History::load(),
                cancel_flag: None,
                logs: Vec::new(),
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("retrocode.io压图")
    }

    fn theme(&self) -> Theme {
        match self.config.theme {
            ThemeMode::Light => Theme::Light,
            ThemeMode::Dark => Theme::Dark,
            ThemeMode::System => {
                if detect_system_is_dark() {
                    Theme::Dark
                } else {
                    Theme::Light
                }
            }
        }
    }

    fn subscription(&self) -> Subscription<Message> {
        let file_events = iced::event::listen_with(|event, _status| match event {
            iced::Event::Window(_, iced::window::Event::FileHovered(path)) => {
                Some(Message::FileHovered(path))
            }
            iced::Event::Window(_, iced::window::Event::FileDropped(path)) => {
                Some(Message::FileDropped(path))
            }
            iced::Event::Window(_, iced::window::Event::FilesHoveredLeft) => {
                Some(Message::FileHoveredLeft)
            }
            _ => None,
        });

        let keyboard = iced::keyboard::on_key_press(|key, modifiers| {
            Some(Message::KeyPressed(key, modifiers))
        });

        let subs = vec![file_events, keyboard];

        // Toast 自动消失不再使用 subscription，改用 Command::perform（见 update）
        // 避免每 3 秒重复触发的 bug

        Subscription::batch(subs)
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::Header(header::Message::OpenSettings) => {
                match self.state {
                    AppState::Settings => {
                        self.state = AppState::Idle;
                    }
                    _ => {
                        self.state = AppState::Settings;
                    }
                }
                Command::none()
            }
            Message::Header(header::Message::OpenHistory) => {
                match self.state {
                    AppState::History => {
                        self.state = AppState::Idle;
                    }
                    _ => {
                        self.history = History::load();
                        self.state = AppState::History;
                    }
                }
                Command::none()
            }
            Message::Header(header::Message::OpenErrorLog) => {
                match self.state {
                    AppState::ErrorLog => {
                        self.state = AppState::Idle;
                    }
                    _ => {
                        self.state = AppState::ErrorLog;
                    }
                }
                Command::none()
            }
            Message::Header(header::Message::OpenEmojiTest) => {
                match self.state {
                    AppState::EmojiTest => self.state = AppState::Idle,
                    _ => self.state = AppState::EmojiTest,
                }
                Command::none()
            }
            Message::Header(header::Message::Minimize) => {
                window::minimize(window::Id::MAIN, true)
            }
            Message::Header(header::Message::Close) => {
                window::close(window::Id::MAIN)
            }
            Message::Header(header::Message::Drag) => window::drag(window::Id::MAIN),
            Message::DropZone(drop_zone::Message::SelectFiles) => {
                Command::perform(select_files(), Message::FilesSelected)
            }
            Message::FilesSelected(files) => {
                if !files.is_empty() {
                    self.files = files;
                    self.results.clear();
                    self.output_dir = None;
                    self.start_compression()
                } else {
                    Command::none()
                }
            }
            Message::FileHovered(path) => {
                self.hovered_file = Some(path);
                Command::none()
            }
            Message::FileDropped(path) => {
                self.hovered_file = None;

                // 如果是目录，扫描其中的图片文件
                let files: Vec<PathBuf> = if path.is_dir() {
                    collect_images_from_dir(&path)
                } else {
                    let ext = path
                        .extension()
                        .and_then(|e| e.to_str())
                        .unwrap_or("")
                        .to_lowercase();
                    if SUPPORTED_EXTENSIONS.contains(&ext.as_str()) {
                        vec![path]
                    } else {
                        Vec::new()
                    }
                };

                if !files.is_empty() {
                    self.files = files;
                    self.results.clear();
                    self.output_dir = None;
                    self.start_compression()
                } else {
                    Command::none()
                }
            }
            Message::FileHoveredLeft => {
                self.hovered_file = None;
                Command::none()
            }
            Message::CompressProgress(index, total, row, output_dir) => {
                if let Some(row) = row {
                    // 记录失败日志
                    if let Err(ref err) = row.status {
                        self.logs.push(LogEntry {
                            timestamp: chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
                            filename: row.name.clone(),
                            error: err.clone(),
                            input_path: row.input_path.as_ref().map(|p| p.to_string_lossy().to_string()).unwrap_or_default(),
                        });
                    }
                    self.results.push(row);
                }
                if index == 0 {
                    self.output_dir = output_dir;
                }

                let completed = self.results.len();
                // 如果取消了压缩，忽略后续进度消息
                if self.cancel_flag.is_none() {
                    return Command::none();
                }
                if completed < total {
                    self.state = AppState::Compressing { completed, total };
                    Command::none()
                } else {
                    self.state = AppState::Completed;
                    // 保存历史记录
                    let history_results: Vec<imagemin_core::history::HistoryResult> = self.results.iter().map(|r| {
                        imagemin_core::history::HistoryResult {
                            name: r.name.clone(),
                            original_size: r.original_size,
                            compressed_size: r.compressed_size,
                            success: r.status.is_ok(),
                        }
                    }).collect();
                    let total_original: u64 = history_results.iter().filter(|r| r.success).map(|r| r.original_size).sum();
                    let total_compressed: u64 = history_results.iter().filter(|r| r.success).map(|r| r.compressed_size).sum();
                    let entry = HistoryEntry {
                        timestamp_ms: std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap_or_default().as_millis() as u64,
                        timestamp_str: chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
                        results: history_results,
                        output_dir: self.output_dir.clone().unwrap_or_default(),
                        total_original,
                        total_compressed,
                    };
                    self.history.add(entry);
                    let _ = self.history.save();
                    // 计算总节省量并触发 toast
                    let total_original: u64 = self.results.iter().filter_map(|r| r.status.is_ok().then(|| r.original_size)).sum();
                    let total_compressed: u64 = self.results.iter().filter_map(|r| r.status.is_ok().then(|| r.compressed_size)).sum();
                    let saved = total_original as i64 - total_compressed as i64;
                    if saved >= 0 {
                        self.toast = Some(toast::Toast::success(format!(
                            "压缩完成，共节省 {:.1} KB",
                            saved as f64 / 1024.0
                        )));
                    } else {
                        self.toast = Some(toast::Toast::info("压缩完成".to_string()));
                    }
                    toast_timeout()
                }
            }
            Message::CancelCompression => {
                if let Some(flag) = &self.cancel_flag {
                    flag.store(true, Ordering::Relaxed);
                }
                self.cancel_flag = None;
                self.state = AppState::Idle;
                self.toast = Some(toast::Toast::info("压缩已取消"));
                toast_timeout()
            }
            Message::History(history::Message::Back) => {
                self.state = AppState::Idle;
                Command::none()
            }
            Message::History(history::Message::OpenDir(dir)) => {
                open::that(&dir).ok();
                Command::none()
            }
            Message::History(history::Message::ClearAll) => {
                self.history.clear();
                let _ = self.history.save();
                self.toast = Some(toast::Toast::info("历史已清空"));
                toast_timeout()
            }
            Message::ErrorLog(error_log::Message::Back) => {
                self.state = AppState::Idle;
                Command::none()
            }
            Message::ErrorLog(error_log::Message::CopyLog) => {
                let log_text: String = self
                    .logs
                    .iter()
                    .map(|l| format!("[{}] {} — {}\n  路径: {}", l.timestamp, l.filename, l.error, l.input_path))
                    .collect::<Vec<_>>()
                    .join("\n");
                if log_text.is_empty() {
                    self.toast = Some(toast::Toast::info("暂无失败日志"));
                    toast_timeout()
                } else {
                    self.toast = Some(toast::Toast::success("日志已复制到剪贴板"));
                    iced::clipboard::write(log_text)
                }
            }
            Message::ErrorLog(error_log::Message::ClearLog) => {
                self.logs.clear();
                self.toast = Some(toast::Toast::info("日志已清空"));
                toast_timeout()
            }
            Message::EmojiTest(emoji_test::Message::Back) => {
                self.state = AppState::Idle;
                Command::none()
            }
            Message::ResultTable(result_table::Message::OpenOutputDir) => {
                if let Some(dir) = &self.output_dir {
                    open::that(dir).ok();
                }
                Command::none()
            }
            Message::ResultTable(result_table::Message::RetryCompress) => {
                if self.files.is_empty() {
                    self.state = AppState::Idle;
                    Command::none()
                } else {
                    self.results.clear();
                    self.output_dir = None;
                    self.start_compression()
                }
            }
            Message::ResultTable(result_table::Message::ClearResults) => {
                // 显示确认弹窗而非直接清空
                self.show_clear_modal = true;
                Command::none()
            }
            Message::ResultTable(result_table::Message::Preview(idx)) => {
                if let Some(row) = self.results.get(idx) {
                    // 用系统默认图片查看器打开原图和压缩后的图
                    if let Some(p) = &row.input_path {
                        open::that(p).ok();
                    }
                    if let Some(p) = &row.output_path {
                        open::that(p).ok();
                    }
                }
                Command::none()
            }
            Message::ConfirmClear => {
                self.show_clear_modal = false;
                self.results.clear();
                self.files.clear();
                self.output_dir = None;
                self.state = AppState::Idle;
                self.toast = Some(toast::Toast::info("列表已清空"));
                toast_timeout()
            }
            Message::CancelClear => {
                self.show_clear_modal = false;
                Command::none()
            }
            Message::Settings(settings::Message::JpegChanged(q)) => {
                self.config.quality.jpeg = q;
                let _ = self.config.save();
                Command::none()
            }
            Message::Settings(settings::Message::PngChanged(q)) => {
                self.config.quality.png = q;
                let _ = self.config.save();
                Command::none()
            }
            Message::Settings(settings::Message::PngLosslessChanged(v)) => {
                self.config.png_lossless = v;
                let _ = self.config.save();
                Command::none()
            }
            Message::Settings(settings::Message::OutputModeChanged(mode)) => {
                self.config.output_mode = mode;
                let _ = self.config.save();
                Command::none()
            }
            Message::Settings(settings::Message::ThemeChanged(theme)) => {
                self.config.theme = theme;
                let _ = self.config.save();
                Command::none()
            }
            Message::Settings(settings::Message::SelectCustomOutputDir) => {
                Command::perform(select_output_dir(), Message::Settings)
            }
            Message::Settings(settings::Message::CustomOutputDirSelected(path)) => {
                if !path.as_os_str().is_empty() {
                    self.config.custom_output_dir = Some(path);
                    self.config.output_mode = OutputMode::Custom;
                    let _ = self.config.save();
                }
                Command::none()
            }
            Message::Settings(settings::Message::MaxWidthChanged(val)) => {
                self.config.max_width = val.parse::<u32>().ok().filter(|&v| v > 0);
                let _ = self.config.save();
                Command::none()
            }
            Message::Settings(settings::Message::MaxHeightChanged(val)) => {
                self.config.max_height = val.parse::<u32>().ok().filter(|&v| v > 0);
                let _ = self.config.save();
                Command::none()
            }
            Message::Settings(settings::Message::StripMetadataChanged(val)) => {
                self.config.strip_metadata = val;
                let _ = self.config.save();
                Command::none()
            }
            Message::Settings(settings::Message::OutputFormatChanged(val)) => {
                self.config.output_format = val;
                let _ = self.config.save();
                Command::none()
            }
            Message::KeyPressed(key, modifiers) => {
                use iced::keyboard::Key;
                let ctrl = modifiers.control();
                match (key, ctrl) {
                    // Escape: 返回主界面
                    (Key::Named(iced::keyboard::key::Named::Escape), _) => {
                        match self.state {
                            AppState::Settings | AppState::History | AppState::ErrorLog | AppState::EmojiTest => {
                                self.state = AppState::Idle;
                            }
                            _ => {}
                        }
                        Command::none()
                    }
                    // Ctrl+O: 打开文件选择
                    (Key::Character(c), true) if c.as_str() == "o" => {
                        Command::perform(select_files(), Message::FilesSelected)
                    }
                    // Ctrl+R: 重新压缩
                    (Key::Character(c), true) if c.as_str() == "r" => {
                        self.results.clear();
                        self.output_dir = None;
                        self.start_compression()
                    }
                    _ => Command::none(),
                }
            }
            Message::ToastTimeout => {
                self.toast = None;
                Command::none()
            }
        }
    }

    fn view(&self) -> Element<'_, Message> {
        let header = header::view().map(Message::Header);

        let content = match &self.state {
            AppState::Idle => {
                if self.hovered_file.is_some() {
                    drop_zone::view_hovered().map(Message::DropZone)
                } else {
                    drop_zone::view(self.files.len()).map(Message::DropZone)
                }
            }
            AppState::Compressing { completed, total } => {
                let current_file = self
                    .files
                    .get(*completed)
                    .and_then(|p| p.file_name())
                    .map(|n| n.to_string_lossy().to_string())
                    .unwrap_or_else(|| "…".to_string());
                progress::view(*completed, *total, &current_file)
            }
            AppState::Completed => result_table::view(&self.results, self.output_dir.is_some())
                .map(Message::ResultTable),
            AppState::Settings => settings::view(&self.config).map(Message::Settings),
            AppState::History => history::view(&self.history.entries).map(Message::History),
            AppState::ErrorLog => error_log::view(&self.logs).map(Message::ErrorLog),
            AppState::EmojiTest => emoji_test::view().map(Message::EmojiTest),
        };

        let toast_element: Element<'_, Message> = match &self.toast {
            Some(t) => container(toast::view(t).map(|_| Message::ToastTimeout))
                .width(Length::Fill)
                .align_x(iced::alignment::Horizontal::Center)
                .padding([0, 0, 16, 0])
                .into(),
            None => container(iced::widget::text(""))
                .height(Length::Fixed(0.0))
                .into(),
        };

        let main = container(
            column![header, content, toast_element]
                .width(Length::Fill)
                .height(Length::Fill),
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .style(|theme: &iced::Theme| {
            let is_dark = matches!(theme, iced::Theme::Dark | iced::Theme::CatppuccinMocha | iced::Theme::TokyoNight | iced::Theme::Dracula | iced::Theme::Nord);
            theme::container_surface(is_dark)
        });

        // 弹窗激活时使用 Stack 叠加在半透明蒙层之上
        if self.show_clear_modal {
            let clear_modal = modal::Modal {
                title: "确认清空".to_string(),
                message: "确定要清空所有压缩结果和文件列表吗？此操作不可撤销。".to_string(),
                confirm_label: "确定清空".to_string(),
                cancel_label: "取消".to_string(),
                on_confirm: Message::ConfirmClear,
                on_cancel: Message::CancelClear,
            };
            stack::Stack::new()
                .push(main)
                .push(modal::view(&clear_modal))
                .into()
        } else {
            main.into()
        }
    }
}

impl App {
    fn start_compression(&mut self) -> Command<Message> {
        let total = self.files.len();
        if total == 0 {
            self.state = AppState::Idle;
            return Command::none();
        }

        self.state = AppState::Compressing { completed: 0, total };
        let cancel_flag = Arc::new(AtomicBool::new(false));
        self.cancel_flag = Some(cancel_flag.clone());

        // 使用 Semaphore 限制最多 4 个并行压缩任务，避免高并发导致资源耗尽
        let semaphore = Arc::new(tokio::sync::Semaphore::new(4));

        // 并行发起所有压缩任务
        let cmds: Vec<Command<Message>> = self
            .files
            .iter()
            .enumerate()
            .map(|(i, path)| {
                let path = path.clone();
                let config = self.config.clone();
                let flag = cancel_flag.clone();
                let sem = semaphore.clone();
                Command::perform(
                    compress_single(i, total, path, config, flag, sem),
                    move |(idx, tot, row, out)| Message::CompressProgress(idx, tot, row, out),
                )
            })
            .collect();

        Command::batch(cmds)
    }
}

fn toast_timeout() -> Command<Message> {
    Command::perform(
        async move {
            tokio::time::sleep(std::time::Duration::from_secs(3)).await;
        },
        |_| Message::ToastTimeout,
    )
}

async fn select_files() -> Vec<PathBuf> {
    let files = rfd::AsyncFileDialog::new()
        .add_filter("Images", SUPPORTED_EXTENSIONS)
        .pick_files()
        .await;

    files
        .map(|f| f.into_iter().map(|f| f.path().to_path_buf()).collect())
        .unwrap_or_default()
}

async fn select_output_dir() -> settings::Message {
    let dir = rfd::AsyncFileDialog::new().pick_folder().await;
    match dir {
        Some(d) => settings::Message::CustomOutputDirSelected(d.path().to_path_buf()),
        None => settings::Message::CustomOutputDirSelected(PathBuf::new()),
    }
}

fn resolve_output_dir(config: &Config, path: &Path) -> PathBuf {
    match config.output_mode {
        OutputMode::Timestamped | OutputMode::Custom => config.resolve_output_dir(Some(path)),
        OutputMode::SameDir => path
            .parent()
            .map(Path::to_path_buf)
            .unwrap_or_else(|| Config::base_output_dir().join("same_dir")),
    }
}

/// 递归扫描目录中的图片文件。最多递归 10 层，最多收集 10000 个文件。
fn collect_images_from_dir(dir: &Path) -> Vec<PathBuf> {
    collect_images_from_dir_depth(dir, 0)
}

fn collect_images_from_dir_depth(dir: &Path, depth: u32) -> Vec<PathBuf> {
    const MAX_DEPTH: u32 = 10;
    const MAX_FILES: usize = 10000;

    if depth > MAX_DEPTH {
        return Vec::new();
    }

    let mut files = Vec::new();
    let supported = SUPPORTED_EXTENSIONS;
    if let Ok(entries) = std::fs::read_dir(dir) {
        for entry in entries.flatten() {
            if files.len() >= MAX_FILES {
                break;
            }
            let path = entry.path();
            if path.is_dir() {
                files.extend(collect_images_from_dir_depth(&path, depth + 1));
            } else if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
                if supported.contains(&ext.to_lowercase().as_str()) {
                    files.push(path);
                }
            }
        }
    }
    files
}

/// 检测系统是否为深色模式。
fn detect_system_is_dark() -> bool {
    match dark_light::detect() {
        dark_light::Mode::Dark => true,
        _ => false,
    }
}

async fn compress_single(
    index: usize,
    total: usize,
    path: PathBuf,
    config: Config,
    cancel_flag: Arc<AtomicBool>,
    semaphore: Arc<tokio::sync::Semaphore>,
) -> (usize, usize, Option<result_table::Row>, Option<PathBuf>) {
    // 获取并发许可，最多 4 个任务同时运行
    let _permit = semaphore.acquire().await.unwrap_or_else(|_| {
        // semaphore 已关闭时添加回一个 permit 并继续
        semaphore.add_permits(1);
        semaphore.try_acquire().unwrap()
    });

    // 快速检查取消标志
    if cancel_flag.load(Ordering::Relaxed) {
        return (index, total, None, None);
    }

    let fallback_name = path
        .file_name()
        .map(|n| n.to_string_lossy().to_string())
        .unwrap_or_else(|| "未知文件".to_string());
    let panic_name = fallback_name.clone();

    let result = tokio::task::spawn_blocking(move || {
        // 用 catch_unwind 保护，防止 Rust panic 导致整个进程终止
        let panic_path = path.clone();
        let output_dir = resolve_output_dir(&config, &path);
        let fallback_name2 = fallback_name.clone();
        let output_dir2 = output_dir.clone();

        let inner = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        // 检查是否已被取消
        if cancel_flag.load(Ordering::Relaxed) {
            return (None, None);
        }

        if let Err(e) = std::fs::create_dir_all(&output_dir) {
            return (Some(result_table::Row {
                name: fallback_name,
                original_size: 0,
                compressed_size: 0,
                status: Err(format!("创建输出目录失败: {}", e)),
                input_path: Some(path.clone()),
                output_path: None,
            }), Some(output_dir));
        }

        match imagemin_core::compress_image(
            &path, &output_dir, &config.quality, config.png_lossless,
            config.output_format, config.max_width, config.max_height,
            config.strip_metadata,
        ) {
            Ok(result) => {
                let output_parent = result.output_path.parent().map(Path::to_path_buf)
                    .unwrap_or_else(|| output_dir.clone());
                let row = result_table::Row {
                    name: result.name,
                    original_size: result.original_size,
                    compressed_size: result.compressed_size,
                    status: Ok(()),
                    input_path: Some(path.clone()),
                    output_path: Some(result.output_path.clone()),
                };
                (Some(row), Some(output_parent))
            }
            Err(e) => {
                let row = result_table::Row {
                    name: fallback_name,
                    original_size: 0,
                    compressed_size: 0,
                    status: Err(e.to_string()),
                    input_path: Some(path.clone()),
                    output_path: None,
                };
                (Some(row), Some(output_dir))
            }
        }
        })); // 关闭 catch_unwind

        // catch_unwind 失败时返回 panic 信息
        match inner {
            Ok(r) => r,
            Err(panic) => {
                let msg = if let Some(s) = panic.downcast_ref::<&str>() {
                    s.to_string()
                } else if let Some(s) = panic.downcast_ref::<String>() {
                    s.clone()
                } else {
                    "压缩线程发生未知 panic".to_string()
                };
                (Some(result_table::Row {
                    name: fallback_name2,
                    original_size: 0, compressed_size: 0,
                    status: Err(format!("压缩线程崩溃: {}", msg)),
                    input_path: Some(panic_path),
                    output_path: None,
                }), Some(output_dir2))
            }
        }
    }) // 关闭 spawn_blocking
    .await
    .unwrap_or_else(|e| {
        (
            Some(result_table::Row {
                name: panic_name,
                original_size: 0,
                compressed_size: 0,
                status: Err(format!("压缩任务异常: {}", e)),
                input_path: None,
                output_path: None,
            }),
            None,
        )
    });

    (index, total, result.0, result.1)
}
