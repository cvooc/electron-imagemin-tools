use iced::widget::{button, container, horizontal_space, mouse_area, row, text};
use iced::{Element, Length, Theme};

#[derive(Debug, Clone)]
pub enum Message {
    OpenSettings,
    Minimize,
    Close,
    Drag,
}

fn header_style(_theme: &Theme) -> container::Appearance {
    container::Appearance {
        background: Some(iced::Background::Color(iced::Color::from_rgb(0.37, 0.47, 0.53))),
        text_color: Some(iced::Color::WHITE),
        ..Default::default()
    }
}

pub fn view() -> Element<'static, Message> {
    let title = mouse_area(text("retrocode.io压图").size(18))
        .on_press(Message::Drag);

    let content = row![
        title,
        horizontal_space(),
        button(text("设置").size(14)).on_press(Message::OpenSettings),
        button(text("最小化").size(14)).on_press(Message::Minimize),
        button(text("关闭").size(14)).on_press(Message::Close),
    ]
    .spacing(8)
    .padding([8, 12]);

    container(content)
        .width(Length::Fill)
        .style(header_style)
        .into()
}
