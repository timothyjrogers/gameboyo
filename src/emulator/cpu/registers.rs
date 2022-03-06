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

    //Adds two 8-bit numbers and sets flags accordingly. Returns the result rather than storing to a register.
    pub fn add8(&mut self, val1: u8, val2: u8, flags: Vec<Flags>) -> u8 {
        if flags.contains(&Flags::H) { self.check_and_set_half_carry_add(val1, val2) }
        let result = val1.overflowing_add(val2);
        if flags.contains(&Flags::Z) {
            if result.0 == 0 { self.set_flag(Flags::Z) } else { self.unset_flag(Flags::Z) }
        }
        if flags.contains(&Flags::N) { self.unset_flag(Flags::N) }
        if flags.contains(&Flags::C) {
            if result.1 { self.set_flag(Flags::C) } else { self.unset_flag(Flags::C) }
        }
        return result.0;
    }

    //Subtracts two 8-bit numbers and sets flags accordingly. Returns the result rather than storing to a register.
    pub fn sub8(&mut self, val1: u8, val2: u8, flags: Vec<Flags>) -> u8 {
        if flags.contains(&Flags::H) { self.check_and_set_half_carry_sub(val1, val2) }
        let result = val1.overflowing_sub(val2);
        if flags.contains(&Flags::Z) {
            if result.0 == 0 { self.set_flag(Flags::Z) } else { self.unset_flag(Flags::Z) }
        }
        if flags.contains(&Flags::N) { self.set_flag(Flags::N) }
        if flags.contains(&Flags::C) {
            if result.1 { self.set_flag(Flags::C) } else { self.unset_flag(Flags::C) }
        }
        return result.0;
    }

    //Adds an unsigned 8-bit value to a register, with overflow, and sets flags accordingly. Stores result in register r.
    pub fn add8_val(&mut self, r: Targets8, val: u8, flags: Vec<Flags>) {
        let mut reg = self.get8(r);
        if flags.contains(&Flags::H) { self.check_and_set_half_carry_add(reg, val) }
        let result = reg.overflowing_add(val);
        self.set8(r, result.0);
        if flags.contains(&Flags::Z) {
            if result.0 == 0 { self.set_flag(Flags::Z) } else { self.unset_flag(Flags::Z) }
        }
        if flags.contains(&Flags::N) { self.unset_flag(Flags::N) }
        if flags.contains(&Flags::C) {
            if result.1 { self.set_flag(Flags::C) } else { self.unset_flag(Flags::C) }
        }
    }

    //Subtracts an 8-bit value from a register, with borrow, and sets flags accordingly. Stores result in register r.
    pub fn sub8_val(&mut self, r: Targets8, val: u8, flags: Vec<Flags>)  {
        let mut reg = self.get8(r);
        if flags.contains(&Flags::H) { self.check_and_set_half_carry_sub(reg, val) }
        let result = reg.overflowing_sub(val);
        self.set8(r, result.0);
        if flags.contains(&Flags::Z) {
            if result.0 == 0 { self.set_flag(Flags::Z) } else { self.unset_flag(Flags::Z) }
        }
        if flags.contains(&Flags::N) { self.set_flag(Flags::N) }
        if flags.contains(&Flags::C) {
            if result.1 { self.set_flag(Flags::C) } else { self.unset_flag(Flags::C) }
        }
    }

    //Adds two 8-bit registers together, with overflow, and sets flags accordingly. Stores value in register r1.
    pub fn add8_reg(&mut self, r1: Targets8, r2: Targets8, flags: Vec<Flags>) {
        let mut reg1 = self.get8(r1);
        let reg2 = self.get8(r2);
        if flags.contains(&Flags::H) { self.check_and_set_half_carry_add(reg1, reg2) }
        let result = reg1.overflowing_add(reg2);
        self.set8(r1, result.0);
        if flags.contains(&Flags::Z) {
            if result.0 == 0 { self.set_flag(Flags::Z) } else { self.unset_flag(Flags::Z) }
        }
        if flags.contains(&Flags::N) { self.unset_flag(Flags::N) }
        if flags.contains(&Flags::C) {
            if result.1 { self.set_flag(Flags::C) } else { self.unset_flag(Flags::C) }
        }
    }

    //Subtracts two 8-bit registers, with borrow, and sets flags accordingly. Stores value in register r1.
    pub fn sub8_reg(&mut self, r1: Targets8, r2: Targets8, flags: Vec<Flags>) {
        let mut reg1 = self.get8(r1);
        let reg2 = self.get8(r2);
        if flags.contains(&Flags::H) { self.check_and_set_half_carry_sub(reg1, reg2) }
        let result = reg1.overflowing_sub(reg2);
        self.set8(r1, result.0);
        if flags.contains(&Flags::Z) {
            if result.0 == 0 { self.set_flag(Flags::Z) } else { self.unset_flag(Flags::Z) }
        }
        if fags.contains(&Flags::N) { self.set_flag(Flags::N) }
        if flags.contains(&Flags::C) {
            if result.1 { self.set_flag(Flags::C) } else { self.unset_flag(Flags::C) }
        }
    }

    //Adds two 16-bit numbers and sets flags accordingly. Returns the result rather than storing to a register.
    pub fn add16(&mut self, val1: u16, val2: u16, flags: Vec<Flags>) -> u16 {
        if flags.contains(&Flags::H) { self.check_and_set_half_carry_add((val1 >> 8) as u8, (val2 >> 8) as u8); }
        let result = val1.overflowing_add(val2);
        if flags.contains(&Flags::Z) {
            if result.0 == 0 { self.set_flag(Flags::Z) } else { self.unset_flag(Flags::Z) }
        }
        if fags.contains(&Flags::N) { self.set_flag(Flags::N) }
        if flags.contains(&Flags::C) {
            if result.1 { self.set_flag(Flags::C) } else { self.unset_flag(Flags::C) }
        }
        return result.0;
    }

    //Subtracts two 16-bit numbers and sets flags accordingly. Returns the result rather than storing to a register.
    pub fn sub16(&mut self, val1: u16, val2: u16, flags: Vec<Flags>) -> u16 {
        if flags.contains(&Flags::H) { self.check_and_set_half_carry_sub((val1 >> 8) as u8, (val2 >> 8) as u8); }
        let result = val1.overflowing_sub(val2);
        if flags.contains(&Flags::Z) {
            if result.0 == 0 { self.set_flag(Flags::Z) } else { self.unset_flag(Flags::Z) }
        }
        if fags.contains(&Flags::N) { self.set_flag(Flags::N) }
        if flags.contains(&Flags::C) {
            if result.1 { self.set_flag(Flags::C) } else { self.unset_flag(Flags::C) }
        }
        return result.0;
    }

    //Adds an unsigned 16-bit value to a register, with overflow, and sets flags accordingly. Stores result in register rr.
    pub fn add16_val(&mut self, rr: Targets16, val: u16, flags: Vec<Flags>) {
        let mut reg = self.get16(rr);
        if flags.contains(&Flags::H) { self.check_and_set_half_carry_add((reg >> 8) as u8, (val >> 8) as u8); }
        let result = reg.overflowing_add(val);
        self.set16(rr, result.0);
        if flags.contains(&Flags::Z) {
            if result.0 == 0 { self.set_flag(Flags::Z) } else { self.unset_flag(Flags::Z) }
        }
        if fags.contains(&Flags::N) { self.set_flag(Flags::N) }
        if flags.contains(&Flags::C) {
            if result.1 { self.set_flag(Flags::C) } else { self.unset_flag(Flags::C) }
        }
    }

    //Subtracts an 16-bit value from a register, with borrow, and sets flags accordingly. Stores result in register rr.
    pub fn sub16_val(&mut self, rr: Targets16, val: u16, flags: Vec<Flags>)  {
        let mut reg = self.get16(rr);
        if flags.contains(&Flags::H) { self.check_and_set_half_carry_sub((reg >> 8) as u8, (val >> 8) as u8); }
        let result = reg.overflowing_sub(val);
        self.set16(rr, result.0);
        if flags.contains(&Flags::Z) {
            if result.0 == 0 { self.set_flag(Flags::Z) } else { self.unset_flag(Flags::Z) }
        }
        if fags.contains(&Flags::N) { self.set_flag(Flags::N) }
        if flags.contains(&Flags::C) {
            if result.1 { self.set_flag(Flags::C) } else { self.unset_flag(Flags::C) }
        }
    }

    //Adds an unsigned 16-bit value to a register, with overflow, and sets flags accordingly. Stores result in register rr.
    pub fn add16_reg(&mut self, rr1: Targets16, rr2: Targets16, flags: Vec<Flags>) {
        let mut reg1 = self.get16(rr1);
        let reg2 = self.get16(rr2);
        if flags.contains(&Flags::H) { self.check_and_set_half_carry_sub((reg1 >> 8) as u8, (reg2 >> 8) as u8); }
        self.set16(rr1, result.0);
        if flags.contains(&Flags::Z) {
            if result.0 == 0 { self.set_flag(Flags::Z) } else { self.unset_flag(Flags::Z) }
        }
        if fags.contains(&Flags::N) { self.set_flag(Flags::N) }
        if flags.contains(&Flags::C) {
            if result.1 { self.set_flag(Flags::C) } else { self.unset_flag(Flags::C) }
        }
    }

    //Subtracts an 16-bit value from a register, with borrow, and sets flags accordingly. Stores result in register rr.
    pub fn sub16_reg(&mut self, rr: Targets16, rr2: Targets16, flags: Vec<Flags>)  {
        let mut reg1 = self.get16(rr1);
        let reg2 = self.get16(rr2);
        if flags.contains(&Flags::H) { self.check_and_set_half_carry_sub((reg1 >> 8) as u8, (reg2 >> 8) as u8); }
        let result = reg1.overflowing_sub(reg2);
        self.set16(rr1, result.0);
        if flags.contains(&Flags::Z) {
            if result.0 == 0 { self.set_flag(Flags::Z) } else { self.unset_flag(Flags::Z) }
        }
        if fags.contains(&Flags::N) { self.set_flag(Flags::N) }
        if flags.contains(&Flags::C) {
            if result.1 { self.set_flag(Flags::C) } else { self.unset_flag(Flags::C) }
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

    pub fn adc_val(&mut self, reg: Targets8, val: u8) {
        let mut set_h = self.check_half_carry_add(self.get8(reg), val);
        let mut res = self.get8(reg).overflowing_add(val);
        let mut set_c = res.1;
        if self.get_flag(Flags::C) {
            if self.check_half_carry_add(res.0, 1) { set_h = true }
            res = res.0.overflowing_add(1);
            if res.1 { set_c = true }
        }
        if res.0 == 0 { self.set_flag(Flags::Z) } else { self.unset_flag(Flags::Z) }
        if set_h { self.set_flag(Flags::H) } else { self.unset_flag(Flags::H) }
        if set_c { self.set_flag(Flags::C) } else { self.unset_flag(Flags::C) }
        self.unset_flag(Flags::N);
        self.set8(reg, res.0);
    }

    pub fn adc_reg(&mut self, r1: Targets8, r2: Targets8) {
        let reg1 = self.get8(r1);
        let reg2 = self.get8(r2);
        let mut set_h = self.check_half_carry_add(reg1, reg2);
        let mut res = reg1.overflowing_add(reg2);
        let mut set_c = res.1;
        if self.get_flag(Flags::C) {
            if self.check_half_carry_add(res.0, 1) { set_h = true }
            res = res.0.overflowing_add(1);
            if res.1 { set_c = true }
        }
        if res.0 == 0 { self.set_flag(Flags::Z) } else { self.unset_flag(Flags::Z) }
        if set_h { self.set_flag(Flags::H) } else { self.unset_flag(Flags::H) }
        if set_c { self.set_flag(Flags::C) } else { self.unset_flag(Flags::C) }
        self.unset_flag(Flags::N);
        self.set8(r1, res.0);
    }

    pub fn sbc_val(&mut self, reg: Targets8, val: u8) {
        let mut set_h = self.check_half_carry_sub(self.get8(reg), val);
        let mut res = self.get8(reg).overflowing_sub(val);
        let mut set_c = res.1;
        if self.get_flag(Flags::C) {
            if self.check_half_carry_sub(res.0, 1) { set_h = true }
            res = res.0.overflowing_sub(1);
            if res.1 { set_c = true }
        }
        if res.0 == 0 { self.set_flag(Flags::Z) } else { self.unset_flag(Flags::Z) }
        if set_h { self.set_flag(Flags::H) } else { self.unset_flag(Flags::H) }
        if set_c { self.set_flag(Flags::C) } else { self.unset_flag(Flags::C) }
        self.set_flag(Flags::N);
        self.set8(reg, res.0);
    }

    pub fn sbc_reg(&mut self, r1: Targets8, r2: Targets8) {
        let reg1 = self.get8(r1);
        let reg2 = self.get8(r2);
        let mut set_h = self.check_half_carry_sub(reg1, reg2);
        let mut res = reg1.overflowing_sub(reg2);
        let mut set_c = res.1;
        if self.get_flag(Flags::C) {
            if self.check_half_carry_sub(res.0, 1) { set_h = true }
            res = res.0.overflowing_sub(1);
            if res.1 { set_c = true }
        }
        if res.0 == 0 { self.set_flag(Flags::Z) } else { self.unset_flag(Flags::Z) }
        if set_h { self.set_flag(Flags::H) } else { self.unset_flag(Flags::H) }
        if set_c { self.set_flag(Flags::C) } else { self.unset_flag(Flags::C) }
        self.set_flag(Flags::N);
        self.set8(r1, res.0);
    }

    pub fn and8_reg(&mut self, r1: Targets8, r2: Targets8) {
        let reg1 = self.get8(r1);
        let reg2 = self.get8(r2);
        let res = reg1 & reg2;
        self.set8(r1, res);
        if res == 0 { self.set_flag(Flags::Z) } else { self.unset_flag(Flags::Z) }
        self.set_flag(Flags::H);
        self.unset_flag(Flags::C);
        self.unset_flag(Flags::N)
    }

    pub fn and8_val(&mut self, reg: Targets8, val: u8) {
        let res = self.get8(reg) & val;
        self.set8(reg, res);
        if res == 0 { self.set_flag(Flags::Z) } else { self.unset_flag(Flags::Z) }
        self.set_flag(Flags::H);
        self.unset_flag(Flags::C);
        self.unset_flag(Flags::N)
    }

    pub fn xor8_reg(&mut self, r1: Targets8, r2: Targets8) {
        let reg1 = self.get8(r1);
        let reg2 = self.get8(r2);
        let res = reg1 ^ reg2;
        self.set8(r1, res);
        if res == 0 { self.set_flag(Flags::Z) } else { self.unset_flag(Flags::Z) }
        self.unset_flag(Flags::H);
        self.unset_flag(Flags::C);
        self.unset_flag(Flags::N)
    }

    pub fn xor8_val(&mut self, reg: Targets8, val: u8) {
        let res = self.get8(reg) ^ val;
        self.set8(reg, res);
        if res == 0 { self.set_flag(Flags::Z) } else { self.unset_flag(Flags::Z) }
        self.unset_flag(Flags::H);
        self.unset_flag(Flags::C);
        self.unset_flag(Flags::N)
    }

    pub fn or8_reg(&mut self, r1: Targets8, r2: Targets8) {
        let reg1 = self.get8(r1);
        let reg2 = self.get8(r2);
        let res = reg1 | reg2;
        self.set8(r1, res);
        if res == 0 { self.set_flag(Flags::Z) } else { self.unset_flag(Flags::Z) }
        self.unset_flag(Flags::H);
        self.unset_flag(Flags::C);
        self.unset_flag(Flags::N)
    }

    pub fn or8_val(&mut self, reg: Targets8, val: u8) {
        let res = self.get8(reg) | val);
        self.set8(reg, res);
        if res == 0 { self.set_flag(Flags::Z) } else { self.unset_flag(Flags::Z) }
        self.unset_flag(Flags::H);
        self.unset_flag(Flags::C);
        self.unset_flag(Flags::N)
    }

    pub fn cp8_reg(&mut self, r1: Targets8, r2: Targets8) {
        let reg1 = self.get8(r1);
        let reg2 = self.get8(r2);
        self.check_and_set_half_carry_sub(reg1, reg2);
        let res = reg1.overflowing_sub(reg2);
        if res.0 == 0 { self.set_flag(Flags::Z) } else { self.unset_flag(Flags::Z) }
        if res.1 { self.set_flag(Flags::C) } else { self.unset_flag(Flags::C) }
        self.set_flag(Flags::N)
    }

    pub fn cp8_val(&mut self, reg: Targets8, val: u8) {
        self.check_and_set_half_carry_sub(self.get8(reg), val);
        let res = self.get8(reg).overflowing_sub(val);
        if res.0 == 0 { self.set_flag(Flags::Z) } else { self.unset_flag(Flags::Z) }
        if res.1 { self.set_flag(Flags::C) } else { self.unset_flag(Flags::C) }
        self.set_flag(Flags::N)
    }

    fn check_half_carry_add(&mut self, v1: u8, v2: u8) -> bool {
        (v1 & 0xF) + (v2 & 0xF) > 0xF
    }

    fn check_and_set_half_carry_add(&mut self, v1: u8, v2: u8) {
        if (v1 & 0xF) + (v2 & 0xF) > 0xF { self.set_flag(Flags::H) } else { self.unset_flag(Flags::H) }
    }

    fn check_half_carry_sub(&mut self, v1: u8, v2: u8) -> bool {
        (v1 & 0xF) - (v2 & 0xF) > 0xF
    }

    fn check_and_set_half_carry_sub(&mut self, v1: u8, v2: u8) {
        if (v1 & 0xF) - (v2 & 0xF) > 0xF { self.set_flag(Flags::H) } else { self.unset_flag(Flags::H) }
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