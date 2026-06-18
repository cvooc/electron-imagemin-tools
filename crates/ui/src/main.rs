#![windows_subsystem = "windows"]

use iced::Application;
use std::borrow::Cow;

mod app;
mod views;

fn main() -> iced::Result {
    let font_data = include_bytes!("C:\\Windows\\Fonts\\msyh.ttc").to_vec();

    let settings = iced::Settings {
        window: iced::window::Settings {
            size: iced::Size::new(800.0, 600.0),
            min_size: Some(iced::Size::new(800.0, 600.0)),
            decorations: false,
            ..Default::default()
        },
        fonts: vec![Cow::Owned(font_data)],
        default_font: iced::Font {
            family: iced::font::Family::Name("Microsoft YaHei"),
            ..Default::default()
        },
        ..Default::default()
    };

    app::App::run(settings)
}
