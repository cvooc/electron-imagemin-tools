use iced::widget::{button, checkbox, column, container, row, scrollable, slider, text};
use iced::{Element, Length};
use imagemin_core::{Config, OutputMode, ThemeMode};

#[derive(Debug, Clone)]
pub enum Message {
    JpegChanged(u8),
    PngChanged(u8),
    PngLosslessChanged(bool),
    OutputModeChanged(OutputMode),
    ThemeChanged(ThemeMode),
    SelectCustomOutputDir,
    CustomOutputDirSelected(std::path::PathBuf),
}

pub fn view(config: &Config) -> Element<'static, Message> {
    let jpeg_slider = row![
        text(format!("JPEG (当前质量: {})", config.quality.jpeg)).width(Length::FillPortion(1)),
        slider(0..=100, config.quality.jpeg, Message::JpegChanged).width(Length::FillPortion(2)),
    ]
    .spacing(12);

    let png_slider = row![
        text(format!("PNG (当前质量: {})", config.quality.png)).width(Length::FillPortion(1)),
        slider(0..=100, config.quality.png, Message::PngChanged).width(Length::FillPortion(2)),
    ]
    .spacing(12);

    let png_lossless = checkbox("PNG 纯无损优化（不启用有损量化）", config.png_lossless)
        .on_toggle(Message::PngLosslessChanged);

    let theme_buttons = row![
        theme_button("跟随系统", ThemeMode::System, config.theme),
        theme_button("浅色", ThemeMode::Light, config.theme),
        theme_button("深色", ThemeMode::Dark, config.theme),
    ]
    .spacing(8);

    let mode_buttons = row![
        mode_button("时间戳子目录", OutputMode::Timestamped, config.output_mode),
        mode_button("与输入文件同目录", OutputMode::SameDir, config.output_mode),
        mode_button("自定义目录", OutputMode::Custom, config.output_mode),
    ]
    .spacing(8);

    let custom_dir_text = config
        .custom_output_dir
        .as_ref()
        .map(|p| p.to_string_lossy().to_string())
        .unwrap_or_else(|| "未选择".to_string());
    let custom_dir_row = row![
        text(format!("自定义目录: {}", custom_dir_text)).width(Length::FillPortion(2)),
        button(text("选择目录")).on_press(Message::SelectCustomOutputDir),
    ]
    .spacing(12);

    let version = env!("CARGO_PKG_VERSION");
    let content = column![
        text("主题").size(18),
        theme_buttons,
        text(""),
        text("压缩质量").size(18),
        jpeg_slider,
        png_slider,
        png_lossless,
        text("压缩质量推荐设为80").size(12),
        text(""),
        text("输出设置").size(18),
        mode_buttons,
        custom_dir_row,
        text(""),
        text("关于本软件").size(16),
        text(format!("版本: {}", version)),
        text("GITHUB: https://github.com/ShowMeBaby/electron-imagemin-tools"),
    ]
    .spacing(16)
    .padding(20);

    container(scrollable(content))
        .width(Length::Fill)
        .height(Length::Fill)
        .center_x()
        .into()
}

fn mode_button(label: &'static str, mode: OutputMode, current: OutputMode) -> Element<'static, Message> {
    let active = mode == current;
    let btn = button(text(label));
    if active {
        btn.style(iced::theme::Button::Primary).into()
    } else {
        btn.on_press(Message::OutputModeChanged(mode)).into()
    }
}

fn theme_button(label: &'static str, mode: ThemeMode, current: ThemeMode) -> Element<'static, Message> {
    let active = mode == current;
    let btn = button(text(label));
    if active {
        btn.style(iced::theme::Button::Primary).into()
    } else {
        btn.on_press(Message::ThemeChanged(mode)).into()
    }
}
