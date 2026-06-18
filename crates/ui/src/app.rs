use iced::widget::{column, container};
use iced::{window, Application, Command, Element, Length, Subscription, Theme};
use std::path::PathBuf;

use crate::views::{drop_zone, header, progress, result_table, settings};

#[derive(Debug, Clone)]
pub enum AppState {
    Idle,
    Compressing,
    Completed(Vec<result_table::CompressResult>),
    Settings,
}

pub struct App {
    state: AppState,
    quality: imagemin_core::Quality,
    files: Vec<PathBuf>,
    hovered_file: Option<PathBuf>,
    output_dir: Option<PathBuf>,
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
    CompressFinished(PathBuf, Vec<result_table::CompressResult>),
}

impl Application for App {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        (
            Self {
                state: AppState::Idle,
                quality: imagemin_core::Quality::default(),
                files: Vec::new(),
                hovered_file: None,
                output_dir: None,
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
            Message::Header(header::Message::Drag) => {
                window::drag(window::Id::MAIN)
            }
            Message::DropZone(drop_zone::Message::SelectFiles) => {
                Command::perform(select_files(), Message::FilesSelected)
            }
            Message::FilesSelected(files) => {
                if !files.is_empty() {
                    self.files = files;
                    self.state = AppState::Compressing;
                    let files = self.files.clone();
                    let quality = self.quality.clone();
                    Command::perform(compress_files(files, quality), |(output_dir, results)| {
                        Message::CompressFinished(output_dir, results)
                    })
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

                if matches!(ext.as_str(), "jpg" | "jpeg" | "png" | "gif" | "svg") {
                    self.files = vec![path];
                    self.state = AppState::Compressing;
                    let files = self.files.clone();
                    let quality = self.quality.clone();
                    Command::perform(compress_files(files, quality), |(output_dir, results)| {
                        Message::CompressFinished(output_dir, results)
                    })
                } else {
                    Command::none()
                }
            }
            Message::FileHoveredLeft => {
                self.hovered_file = None;
                Command::none()
            }
            Message::CompressFinished(output_dir, results) => {
                self.state = AppState::Completed(results);
                self.output_dir = Some(output_dir);
                Command::none()
            }
            Message::ResultTable(result_table::Message::OpenOutputDir) => {
                if let Some(dir) = &self.output_dir {
                    open::that(dir).ok();
                }
                Command::none()
            }
            Message::Settings(settings::Message::JpegChanged(q)) => {
                self.quality.jpeg = q;
                Command::none()
            }
            Message::Settings(settings::Message::PngChanged(q)) => {
                self.quality.png = q;
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
            AppState::Compressing => {
                progress::view().map(|_| Message::DropZone(drop_zone::Message::SelectFiles))
            }
            AppState::Completed(results) => result_table::view(results).map(Message::ResultTable),
            AppState::Settings => settings::view(&self.quality).map(Message::Settings),
        };

        container(column![header, content].width(Length::Fill).height(Length::Fill))
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
}

async fn select_files() -> Vec<PathBuf> {
    let files = rfd::AsyncFileDialog::new()
        .add_filter("Images", &["jpg", "jpeg", "png", "gif", "svg"])
        .pick_files()
        .await;

    files
        .map(|f| f.into_iter().map(|f| f.path().to_path_buf()).collect())
        .unwrap_or_default()
}

async fn compress_files(
    files: Vec<PathBuf>,
    quality: imagemin_core::Quality,
) -> (PathBuf, Vec<result_table::CompressResult>) {
    let output_dir = imagemin_core::Config::output_dir();
    let timestamp = chrono::Local::now().format("%Y-%m-%d-%H_%M_%S").to_string();
    let output = output_dir.join(timestamp);

    if let Err(e) = std::fs::create_dir_all(&output) {
        eprintln!("创建输出目录失败: {}", e);
        return (output, Vec::new());
    }

    let results = imagemin_core::compress_images(&files, &output, &quality);

    let results: Vec<result_table::CompressResult> = results
        .into_iter()
        .filter_map(|r| match r {
            Ok(result) => Some(result_table::CompressResult {
                name: result.name,
                original_size: result.original_size,
                compressed_size: result.compressed_size,
            }),
            Err(e) => {
                eprintln!("压缩失败: {}", e);
                None
            }
        })
        .collect();

    (output, results)
}
