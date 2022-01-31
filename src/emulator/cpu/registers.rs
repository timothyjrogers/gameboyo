use crate::emulator::emulator::Platform;
use crate::emulator::constants;

pub struct Register {
    high: u8,
    low: u8,
}

pub struct Flags {
    z: bool,
    n: bool,
    h: bool,
    c: bool,
}

impl Register {
    pub fn new(data: u16) -> Self {
        Self {
            high: (data >> 8) as u8,
            low: (data & 0x00FF) as u8,
        }
    }

    pub fn write(&mut self, data: u16) {
        self.high = (data >> 8) as u8;
        self.low = (data & 0x00FF) as u8;
    }

    pub fn write_high(&mut self, data: u8) {
        self.high = data;
    }

    pub fn write_low(&mut self, data: u8) {
        self.low = data;
    }

    pub fn read(&self) -> u16 {
        let mut val: u16 = self.high as u16;
        val = val << 8;
        val += self.low as u16;
        return val;
    }

    pub fn read_high(&self) -> u8 {
        return self.high;
    }

    pub fn read_low(&self) -> u8 {
        return self.low;
    }

    pub fn add(&mut self, val: u16) -> Flags {

    }
}

pub struct Registers {
    a: u8,
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    f: u8,
    h: u8,
    l: u8,
}

pub enum Targets8 {
    A,
    B,
    C,
    D,
    E,
    F,
    H,
    L,
}

pub enum Targets16 {
    AF,
    BC,
    DE,
    HL,
}

impl Registers {
    pub fn new(platform: &Platform) -> Self {
        match platform {
            Platform::DMG => {
                Self {
                    a: ((constants::DMG_AF & 0xFF00) >> 8) as u8,
                    b: ((constants::DMG_BC & 0xFF00) >> 8) as u8,
                    c: (constants::DMG_BC & 0x00FF) as u8,
                    d: ((constants::DMG_DE & 0xFF00) >> 8) as u8,
                    e: (constants::DMG_DE & 0x00FF) as u8,
                    f: (constants::DMG_AF & 0x00FF) as u8,
                    h: ((constants::DMG_HL & 0xFF00) >> 8) as u8,
                    l: (constants::DMG_HL & 0x00FF) as u8,
                }
            },
            Platform::GBC => {
                Self {
                    a: ((constants::GBC_AF & 0xFF00) >> 8) as u8,
                    b: ((constants::GBC_BC & 0xFF00) >> 8) as u8,
                    c: (constants::GBC_BC & 0x00FF) as u8,
                    d: ((constants::GBC_DE & 0xFF00) >> 8) as u8,
                    e: (constants::GBC_DE & 0x00FF) as u8,
                    f: (constants::GBC_AF & 0x00FF) as u8,
                    h: ((constants::GBC_HL & 0xFF00) >> 8) as u8,
                    l: (constants::GBC_HL & 0x00FF) as u8,
                }
            }
        }
    }

    pub fn get8(&self, r: Targets8) -> u8 {
        match r {
            Targets8::A => self.a,
            Targets8::B => self.b,
            Targets8::C => self.c,
            Targets8::D => self.d,
            Targets8::E => self.e,
            Targets8::F => self.f,
            Targets8::H => self.h,
            Targets8::L => self.l,
        }
    }

    pub fn set8(&mut self, r: Targets8, val: u8) {
        match r {
            Targets8::A => self.a = val,
            Targets8::B => self.b = val,
            Targets8::C => self.c = val,
            Targets8::D => self.d = val,
            Targets8::E => self.e = val,
            Targets8::F => self.f = val,
            Targets8::H => self.h = val,
            Targets8::L => self.l = val,
        }
    }

    pub fn get16(&self, rr: Targets16) -> u16 {
        match rr {
            Targets16::AF => (self.a as u16) << 8 + self.f,
            Targets16::BC => (self.b as u16) << 8 + self.c,
            Targets16::DE => (self.d as u16) << 8 + self.e,
            Targets16::HL => (self.h as u16) << 8 + self.l,
        }
    }

