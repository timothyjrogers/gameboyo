use crate::emulator::registers::register::{InterruptEnable, InterruptFlags, Register};
use crate::emulator::registers::register::timer::{DIV, TIMA, TAC};

enum Flag {
    Zero,
    Subtraction,
    HalfCarry,
    Carry,
}

pub struct Registers {
    AF: Register,
    BC: Register,
    DE: Register,
    HL: Register,
    SP: Register,
    PC: Register,
    IF: InterruptFlags,
    IE: InterruptEnable,
    DIV: DIV,

}

impl Registers {
    fn new() -> Self {
        Self {
            AF: Register::new(0),
            BC: Register::new(0),
            DE: Register::new(0),
            HL: Register::new(0),
            SP: Register::new(0),
            PC: Register::new(0),
            IF: InterruptFlags::new(),
            IE: InterruptEnable::new(),
        }
    }

    fn update_flags(&mut self, flag: Flag, set: bool) {
        let mut mask: u16 = 0;
        if set {
            match flag {
                Flag::Zero => mask = constants::SET_ZERO_FLAG_MASK,
                Flag::Subtraction => mask = constants::SET_SUBTRACTION_FLAG_MASK,
                Flag::HalfCarry => mask = constants::SET_HALFCARRY_FLAG_MASK,
                Flag::Carry => mask = constants::SET_CARRY_FLAG_MASK
            }
            self.AF = self.AF | mask;
        } else {
            match flag {
                Flag::Zero => mask = constants::UNSET_ZERO_FLAG_MASK,
                Flag::Subtraction => mask = constants::UNSET_SUBTRACTION_FLAG_MASK,
                Flag::HalfCarry => mask = constants::UNSET_HALFCARRY_FLAG_MASK,
                Flag::Carry => mask = constants::UNSET_CARRY_FLAG_MASK
            }
            self.AF = self.AF & mask;
        }
    }
}