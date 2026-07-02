use iced::widget::{button, column, container, horizontal_space, row, text};
use iced::{Background, Border, Color, Element, Length};

use crate::theme;

/// Modal 确认对话框
#[derive(Debug, Clone)]
pub struct Modal<M: Clone> {
    pub title: String,
    pub message: String,
    pub confirm_label: String,
    pub cancel_label: String,
    pub on_confirm: M,
    pub on_cancel: M,
}

fn card_style(_theme: &iced::Theme) -> container::Appearance {
    container::Appearance {
        background: Some(Background::Color(Color::WHITE)),
        border: Border {
            radius: 8.0.into(),
            ..Default::default()
        },
        shadow: theme::shadow_modal(),
        ..Default::default()
    }
}

fn overlay_style(_theme: &iced::Theme) -> container::Appearance {
    container::Appearance {
        background: Some(Background::Color(Color::from_rgba(0.0, 0.0, 0.0, 0.5))),
        ..Default::default()
    }
}

/// 渲染全屏遮罩 + 居中对话框
pub fn view<'a, M: Clone + 'a>(modal: &Modal<M>) -> Element<'a, M> {
    let card = column![
        text(&modal.title).size(18),
        text(&modal.message).size(14),
        row![
            button(text(&modal.cancel_label))
                .on_press(modal.on_cancel.clone())
                .style(iced::theme::Button::Secondary),
            horizontal_space(),
            button(text(&modal.confirm_label))
                .on_press(modal.on_confirm.clone())
                .style(iced::theme::Button::Destructive),
        ]
        .spacing(16),
    ]
    .spacing(16)
    .padding(24);

    let dialog = container(card).style(card_style).padding(8);

    // 全屏半透明遮罩 + 居中卡片
    container(dialog)
        .width(Length::Fill)
        .height(Length::Fill)
        .center_x()
        .center_y()
        .style(overlay_style)
        .into()
}
