use iced::widget::{button, column, container, progress_bar, text, text::Shaping};
use iced::{Element, Length, Theme};

use crate::app::Message;

fn progress_style(_theme: &Theme) -> container::Appearance {
    container::Appearance {
        ..Default::default()
    }
}

pub fn view(current: usize, total: usize, current_file: &str) -> Element<'static, Message> {
    let pct = if total > 0 { current as f32 / total as f32 } else { 0.0 };

    let content = column![
        text("⏳ 正在压缩...").shaping(Shaping::Advanced).size(22),
        text(current_file).size(14),
        text(format!("{}/{}", current, total)).size(16),
        progress_bar(0.0..=1.0, pct).width(Length::Fill),
        button(text("取消压缩"))
            .on_press(Message::CancelCompression)
            .style(iced::theme::Button::Secondary),
    ]
    .spacing(20)
    .align_items(iced::Alignment::Center);

    container(content)
        .width(Length::Fill)
        .height(Length::Fill)
        .center_x()
        .center_y()
        .style(progress_style)
        .into()
}
