use crate::emulator::constants;
use crate::emulator::memory::memory::Memory;
use crate::emulator::registers::register::{Register, Registers};

pub struct Emulator {
    memory: :Memory,
    registers: Registers,
    //TODO cpu: CPU,
    //TODO timer: Timer,
    //TODO video: VideoController,
    //TODO joypad: Joypad,
}

pub enum Platform {
    DMG,
    GBC
}

impl Emulator {
    pub fn new(path: String) -> Self {
        let memory = memory::Memory::new(path);
        Self {
            registers: Registers::new(),
            memory: memory,
        }
    }

    pub fn validate_logo(&self) -> bool {
        let mut valid = true;
        for i in constants::LOGO_START..=constants::LOGO_END {
            if self.memory.read(i as u16) != constants::NINTENDO_LOGO[i - constants::LOGO_START] {
                valid = false;
            }
        }
        println!("Logo validation = {}", valid);
        return valid;
    }
}