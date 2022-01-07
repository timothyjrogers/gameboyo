use crate::emulator::cpu::registers::{Interrupt, InterruptFlags, InterruptEnable, Register};
use crate::emulator::constants;
use crate::emulator::emulator::Platform;
use crate::emulator::memory::memory::Memory;

enum Flag {
    Zero,
    Subtraction,
    HalfCarry,
    Carry,
}

pub struct CPU {
    AF: Register,
    BC: Register,
    DE: Register,
    HL: Register,
    SP: Register,
    PC: Register,
    IF: InterruptFlags,
    IE: InterruptEnable,
    IME: u8,
}

impl CPU {
    pub fn new(platform: &Platform) -> Self {
        match platform {
            Platform::DMG => {
                Self {
                    AF: Register::new(constants::DMG_AF),
                    BC: Register::new(constants::DMG_BC),
                    DE: Register::new(constants::DMG_DE),
                    HL: Register::new(constants::DMG_HL),
                    SP: Register::new(constants::DMG_SP),
                    PC: Register::new(constants::DMG_PC),
                    IF: InterruptFlags::new(),
                    IE: InterruptEnable::new(),
                    IME: 1,
                }
            },
            Platform::GBC => {
                Self {
                    AF: Register::new(constants::GBC_AF),
                    BC: Register::new(constants::GBC_BC),
                    DE: Register::new(constants::GBC_DE),
                    HL: Register::new(constants::GBC_HL),
                    SP: Register::new(constants::GBC_SP),
                    PC: Register::new(constants::GBC_PC),
                    IF: InterruptFlags::new(),
                    IE: InterruptEnable::new(),
                    IME: 1,
                }
            }
        }
    }

    pub fn interrupt_ready(&self) -> bool {
        return self.IF.enabled_any();
    }

    pub fn enable_interrupt(&mut self, interrupt: Interrupt) {
        self.IF.enable(interrupt);
    }

    pub fn setup_interrupts(&mut self, memory: &mut Memory) {
        self.SP.write(self.SP.read() - 1);
        memory.write(self.SP.read(), self.PC.read_low());
        self.SP.wrie(self.SP.read() - 1);
        memory.write(self.SP.read(), self.pc.read_high());
    }

    /*
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
     */
}