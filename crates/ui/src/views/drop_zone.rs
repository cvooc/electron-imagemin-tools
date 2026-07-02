use iced::widget::{button, column, container, text, text::Shaping};
use iced::{Background, Border, Color, Element, Length, Theme};

use crate::theme;

#[derive(Debug, Clone)]
pub enum Message {
    SelectFiles,
}

fn drop_zone_style(_theme: &Theme) -> container::Appearance {
    container::Appearance {
        background: Some(Background::Color(Color::from_rgb(0.96, 0.96, 0.97))),
        border: Border {
            color: Color::from_rgb(0.7, 0.7, 0.7),
            width: 2.0,
            radius: theme::RADIUS_LARGE.into(),
        },
        text_color: Some(Color::from_rgb(0.4, 0.4, 0.4)),
        ..Default::default()
    }
}

fn drop_zone_hover_style(_theme: &Theme) -> container::Appearance {
    container::Appearance {
        background: Some(Background::Color(theme::ACCENT_LIGHT)),
        border: Border {
            color: theme::ACCENT,
            width: 2.5,
            radius: theme::RADIUS_LARGE.into(),
        },
        text_color: Some(theme::ACCENT_HOVER),
        ..Default::default()
    }
}

fn drop_zone_content(hint: &str, icon: &str, style: fn(&Theme) -> container::Appearance) -> Element<'static, Message> {
    let content = column![
        text(icon).shaping(Shaping::Advanced).size(48),
        text(hint).shaping(Shaping::Advanced).size(16),
        text("支持格式：JPG / PNG / GIF / SVG / WebP / AVIF").shaping(Shaping::Advanced).size(13),
    ]
    .spacing(10)
    .align_items(iced::Alignment::Center);

    let inner = container(content)
        .width(Length::Fill)
        .height(Length::Fill)
        .center_x()
        .center_y()
        .style(style);

    button(inner)
        .width(Length::Fill)
        .height(Length::Fill)
        .style(iced::theme::Button::Text)
        .on_press(Message::SelectFiles)
        .into()
}

pub fn view(file_count: usize) -> Element<'static, Message> {
    let hint = if file_count > 0 {
        format!("📋 已选择 {} 个文件，点击开始压缩", file_count)
    } else {
        String::from("📂 点击选择图片 / 拖放图片或文件夹")
    };
    drop_zone_content(&hint, "📂", drop_zone_style)
}

pub fn view_hovered() -> Element<'static, Message> {
    drop_zone_content("释放鼠标以添加图片", "📥", drop_zone_hover_style)
}
