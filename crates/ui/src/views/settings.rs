use iced::widget::{button, checkbox, column, container, row, scrollable, slider, text, text_input};
use iced::{Element, Length};
use imagemin_core::{Config, OutputFormat, OutputMode, ThemeMode};

use crate::theme;

#[derive(Debug, Clone)]
pub enum Message {
    JpegChanged(u8),
    PngChanged(u8),
    PngLosslessChanged(bool),
    OutputModeChanged(OutputMode),
    ThemeChanged(ThemeMode),
    SelectCustomOutputDir,
    CustomOutputDirSelected(std::path::PathBuf),
    MaxWidthChanged(String),
    MaxHeightChanged(String),
    StripMetadataChanged(bool),
    OutputFormatChanged(OutputFormat),
}

fn card_style(_theme: &iced::Theme) -> container::Appearance {
    container::Appearance {
        background: Some(iced::Background::Color(iced::Color::WHITE)),
        border: iced::Border { radius: 8.0.into(), ..Default::default() },
        shadow: theme::shadow_card(),
        ..Default::default()
    }
}

fn card_container(content: impl Into<Element<'static, Message>>) -> container::Container<'static, Message> {
    container(content).style(card_style).padding(16)
}

fn section_title(s: &str) -> text::Text<'static, iced::Theme> {
    text(s).size(18)
}

pub fn view(config: &Config) -> Element<'static, Message> {
    let jpeg_slider = row![
        text(format!("JPEG 质量: {}", config.quality.jpeg)).width(Length::FillPortion(1)),
        slider(0..=100, config.quality.jpeg, Message::JpegChanged).width(Length::FillPortion(2)),
    ]
    .spacing(12);

    let png_slider = row![
        text(format!("PNG 质量: {}", config.quality.png)).width(Length::FillPortion(1)),
        slider(0..=100, config.quality.png, Message::PngChanged).width(Length::FillPortion(2)),
    ]
    .spacing(12);

    let quality_note = text("JPEG 质量越高文件越大；PNG 质量影响颜色精度").size(12);

    let png_lossless = checkbox("PNG 纯无损优化", config.png_lossless)
        .on_toggle(Message::PngLosslessChanged);

    let quality_card = card_container(column![
        section_title("压缩质量"),
        jpeg_slider,
        png_slider,
        png_lossless,
        text("推荐设为 80").size(12),
        quality_note,
    ].spacing(12));

    let theme_buttons = row![
        theme_button("跟随系统", ThemeMode::System, config.theme),
        theme_button("浅色", ThemeMode::Light, config.theme),
        theme_button("深色", ThemeMode::Dark, config.theme),
    ]
    .spacing(8);

    let theme_card = card_container(column![
        section_title("主题"),
        theme_buttons,
    ].spacing(12));

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
        text(format!("目录: {}", custom_dir_text)).width(Length::FillPortion(2)),
        button(text("选择")).on_press(Message::SelectCustomOutputDir),
    ]
    .spacing(12);

    let output_card = card_container(column![
        section_title("输出设置"),
        mode_buttons,
        custom_dir_row,
    ].spacing(12));

    // 尺寸限制卡片
    let max_w_str = config.max_width.map(|v| v.to_string()).unwrap_or_default();
    let max_h_str = config.max_height.map(|v| v.to_string()).unwrap_or_default();

    let resize_card_content = column![
        section_title("图片尺寸"),
        row![
            text("最大宽度: ").width(Length::FillPortion(1)),
            text_input("不限制", &max_w_str)
                .on_input(Message::MaxWidthChanged)
                .width(Length::FillPortion(1)),
            text("px").size(12),
        ].spacing(8).align_items(iced::Alignment::Center),
        row![
            text("最大高度: ").width(Length::FillPortion(1)),
            text_input("不限制", &max_h_str)
                .on_input(Message::MaxHeightChanged)
                .width(Length::FillPortion(1)),
            text("px").size(12),
        ].spacing(8).align_items(iced::Alignment::Center),
        text("留空则不限制，超出时等比缩小").size(12),
    ].spacing(8);

    let resize_card = card_container(resize_card_content);

    // 高级卡片
    let output_fmt_buttons = row![
        fmt_button("原格式", OutputFormat::Original, config.output_format),
        fmt_button("JPEG", OutputFormat::Jpeg, config.output_format),
        fmt_button("PNG", OutputFormat::Png, config.output_format),
        fmt_button("WebP", OutputFormat::WebP, config.output_format),
        fmt_button("AVIF", OutputFormat::Avif, config.output_format),
    ].spacing(4);

    let advanced_card = card_container(column![
        section_title("高级"),
        checkbox("剥离元数据 (EXIF/XMP/ICC)", config.strip_metadata)
            .on_toggle(Message::StripMetadataChanged),
        text("输出格式").size(14),
        output_fmt_buttons,
    ].spacing(8));

    let version = env!("CARGO_PKG_VERSION");
    let about_card = card_container(column![
        section_title("关于"),
        text(format!("版本: {}", version)),
        text("GITHUB: https://github.com/cvooc/electron-imagemin-tools"),
    ].spacing(8));

    let content = column![
        theme_card,
        quality_card,
        output_card,
        resize_card,
        advanced_card,
        about_card,
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

fn fmt_button(label: &'static str, mode: OutputFormat, current: OutputFormat) -> Element<'static, Message> {
    let active = mode == current;
    let btn = button(text(label));
    if active {
        btn.style(iced::theme::Button::Primary).into()
    } else {
        btn.on_press(Message::OutputFormatChanged(mode)).into()
    }
}
