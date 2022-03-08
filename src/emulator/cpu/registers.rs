use crate::emulator::emulator::Platform;
use crate::emulator::constants;

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

#[derive(Clone, Copy)]
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

#[derive(Clone, Copy)]
pub enum Targets16 {
    AF,
    BC,
    DE,
    HL,
}

#[derive(Clone, Copy)]
pub enum Flags {
    Z,
    N,
    H,
    C,
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

    pub fn rotate_left_circular8(&mut self, r: Targets8) {
        match r {
            Targets8::A => {
                let high = (self.a & 0xA0) >> 7;
                self.set_c_flag(high == 1);
                self.a = self.a << 1;
                self.a += high;
            },
            Targets8::B => {
                let high = (self.b & 0xA0) >> 7;
                self.set_c_flag(self.b & 0xA0 == 0xA0);
                self.b = self.b << 1;
                self.b += high;
            },
            Targets8::C => {
                let high = (self.c & 0xA0) >> 7;
                self.set_c_flag(self.c & 0xA0 == 0xA0);
                self.c = self.c << 1;
                self.c += high;
            },
            Targets8::D => {
                let high = (self.d & 0xA0) >> 7;
                self.set_c_flag(self.d & 0xA0 == 0xA0);
                self.d = self.d << 1;
                self.d += high;
            },
            Targets8::E => {
                let high = (self.e & 0xA0) >> 7;
                self.set_c_flag(self.e & 0xA0 == 0xA0);
                self.e = self.e << 1;
                self.e += high;
            },
            Targets8::F => {
                let high = (self.f & 0xA0) >> 7;
                self.set_c_flag(self.f & 0xA0 == 0xA0);
                self.f = self.f << 1;
                self.f += high;
            },
            Targets8::H => {
                let high = (self.h & 0xA0) >> 7;
                self.set_c_flag(self.h & 0xA0 == 0xA0);
                self.h = self.h << 1;
                self.h += high;
            },
            Targets8::L => {
                let high = (self.l & 0xA0) >> 7;
                self.set_c_flag(self.l & 0xA0 == 0xA0);
                self.l = self.l << 1;
                self.l += high;
            }
        }
        self.set_z_flag(false);
        self.set_n_flag(false);
        self.set_h_flag(false);
    }

    pub fn rotate_left8(&mut self, r: Targets8) {
        let cur_c = self.get_c_flag();
        match r {
            Targets8::A => {
                self.set_c_flag(self.a & 0xA0 == 0xA0);
                self.a = self.a << 1;
                self.a += cur_c;
            },
            Targets8::B => {
                self.set_c_flag(self.b & 0xA0 == 0xA0);
                self.b = self.b << 1;
                self.b += cur_c;
            },
            Targets8::C => {
                self.set_c_flag(self.c & 0xA0 == 0xA0);
                self.c = self.c << 1;
                self.c += cur_c;
            },
            Targets8::D => {
                self.set_c_flag(self.d & 0xA0 == 0xA0);
                self.d = self.d << 1;
                self.d += cur_c;
            },
            Targets8::E => {
                self.set_c_flag(self.e & 0xA0 == 0xA0);
                self.e = self.e << 1;
                self.e += cur_c;
            },
            Targets8::F => {
                self.set_c_flag(self.f & 0xA0 == 0xA0);
                self.f = self.f << 1;
                self.f += cur_c;
            },
            Targets8::H => {
                self.set_c_flag(self.h & 0xA0 == 0xA0);
                self.h = self.h << 1;
                self.h += cur_c;
            },
            Targets8::L => {
                self.set_c_flag(self.l & 0xA0 == 0xA0);
                self.l = self.l << 1;
                self.l += cur_c;
            }
        }
        self.set_z_flag(false);
        self.set_n_flag(false);
        self.set_h_flag(false);
    }

