use iced::widget::{column, container};
use iced::{window, Application, Command, Element, Length, Subscription, Theme};
use std::path::{Path, PathBuf};

use crate::views::{drop_zone, header, progress, result_table, settings};
use imagemin_core::{Config, OutputMode};

#[derive(Debug, Clone)]
pub enum AppState {
    Idle,
    Compressing { current: usize, total: usize },
    Completed,
    Settings,
}

pub struct App {
    state: AppState,
    config: Config,
    files: Vec<PathBuf>,
    hovered_file: Option<PathBuf>,
    output_dir: Option<PathBuf>,
    results: Vec<result_table::Row>,
}

#[derive(Debug, Clone)]
pub enum Message {
    Header(header::Message),
    DropZone(drop_zone::Message),
    ResultTable(result_table::Message),
    Settings(settings::Message),
    FilesSelected(Vec<PathBuf>),
    FileHovered(PathBuf),
    FileDropped(PathBuf),
    FileHoveredLeft,
    CompressProgress(usize, usize, Option<result_table::Row>, Option<PathBuf>),
}

impl Application for App {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        let config = Config::load();

        (
            Self {
                state: AppState::Idle,
                config,
                files: Vec::new(),
                hovered_file: None,
                output_dir: None,
                results: Vec::new(),
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("retrocode.io压图")
    }

    fn subscription(&self) -> Subscription<Message> {
        iced::event::listen_with(|event, _status| match event {
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
        })
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
            Message::Header(header::Message::Minimize) => {
                window::minimize(window::Id::MAIN, true)
            }
            Message::Header(header::Message::Close) => {
                std::process::exit(0);
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
                let ext = path
                    .extension()
                    .and_then(|e| e.to_str())
                    .unwrap_or("")
                    .to_lowercase();

                if matches!(ext.as_str(), "jpg" | "jpeg" | "png" | "gif" | "svg" | "webp") {
                    self.files = vec![path];
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
                    self.results.push(row);
                }
                if index == 0 {
                    self.output_dir = output_dir;
                }

                let next = index + 1;
                if next < total {
                    self.state = AppState::Compressing {
                        current: next,
                        total,
                    };
                    let files = self.files.clone();
                    let config = self.config.clone();
                    Command::perform(compress_next(next, files, config), |(
                        idx,
                        tot,
                        row,
                        out,
                    )| {
                        Message::CompressProgress(idx, tot, row, out)
                    })
                } else {
                    self.state = AppState::Completed;
                    Command::none()
                }
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
                self.results.clear();
                self.files.clear();
                self.output_dir = None;
                self.state = AppState::Idle;
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
        }
    }

    fn view(&self) -> Element<'_, Message> {
        let header = header::view().map(Message::Header);

        let content = match &self.state {
            AppState::Idle => {
                if self.hovered_file.is_some() {
                    drop_zone::view_hovered().map(Message::DropZone)
                } else {
                    drop_zone::view().map(Message::DropZone)
                }
            }
            AppState::Compressing { current, total } => progress::view(*current, *total)
                .map(|_| Message::DropZone(drop_zone::Message::SelectFiles)),
            AppState::Completed => result_table::view(&self.results, self.output_dir.is_some())
                .map(Message::ResultTable),
            AppState::Settings => settings::view(&self.config).map(Message::Settings),
        };

        container(column![header, content].width(Length::Fill).height(Length::Fill))
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
}

impl App {
    fn start_compression(&mut self) -> Command<Message> {
        let total = self.files.len();
        if total == 0 {
            self.state = AppState::Idle;
            return Command::none();
        }

        self.state = AppState::Compressing { current: 0, total };
        let files = self.files.clone();
        let config = self.config.clone();
        Command::perform(compress_next(0, files, config), |(idx, tot, row, out)| {
            Message::CompressProgress(idx, tot, row, out)
        })
    }
}

async fn select_files() -> Vec<PathBuf> {
    let files = rfd::AsyncFileDialog::new()
        .add_filter("Images", &["jpg", "jpeg", "png", "gif", "svg", "webp"])
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

async fn compress_next(
    index: usize,
    files: Vec<PathBuf>,
    config: Config,
) -> (usize, usize, Option<result_table::Row>, Option<PathBuf>) {
    let total = files.len();
    if index >= total {
        return (index, total, None, None);
    }

    let path = files[index].clone();
    let fallback_name = path
        .file_name()
        .map(|n| n.to_string_lossy().to_string())
        .unwrap_or_else(|| "未知文件".to_string());
    let panic_name = fallback_name.clone();

    let result = tokio::task::spawn_blocking(move || {
        let output_dir = resolve_output_dir(&config, &path);

        if let Err(e) = std::fs::create_dir_all(&output_dir) {
            return (
                Some(result_table::Row {
                    name: fallback_name,
                    original_size: 0,
                    compressed_size: 0,
                    status: Err(format!("创建输出目录失败: {}", e)),
                }),
                Some(output_dir),
            );
        }

        match imagemin_core::compress_image(&path, &output_dir, &config.quality, config.png_lossless)
        {
            Ok(result) => {
                let output_parent = result
                    .output_path
                    .parent()
                    .map(Path::to_path_buf)
                    .unwrap_or_else(|| output_dir.clone());
                let row = result_table::Row {
                    name: result.name,
                    original_size: result.original_size,
                    compressed_size: result.compressed_size,
                    status: Ok(()),
                };
                (Some(row), Some(output_parent))
            }
            Err(e) => {
                let row = result_table::Row {
                    name: fallback_name,
                    original_size: 0,
                    compressed_size: 0,
                    status: Err(e.to_string()),
                };
                (Some(row), Some(output_dir))
            }
        }
    })
    .await
    .unwrap_or_else(|e| {
        (
            Some(result_table::Row {
                name: panic_name,
                original_size: 0,
                compressed_size: 0,
                status: Err(format!("压缩任务异常: {}", e)),
            }),
            None,
        )
    });

    (index, total, result.0, result.1)
}
