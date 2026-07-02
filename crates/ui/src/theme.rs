//! Fluent 设计系统常量 — 颜色 / 阴影 / 圆角 / 字体
#![allow(dead_code)]
use iced::widget::container;
use iced::{Background, Border, Color, Shadow, Vector};

// ========== 色板 ==========
/// Fluent accent 蓝色
pub const ACCENT: Color = Color::from_rgb(0.0, 0.47, 0.83);
pub const ACCENT_HOVER: Color = Color::from_rgb(0.0, 0.39, 0.71);
pub const ACCENT_LIGHT: Color = Color::from_rgb(0.9, 0.94, 0.98);

/// 浅色主题
pub const SURFACE_LIGHT: Color = Color::from_rgb(0.95, 0.95, 0.95);
pub const CARD_LIGHT: Color = Color::from_rgb(1.0, 1.0, 1.0);

/// 深色主题
pub const SURFACE_DARK: Color = Color::from_rgb(0.11, 0.11, 0.12);
pub const CARD_DARK: Color = Color::from_rgb(0.17, 0.17, 0.18);

/// 状态色
pub const SUCCESS: Color = Color::from_rgb(0.12, 0.59, 0.29);
pub const SUCCESS_BG: Color = Color::from_rgb(0.94, 0.98, 0.94);
pub const ERROR: Color = Color::from_rgb(0.8, 0.2, 0.2);
pub const ERROR_BG: Color = Color::from_rgb(1.0, 0.94, 0.94);
pub const BORDER_COLOR: Color = Color::from_rgb(0.88, 0.88, 0.88);

// ========== 圆角 ==========
pub const RADIUS_SMALL: f32 = 4.0;
pub const RADIUS_MEDIUM: f32 = 8.0;
pub const RADIUS_LARGE: f32 = 12.0;

// ========== 阴影 ==========
pub fn shadow_card() -> Shadow {
    Shadow {
        color: Color::from_rgba(0.0, 0.0, 0.0, 0.06),
        offset: Vector::new(0.0, 1.0),
        blur_radius: 4.0,
    }
}

pub fn shadow_modal() -> Shadow {
    Shadow {
        color: Color::from_rgba(0.0, 0.0, 0.0, 0.15),
        offset: Vector::new(0.0, 4.0),
        blur_radius: 16.0,
    }
}

// ========== 容器工厂函数 ==========
pub fn container_surface(is_dark: bool) -> container::Appearance {
    container::Appearance {
        background: Some(Background::Color(if is_dark { SURFACE_DARK } else { SURFACE_LIGHT })),
        border: Border { ..Default::default() },
        ..Default::default()
    }
}

pub fn container_card(is_dark: bool) -> container::Appearance {
    container::Appearance {
        background: Some(Background::Color(if is_dark { CARD_DARK } else { CARD_LIGHT })),
        border: Border {
            color: BORDER_COLOR,
            width: 1.0,
            radius: RADIUS_MEDIUM.into(),
        },
        shadow: shadow_card(),
        ..Default::default()
    }
}