    pub fn rotate_right_circular8(&mut self, r: Targets8) {
        match r {
            Targets8::A => {
                let low = self.a & 0x01;
                self.set_c_flag(self.a & 0x01 == 0x01);
                self.a = self.a >> 1;
                self.a += low << 7;
            },
            Targets8::B => {
                let low = self.b & 0x01;
                self.set_c_flag(self.b & 0x01 == 0x01);
                self.b = self.b >> 1;
                self.b += low << 7;
            },
            Targets8::C => {
                let low = self.c & 0x01;
                self.set_c_flag(self.c & 0x01 == 0x01);
                self.c = self.c >> 1;
                self.c += low << 7;
            },
            Targets8::D => {
                let low = self.d & 0x01;
                self.set_c_flag(self.d & 0x01 == 0x01);
                self.d = self.d >> 1;
                self.d += low << 7;
            },
            Targets8::E => {
                let low = self.e & 0x01;
                self.set_c_flag(self.e & 0x01 == 0x01);
                self.e = self.e >> 1;
                self.e += low << 7;
            },
            Targets8::F => {
                let low = self.f & 0x01;
                self.set_c_flag(self.f & 0x01 == 0x01);
                self.f = self.f >> 1;
                self.f += low << 7;
            },
            Targets8::H => {
                let low = self.h & 0x01;
                self.set_c_flag(self.h & 0x01 == 0x01);
                self.h = self.h >> 1;
                self.h += low << 7;
            },
            Targets8::L => {
                let low = self.l & 0x01;
                self.set_c_flag(self.l & 0x01 == 0x01);
                self.l = self.l >> 1;
                self.l += low << 7;
            }
        }
        self.set_z_flag(false);
        self.set_n_flag(false);
        self.set_h_flag(false);
    }

    pub fn rotate_right8(&mut self, r: Targets8) {
        let cur_c = self.get_c_flag();
        match r {
            Targets8::A => {
                self.set_c_flag(self.a & 0x01 == 0x01);
                self.a = self.a >> 1;
                self.a += cur_c << 7;
            },
            Targets8::B => {
                self.set_c_flag(self.b & 0x01 == 0x01);
                self.b = self.b >> 1;
                self.b += cur_c << 7;
            },
            Targets8::C => {
                self.set_c_flag(self.c & 0x01 == 0x01);
                self.c = self.c >> 1;
                self.c += cur_c << 7;
            },
            Targets8::D => {
                self.set_c_flag(self.d & 0x01 == 0x01);
                self.d = self.d >> 1;
                self.d += cur_c << 7;
            },
            Targets8::E => {
                self.set_c_flag(self.e & 0x01 == 0x01);
                self.e = self.e >> 1;
                self.e += cur_c << 7;
            },
            Targets8::F => {
                self.set_c_flag(self.f & 0x01 == 0x01);
                self.f = self.f >> 1;
                self.f += cur_c << 7;
            },
            Targets8::H => {
                self.set_c_flag(self.h & 0x01 == 0x01);
                self.h = self.h >> 1;
                self.h += cur_c << 7;
            },
            Targets8::L => {
                self.set_c_flag(self.l & 0x01 == 0x01);
                self.l = self.l >> 1;
                self.l += cur_c << 7;
            }
        }
        self.set_z_flag(false);
        self.set_n_flag(false);
        self.set_h_flag(false);
    }

    //Set individual flag bit
    pub fn set_flag(&mut self, flag: Flags) {
        match flag {
            Flags::Z => self.f = self.f | 0b10000000,
            Flags::N => self.f = self.f | 0b01000000,
            Flags::H => self.f = self.f | 0b00100000,
            Flags::C => self.f = self.f | 0b00010000,
        }
    }

    //Unset individual flag bit
    pub fn unset_flag(&mut self, flag: Flags) {
        match flag {
            Flags::Z => self.f = self.f & 0b01111111,
            Flags::N => self.f = self.f & 0b10111111,
            Flags::H => self.f = self.f & 0b11011111,
            Flags::C => self.f = self.f & 0b11101111,
        }
    }

    //Returns true if individual flag bit is set, else false
    pub fn get_flag(&self, flag: Flags) -> bool {
        match flag {
            Flags::Z => (self.f & 0b10000000) >> 7 == 1,
            Flags::N => (self.f & 0b01000000) >> 6 == 1,
            Flags::H => (self.f & 0b00100000) >> 5 == 1,
            Flags::C => (self.f & 0b00010000) >> 4 == 1,
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