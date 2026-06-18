use iced::widget::{column, container, text};
use iced::{Element, Length, Theme};

fn progress_style(_theme: &Theme) -> container::Appearance {
    container::Appearance {
        background: Some(iced::Background::Color(iced::Color::WHITE)),
        ..Default::default()
    }
}

pub fn view() -> Element<'static, ()> {
    let content = column![
        text("正在压缩...").size(24),
        text("请稍候").size(16),
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
