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
    M2(CycleState),
    M3(CycleState),
    M4(CycleState),
    M5(CycleState),
    M6(CycleState),
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
    d16: u16,
    d8: u8,
}

impl CycleState {
    fn new() -> Self {
        Self { instruction: 0, d16: 0, d8: 0}
    }
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
        let mut pc = self.PC.read();
        match &self.state {
            CpuState::Ready => {
                //fetch instruction at [PC]
                let mut instr = memory.read(pc);
                self.PC.write(pc + 1);
                let mut cycle_state = CycleState::new();
                cycle_state.instruction = instr as u16;
                if instr == 0xCB {
                    self.state = CpuState::M2(cycle_state);
                } else if instr == 0x00 {
                    self.state = CpuState::Ready;
                } else if instr == 0x01 {
                    self.state = CpuState::M2(cycle_state);
                } else if instr == 0x02 {
                    self.state = CpuState::M2(cycle_state);
                } else if instr == 0x03 {
                    self.state = CpuState::M2(cycle_state);
                } else if instr == 0x04 {
                    self.state = CpuState::M2(cycle_state);
                }
            },
            CpuState::M2(x) => {
                let mut cycle_state = (*x).clone();
                if x.instruction == 0xCB00 {
                    let mut instr = memory.read(pc);
                    self.PC.write(pc + 1);
                    cycle_state.instruction = 0xCB00 + instr;
                    self.state = CpuState::M3(cycle_state);
                } else if x.instruction == 0x01 {
                    let val = memory.read(pc);
                    self.PC.write(pc + 1);
                    cycle_state.d16 += val;
                    self.state = CpuState::M3(cycle_state);
                } else if x.instruction == 0x02 {
                    memory.write(self.BC.read(), self.AF.read_high());
                    self.state = CpuState::Ready;
                } else if x.instruction == 0x03 {
                    self.BC.write(self.BC.read() + 1);
                    self.state = CpuState::Ready;
                }
            },
            CpuState::M3(x) => {
                let mut cycle_state = (*x).clone();
                if x.instruction == 0x01 {
                    let val = memory.read(pc);
                    cycle_state.d16 += (val as u16) << 8;
                    self.BC.write(cycle_state.d16 + ((val as u16) << 8));
                    self.state = CpuState::Ready;
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