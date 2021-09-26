mod frontend;
mod emulator;
use iced::Application;

fn main() {
    frontend::application::Gameboyo::run(iced::Settings::default());
}
