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

    pub fn add8(&mut self, r: Targets8, val: u8, add: bool) {
        let res: (u8, bool);
        match r {
            Targets8::A => {
                res = if add { self.a.overflowing_add(val) } else { self.a.overflowing_sub(val) };
                set_h_flag(check_half_carry(self.a, val, add));
                self.a = res.0;
            },
            Targets8::B => {
                res = if add { self.b.overflowing_add(val) } else { self.b.overflowing_sub(val) };
                set_h_flag(check_half_carry(self.b, val, add));
                self.b = res.0;
            },
            Targets8::C => {
                res = if add { self.c.overflowing_add(val) } else { self.c.overflowing_sub(val) };
                set_h_flag(check_half_carry(self.c, val, add));
                self.c = res.0;
            },
            Targets8::D => {
                res = if add { self.d.overflowing_add(val) } else { self.d.overflowing_sub(val) };
                set_h_flag(check_half_carry(self.d, val, add));
                self.d = res.0;
            },
            Targets8::E => {
                res = if add { self.e.overflowing_add(val) } else { self.e.overflowing_sub(val) };
                set_h_flag(check_half_carry(self.e, val, add));
                self.e = res.0;
            },
            Targets8::F => {
                res = if add { self.f.overflowing_add(val) } else { self.f.overflowing_sub(val) };
                set_h_flag(check_half_carry(self.f, val, add));
                self.f = res.0;
            },
            Targets8::H => {
                res = if add { self.h.overflowing_add(val) } else { self.h.overflowing_sub(val) };
                set_h_flag(check_half_carry(self.h, val, add));
                self.h = res.0;
            },
            Targets8::L => {
                res = if add { self.l.overflowing_add(val) } else { self.l.overflowing_sub(val) };
                set_h_flag(check_half_carry(self.l, val, add));
                self.l = res.0;
            }
        }
        self.z_flag(res.0 == 0);
        set_n_flag(!add);
    }


    pub fn rotate_left8(&mut self, r: Targets8) {
        match r {
            Targets8::A => {
                self.set_c_flag(self.a & 0xA0 == 0xA0);
                self.a = self.a << 1;
            },
            Targets8::B => {
                self.set_c_flag(self.b & 0xA0 == 0xA0);
                self.b = self.b << 1;
            },
            Targets8::C => {
                self.set_c_flag(self.c & 0xA0 == 0xA0);
                self.c = self.c << 1;
            },
            Targets8::D => {
                self.set_c_flag(self.d & 0xA0 == 0xA0);
                self.d = self.d << 1;
            },
            Targets8::E => {
                self.set_c_flag(self.e & 0xA0 == 0xA0);
                self.e = self.e << 1;
            },
            Targets8::F => {
                self.set_c_flag(self.f & 0xA0 == 0xA0);
                self.f = self.f << 1;
            },
            Targets8::H => {
                self.set_c_flag(self.h & 0xA0 == 0xA0);
                self.h = self.h << 1;
            },
            Targets8::L => {
                self.set_c_flag(self.l & 0xA0 == 0xA0);
                self.l = self.l << 1;
            }
        }
        self.set_z_flag(false);
        self.set_n_flag(false);
        self.set_h_flag(false);
    }

    fn check_half_carry(v1: u8, v2: u8, add: bool) -> bool {
        if add {
            (v1 & 0xF) + (v2 & 0xF) > 0xF
        }  else {
            (v1 & 0xF) - (v2 & 0xF) > 0xF
        }
    }

    fn set_z_flag(&mut self, set: bool) {
        if set {
            self.f = self.f | 0b10000000;
        } else {
            self.f = self.f & 0b01111111;
        }
    }

    fn set_n_flag(&mut self, set: bool) {
        if set {
            self.f = self.f | 0b01000000;
        } else {
            self.f = self.f & 0b10111111;
        }
    }

    fn set_h_flag(&mut self, set: bool) {
        if set {
            self.f = self.f | 0b00100000;
        } else {
            self.f = self.f & 0b11011111;
        }
    }

    fn set_c_flag(&mut self, set: bool) {
        if set {
            self.f = self.f | 0b00010000;
        } else {
            self.f = self.f & 0b11101111;
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