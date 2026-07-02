use iced::widget::{button, container, horizontal_space, mouse_area, row, text, text::Shaping};
use iced::{font::Weight, Element, Font, Length, Theme};

#[derive(Debug, Clone)]
pub enum Message {
    OpenSettings,
    OpenHistory,
    OpenErrorLog,
    OpenEmojiTest,
    Minimize,
    Close,
    Drag,
}

fn header_style(_theme: &Theme) -> container::Appearance {
    container::Appearance {
        background: Some(iced::Background::Color(iced::Color::from_rgb(0.30, 0.42, 0.48))),
        text_color: Some(iced::Color::WHITE),
        ..Default::default()
    }
}

fn title_text(s: &str) -> text::Text<'static, Theme> {
    text(s).font(Font { weight: Weight::Semibold, ..Default::default() }).shaping(Shaping::Advanced)
}

pub fn view() -> Element<'static, Message> {
    let title = mouse_area(
        row![
            title_text("retrocode.io压图").size(16),
            text(" v1").size(11).style(iced::Color::from_rgba(1.0, 1.0, 1.0, 0.6)),
        ]
        .spacing(4)
        .align_items(iced::Alignment::Center),
    )
    .on_press(Message::Drag);

    let btn = |label: &str, msg: Message| -> Element<'static, Message> {
        button(text(label).shaping(Shaping::Advanced).size(13))
            .on_press(msg)
            .style(iced::theme::Button::Text)
            .into()
    };

    let content = row![
        title,
        horizontal_space(),
        btn("📋 历史", Message::OpenHistory),
        btn("📋 日志", Message::OpenErrorLog),
        btn("🔍 Emoji", Message::OpenEmojiTest),
        btn("设置", Message::OpenSettings),
        btn("—", Message::Minimize),
        btn("✕", Message::Close),
    ]
    .spacing(4)
    .padding([8, 12])
    .align_items(iced::Alignment::Center);

    container(content)
        .width(Length::Fill)
        .style(header_style)
        .into()
}
