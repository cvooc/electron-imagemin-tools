use iced::widget::{column, container, progress_bar, text};
use iced::{Element, Length, Theme};

fn progress_style(_theme: &Theme) -> container::Appearance {
    container::Appearance {
        background: Some(iced::Background::Color(iced::Color::WHITE)),
        ..Default::default()
    }
}

pub fn view(current: usize, total: usize, current_file: &str) -> Element<'static, ()> {
    let progress = if total > 0 {
        current as f32 / total as f32
    } else {
        0.0
    };

    let content = column![
        text("正在压缩...").size(24),
        text(current_file).size(14),
        text(format!("{}/{}", current, total)).size(16),
        progress_bar(0.0..=1.0, progress).width(Length::Fill),
    ]
    .spacing(20);

    container(content)
        .width(Length::Fill)
        .height(Length::Fill)
        .center_x()
        .center_y()
        .style(progress_style)
        .into()
}
