use crate::emulator::constants;

pub struct Emulator {
    registers: Registers,
    memory: Memory,
}

struct Registers {
    AF: u16,
    BC: u16,
    DE: u16,
    HL: u16,
    SP: u16,
    PC: u16,
}

enum Flag {
    Zero,
    Subtraction,
    HalfCarry,
    Carry,
}

impl Emulator {
    pub fn new(path: String) -> Self {
        let rom_data = std::fs::read(rom_path).unwrap();
        let mut cartridge_type = rom_data[constants::CARTRIDGE_TYPE];
        let mut rom_size = rom_data[constants::ROM_SIZE];
        let mut ram_size = rom_data[constants::RAM_SIZE];
    }

    //TODO
    //pub fn validate_logo(self) -> Bool
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