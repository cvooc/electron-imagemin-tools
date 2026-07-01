#![windows_subsystem = "windows"]

use iced::Application;
use std::borrow::Cow;
use std::path::PathBuf;

mod app;
mod util;
mod views;

/// 尝试从常见系统路径加载中文字体。
/// 返回字体数据与字体族名，用于 iced 初始化。
fn load_system_font() -> (Vec<Cow<'static, [u8]>>, iced::Font) {
    let candidates: Vec<(PathBuf, &'static str)> = if cfg!(target_os = "windows") {
        vec![
            (PathBuf::from("C:/Windows/Fonts/msyh.ttc"), "Microsoft YaHei"),
            (PathBuf::from("C:/Windows/Fonts/simhei.ttf"), "SimHei"),
            (PathBuf::from("C:/Windows/Fonts/simsun.ttc"), "SimSun"),
        ]
    } else if cfg!(target_os = "macos") {
        vec![
            (PathBuf::from("/System/Library/Fonts/PingFang.ttc"), "PingFang SC"),
            (PathBuf::from("/System/Library/Fonts/STHeiti Light.ttc"), "STHeiti"),
            (PathBuf::from("/Library/Fonts/Arial Unicode.ttf"), "Arial Unicode MS"),
        ]
    } else {
        vec![
            (
                PathBuf::from("/usr/share/fonts/truetype/wqy/wqy-zenhei.ttc"),
                "WenQuanYi Zen Hei",
            ),
            (
                PathBuf::from("/usr/share/fonts/opentype/noto/NotoSansCJK-Regular.ttc"),
                "Noto Sans CJK SC",
            ),
            (
                PathBuf::from("/usr/share/fonts/truetype/noto/NotoSansCJK-Regular.ttc"),
                "Noto Sans CJK SC",
            ),
        ]
    };

    for (path, family) in candidates {
        if let Ok(data) = std::fs::read(&path) {
            return (
                vec![Cow::Owned(data)],
                iced::Font {
                    family: iced::font::Family::Name(family),
                    ..Default::default()
                },
            );
        }
    }

    // 未找到系统中文字体时回退到默认无衬线字体。
    (
        Vec::new(),
        iced::Font {
            family: iced::font::Family::SansSerif,
            ..Default::default()
        },
    )
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
