pub struct App;

impl iced::Sandbox for App {
    type Message = ();

    fn new() -> Self {
        Self
    }

    fn title(&self) -> String {
        String::from("Cvooc Imagemin Compressor")
    }

    fn update(&mut self, _message: ()) {}
    fn view(&self) -> iced::Element<()> {
        iced::widget::text("Hello").into()
    }
}
