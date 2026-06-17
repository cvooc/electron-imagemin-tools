use iced::{Sandbox, Settings};

mod app;

fn main() -> iced::Result {
    app::App::run(Settings {
        window: iced::window::Settings {
            size: iced::Size::new(800.0, 600.0),
            min_size: Some(iced::Size::new(800.0, 600.0)),
            ..Default::default()
        },
        ..Default::default()
    })
}
