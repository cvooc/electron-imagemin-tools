#![windows_subsystem = "windows"]

use iced::Application;
use std::borrow::Cow;
use std::path::PathBuf;

mod app;
mod theme;
mod util;
mod views;

/// 加载系统字体。返回 (所有可用字体数据, 默认字体族)。
/// Segoe UI 作为默认（Fluent 风格），中文字体作 fallback。
fn load_system_font() -> (Vec<Cow<'static, [u8]>>, iced::Font) {
    let candidates: Vec<(PathBuf, &'static str)> = if cfg!(target_os = "windows") {
        vec![
            (PathBuf::from("C:/Windows/Fonts/msyh.ttc"), "Microsoft YaHei"),
            (PathBuf::from("C:/Windows/Fonts/segoeui.ttf"), "Segoe UI"),
            (PathBuf::from("C:/Windows/Fonts/seguiemj.ttf"), "Segoe UI Emoji"),
            (PathBuf::from("C:/Windows/Fonts/simhei.ttf"), "SimHei"),
        ]
    } else if cfg!(target_os = "macos") {
        vec![
            (PathBuf::from("/System/Library/Fonts/SFNSDisplay.ttf"), ".SF NS Display"),
            (PathBuf::from("/System/Library/Fonts/PingFang.ttc"), "PingFang SC"),
        ]
    } else {
        vec![
            (PathBuf::from("/usr/share/fonts/truetype/wqy/wqy-zenhei.ttc"), "WenQuanYi Zen Hei"),
            (PathBuf::from("/usr/share/fonts/opentype/noto/NotoSansCJK-Regular.ttc"), "Noto Sans CJK SC"),
        ]
    };

    let mut fonts: Vec<Cow<'static, [u8]>> = Vec::new();
    let mut default_font = iced::Font {
        family: iced::font::Family::SansSerif,
        ..Default::default()
    };

    for (path, family) in &candidates {
        if let Ok(data) = std::fs::read(path) {
            // 第一个找到的字体设为默认
            if default_font.family == iced::font::Family::SansSerif {
                default_font = iced::Font {
                    family: iced::font::Family::Name(family),
                    ..Default::default()
                };
            }
            fonts.push(Cow::Owned(data));
        }
    }

    (fonts, default_font)
}

fn main() -> iced::Result {
    let (fonts, default_font) = load_system_font();

    let settings = iced::Settings {
        window: iced::window::Settings {
            size: iced::Size::new(800.0, 600.0),
            min_size: Some(iced::Size::new(800.0, 600.0)),
            decorations: false,
            ..Default::default()
        },
        fonts,
        default_font,
        ..Default::default()
    };

    app::App::run(settings)
}
