use iced::{
    alignment,
    widget::{container, text},
    Element, Length, Sandbox, Settings,
};

fn main() -> iced::Result {
    HApp::run(Settings::default())
}

struct HApp {
    // project: bool,
}

#[derive(Debug)]
enum HMessage {
    Session(SessionMessage),
}

#[derive(Debug)]
enum SessionMessage {
    OpenFileBrowser,
    LoadFile,
}

impl HApp {
    fn new() -> Self {
        Self {}
    }

    fn title(&self) -> String {
        String::from("Halogen")
    }

    fn update(&mut self, _message: Self::Message) {}

    fn view(&self) -> iced::Element<Self::Message> {
        "Hello Halogen".into()
    }
}
