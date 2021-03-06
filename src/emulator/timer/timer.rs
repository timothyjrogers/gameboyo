use crate::emulator::constants;
use crate::emulator::emulator::Platform;

pub struct Timer {
    counter: u16,
    tima: u8,
    tma: u8,
    tac: u8,
}

impl Timer {
    pub fn new(platform: &Platform) -> Self {
        match platform {
            Platform::DMG => {
                Self {
                    counter: constants::DMG_DIV,
                    tima: 0,
                    tma: 0,
                    tac: 0,
                }
            },
            Platform::GBC => {
                Self {
                    counter: constants::GBC_DIV,
                    tima: 0,
                    tma: 0,
                    tac: 0,
                }
            }
        }
    }

    /* Returns true if TIMA overflowed*/
    pub fn tick(&mut self) -> bool {
        let freq_bit = match self.tac & 0b11 {
            0b00 => 9,
            0b01 => 3,
            0b10 => 5,
            0b11 => 7,
            _ => panic!("Unreachable")
        };
        self.counter = self.counter.overflowing_add(1).0;
        if (self.counter & (0b1 << freq_bit)) > 0 && (self.tac & 0b100) as u16 > 0 {
            let update = self.tima.overflowing_add(1);
            if update.1 {
                self.tima = 0;
                return true;
            } else {
                self.tima += 1;
                return false;
            }
        }
        false
    }

    pub fn read_tima(&self) -> u8 { self.tima }

    pub fn set_tima(&mut self) {
        self.tima = self.tma;
    }

    pub fn reset_tima(&mut self) { self.tima = 0; }

    pub fn write_counter(&mut self) {
        self.counter = 0;
    }

    pub fn div(&self) -> u8 {
        return (self.counter >> 8) as u8;
    }
}