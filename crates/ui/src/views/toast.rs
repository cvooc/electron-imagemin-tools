use iced::widget::{container, text};
use iced::{Color, Element};

/// Toast 类型
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ToastKind {
    Success,
    Info,
}

/// Toast 消息
#[derive(Debug, Clone)]
pub struct Toast {
    pub message: String,
    pub kind: ToastKind,
}

impl Toast {
    pub fn success(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            kind: ToastKind::Success,
        }
    }

    pub fn info(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            kind: ToastKind::Info,
        }
    }
}

fn toast_bg(kind: ToastKind) -> Color {
    match kind {
        ToastKind::Success => Color::from_rgb(0.2, 0.7, 0.3),
        ToastKind::Info => Color::from_rgb(0.2, 0.5, 0.8),
    }
}

pub fn view(toast: &Toast) -> Element<'static, ()> {
    let bg = toast_bg(toast.kind);

    let toast_content = text(&toast.message)
        .size(14)
        .style(Color::WHITE);

    let appearance = move |_theme: &iced::Theme| container::Appearance {
        background: Some(iced::Background::Color(bg)),
        border: iced::Border {
            radius: 6.0.into(),
            ..Default::default()
        },
        ..Default::default()
    };

    container(toast_content)
        .padding([8, 16])
        .style(iced::theme::Container::Custom(Box::new(appearance)))
        .into()
}
