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

#[derive(Copy, Clone)]
pub enum CpuState {
    Ready,
    M2(u16),
    M3(u16),
    M4(u16),
    M5(u16),
    M6(u16),
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

struct CycleState {
    instruction: u16,

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
        match self.state {
            CpuState::Ready => {
                //fetch instruction at [PC]
                let mut pc = self.PC.read();
                let mut instr = memory.read(pc);
                self.PC.write(pc + 1);
                if instr == 0xCB {
                    self.state = CpuState::M2((instr as u16) << 8);
                } else if instr == 0x00 {
                    self.state = CpuState::Ready;
                } else if instr ==
            },
            CpuState::M2(x) => {
                if x == 0xCB00 {
                    let mut pc = self.PC.read();
                    let mut instr = memory.read(pc);
                    self.PC.write(pc + 1);
                    self.state = CpuState::M3(x + instr);
                    return self.state;
                }
            }
            _ => {},
        }
        return self.state;
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

    pub fn x01(&mut self, memory: &mut Memory) {

    }
}