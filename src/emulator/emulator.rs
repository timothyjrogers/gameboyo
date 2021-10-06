use crate::emulator::constants;
use crate::emulator::memory;

pub struct Emulator {
    registers: Registers,
    memory: memory::Memory,
}

struct Registers {
    AF: u16,
    BC: u16,
    DE: u16,
    HL: u16,
    SP: u16,
    PC: u16,
}

impl Registers {
    fn new() -> Self {
        Self {
            AF: 0,
            BC: 0,
            DE: 0,
            HL: 0,
            SP: 0,
            PC: 0,
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

enum Flag {
    Zero,
    Subtraction,
    HalfCarry,
    Carry,
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