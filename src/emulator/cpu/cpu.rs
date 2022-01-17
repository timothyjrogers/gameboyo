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

pub enum CpuState {
    Ready,
    Fetch,

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
    cycle: u32,
    pub state: CpuState,
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
                    cycle: 0,
                    state: CpuState::Ready,
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
                    cycle: 0,
                    state: CpuState::Ready,
                }
            }
        }
    }

    pub fn tick(&mut self, memory: &mut Memory) -> CpuState {

    }

    pub fn set_pc(&mut self, val: u16) {
        self.pc.write(val);
    }

    pub fn get_interrupt(&self) -> Option<Interrupt> {
        if self.IF.enabled(Interrupt::VerticalBlanking) && self.IE.enabled(Interrupt::VerticalBlanking) && self.IME == 1 {
            return Some(Interrupt::VerticalBlanking);
        } else if self.IF.enabled(Interrupt::LcdStat) && self.IE.enabled(Interrupt::LcdStat) && self.IME == 1 {
            return Some(Interrupt::LcdStat);
        } else if self.IF.enabled(Interrupt::Timer) && self.IE.enabled(Interrupt::Timer) && self.IME == 1 {
            return Some(Interrupt::Timer);
        } else if self.IF.enabled(Interrupt::Serial) && self.IE.enabled(Interrupt::Serial) && self.IME == 1 {
            return Some(Interrupt::Serial);
        } else if self.IF.enabled(Interrupt::Joypad) && self.IE.enabled(Interrupt::Joypad) && self.IME == 1 {
            return Some(Interrupt::Joypad);
        }
        return None;
    }

    pub fn reset_ime(&mut self) { self.IME = 0; }

    pub fn set_interrupt_flag(&mut self, interrupt: Interrupt) {
        self.IF.enable(interrupt);
    }

    pub fn reset_interrupt_flag(&mut self, interrupt: Interrupt) {
        self.IF.disable(interrupt);
    }

    pub fn push_pc(&mut self, memory: &mut Memory) {
        let pc_high: u8 = (&self.PC & 0xFF00) >> 8 as u8;
        let pc_low: u8 = (&self.PC & 0x00FF) as u8;
        self.SP.write(self.SP.read() - 1);
        memory.write(self.sp.read(), pc_high);
        self.SP.write(self.SP.read() - 1);
        memory.write(self.sp.read(), pc_low);
    }

    pub fn load_vector(&mut self, interrupt: Interrupt) {
        match interrupt {
            Interrupt::VerticalBlanking => self.set_pc(constants::INT_VBL),
            Interrupt::LcdStat => self.set_pc(constants::INT_STAT),
            Interrupt::Timer => self.set_pc(constants::INT_TIMER),
            Interrupt::Serial => self.set_pc(constants::INT_SERIAL),
            Interrupt::Joypad => self.set_pc(constants::INT_JOYPAD),
        }
    }

    pub fn push_stack(&mut self, memory: &mut Memory, data: u8) {
        self.SP.write(self.SP.read() - 1);
        memory.write(self.sp.read(), data);
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