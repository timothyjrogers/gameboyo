use iced::{button, executor, keyboard, slider, time,
           Application, Clipboard, Column, Command, Container, Element, Subscription};
use rodio::{
    source::{SineWave, Source},
    Sink,
};
use std::time::Duration;
use nfd2::Response;

use crate::frontend::constants;
use crate::frontend::views;
use crate::emulator::emulator;

//ICED STATE
pub struct Gameboyo {
    current_view: PageModel,
    emulator: Option<emulator::Emulator>,
}

#[derive(Debug, Clone)]
pub enum Message {
    IcedEvent(iced_native::Event),
    Goto(PageModel),
    LaunchEmulator,
    ChooseRom,
    Tick,
    RedrawScreen,
}

#[derive(Debug, Clone)]
pub enum PageModel {
    Init {
        rom_button: button::State,
        start_button: button::State,
    }
}

impl Default for Gameboyo {
    fn default() -> Self {
        Self {
            current_view: PageModel::Init{ rom_button: button::State::new(), start_button: button::State::new() },
            emulator: None,
        }
    }
}

impl Application for Gameboyo {
    type Executor = executor::Default;
    type Message = Message;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        (Gameboyo::default(), Command::none())
    }

    fn title(&self) -> String {
        String::from(constants::APPLICATION_TITLE)
    }

    fn view(&mut self) -> Element<Message> {
        //TODO: match on current page model
        match &mut self.current_view {
            PageModel::Init{ rom_button, start_button } => views::init::draw(rom_button, start_button),
        }
    }

    fn update(&mut self, message: Message, _clipboard: &mut Clipboard) -> Command<Message> {
        match message {
            Message::ChooseRom => {
                let mut path: String = String::from("");
                match nfd2::open_file_dialog(None, None).expect("Unable to open file dialog") {
                    Response::Okay(file_path) => {
                        path = file_path.clone().into_os_string().into_string().unwrap();
                    },
                    _ => println!("User canceled")
                }
                self.emulator = Some(emulator::Emulator::new(path));
                match &self.emulator {
                    Some(x) => true, //x.validate_logo(),
                    _ => false
                };
            },
            Message::LaunchEmulator => (),
            Message::Tick => {
                /*
                    - Fires 59.7275 times per second
                    Loop 70224 times (Machine Cycles per Frame):
                        Execute single machine-cycle in CPU
                            - Individual OP dispatch must understand cycle-context
                        Execute single machine-cycle in PPU
                        Execute single machine cycle in APU
                 */
                /*
                for i in 0..constants::CYCLES_PER_FRAME {
                  self.emulator.tick()
                }
                */
                 */
            }
            Message::Goto(p) => {
                self.current_view = p;
            },
            Message::IcedEvent(event) => {
                match event {
                    iced_native::Event::Keyboard(keyboard_event) => match keyboard_event {
                        keyboard::Event::KeyPressed { key_code, .. } => {
                            return Command::none();
                        },
                        keyboard::Event::KeyReleased { key_code, .. } => {
                            return Command::none();
                        },
                        _ => ()
                    },
                    _ => ()
                }
            },
            _ => ()
        }
        Command::none()
    }

    fn subscription(&self) -> Subscription<Self::Message> {
        let runtime_events = iced_native::subscription::events().map(Message::IcedEvent);


        let ticks = time::every(Duration::from_millis(constants::FPS_MILLIS))
            .map(|_| -> Message { Message::Tick });

        Subscription::batch(vec![runtime_events, ticks])
    }
}