use crate::emulator::cpu::registers::{Interrupt, InterruptFlags, InterruptEnable, Registers, Targets8, Targets16};
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
    Wait(u32),
    M2(CycleState),
    M3(CycleState),
    M4(CycleState),
    M5(CycleState),
    M6(CycleState),
}

pub struct CPU {
    registers: Registers,
    sp: u16,
    pc: u16,
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
                    registers: Registers::new(platform),
                    sp: constants::DMG_SP,
                    pc: constants::DMG_PC,
                    IF: InterruptFlags::new(),
                    IE: InterruptEnable::new(),
                    IME: 1,
                    cycle: 0,
                    state: CpuState::Ready,
                }
            },
            Platform::GBC => {
                Self {
                    registers: Registers::new(platform),
                    sp: constants::GBC_SP,
                    pc: constants::GBC_PC,
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
        match &self.state {
            CpuState::Ready => {
                let mut instr = memory.read(self.pc);             // fetch instruction at [PC]
                let (cycles, length) = constants::OPCODES.get(instr).unwrap();
                let mut cycle_state = CycleState::new();         // create new cycle state for multi-cycle instructions
                cycle_state.instruction = instr as u16;                   // save current instruction in cycle state
                if instr == 0xCB || (instr >= 0x01 && instr <= 0x06) || instr == 0x08 {    // instr = 0xCB prefix OR 0x01 - 0x06
                    self.state = CpuState::M2(cycle_state);
                } else if instr == 0x00 {                                 // instr = 0x00 (NOP) -- one cycle
                    self.state = CpuState::Wait(cycles);
                } else if instr == 0x01 {
                    let mut d16 = memory.read(self.pc + 1) as u16 + ((memory.read(self.pc + 2) as u16) << 8);
                    self.registers.set16(Targets16::BC, d16);
                } else if instr == 0x02 {
                    memory.write(self.registers.get16(Targets16::BC), self.registers.get8(Targets8::A));
                } else if instr == 0x03 {
                    self.registers.add16(Targets16::BC, 1, true);
                } else if instr == 0x04 {
                    self.registers.add8(Targets8::B, 1, true);
                } else if instr == 0x05 {
                    self.registers.add8(Targets8::B, 1, false);
                } else if instr == 0x06 {
                    self.registers.set8(Targets8::B, memory.read(self.pc + 1));
                } else if instr == 0x07 {                                //RLCA
                    self.registers.rotate_left8(Targets8::A);
                } else if instr == 0x08 {
                    let mut d16 = memory.read(self.pc + 1) as u16 + ((memory.read(self.pc + 2) as u16) << 8);
                    memory.write(d16, (self.sp & 0x0FF) as u8);
                    memory.write(d16 + 1, (self.sp >> 8) as u8);
                } else if instr == 0x09 {

                }
                self.pc += length;
                self.state = CpuState::Wait(cycles);
            },
            CpuState::Wait(x) => {
                if *x == 1 {
                    self.state = CpuState::Ready;
                } else {
                    self.stat = CpuState::Wait(*x - 1);
                }
            }
            /*
            CpuState::M2(x) => {
                let mut cycle_state = (*x).clone();
                if x.instruction == 0xCB00 {
                    let mut instr = memory.read(self.pc);
                    self.pc = self.pc + 1;
                    cycle_state.instruction = 0xCB00 + instr;
                    self.state = CpuState::M3(cycle_state);
                } else if x.instruction == 0x01 {
                    let val = memory.read(pc);
                    self.pc = self.pc + 1;
                    cycle_state.d16 = val;
                    self.state = CpuState::M3(cycle_state);
                } else if x.instruction == 0x02 {
                    memory.write(self.registers.get16(Targets16::BC), self.registers.get8(Targets8::A));
                    self.state = CpuState::Ready;
                } else if x.instruction == 0x03 {
                    let val = self.registers.get16(Targets16::BC).overflowing_add(1);
                    self.registers.set16(Targets16::BC, val.0);
                    self.state = CpuState::Ready;
                } else if x.instruction == 0x04 {
                    self.registers.add8(Targets8::B, 1, true);
                    self.state = CpuState::Ready;
                } else if x.instruction == 0x05 {
                    self.registers.add8(Targets8::B, 1, false);
                    self.state = CpuState::Ready;
                } else if x.instruction == 0x06 {
                    let val = memory.read(pc);
                    self.pc = self.pc + 1;
                    self.registers.set8(Targets8::B, val);
                    self.state = CpuState::Ready;
                } else if x.instruction == 0x08 {
                    let val = memory.read(pc);
                    self.pc = self.pc + 1;
                    cycle_state.d16 = val;
                    self.state = CpuState::M3(cycle_state);
                }
            },
            CpuState::M3(x) => {
                let mut cycle_state = (*x).clone();
                if x.instruction == 0x01 {
                    let val = memory.read(pc);
                    self.pc = self.pc + 1;
                    cycle_state.d16 += (val as u16) << 8;
                    self.registers.set16(Targets16::BC, cycle_state.d16);
                    self.state = CpuState::Ready;
                } else if x.instruction == 0x08 {
                    let val = memory.read(pc);
                    self.pc = self.pc + 1;
                    cycle_state.d16 += (val as u16) << 8;
                    self.state = CpuState::M4(cycle_state);
                }
            },
            CpuState::M4(x) => {
                let mut cycle_state = (*x).clone();
                if x.instruction == 0x08 {
                    memory.write(cycle_state.d16, (self.sp & 0x00FF) as u8);
                    self.state = CpuState::M5(cycle_state);
                }
            },
            CpuState::M5(x) => {
                let mut cycle_state = (*x).clone();
                if x.instruction == 0x08 {
                    memory.write(cycle_state.d16 + 1, (self.sp >> 8) as u8);
                    self.state = CpuState::Ready;
                }
            }
             */
            _ => {},
        }
        return self.state;
    }

    pub fn set_pc(&mut self, val: u16) {
        self.pc = val;
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