use iced::widget::{button, column, container, text};
use iced::{Background, Border, Color, Element, Length, Theme};

#[derive(Debug, Clone)]
pub enum Message {
    SelectFiles,
}

fn drop_zone_style(_theme: &Theme) -> container::Appearance {
    container::Appearance {
        background: Some(Background::Color(Color::WHITE)),
        border: Border {
            color: Color::from_rgb(0.6, 0.6, 0.6),
            width: 2.0,
            radius: 8.0.into(),
        },
        text_color: Some(Color::from_rgb(0.3, 0.3, 0.3)),
        ..Default::default()
    }
}

pub fn view(file_count: usize) -> Element<'static, Message> {
    let hint = if file_count > 0 {
        format!("已选择 {} 个文件", file_count)
    } else {
        String::from("点击选择图片 / 拖放图片或文件夹")
    };

    let content = column![
        text(hint).size(20),
        text("支持格式：JPG/PNG/GIF/SVG/WebP").size(14),
    ]
    .spacing(12);

    let inner = container(content)
        .width(Length::Fill)
        .height(Length::Fill)
        .center_x()
        .center_y()
        .style(drop_zone_style);

    button(inner)
        .width(Length::Fill)
        .height(Length::Fill)
        .style(iced::theme::Button::Text)
        .on_press(Message::SelectFiles)
        .into()
}

pub fn view_hovered() -> Element<'static, Message> {
    let content = column![
        text("释放鼠标以添加图片").size(20),
        text("支持格式：JPG/PNG/GIF/SVG/WebP").size(14),
    ]
    .spacing(12);

    let inner = container(content)
        .width(Length::Fill)
        .height(Length::Fill)
        .center_x()
        .center_y()
        .style(drop_zone_style);

    button(inner)
        .width(Length::Fill)
        .height(Length::Fill)
        .style(iced::theme::Button::Text)
        .on_press(Message::SelectFiles)
        .into()
}
