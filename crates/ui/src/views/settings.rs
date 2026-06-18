use iced::widget::{column, container, row, slider, text};
use iced::{Element, Length};

#[derive(Debug, Clone)]
pub enum Message {
    JpegChanged(u8),
    PngChanged(u8),
}

pub fn view(quality: &imagemin_core::Quality) -> Element<'static, Message> {
    let jpeg_slider = row![
        text(format!("JPEG (当前质量: {})", quality.jpeg)).width(Length::FillPortion(1)),
        slider(0..=100, quality.jpeg, Message::JpegChanged).width(Length::FillPortion(2)),
    ]
    .spacing(12);

    let png_slider = row![
        text(format!("PNG (当前质量: {})", quality.png)).width(Length::FillPortion(1)),
        slider(21..=100, quality.png, Message::PngChanged).width(Length::FillPortion(2)),
    ]
    .spacing(12);

    let content = column![
        text("压缩质量").size(18),
        jpeg_slider,
        png_slider,
        text("压缩质量推荐设为80").size(12),
        text(""),
        text("关于本软件").size(16),
        text("GITHUB: https://github.com/ShowMeBaby/electron-imagemin-tools"),
    ]
    .spacing(16)
    .padding(20);

    container(content)
        .width(Length::Fill)
        .height(Length::Fill)
        .center_x()
        .center_y()
        .into()
}
