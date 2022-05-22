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
pub enum Register8 {
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
pub enum Register16 {
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

    pub fn get8(&self, r: Register8) -> u8 {
        match r {
            Register8::A => self.a,
            Register8::B => self.b,
            Register8::C => self.c,
            Register8::D => self.d,
            Register8::E => self.e,
            Register8::F => self.f,
            Register8::H => self.h,
            Register8::L => self.l,
        }
    }

    pub fn set8(&mut self, r: Register8, val: u8) {
        match r {
            Register8::A => self.a = val,
            Register8::B => self.b = val,
            Register8::C => self.c = val,
            Register8::D => self.d = val,
            Register8::E => self.e = val,
            Register8::F => self.f = val,
            Register8::H => self.h = val,
            Register8::L => self.l = val,
        }
    }

    pub fn get16(&self, rr: Register16) -> u16 {
        match rr {
            Register16::AF => (self.a as u16) << 8 + self.f,
            Register16::BC => (self.b as u16) << 8 + self.c,
            Register16::DE => (self.d as u16) << 8 + self.e,
            Register16::HL => (self.h as u16) << 8 + self.l,
        }
    }

    pub fn set16(&mut self, rr: Register16, val: u16) {
        match rr {
            Register16::AF => {
                self.a = (val >> 8) as u8;
                self.f = (val & 0x00FF) as u8;
            },
            Register16::BC => {
                self.b = (val >> 8) as u8;
                self.c = (val & 0x00FF) as u8;
            },
            Register16::DE => {
                self.d = (val >> 8) as u8;
                self.e = (val & 0x00FF) as u8;
            },
            Register16::HL => {
                self.h = (val >> 8) as u8;
                self.l = (val & 0x00FF) as u8;
            }
        }
    }

    pub fn rotate_left_circular8(&mut self, r: Register8) {
        match r {
            Register8::A => {
                let high = (self.a & 0xA0) >> 7;
                self.set_c_flag(high == 1);
                self.a = self.a << 1;
                self.a += high;
            },
            Register8::B => {
                let high = (self.b & 0xA0) >> 7;
                self.set_c_flag(self.b & 0xA0 == 0xA0);
                self.b = self.b << 1;
                self.b += high;
            },
            Register8::C => {
                let high = (self.c & 0xA0) >> 7;
                self.set_c_flag(self.c & 0xA0 == 0xA0);
                self.c = self.c << 1;
                self.c += high;
            },
            Register8::D => {
                let high = (self.d & 0xA0) >> 7;
                self.set_c_flag(self.d & 0xA0 == 0xA0);
                self.d = self.d << 1;
                self.d += high;
            },
            Register8::E => {
                let high = (self.e & 0xA0) >> 7;
                self.set_c_flag(self.e & 0xA0 == 0xA0);
                self.e = self.e << 1;
                self.e += high;
            },
            Register8::F => {
                let high = (self.f & 0xA0) >> 7;
                self.set_c_flag(self.f & 0xA0 == 0xA0);
                self.f = self.f << 1;
                self.f += high;
            },
            Register8::H => {
                let high = (self.h & 0xA0) >> 7;
                self.set_c_flag(self.h & 0xA0 == 0xA0);
                self.h = self.h << 1;
                self.h += high;
            },
            Register8::L => {
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

    pub fn rotate_left8(&mut self, r: Register8) {
        let cur_c = self.get_c_flag();
        match r {
            Register8::A => {
                self.set_c_flag(self.a & 0xA0 == 0xA0);
                self.a = self.a << 1;
                self.a += cur_c;
            },
            Register8::B => {
                self.set_c_flag(self.b & 0xA0 == 0xA0);
                self.b = self.b << 1;
                self.b += cur_c;
            },
            Register8::C => {
                self.set_c_flag(self.c & 0xA0 == 0xA0);
                self.c = self.c << 1;
                self.c += cur_c;
            },
            Register8::D => {
                self.set_c_flag(self.d & 0xA0 == 0xA0);
                self.d = self.d << 1;
                self.d += cur_c;
            },
            Register8::E => {
                self.set_c_flag(self.e & 0xA0 == 0xA0);
                self.e = self.e << 1;
                self.e += cur_c;
            },
            Register8::F => {
                self.set_c_flag(self.f & 0xA0 == 0xA0);
                self.f = self.f << 1;
                self.f += cur_c;
            },
            Register8::H => {
                self.set_c_flag(self.h & 0xA0 == 0xA0);
                self.h = self.h << 1;
                self.h += cur_c;
            },
            Register8::L => {
                self.set_c_flag(self.l & 0xA0 == 0xA0);
                self.l = self.l << 1;
                self.l += cur_c;
            }
        }
        self.set_z_flag(false);
        self.set_n_flag(false);
        self.set_h_flag(false);
    }

    pub fn rotate_right_circular8(&mut self, r: Register8) {
        match r {
            Register8::A => {
                let low = self.a & 0x01;
                self.set_c_flag(self.a & 0x01 == 0x01);
                self.a = self.a >> 1;
                self.a += low << 7;
            },
            Register8::B => {
                let low = self.b & 0x01;
                self.set_c_flag(self.b & 0x01 == 0x01);
                self.b = self.b >> 1;
                self.b += low << 7;
            },
            Register8::C => {
                let low = self.c & 0x01;
                self.set_c_flag(self.c & 0x01 == 0x01);
                self.c = self.c >> 1;
                self.c += low << 7;
            },
            Register8::D => {
                let low = self.d & 0x01;
                self.set_c_flag(self.d & 0x01 == 0x01);
                self.d = self.d >> 1;
                self.d += low << 7;
            },
            Register8::E => {
                let low = self.e & 0x01;
                self.set_c_flag(self.e & 0x01 == 0x01);
                self.e = self.e >> 1;
                self.e += low << 7;
            },
            Register8::F => {
                let low = self.f & 0x01;
                self.set_c_flag(self.f & 0x01 == 0x01);
                self.f = self.f >> 1;
                self.f += low << 7;
            },
            Register8::H => {
                let low = self.h & 0x01;
                self.set_c_flag(self.h & 0x01 == 0x01);
                self.h = self.h >> 1;
                self.h += low << 7;
            },
            Register8::L => {
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

    pub fn rotate_right8(&mut self, r: Register8) {
        let cur_c = self.get_c_flag();
        match r {
            Register8::A => {
                self.set_c_flag(self.a & 0x01 == 0x01);
                self.a = self.a >> 1;
                self.a += cur_c << 7;
            },
            Register8::B => {
                self.set_c_flag(self.b & 0x01 == 0x01);
                self.b = self.b >> 1;
                self.b += cur_c << 7;
            },
            Register8::C => {
                self.set_c_flag(self.c & 0x01 == 0x01);
                self.c = self.c >> 1;
                self.c += cur_c << 7;
            },
            Register8::D => {
                self.set_c_flag(self.d & 0x01 == 0x01);
                self.d = self.d >> 1;
                self.d += cur_c << 7;
            },
            Register8::E => {
                self.set_c_flag(self.e & 0x01 == 0x01);
                self.e = self.e >> 1;
                self.e += cur_c << 7;
            },
            Register8::F => {
                self.set_c_flag(self.f & 0x01 == 0x01);
                self.f = self.f >> 1;
                self.f += cur_c << 7;
            },
            Register8::H => {
                self.set_c_flag(self.h & 0x01 == 0x01);
                self.h = self.h >> 1;
                self.h += cur_c << 7;
            },
            Register8::L => {
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