use iced::{button, Align, Button, Column, Container, Element, Length, Row, Text};
use crate::frontend::application::Message;

pub fn draw<'a>(rom_button: &'a mut button::State, start_button: &'a mut button::State) -> Element<'a, Message> {
    let content = Column::new()
        .spacing(20)
        .align_items(Align::Center)
        .push(
            Row::new()
                .push(
                    Button::new(rom_button, Text::new(String::from("Choose ROM")))
                        .on_press(Message::ChooseRom)
                )
                .push(
                    Button::new(start_button, Text::new(String::from("Launch")))
                )
        );
    Container::new(content)
        .width(Length::Fill)
        .height(Length::Fill)
        .center_x()
        .center_y()
        .into()
}