    pub fn set16(&mut self, rr: Targets16, val: u16) {
        match rr {
            Targets16::AF => {
                self.a = (val >> 8) as u8;
                self.f = (val & 0x00FF) as u8;
            },
            Targets16::BC => {
                self.b = (val >> 8) as u8;
                self.c = (val & 0x00FF) as u8;
            },
            Targets16::DE => {
                self.d = (val >> 8) as u8;
                self.e = (val & 0x00FF) as u8;
            },
            Targets16::HL => {
                self.h = (val >> 8) as u8;
                self.l = (val & 0x00FF) as u8;
            }
        }
    }

    pub fn add8(&mut self, r: Targets8, val: u8) {
        match r {
            Targets8::A => {
                let res = self.a.overflowing_add(val);
                self.a = res.0;
                //TODO FLAGS if res.0 == 0 {}
            }
        }
    }

}

#[derive(Clone, Copy)]
pub enum Interrupt {
    VerticalBlanking,
    LcdStat,
    Timer,
    Serial,
    Joypad,
}

pub struct InterruptEnable {
    bits: u8,
}

impl InterruptEnable {
    pub fn new() -> Self {
        Self { bits: 0 }
    }

    pub fn enabled(&self, interrupt: Interrupt) -> bool {
        let mut mask;
        match interrupt {
            Interrupt::VerticalBlanking => mask = 0b00000001,
            Interrupt::LcdStat => mask = 0b00000010,
            Interrupt::Timer => mask = 0b00000100,
            Interrupt::Serial => mask = 0b00001000,
            Interrupt::Joypad => mask = 0b00010000,
        }
        return (self.bits & mask) > 0;
    }

     pub fn enable(&mut self, interrupt: Interrupt) {
        let mut mask;
        match interrupt {
            Interrupt::VerticalBlanking => mask = 0b00000001,
            Interrupt::LcdStat => mask = 0b00000010,
            Interrupt::Timer => mask = 0b00000100,
            Interrupt::Serial => mask = 0b00001000,
            Interrupt::Joypad => mask = 0b00010000,
        }
        self.bits = self.bits | mask;
    }

    pub fn disable(&mut self, interrupt: Interrupt) {
        let mut mask;
        match interrupt {
            Interrupt::VerticalBlanking => mask = 0b11111110,
            Interrupt::LcdStat => mask = 0b11111101,
            Interrupt::Timer => mask = 0b11111011,
            Interrupt::Serial => mask = 0b11110111,
            Interrupt::Joypad => mask = 0b11101111,
        }
        self.bits = self.bits & mask;
    }
}

/*

 */
pub struct InterruptFlags {
    bits: u8,
}

impl InterruptFlags {
    pub fn new() -> Self {
        Self { bits: 0b11100000 }
    }

    pub fn enabled(&self, interrupt: Interrupt) -> bool {
        let mut mask;
        match interrupt {
            Interrupt::VerticalBlanking => mask = 0b00000001,
            Interrupt::LcdStat => mask = 0b00000010,
            Interrupt::Timer => mask = 0b00000100,
            Interrupt::Serial => mask = 0b00001000,
            Interrupt::Joypad => mask = 0b00010000,
        }
        return (self.bits & mask) > 0;
    }

    pub fn enable(&mut self, interrupt: Interrupt) {
        let mut mask;
        match interrupt {
            Interrupt::VerticalBlanking => mask = 0b00000001,
            Interrupt::LcdStat => mask = 0b00000010,
            Interrupt::Timer => mask = 0b00000100,
            Interrupt::Serial => mask = 0b00001000,
            Interrupt::Joypad => mask = 0b00010000,
        }
        self.bits = self.bits | mask;
    }

    pub fn disable(&mut self, interrupt: Interrupt) {
        let mut mask;
        match interrupt {
            Interrupt::VerticalBlanking => mask = 0b11111110,
            Interrupt::LcdStat => mask = 0b11111101,
            Interrupt::Timer => mask = 0b11111011,
            Interrupt::Serial => mask = 0b11110111,
            Interrupt::Joypad => mask = 0b11101111,
        }
        self.bits = self.bits & mask;
    }

    pub fn enabled_any(&self) -> bool {
        return self.bits & 0b00011111 > 0;
    }
}