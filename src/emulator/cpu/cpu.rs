use crate::emulator::cpu::registers::{Flags, FlagSettings, Registers, Register8, Register16};
use crate::emulator::cpu::interrupts::{Interrupt, InterruptRegisters};
use crate::emulator::constants;
use crate::emulator::emulator::Platform;
use crate::emulator::memory::memory::Memory;

#[derive(Copy, Clone)]
pub enum CpuState {
    Ready,
    Wait(u32),

}

/*
registers: contains registers A F B C D E HL plus 8- and 16-bit access functions
sp: 16-bit stack pointer
pc: 16-bit program counter
IF: contains interrupt flags and access functions
IE: contains interrupt enable register and access functions
IME: master interrut enable register

 */
pub struct CPU {
    registers: Registers,
    sp: u16,
    pc: u16,
    interrupts: InterruptRegisters,
    cycle: u32,
    pub state: CpuState,
    instr_state: Option<InstructionState>
}

struct InstructionState {
    cycle: u8,
    instruction: u16,
    d16: u16,
    d8: u8,
}

impl InstructionState {
    fn new(instruction: u16) -> Self {
        Self {
            cycle: 1,
            instruction,
            d16: 0,
            d8: 0,
        }
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
                    interrupts: InterruptRegisters::new(),
                    cycle: 0,
                    state: CpuState::Ready,
                    instr_state: None,
                }
            },
            Platform::GBC => {
                Self {
                    registers: Registers::new(platform),
                    sp: constants::GBC_SP,
                    pc: constants::GBC_PC,
                    interrupts: InterruptRegisters::new(),
                    cycle: 0,
                    state: CpuState::Ready,
                    instr_state: None,
                }
            }
        }
    }

    pub fn tick(&mut self, memory: &mut Memory) {
        let mut update_pc = true;
        match &self.state {
            CpuState::Ready => {
                //let mut instr = memory.read(self.pc);             // fetch instruction at [PC]
                let mut instr: u8 = 0;
                let mut cycle: u32 = 0;
                match &mut self.instr_state {
                    Some(x) => {
                        if 1 <= *x.cycle && *x.cycle < 4 {
                            *x.cycle = *x.cycle + 1;
                            return;
                        }
                        instr = *x.instruction;
                        *x.cycle = *x.cycle + 1;
                        cycle = *x.cycle;
                    },
                    None => {
                        instr = memory.read(self.pc);
                        self.pc = self.pc + 1;
                        self.instr_state = Some(InstructionState::new(instr as u16));
                        return;
                    }
                }
                match instr {
                    0xCB  => self.state = CpuState::Wait(4),                                                                       //TODO PREFIX CB
                    0x00 => self.nop(),                                                                                             //NOP
                    0x01 => self.lsm16_ld(cycle, memory, Register8::B, Register8::C),                                                           //LD BC,u16
                    0x02 => self.lsm8_sti(cycle, memory, Register16::BC, Register8::A),                                     //LD (BC),A
                    0x03 => self.alu16_inc(cycle, Register8:B, Register8:C),                                                                 //INC BC
                    0x04 => self.alu8_inc(Register8::B),                                                                    //INC B
                    0x05 => self.alu8_dec(Register8::B),                                                                    //DEC B
                    0x06 => self.lsm8_ld(cycle, memory, Register8::B),                                                             //LD B,u8
                    0x07 => self.rsb8_rlca(),                                                                                        //RLCA
                    0x08 => self.lsm16_st_sp(memory),                                                                            //LD (u16),SP
                    0x09 => self.alu16_add(Register16::HL, Register16::BC),                                             //ADD HL,BC
                    0x0A => self.lsm8_ldi(memory, Register8::A, Register16::BC),                                    //LD A,(BC)
                    0x0B => self.alu16_dec(Register16::BC),                                                                //DEC BC
                    0x0C => self.alu8_inc(Register8::C),                                                                   //INC C
                    0x0D => self.alu8_dec(Register8::C),                                                                   //DEC C
                    0x0E => self.lsm8_ld(cycle, memory, Register8::C),                                                            //LD C,u8
                    0x0F => self.rsb8_rrca(),                                                                                    //RRCA
                    0x10 => {                                                                                                    //TODO STOP
                        self.state = CpuState::Wait(4);
                    },
                    0x11 => self.lsm16_ld(cycle,memory, Register8::D, Register8::E),                                                         //LD DE,u16
                    0x12 => self.lsm8_sti(cycle, memory, Register16::DE, Register8::A),                                    //LD (DE),A
                    0x13 => self.alu16_inc(cycle, Register8:D, Register8:E),                                                                //INC DE
                    0x14 => self.alu8_inc(Register8::D),                                                                   //INC D
                    0x15 => self.alu8_dec(Register8::D),                                                                   //DEC D
                    0x16 => self.lsm8_ld(cycle, memory, Register8::D),                                                            //LD D,u8
                    0x17 => self.rsb8_rla(),                                                                                     //RLA
                    0x18 => cycles += self.jr(memory, vec![]),                                                          //JR i8
                    0x19 => self.alu16_add(Register16::HL, Register16::DE), //ADD HL,DE
                    0x1A => self.lsm8_ldi(memory, Register8::A, Register16::DE),                                   //LD A,(DE)
                    0x1B => self.alu16_dec(Register16::DE),                                                               //DEC DE
                    0x1C => self.alu8_inc(Register8::E),                                                                  //INC E
                    0x1D => self.alu8_dec(Register8::E),                                                                  //DEC E
                    0x1E => self.lsm8_ld(cycle, memory, Register8::E),                                                           //LD E,u8
                    0x1F => self.rsb8_rra(),                                                                                    //RRA
                    0x20 => cycles += self.jr(memory, vec![Flags::N, Flags::Z]),                                       //JR NZ,e8
                    0x21 => self.lsm16_ld(cycle, memory, Register8::H, Register8::L),                                                        //LD HL,u16
                    0x22 => {                                                                                                   //LD (HL+), A
                        self.lsm8_sti(cycle, memory, Register16::HL, Register8::A);
                        self.alu16_inc(cycle, Register8:H, Register8:L);
                    },
                    0x23 => self.alu16_inc(cycle, Register8:H, Register8:L),                                                               //INC HL
                    0x24 => self.alu8_inc(Register8::H),                                                                  //INC H
                    0x25 => self.alu8_dec(Register8::H),                                                                  //DEC H
                    0x26 => self.lsm8_ld(cycle, memory, Register8::H),                                                           //LD H,u8
                    0x27 => self.alu8_daa(),                                                                                    //DAA
                    0x28 => cycles += self.jr(memory, vec![Flags::Z]),                                                 //JR Z,e8
                    0x29 => self.alu16_add(Register16::HL, Register16::HL),  //ADD HL,HL
                    0x2A => {                                                                                                   //LD A, (HL+)
                        self.lsm8_ldi(memory, Register8::A, Register16::HL);
                        self.alu16_inc(cycle, Register8:H, Register8:L);
                    },
                    0x2B => self.alu16_dec(Register16::HL),                                                               //DEC HL
                    0x2C => self.alu8_inc(Register8::L),                                                                  //INC L
                    0x2D => self.alu8_dec(Targets::L),                                                                   //DEC L
                    0x2E => self.lsm8_ld(cycle, memory, Register8::L),                                                           //LD L,u8
                    0x2F => self.alu8_cpl(),                                                                                    //CPL
                    0x30 => cycles += self.jr(memory, vec![Flags::N, Flags::C]),                                       //JR NC,e8
                    0x31 => self.sp = self.fetch_u16_immediate(memory),                                                         //LD SP,u16 TODO
                    0x32 => {                                                                                                   //LD (HL-), A
                        self.lsm8_sti(cycle, memory, Register16::HL, Register8::A);
                        self.alu16_dec(Register16::HL);
                    },
                    0x33 => self.sp = self.sp.overflowing_add(1).0,                                                         //INC SP
                    0x34 => self.alu8_inci(memory, Register16::HL),                                                         //INC (HL)
                    0x35 => self.alu8_deci(memory, Register16::HL),                                                         //DEC (HL)
                    0x36 => self.lsm8_sti_imm(memory, Register16::HL),                                                    //LD (HL), u8
                    0x37 => self.alu8_scf(),                                                                                   //SCF
                    0x38 => cycles += self.jr(memory, vec![Flags::C]),                                                 //JR C,e8
                    0x39 => self.alu16_add_rr_sp(Register16::HL),                                                          //ADD HL,SP TODO
                    0x3A => {                                                                                                   //LD A, (HL-)
                        self.lsm8_ldi(memory, Register8::A, Register16::HL);
                        self.alu16_dec(Register16::HL);
                    },
                    0x3B => self.sp = self.sp.overflowing_sub(1).0,                                                         //DEC SP
                    0x3C => self.alu8_inc(Register8::A),                                                                  //INC A
                    0x3D => self.alu8_dec(Register8::A),                                                                  //DEC A
                    0x3E => self.lsm8_ld(cycle, memory, Register8::A),                                                           //LD A,u8
                    0x3F => self.alu8_ccf(),                                                                                    //CCF
                    0x40 => self.lsm8_mv(Register8::B, Register8::B),                                                     //LD B, B
                    0x41 => self.lsm8_mv(Register8::B, Register8::C),                                                     //LD B, C
                    0x42 => self.lsm8_mv(Register8::B, Register8::D),                                                     //LD B, D
                    0x43 => self.lsm8_mv(Register8::B, Register8::E),                                                     //LD B, E
                    0x44 => self.lsm8_mv(Register8::B, Register8::H),                                                     //LD B, H
                    0x45 => self.lsm8_mv(Register8::B, Register8::L),                                                     //LD B, L
                    0x46 => self.lsm8_ldi(memory, Register8::B, Register16::HL),                                   //LD B, (HL)
                    0x47 => self.lsm8_mv(Register8::B, Register8::A),                                                     //LD B, A
                    0x48 => self.lsm8_mv(Register8::C, Register8::B),                                                     //LD C, B
                    0x49 => self.lsm8_mv(Register8::C, Register8::C),                                                     //LD C, C
                    0x4A => self.lsm8_mv(Register8::C, Register8::D),                                                     //LD C, D
                    0x4B => self.lsm8_mv(Register8::C, Register8::E),                                                     //LD C, E
                    0x4C => self.lsm8_mv(Register8::C, Register8::H),                                                     //LD C, H
                    0x4D => self.lsm8_mv(Register8::C, Register8::L),                                                     //LD C, L
                    0x4E => self.lsm8_ldi(memory, Register8::C, Register16::HL),                                   //LD C, (HL)
                    0x4F => self.lsm8_mv(Register8::C, Register8::A),                                                     //LD C, A
                    0x50 => self.lsm8_mv(Register8::D, Register8::B),                                                     //LD D, B
                    0x51 => self.lsm8_mv(Register8::D, Register8::C),                                                     //LD D, C
                    0x52 => self.lsm8_mv(Register8::D, Register8::D),                                                     //LD D, D
                    0x53 => self.lsm8_mv(Register8::D, Register8::E),                                                     //LD D, E
                    0x54 => self.lsm8_mv(Register8::D, Register8::H),                                                     //LD D, H
                    0x55 => self.lsm8_mv(Register8::D, Register8::L),                                                     //LD D, L
                    0x56 => self.lsm8_ldi(memory, Register8::D, Register16::HL),                                   //LD D, (HL)
                    0x57 => self.lsm8_mv(Register8::D, Register8::A),                                                     //LD D, A
                    0x58 => self.lsm8_mv(Register8::E, Register8::B),                                                     //LD E, B
                    0x59 => self.lsm8_mv(Register8::E, Register8::C),                                                     //LD E, C
                    0x5A => self.lsm8_mv(Register8::E, Register8::D),                                                     //LD E, D
                    0x5B => self.lsm8_mv(Register8::E, Register8::E),                                                     //LD E, E
                    0x5C => self.lsm8_mv(Register8::E, Register8::H),                                                     //LD E, H
                    0x5D => self.lsm8_mv(Register8::E, Register8::L),                                                     //LD E, L
                    0x5E => self.lsm8_ldi(memory, Register8::E, Register16::HL),                                   //LD E, (HL)
                    0x5F => self.lsm8_mv(Register8::E, Register8::A),                                                      //LD E, A
                    0x60 => self.lsm8_mv(Register8::H, Register8::B),                                                     //LD H, B
                    0x61 => self.lsm8_mv(Register8::H, Register8::C),                                                     //LD H, C
                    0x62 => self.lsm8_mv(Register8::H, Register8::D),                                                     //LD H, D
                    0x63 => self.lsm8_mv(Register8::H, Register8::E),                                                     //LD H, E
                    0x64 => self.lsm8_mv(Register8::H, Register8::H),                                                     //LD H, H
                    0x65 => self.lsm8_mv(Register8::H, Register8::L),                                                     //LD H, L
                    0x66 => self.lsm8_ldi(memory, Register8::H, Register16::HL),                                  //LD H, (HL)
                    0x67 => self.lsm8_mv(Register8::H, Register8::A),                                                     //LD H, A
                    0x68 => self.lsm8_mv(Register8::L, Register8::B),                                                     //LD L, B
                    0x69 => self.lsm8_mv(Register8::L, Register8::C),                                                     //LD L, C
                    0x6A => self.lsm8_mv(Register8::L, Register8::D),                                                     //LD L, D
                    0x6B => self.lsm8_mv(Register8::L, Register8::E),                                                     //LD L, E
                    0x6C => self.lsm8_mv(Register8::L, Register8::H),                                                     //LD L, H
                    0x6D => self.lsm8_mv(Register8::L, Register8::L),                                                     //LD L, L
                    0x6E => self.lsm8_ldi(memory, Register8::L, Register16::HL),                                   //LD L, (HL)
                    0x6F=> self.lsm8_mv(Register8::L, Register8::A),                                                      //LD L, A
                    0x70 => self.lsm8_sti(cycle, memory, Register16::HL, Register8::B),                                  //LD (HL), B
                    0x71 => self.lsm8_sti(cycle, memory, Register16::HL, Register8::C),                                  //LD (HL), C
                    0x72 => self.lsm8_sti(cycle, memory, Register16::HL, Register8::D),                                  //LD (HL), D
                    0x73 => self.lsm8_sti(cycle, memory, Register16::HL, Register8::E),                                  //LD (HL), E
                    0x74 => self.lsm8_sti(cycle, memory, Register16::HL, Register8::H),                                  //LD (HL), H
                    0x75 => self.lsm8_sti(cycle, memory, Register16::HL, Register8::L),                                  //LD (HL), L
                    0x76 => {                                                                                                   //HALT
                        CpuState::Wait(cycles)
                    },
                    0x77 => self.lsm8_sti(cycle, memory, Register16::HL, Register8::A),                                  //LD (HL), A
                    0x78 => self.lsm8_mv(Register8::A, Register8::B),                                                     //LD A, B
                    0x79 => self.lsm8_mv(Register8::A, Register8::C),                                                     //LD A, C
                    0x7A => self.lsm8_mv(Register8::A, Register8::D),                                                     //LD A, D
                    0x7B => self.lsm8_mv(Register8::A, Register8::E),                                                     //LD A, E
                    0x7C => self.lsm8_mv(Register8::A, Register8::H),                                                     //LD A, H
                    0x7D => self.lsm8_mv(Register8::A, Register8::L),                                                     //LD A, L
                    0x7E => self.lsm8_ldi(memory, Register8::A, Register16::HL),                                  //LD A, (HL)
                    0x7F=> self.lsm8_mv(Register8::A, Register8::A),                                                      //LD A, A
                    0x80 => self.alu8_add(Register8::A, Register8::B),                                            //ADD A, B
                    0x81 => self.alu8_add(Register8::A, Register8::C),                                            //ADD A, C
                    0x82 => self.alu8_add(Register8::A, Register8::D),                                            //ADD A, D
                    0x83 => self.alu8_add(Register8::A, Register8::E),                                            //ADD A, E
                    0x84 => self.alu8_add(Register8::A, Register8::H),                                            //ADD A, H
                    0x85 => self.alu8_add(Register8::A, Register8::L),                                            //ADD A, L
                    0x86 => self.alu8_addi(memory, Register8::A, Register16::HL),                                     //ADD A, (HL)
                    0x87 => self.alu8_add(Register8::A, Register8::A),                                            //ADD A, A
                    0x88 => self.alu8_adc(Register8::A, Register8::B),                                            //ADC A, B
                    0x89 => self.alu8_adc(Register8::A, Register8::C),                                            //ADC A, C
                    0x8A => self.alu8_adc(Register8::A, Register8::D),                                            //ADC A, D
                    0x8B => self.alu8_adc(Register8::A, Register8::E),                                            //ADC A, E
                    0x8C => self.alu8_adc(Register8::A, Register8::H),                                            //ADC A, H
                    0x8D => self.alu8_adc(Register8::A, Register8::L),                                            //ADC A, L
                    0x8E => self.alu8_adci(memory, Register8::A, Register16::HL),                                     //ADC A, (HL)
                    0x8F => self.alu8_adc(Register8::A, Register8::A),                                            //ADC A, A
                    0x90 => self.alu8_sub(Register8::A, Register8::B),                                            //SUB A, B
                    0x91 => self.alu8_sub(Register8::A, Register8::C),                                            //SUB A, C
                    0x92 => self.alu8_sub(Register8::A, Register8::D),                                            //SUB A, D
                    0x93 => self.alu8_sub(Register8::A, Register8::E),                                            //SUB A, E
                    0x94 => self.alu8_sub(Register8::A, Register8::H),                                            //SUB A, H
                    0x95 => self.alu8_sub(Register8::A, Register8::L),                                            //SUB A, L
                    0x96 => self.alu8_subi(memory, Register8::A, Register16::HL),                                     //SUB A, (HL)
                    0x97 => self.alu8_sub(Register8::A, Register8::A),                                            //SUB A, A
                    0x98 => self.alu8_subc(Register8::A, Register8::B),                                                           //SBC A, B
                    0x99 => self.alu8_subc(Register8::A, Register8::C),                                                           //SBC A, C
                    0x9A => self.alu8_subc(Register8::A, Register8::D),                                                           //SBC A, D
                    0x9B => self.alu8_subc(Register8::A, Register8::E),                                                           //SBC A, E
                    0x9C => self.alu8_subc(Register8::A, Register8::H),                                                           //SBC A, H
                    0x9D => self.alu8_subc(Register8::A, Register8::L),                                                           //SBC A, L
                    0x9E => self.alu8_subci(memory, Register8::A, Register16::HL),                                                //SBC A, (HL)
                    0x9F => self.alu8_subc(Register8::A, Register8::A),                                                           //SBC A, A
                    0xA0 => self.alu8_and(Register8::A, Register8::B),                                            //AND A, B
                    0xA1 => self.alu8_and(Register8::A, Register8::C),                                            //AND A, C
                    0xA2 => self.alu8_and(Register8::A, Register8::D),                                            //AND A, D
                    0xA3 => self.alu8_and(Register8::A, Register8::E),                                            //AND A, E
                    0xA4 => self.alu8_and(Register8::A, Register8::H),                                            //AND A, H
                    0xA5 => self.alu8_and(Register8::A, Register8::L),                                            //AND A, L
                    0xA6 => self.alu8_andi(memory, Register8::A, Register16::HL),                                     //AND A, (HL)
                    0xA7 => self.alu8_and(Register8::A, Register8::A),                                            //AND A, A
                    0xA8 => self.alu8_xor(Register8::A, Register8::B),                                            //XOR A, B
                    0xA9 => self.alu8_xor(Register8::A, Register8::C),                                            //XOR A, C
                    0xAA => self.alu8_xor(Register8::A, Register8::D),                                            //XOR A, D
                    0xAB => self.alu8_xor(Register8::A, Register8::E),                                            //XOR A, E
                    0xAC => self.alu8_xor(Register8::A, Register8::H),                                            //XOR A, H
                    0xAD => self.alu8_xor(Register8::A, Register8::L),                                            //XOR A, L
                    0xAE => self.alu8_xori(memory, Register8::A, Register16::HL),                                     //XOR A, (HL)
                    0xAF => self.alu8_xor(Register8::A, Register8::A),                                            //XOR A, A
                    0xB0 => self.alu8_or(Register8::A, Register8::B),                                             //OR A, B
                    0xB1 => self.alu8_or(Register8::A, Register8::C),                                             //OR A, C
                    0xB2 => self.alu8_or(Register8::A, Register8::D),                                             //OR A, D
                    0xB3 => self.alu8_or(Register8::A, Register8::E),                                             //OR A, E
                    0xB4 => self.alu8_or(Register8::A, Register8::H),                                             //OR A, H
                    0xB5 => self.alu8_or(Register8::A, Register8::L),                                             //OR A, L
                    0xB6 => self.alu8_ori(memory, Register8::A, Register16::HL),                                      //OR A, (HL)
                    0xB7 => self.alu8_or(Register8::A, Register8::A),                                             //OR A, A
                    0xB8 => self.alu8_cp(Register8::A, Register8::B),                                             //CP A, B
                    0xB9 => self.alu8_cp(Register8::A, Register8::C),                                             //CP A, C
                    0xBA => self.alu8_cp(Register8::A, Register8::D),                                             //CP A, D
                    0xBB => self.alu8_cp(Register8::A, Register8::E),                                             //CP A, E
                    0xBC => self.alu8_cp(Register8::A, Register8::H),                                             //CP A, H
                    0xBD => self.alu8_cp(Register8::A, Register8::L),                                             //CP A, L
                    0xBE => self.alu8_cpi(memory, Register8::A, Register16::HL),                                      //CP A, (HL)
                    0xBF => self.alu8_cp(Register8::A, Register8::A),                                             //CP A, A
                    0xC0 => {                                                                                                   //RET NZ
                        if self.registers.get_flag(Flags::N) && self.registers.get_flag(Flags::Z) {
                            self.ret(memory);
                            cycles += 12;
                        }
                    },
                    0xC1 => self.lsm16_pop(memory, Register16::BC),                                                       //POP BC
                    0xC2 => {                                                                                                   //JP NZ, u16
                        let mut d16 = memory.read(self.pc + 1) as u16 + ((memory.read(self.pc + 2) as u16) << 8);
                        if self.registers.get_flags(Flags::N) && self.registers.get_flags(Flags::Z) {
                            update_pc = false;
                            self.pc = d16;
                            cycles += 4;
                        }
                    }
                    0xC3 => {                                                                                                   //JP u16
                        let d16 = memory.read(self.pc + 1) as u16 + ((memory.read(self.pc + 2) as u16) << 8);
                        update_pc = false;
                        self.pc = d16;
                    },
                    0xC4 => {                                                                                                   //CALL NZ,u16
                        let d16 = memory.read(self.pc + 1) as u16 + ((memory.read(self.pc + 2) as u16) << 8);
                        if self.registers.get_flags(Flags::N) && self.registers.get_flags(Flags::Z) {
                            update_pc = self.call(memory, d16);
                            cycles += 12;
                        }
                    },
                    0xC5 => self.lsm16_push(memory, Register16::BC),                                                      //PUSH BC
                    0xC6 => self.alu8_add_imm(memory, Register8::A),                                                      //ADD A, u8
                    0xC7 => update_pc = self.rst(memory, 0x00),                                                             //RST 00
                    0xC8 => {                                                                                                   //RET Z
                        if self.registers.get_flag(Flags::Z) {
                            self.ret(memory);
                            cycles += 12;
                        }
                    },
                    0xC9 => self.ret(memory),                                                                                   //RET
                    0xCA => {                                                                                                   //JP Z, u16
                        let mut d16 = memory.read(self.pc + 1) as u16 + ((memory.read(self.pc + 2) as u16) << 8);
                        if self.registers.get_flags(Flags::Z) {
                            update_pc = false;
                            self.pc = d16;
                            cycles += 4;
                        }
                    },
                    0xCC => {                                                                                                   //CALL Z, u16
                        let d16 = memory.read(self.pc + 1) as u16 + ((memory.read(self.pc + 2) as u16) << 8);
                        if self.registers.get_flags(Flags::Z) {
                            update_pc = self.call(memory, d16);
                            cycles += 12;
                        }
                    },
                    0xCD => {                                                                                                   //CALL u16
                        let d16 = memory.read(self.pc + 1) as u16 + ((memory.read(self.pc + 2) as u16) << 8);
                        update_pc = self.call(memory, d16);
                    },
                    0xCE => self.alu8_adc_imm(memory, Targets::A),                                                       //ADC A, u8
                    0xCF => update_pc = self.rst(memory, 0x08),                                                             //RST 08h
                    0xD0 => {                                                                                                   //RET NC
                        if self.registers.get_flag(Flags::N) && self.registers.get_flag(Flags::C) {
                            self.ret(memory);
                            cycles += 12;
                        }
                    },
                    0xD1 => self.lsm16_pop(memory, Register16::DE),                                                       //POP DE
                    0xD2 => {                                                                                                   //JP NC, u16
                        let mut d16 = memory.read(self.pc + 1) as u16 + ((memory.read(self.pc + 2) as u16) << 8);
                        if self.registers.get_flags(Flags::N) && self.registers.get_flags(Flags::C) {
                            update_pc = false;
                            self.pc = d16;
                            cycles += 4;
                        }
                    },
                    0xD4 => {                                                                                                   //CALL NC,u16
                        let d16 = memory.read(self.pc + 1) as u16 + ((memory.read(self.pc + 2) as u16) << 8);
                        if self.registers.get_flags(Flags::N) && self.registers.get_flags(Flags::C) {
                            update_pc = self.call(memory, d16);
                            cycles += 12;
                        }
                    },
                    0xD5 => self.lsm16_push(memory, Register16::DE),                                                      //PUSH DE
                    0xD6 => self.alu8_sub_imm(memory, Targets::A),                                                       //SUB A, u8
                    0xD7 => update_pc = self.rst(memory, 0x10),                                                             //RST 10h
                    0xD8 => {                                                                                                   //RET Z
                        if self.registers.get_flag(Flags::C) {
                            self.ret(memory);
                            cycles += 12;
                        }
                    },
                    0xD9 => {                                                                                                   //RETI
                        self.interrupts.set_ime();
                        self.ret(memory);
                    },
                    0xDA => {                                                                                                   //JP C, u16
                        let mut d16 = memory.read(self.pc + 1) as u16 + ((memory.read(self.pc + 2) as u16) << 8);
                        if self.registers.get_flags(Flags::C) {
                            update_pc = false;
                            self.pc = d16;
                            cycles += 4;
                        }
                    },
                    0xDC => {                                                                                                   //CALL C, u16
                        let d16 = memory.read(self.pc + 1) as u16 + ((memory.read(self.pc + 2) as u16) << 8);
                        if self.registers.get_flags(Flags::C) {
                            update_pc = self.call(memory, d16);
                            cycles += 12;
                        }
                    },
                    0xDE => self.alu8_sbc_imm(memory, Register8::A),                                                      //SBC A, A
                    0xDF => update_pc = self.rst(memory, 0x18),                                                             //RS 18h
                    0xE0 => self.lsm8_st_extended(memory, Register8::A, true),                                   //LD (FF00+u8), A
                    0xE1 => self.lsm16_pop(memory, Register16::HL),                                                       //POP HL
                    0xE2 => self.lsm8_st_extended(memory, Register8::A, false),                                  //LD (FF00+C), A
                    0xE5 => self.lsm16_push(memory, Register16::HL),                                                      //PUSH HL
                    0xE6 => self.alu8_and_imm(memory, Register8::A),                                                      //AND A, u8
                    0xE7 => update_pc = self.rst(memory, 0x20),                                                             //RST 20h
                    0xE8 => self.alu16_add_spimm(memory),                                                                       //ADD SP, i8
                    0xE9 => {                                                                                                   //JP HL
                        let d16 = self.registers.get16(Register16::HL);
                        update_pc = false;
                        self.pc = d16;
                    },
                    0xEA => self.lsm8_st(memory, Register8::A),                                                           //LD (u16), A
                    0xEE => self.alu8_xor_imm(memory, Register8::A),                                                      //XOR A, u8
                    0xEF => update_pc = self.rst(memory, 0x28),                                                             //RST 28h
                    0xF0 => self.lsm8_ld_extended(memory, Register8::A, true),                                   //LD A, (FF00+u8)
                    0xF1 => self.lsm16_pop(memory, Register16::AF),                                                       //POP AF
                    0xF2 => self.lsm8_ld_extended(memory, Register8::A, false),                                  //LD A, (FF00+C)
                    0xF3 => self.interrupts.reset_ime(),                                                                         //DI (Disable interrupts) TODO
                    0xF5 => self.lsm16_push(memory, Register16::AF),                                                      //PUSH AF
                    0xF6 => self.alu8_or_imm(memory, Register8::A),                                                       //OR A, u8
                    0xF7 => update_pc = self.rst(memory, 0x30),                                                             //RST 30h
                    0xF8 => self.alu16_ld_spimm(memory, Register16::HL),                                                    //LD HL, SP+i8
                    0xF9 => self.sp = self.registers.get16(Register16::HL),                                                   //LD SP, HL
                    0xFA => self.lsm8_ldi_imm(memory, Register8::A),                                                       //LD A, (u16)
                    0xFB => self.interrupts.set_ime(),                                                                           //EI (Enable interrupts) TODO
                    0xFE => self.alu8_cp_imm(memory, Register8::A),                                                       //CP A, u8
                    0xFF => update_pc = self.rst(memory, 0x38),                                                             //RST 38h
                    _ => ()
                }
            },
            _ => {},
        }
        return;
    }

    pub fn set_pc(&mut self, val: u16) {
        self.pc = val;
    }

    pub fn get_interrupt(&self) -> Option<Interrupt> {
        if self.interrupts.check_interrupt(Interrupt::VerticalBlanking) {
            return Some(Interrupt::VerticalBlanking);
        } else if self.interrupts.check_interrupt(Interrupt::LcdStat) {
            return Some(Interrupt::LcdStat);
        } else if self.interrupts.check_interrupt(Interrupt::Timer) {
            return Some(Interrupt::Timer);
        } else if self.interrupts.check_interrupt(Interrupt::Serial) {
            return Some(Interrupt::Serial);
        } else if self.interrupts.check_interrupt(Interrupt::Joypad) {
            return Some(Interrupt::Joypad);
        }
        return None;
    }

    pub fn push_pc(&mut self, memory: &mut Memory) {
        let pc_high: u8 = (&self.PC & 0xFF00) >> 8 as u8;
        let pc_low: u8 = (&self.PC & 0x00FF) as u8;
        self.SP.write(self.SP.read() - 1);
        memory.write(self.sp, pc_high);
        self.SP.write(self.SP.read() - 1);
        memory.write(self.sp, pc_low);
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

    /*
        Reads byte at PC+1 and returns as u8
     */
    fn fetch_u8_immdiate(&self, memory: &mut Memory) -> u8 {
        return memory.read(self.pc + 1);
    }

    /*
        Reads bytes PC+1 and PC+2 and returns them as u16
        PC+1 is least-significant byte
     */
    fn fetch_u16_immediate(&self, memory: &mut Memory) -> u16 {
        let mut d16 = memory.read(self.pc + 1) as u16;
        d16 = d16 + ((memory.read(self.pc + 2) as u16) << 8);
        return d16;
    }

    pub fn nop(&mut self) {
        self.instr_state = None;
    }
    /*
        8-bit load/store/move
        * Load
            * ld R, u8; load 8-bit immediate value into R; ex. LD B, u8
            * ldi R1, (R2); load value at memory address pointed to by 16-bit register R2 into R1; ex. LD A, (BC)
            * ldi R, (u16); load value at memory address pointed to by 16-bit immediate value into R; ex. LD A, (u16)
        * Store
            * st (u16), R; store value in R (or PC/SP) to memory address pointed to by 16-bit immediate value; ex. LD (u16), A
            * sti (R1), R2; store value in 16-bit R2 to memory address pointed to by R1; ex. LD (BC), A
            * sti (R), u8; store an 8-bit immediate value at the memory address pointed to by 16-bit R; ex. LD (HL), u8
        * Move
            * mv R1, R2; copy value from R2 to R1; ex. LD B, A
     */
    //ld R, u8
    fn lsm8_ld(&mut self, cycle: u32, memory: &mut Memory, register: Register8) {
        if cycle == 8 {
            let val = self.fetch_u8_immdiate(memory);
            self.registers.set8(register, val);
            self.instr_state = None;
        }
    }

    //st (0xFF + u8), R OR st (0xFF + C), R
    fn lsm8_ld_extended(&mut self, memory:&mut Memory, register: Register8, immediate: boolean) {
        let mut addr: u16 = if immediate { 0xFF00 + self.fetch_u8_immdiate(memory) as u16 } else { 0xFF00 + self.registers.get8(Register8::C) as u16 };
        let val = memory.read(addr);
        self.registers.set8(register, val);
    }

    //ldi R1, (R2)
    fn lsm8_ldi(&mut self, memory: &mut Memory, register1: Register8, register2: Register16) {
        let addr = self.registers.get16(register2);
        let val = memory.read(addr);
        self.registers.set8(register1, val);
    }

    //ldi R, (u16)
    fn lsm8_ldi_imm(&mut self, memory: &mut Memory, register: Register8) {
        let mut addr= self.fetch_u16_immediate(memory);
        let val = memory.read(addr);
        self.registers.set8(register, val);
    }

    //st (u16), R
    fn lsm8_st(&mut self, memory: &mut Memory, register: Register8) {
        let mut addr= self.fetch_u16_immediate(memory);
        let val = self.registers.get8(register);
        memory.write(addr, val);
    }

    //st (0xFF + u8), R OR st (0xFF + C), R
    fn lsm8_st_extended(&mut self, memory:&mut Memory, register: Register8, immediate: boolean) {
        let mut addr: u16 = if immediate { 0xFF00 + self.fetch_u8_immdiate(memory) as u16 } else { 0xFF00 + self.registers.get8(Register8::C) as u16 };
        let val = self.registers.get8(register);
        memory.write(addr, val);
    }
    
    //sti (R1), R2
    fn lsm8_sti(&mut self, cycle: u32, memory: &mut Memory, register1: Register16, register2: Register8) {
        if cycle == 8 {
            let addr = self.registers.get16(register1);
            let val = self.registers.get8(register2);
            memory.write(addr, val);
        }
    }

    //sti (R), u8
    fn lsm8_sti_imm(&mut self, memory: &mut Memory, register: Register16) {
        let addr = self.registers.get16(register);
        let val = self.fetch_u8_immdiate(memory);
        memory.write(addr, val);
    }

    //mv R1, R2
    fn lsm8_mv(&mut self, dst: Register8, src: Register8) {
        self.registers.set8(dst, self.registers.get8(src));
    }

    /*
        16-bit load/store/move
        * Load
            * ld RR, u16; loads 16-bit immediate value into register pair RR; ex. LD BC, u16
            * pop RR; removes top two 8-bit values from the stack and loads them to register pair RR; ex. POP BC
        * Store
            * st (u16); RR; stores value in register pair RR to the memory address pointed to by 16-bit immediate; ex. LD (u16), SP
            * push RR; pushes value stored in register pair RR to the stack; ex. PUSH BC
         * Move
           * mv RR1, RR2; copy the value stored in register pair RR2 into RR1; ex. LD HL, SP
     */
    //ld RR, u16
    fn lsm16_ld(&mut self, cycle: u32, memory: &mut Memory, rhigh: Register8, rlow: Register8) {
        if cycle == 8 {
            self.registers.set8(rlow, memory.read(self.pc));
            self.pc = self.pc + 1;
        } else if cycle == 12 {
            self.registers.set8(rhigh, memory.read(self.pc));
            self.pc = self.pc + 1;
            self.instr_state = None;
        }
    }

    //st (u16), RR
    fn lsm16_st(&mut self, memory: &mut Memory, register: Register16) {
        let mut addr= self.fetch_u16_immediate(memory);
        let val = self.registers.get16(register);
        let low = (val & 0x00FF) as u8;
        let high = (val >> 8) as u8;
        memory.write(addr, low);
        memory.write(addr + 1, high);
    }

    //push RR
    fn lsm16_push(&mut self, memory: &mut Memory, register: Register16) {
        let val = self.registers.get(register);
        let high = (val >> 8) as u8;
        let low = (val & 0x00FF) as u8;
        memory.write(self.sp - 1, high);
        memory.write(self.sp - 2, low);
        self.sp = self.sp - 2;
    }

    //pop RR
    fn lsm16_pop(&mut self, memory: &mut Memory, register: Register16) {
        let low = memory.read(self.sp) as u16;
        let high = memory.read(self.sp + 1) as u16;
        let val = low + (high << 8);
        self.registers.set16(register, val);
        self.sp = self.sp + 2;
    }

    //mv RR1, RR2
    fn lsm16_mv(&mut self, register1: Register16, register2: Register16) {
        self.registers.set16(register1, self.registers.get16(register2));
    }

    //store SP
    fn lsm16_st_sp(mut self, cycle: u32, memory: &mut Memory) {
        if cycle == 8 {
            match self.instr_state {
                Some(x) => *x.d16 = self.sp,
                None => ()
            }
        }
        let addr = self.fetch_u16_immediate(memory);
        memory.write(addr, (self.sp & 0xFF) as u8);
        memory.write(addr + 1, (self.sp >> 8) as u8);
    }

    /*
        8-bit arithmetic / logic
        * INC R
            * Flags: ZNH
        * INC (RR)
            * Flags: ZNH
        * DEC R
        * INC (RR)
        * DEC (RR)
        * DAA
        * SCF
        * CPL
        * CCF
        * ADD R, R
        * ADD R, u8
        * ADD R, (RR)
        * ADC R, R
        * ADC R, u8
        * ADC R, (RR)
        * SUB R, R
        * SUB R, u8
        * SUB R, (RR)
        * SBC R, R
        * SBC R, u8
        * SBC R, (RR)
        * AND R, R
        * AND R, u8
        * AND R, (RR)
        * XOR R, R
        * XOR R, u8
        * XOR R, (RR)
        * OR R, R
        * OR R, u8
        * OR R, (RR)
        * CP R, R
        * CP R, u8
        * CP R, (RR)
     */
    //Increment 8-bit register and set ZNH accordingly
    fn alu8_inc(&mut self, register: Register8) {
        let val = self.registers.get8(register);
        let res = val.overflowing_add(1);
        if res.0 == 0 { self.registers.set_flag(Flags::Z) } else { self.registers.unset_flag(Flags::Z) }
        if (val & 0xF) + 1  > 0xF { self.registers.set_flag(Flags::H) } else { self.registers.unset_flag(Flags::H) }
        self.registers.unset_flag(Flags::N);
        self.registers.set8(register, res.0);
        self.instr_state = None;
    }

    fn alu8_inci(&mut self, memory: &mut Memory, pair: Register16) {
        let mut addr = self.registers.get16(pair);
        let val = memory.read(addr);
        let res = val.overflowing_add(1);
        if res.0 == 0 { self.registers.set_flag(Flags::Z) } else { self.registers.unset_flag(Flags::Z) }
        if (val & 0xF) + 1  > 0xF { self.registers.set_flag(Flags::H) } else { self.registers.unset_flag(Flags::H) }
        self.registers.unset_flag(Flags::N);
        memory.write(addr, res.0);
    }

    //Decrement 8-bit register and set ZNH accordingly
    fn alu8_dec(&mut self, register: Register8) {
        let val = self.registers.get8(register);
        let res = val.overflowing_sub(1);
        if res.0 == 0 { self.registers.set_flag(Flags::Z) } else { self.registers.unset_flag(Flags::Z) }
        if (val & 0xF) - 1  > 0xF { self.registers.set_flag(Flags::H) } else { self.registers.unset_flag(Flags::H) }
        self.registers.set_flag(Flags::N);
        self.registers.set8(register, res.0);
        self.instr_state = None;
    }

    fn alu8_deci(&mut self, memory: &mut Memory, pair: Register16) {
        let mut addr = self.registers.get16(pair);
        let val = memory.read(addr);
        let res = val.overflowing_sub(1);
        if res.0 == 0 { self.registers.set_flag(Flags::Z) } else { self.registers.unset_flag(Flags::Z) }
        if (val & 0xF) - 1  > 0xF { self.registers.set_flag(Flags::H) } else { self.registers.unset_flag(Flags::H) }
        self.registers.set_flag(Flags::N);
        memory.write(addr, res.0);
    }

    //add r1, r2
    fn alu8_add(&mut self, register1: Register8, register2: Register8) {
        let v1 = self.registers.get8(register1);
        let v2 = self.registers.get8(register2);
        let res = v1.overflowing_add(v2);
        if res.0 == 0 { self.registers.set_flag(Flags::Z) } else { self.registers.unset_flag(Flags::Z) }
        if res.1 { self.registers.set_flag(Flags::C) } else { self.registers.unset_flag(Flags::C) }
        if (v1 & 0xF) + (v2 & 0xF) > 0xF { self.registers.set_flag(Flags::H) } else { self.registers.unset_flag(Flags::H) }
        self.registers.unset_flag(Flags::N);
        self.registers.set8(register1, res.0);
    }
    //add r, u8
    fn alu8_add_imm(&mut self, memory: &mut Memory, register: Register8) {
        let v1 = self.registers.get8(register1);
        let v2 = self.fetch_u8_immdiate(memory);
        let res = v1.overflowing_add(v2);
        if res.0 == 0 { self.registers.set_flag(Flags::Z) } else { self.registers.unset_flag(Flags::Z) }
        if res.1 { self.registers.set_flag(Flags::C) } else { self.registers.unset_flag(Flags::C) }
        if (v1 & 0xF) + (v2 & 0xF) > 0xF { self.registers.set_flag(Flags::H) } else { self.registers.unset_flag(Flags::H) }
        self.registers.unset_flag(Flags::N);
        self.registers.set8(register1, res.0);
    }
    //add r, (rr)
    fn alu8_addi(&mut self, memory: &mut Memory, register: Register8, pair: Register16) {
        let v1 = self.registers.get8(register);
        let addr = self.registers.get16(pair);
        let v2 = memory.read(addr);
        let res = v1.overflowing_add(v2);
        if res.0 == 0 { self.registers.set_flag(Flags::Z) } else { self.registers.unset_flag(Flags::Z) }
        if res.1 { self.registers.set_flag(Flags::C) } else { self.registers.unset_flag(Flags::C) }
        if (v1 & 0xF) + (v2 & 0xF) > 0xF { self.registers.set_flag(Flags::H) } else { self.registers.unset_flag(Flags::H) }
        self.registers.unset_flag(Flags::N);
        self.registers.set8(register1, res.0);
    }

    //add r1, r2
    fn alu8_sub(&mut self, register1: Register8, register2: Register8) {
        let v1 = self.registers.get8(register1);
        let v2 = self.registers.get8(register2);
        let res = v1.overflowing_sub(v2);
        if res.0 == 0 { self.registers.set_flag(Flags::Z) } else { self.registers.unset_flag(Flags::Z) }
        if res.1 { self.registers.set_flag(Flags::C) } else { self.registers.unset_flag(Flags::C) }
        if (v1 & 0xF) - (v2 & 0xF) > 0xF { self.registers.set_flag(Flags::H) } else { self.registers.unset_flag(Flags::H) }
        self.registers.set_flag(Flags::N);
        self.registers.set8(register1, res.0);
    }
    //add r, u8
    fn alu8_sub_imm(&mut self, memory: &mut Memory, register: Register8) {
        let v1 = self.registers.get8(register1);
        let v2 = self.fetch_u8_immdiate(memory);
        let res = v1.overflowing_sub(v2);
        if res.0 == 0 { self.registers.set_flag(Flags::Z) } else { self.registers.unset_flag(Flags::Z) }
        if res.1 { self.registers.set_flag(Flags::C) } else { self.registers.unset_flag(Flags::C) }
        if (v1 & 0xF) - (v2 & 0xF) > 0xF { self.registers.set_flag(Flags::H) } else { self.registers.unset_flag(Flags::H) }
        self.registers.set_flag(Flags::N);
        self.registers.set8(register1, res.0);
    }
    //add r, (rr)
    fn alu8_subi(&mut self, memory: &mut Memory, register: Register8, pair: Register16) {
        let v1 = self.registers.get8(register);
        let addr = self.registers.get16(pair);
        let v2 = memory.read(addr);
        let res = v1.overflowing_sub(v2);
        if res.0 == 0 { self.registers.set_flag(Flags::Z) } else { self.registers.unset_flag(Flags::Z) }
        if res.1 { self.registers.set_flag(Flags::C) } else { self.registers.unset_flag(Flags::C) }
        if (v1 & 0xF) - (v2 & 0xF) > 0xF { self.registers.set_flag(Flags::H) } else { self.registers.unset_flag(Flags::H) }
        self.registers.set_flag(Flags::N);
        self.registers.set8(register1, res.0);
    }

    //add r1, r2
    fn alu8_adc(&mut self, register1: Register8, register2: Register8) {
        let v1 = self.registers.get8(register1);
        let v2 = self.registers.get8(register2);
        let mut res = v1.overflowing_add(v2);
        let mut half_carry = (v1 & 0xF) + (v2 & 0xF) > 0xF;
        let mut carry = res.1;
        if self.registers.get_flag(Flags::C) {
            if !half_carry { half_carry = (res.0 & 0xF) + 1 > 0xF }
            res = res.0.overflowing_add(1);
            if !carry { carry = res.1 }
        }
        if res.0 == 0 { self.registers.set_flag(Flags::Z) } else { self.registers.unset_flag(Flags::Z) }
        if carry { self.registers.set_flag(Flags::C) } else { self.registers.unset_flag(Flags::C) }
        if half_carry { self.registers.set_flag(Flags::H) } else { self.registers.unset_flag(Flags::H) }
        self.registers.unset_flag(Flags::N);
        self.registers.set8(register1, res.0);
    }

    //add r, u8
    fn alu8_adc_imm(&mut self, memory: &mut Memory, register: Register8) {
        let v1 = self.registers.get8(register1);
        let v2 = self.fetch_u8_immdiate(memory);
        let mut res = v1.overflowing_add(v2);
        let mut half_carry = (v1 & 0xF) + (v2 & 0xF) > 0xF;
        let mut carry = res.1;
        if self.registers.get_flag(Flags::C) {
            if !half_carry { half_carry = (res.0 & 0xF) + 1 > 0xF }
            res = res.0.overflowing_add(1);
            if !carry { carry = res.1 }
        }
        if res.0 == 0 { self.registers.set_flag(Flags::Z) } else { self.registers.unset_flag(Flags::Z) }
        if carry { self.registers.set_flag(Flags::C) } else { self.registers.unset_flag(Flags::C) }
        if half_carry { self.registers.set_flag(Flags::H) } else { self.registers.unset_flag(Flags::H) }
        self.registers.unset_flag(Flags::N);
        self.registers.set8(register1, res.0);
    }

    //add r, (rr)
    fn alu8_adci(&mut self, memory: &mut Memory, register: Register8, pair: Register16) {
        let v1 = self.registers.get8(register1);
        let addr = self.registers.get16(pair);
        let v2 = memory.read(addr);
        let mut res = v1.overflowing_add(v2);
        let mut half_carry = (v1 & 0xF) + (v2 & 0xF) > 0xF;
        let mut carry = res.1;
        if self.registers.get_flag(Flags::C) {
            if !half_carry { half_carry = (res.0 & 0xF) + 1 > 0xF }
            res = res.0.overflowing_add(1);
            if !carry { carry = res.1 }
        }
        if res.0 == 0 { self.registers.set_flag(Flags::Z) } else { self.registers.unset_flag(Flags::Z) }
        if carry { self.registers.set_flag(Flags::C) } else { self.registers.unset_flag(Flags::C) }
        if half_carry { self.registers.set_flag(Flags::H) } else { self.registers.unset_flag(Flags::H) }
        self.registers.unset_flag(Flags::N);
        self.registers.set8(register1, res.0);
    }

    //add r1, r2
    fn alu8_sbc(&mut self, register1: Register8, register2: Register8) {
        let v1 = self.registers.get8(register1);
        let v2 = self.registers.get8(register2);
        let mut res = v1.overflowing_sub(v2);
        let mut half_carry = (v1 & 0xF) - (v2 & 0xF) > 0xF;
        let mut carry = res.1;
        if self.registers.get_flag(Flags::C) {
            if !half_carry { half_carry = (res.0 & 0xF) - 1 > 0xF }
            res = res.0.overflowing_sub(1);
            if !carry { carry = res.1 }
        }
        if res.0 == 0 { self.registers.set_flag(Flags::Z) } else { self.registers.unset_flag(Flags::Z) }
        if carry { self.registers.set_flag(Flags::C) } else { self.registers.unset_flag(Flags::C) }
        if half_carry { self.registers.set_flag(Flags::H) } else { self.registers.unset_flag(Flags::H) }
        self.registers.set_flag(Flags::N);
        self.registers.set8(register1, res.0);
    }

    //add r, u8
    fn alu8_sbc_imm(&mut self, memory: &mut Memory, register: Register8) {
        let v1 = self.registers.get8(register1);
        let v2 = self.fetch_u8_immdiate(memory);
        let mut res = v1.overflowing_sub(v2);
        let mut half_carry = (v1 & 0xF) - (v2 & 0xF) > 0xF;
        let mut carry = res.1;
        if self.registers.get_flag(Flags::C) {
            if !half_carry { half_carry = (res.0 & 0xF) - 1 > 0xF }
            res = res.0.overflowing_sub(1);
            if !carry { carry = res.1 }
        }
        if res.0 == 0 { self.registers.set_flag(Flags::Z) } else { self.registers.unset_flag(Flags::Z) }
        if carry { self.registers.set_flag(Flags::C) } else { self.registers.unset_flag(Flags::C) }
        if half_carry { self.registers.set_flag(Flags::H) } else { self.registers.unset_flag(Flags::H) }
        self.registers.set_flag(Flags::N);
        self.registers.set8(register1, res.0);
    }

    //add r, (rr)
    fn alu8_sbci(&mut self, memory: &mut Memory, register: Register8, pair: Register16) {
        let v1 = self.registers.get8(register1);
        let addr = self.registers.get16(pair);
        let v2 = memory.read(addr);
        let mut res = v1.overflowing_sub(v2);
        let mut half_carry = (v1 & 0xF) - (v2 & 0xF) > 0xF;
        let mut carry = res.1;
        if self.registers.get_flag(Flags::C) {
            if !half_carry { half_carry = (res.0 & 0xF) - 1 > 0xF }
            res = res.0.overflowing_sub(1);
            if !carry { carry = res.1 }
        }
        if res.0 == 0 { self.registers.set_flag(Flags::Z) } else { self.registers.unset_flag(Flags::Z) }
        if carry { self.registers.set_flag(Flags::C) } else { self.registers.unset_flag(Flags::C) }
        if half_carry { self.registers.set_flag(Flags::H) } else { self.registers.unset_flag(Flags::H) }
        self.registers.unset_flag(Flags::N);
        self.registers.set8(register1, res.0);
    }

    //add r1, r2
    fn alu8_and(&mut self, register1: Register8, register2: Register8) {
        let v1 = self.registers.get8(register1);
        let v2 = self.registers.get8(register2);
        let res = v1 & v2;
        if res == 0 { self.registers.set_flag(Flags::Z) } else { self.registers.unset_flag(Flags::Z) }
        self.registers.unset_flag(Flags::C);
        self.registers.set_flag(Flags::H);
        self.registers.unset_flag(Flags::N);
        self.registers.set8(register1, res.0);
    }

    //add r, u8
    fn alu8_and_imm(&mut self, memory: &mut Memory, register: Register8) {
        let v1 = self.registers.get8(register1);
        let v2 = self.fetch_u8_immdiate(memory);
        let res = v1 & v2;
        if res == 0 { self.registers.set_flag(Flags::Z) } else { self.registers.unset_flag(Flags::Z) }
        self.registers.unset_flag(Flags::C);
        self.registers.set_flag(Flags::H);
        self.registers.unset_flag(Flags::N);
        self.registers.set8(register1, res.0);
    }

    //add r, (rr)
    fn alu8_andi(&mut self, memory: &mut Memory, register: Register8, pair: Register16) {
        let v1 = self.registers.get8(register);
        let addr = self.registers.get16(pair);
        let v2 = memory.read(addr);
        let res = v1 & v2;
        if res == 0 { self.registers.set_flag(Flags::Z) } else { self.registers.unset_flag(Flags::Z) }
        self.registers.unset_flag(Flags::C);
        self.registers.set_flag(Flags::H);
        self.registers.unset_flag(Flags::N);
        self.registers.set8(register1, res.0);
    }

    //add r1, r2
    fn alu8_xor(&mut self, register1: Register8, register2: Register8) {
        let v1 = self.registers.get8(register1);
        let v2 = self.registers.get8(register2);
        let res = v1 ^ v2;
        if res == 0 { self.registers.set_flag(Flags::Z) } else { self.registers.unset_flag(Flags::Z) }
        self.registers.unset_flag(Flags::C);
        self.registers.unset_flag(Flags::H);
        self.registers.unset_flag(Flags::N);
        self.registers.set8(register1, res.0);
    }

    //add r, u8
    fn alu8_xor_imm(&mut self, memory: &mut Memory, register: Register8) {
        let v1 = self.registers.get8(register1);
        let v2 = self.fetch_u8_immdiate(memory);
        let res = v1 ^ v2;
        if res == 0 { self.registers.set_flag(Flags::Z) } else { self.registers.unset_flag(Flags::Z) }
        self.registers.unset_flag(Flags::C);
        self.registers.unset_flag(Flags::H);
        self.registers.unset_flag(Flags::N);
        self.registers.set8(register1, res.0);
    }

    //add r, (rr)
    fn alu8_xori(&mut self, memory: &mut Memory, register: Register8, pair: Register16) {
        let v1 = self.registers.get8(register);
        let addr = self.registers.get16(pair);
        let v2 = memory.read(addr);
        let res = v1 ^ v2;
        if res == 0 { self.registers.set_flag(Flags::Z) } else { self.registers.unset_flag(Flags::Z) }
        self.registers.unset_flag(Flags::C);
        self.registers.unset_flag(Flags::H);
        self.registers.unset_flag(Flags::N);
        self.registers.set8(register1, res.0);
    }

    //add r1, r2
    fn alu8_or(&mut self, register1: Register8, register2: Register8) {
        let v1 = self.registers.get8(register1);
        let v2 = self.registers.get8(register2);
        let res = v1 | v2;
        if res == 0 { self.registers.set_flag(Flags::Z) } else { self.registers.unset_flag(Flags::Z) }
        self.registers.unset_flag(Flags::C);
        self.registers.unset_flag(Flags::H);
        self.registers.unset_flag(Flags::N);
        self.registers.set8(register1, res.0);
    }

    //add r, u8
    fn alu8_or_imm(&mut self, memory: &mut Memory, register: Register8) {
        let v1 = self.registers.get8(register1);
        let v2 = self.fetch_u8_immdiate(memory);
        let res = v1 | v2;
        if res == 0 { self.registers.set_flag(Flags::Z) } else { self.registers.unset_flag(Flags::Z) }
        self.registers.unset_flag(Flags::C);
        self.registers.unset_flag(Flags::H);
        self.registers.unset_flag(Flags::N);
        self.registers.set8(register1, res.0);
    }

    //add r, (rr)
    fn alu8_ori(&mut self, memory: &mut Memory, register: Register8, pair: Register16) {
        let v1 = self.registers.get8(register);
        let addr = self.registers.get16(pair);
        let v2 = memory.read(addr);
        let res = v1 | v2;
        if res == 0 { self.registers.set_flag(Flags::Z) } else { self.registers.unset_flag(Flags::Z) }
        self.registers.unset_flag(Flags::C);
        self.registers.unset_flag(Flags::H);
        self.registers.unset_flag(Flags::N);
        self.registers.set8(register1, res.0);
    }

    //add r1, r2
    fn alu8_cp(&mut self, register1: Register8, register2: Register8) {
        let v1 = self.registers.get8(register1);
        let v2 = self.registers.get8(register2);
        let res = v1.overflowing_sub(v2);
        if res.0 == 0 { self.registers.set_flag(Flags::Z) } else { self.registers.unset_flag(Flags::Z) }
        if res.1 { self.registers.set_flag(Flags::C) } else { self.registers.unset_flag(Flags::C) }
        if (v1 & 0xF) - (v2 & 0xF) > 0xF { self.registers.set_flag(Flags::H) } else { self.registers.unset_flag(Flags::H) }
        self.registers.set_flag(Flags::N);
    }
    //add r, u8
    fn alu8_cp_imm(&mut self, memory: &mut Memory, register: Register8) {
        let v1 = self.registers.get8(register1);
        let v2 = self.fetch_u8_immdiate(memory);
        let res = v1.overflowing_sub(v2);
        if res.0 == 0 { self.registers.set_flag(Flags::Z) } else { self.registers.unset_flag(Flags::Z) }
        if res.1 { self.registers.set_flag(Flags::C) } else { self.registers.unset_flag(Flags::C) }
        if (v1 & 0xF) - (v2 & 0xF) > 0xF { self.registers.set_flag(Flags::H) } else { self.registers.unset_flag(Flags::H) }
        self.registers.set_flag(Flags::N);
    }
    //add r, (rr)
    fn alu8_cpi(&mut self, memory: &mut Memory, register: Register8, pair: Register16) {
        let v1 = self.registers.get8(register);
        let addr = self.registers.get16(pair);
        let v2 = memory.read(addr);
        let res = v1.overflowing_sub(v2);
        if res.0 == 0 { self.registers.set_flag(Flags::Z) } else { self.registers.unset_flag(Flags::Z) }
        if res.1 { self.registers.set_flag(Flags::C) } else { self.registers.unset_flag(Flags::C) }
        if (v1 & 0xF) - (v2 & 0xF) > 0xF { self.registers.set_flag(Flags::H) } else { self.registers.unset_flag(Flags::H) }
        self.registers.set_flag(Flags::N);
    }

    //DAA
    fn alu8_daa(&mut self) {
        let mut adjustment = 0;
        if self.registers.get_flag(Flags::H) == 1 || (self.registers.get_flag(Flags::N) == 0 && (self.registers.get8(Register8::A) > 9)) {
            adjustment |= 0x6;
        }
        if self.registers.get_flag(Flags::C) == 1 || (self.registers.get_flag(Flags::N) == 0 && (self.registers.get8(Register8::A) > 0x99)) {
            adjustment |= 0x60;
            self.registers.set_flag(Flags::C);
        }
        if self.registers.get_flag(Flags::N) == 1 { self.registers.set8(Register8::A, (self.registers.get8(Register8::A) + adjustment) & 0xFF) } else { self.registers.set8(Register8::A, (self.registers.get8(Register8::A) - adjustment) & 0xFF) }
        if self.registers.get8(Register8::A) == 0 { self.registers.set_flag(Flags::Z) } else { self.registers.unset_flag(Flags::Z) }
        self.registers.unset_flag(Flags::H);
    }

    fn alu8_cpl(&mut self) {
        let val = self.registers.get8(Register8::A);
        self.registers.set8(Register8::A, val ^ 0xFF)
    }

    fn alu8_scf(&mut self) {
        self.registers.set_flag(Flags::C);
        self.registers.unset_flag(Flags::N);
        self.registers.unset_flag(Flags::H);
    }

    fn alu8_ccf(&mut self) {
        if self.registers.get_flag(Flags::C) {
            self.registers.unset_flag(Flags::C);
        } else {
            self.registers.set_flag(Flags::C);
        }
    }

    /*
        8-bit rotate/shift bits
            * RLCA
     */

    //Left circular shift register A
    fn rsb8_rlca(&mut self) {
        let val = self.registers.get8(Targets::A);
        let high = (val & 0xA0) >> 7;
        let res = (val << 1) + high;
        if high { self.registers.set_flag(Flags::C) } else { self.registers.unset_flag(Flags::C) }
        self.registers.set8(Register8::A, res);
        self.registers.unset_flag(Flags::Z);
        self.registers.unset_flag(Flags::N);
        self.registers.unset_flag(Flags::H);
    }

    //Left circular shift arbitrary 8-bit register
    fn rsb8_rlc(&mut self, register: Register8) {
        let val = self.registers.get8(register);
        let high = (val & 0xA0) >> 7;
        let res = (val << 1) + high;
        if high { self.registers.set_flag(Flags::C) } else { self.registers.unset_flag(Flags::C) }
        if res == 0 { self.registers.set_flag(Flags::Z) } else { self.registers.unset_flag(Flags::Z) }
        self.registers.unset_flag(Flags::N);
        self.registers.unset_flag(Flags::H);
        self.registers.set8(register, res);
    }

    //Left circular shift value stored in memory pointed to by RR
    fn rsb8_rlci(&mut self, pair: Register16) {
        let addr = self.registers.get16(pair);
        let val = memory.read(addr);
        let high = (val & 0xA0) >> 7;
        let res = (val << 1) + high;
        if high { self.registers.set_flag(Flags::C) } else { self.registers.unset_flag(Flags::C) }
        if res == 0 { self.registers.set_flag(Flags::Z) } else { self.registers.unset_flag(Flags::Z) }
        self.registers.unset_flag(Flags::N);
        self.registers.unset_flag(Flags::H);
        memory.write(addr, res);
    }

    //Left shift register A through carry
    fn rsb8_rla(&mut self) {
        let val = self.registers.get8(Targets::A);
        let high = (val & 0xA0) >> 7;
        let new_low = if self.registers.get_flag(Flags::C) { 1 } else { 0 };
        let res = (val << 1) + new_low;
        if high { self.registers.set_flag(Flags::C) } else { self.registers.unset_flag(Flags::C) }
        self.registers.set8(Register8::A, res);
        self.registers.unset_flag(Flags::Z);
        self.registers.unset_flag(Flags::N);
        self.registers.unset_flag(Flags::H);
    }

    //Left shift arbitrary 8-bit register through carry
    fn rsb8_rl(&mut self, register: Register8) {
        let val = self.registers.get8(register);
        let high = (val & 0xA0) >> 7;
        let new_low = if self.registers.get_flag(Flags::C) { 1 } else { 0 };
        let res = (val << 1) + new_low;
        if high { self.registers.set_flag(Flags::C) } else { self.registers.unset_flag(Flags::C) }
        self.registers.set8(register, res);
        if res == 0 { self.registers.unset_flag(Flags::Z) } else { self.registers.unset_flag(Flags::Z) }
        self.registers.unset_flag(Flags::N);
        self.registers.unset_flag(Flags::H);
    }

    //Left shift value stored in memory pointed to by RR through carry
    fn rsb8_rli(&mut self, pair: Register16) {
        let addr = self.registers.get16(pair);
        let val = memory.read(addr);
        let high = (val & 0xA0) >> 7;
        let new_low = if self.registers.get_flag(Flags::C) { 1 } else { 0 };
        let res = (val << 1) + new_low;
        if high { self.registers.set_flag(Flags::C) } else { self.registers.unset_flag(Flags::C) }
        if res == 0 { self.registers.unset_flag(Flags::Z) } else { self.registers.unset_flag(Flags::Z) }
        self.registers.unset_flag(Flags::N);
        self.registers.unset_flag(Flags::H);
        memory.write(addr, res);
    }

    //Right circular shift register A
    fn rsb8_rrca(&mut self) {
        let val = self.registers.get8(Targets::A);
        let low = val & 0x01;
        let res = (val >> 1) + (low << 7);
        if low { self.registers.set_flag(Flags::C) } else { self.registers.unset_flag(Flags::C) }
        self.registers.unset_flag(Flags::Z);
        self.registers.unset_flag(Flags::N);
        self.registers.unset_flag(Flags::H);
        self.registers.set8(Register8::A, res;
    }

    //Right circular shift arbitrary 8-bit register
    fn rsb8_rrc(&mut self, register: Register8) {
        let val = self.registers.get8(register);
        let low = val & 0x01;
        let res = (val >> 1) + (low << 7);
        if low { self.registers.set_flag(Flags::C) } else { self.registers.unset_flag(Flags::C) }
        if res == 0 { self.registers.set_flag(Flags::Z) } else { self.registers.unset_flag(Flags::Z) }
        self.registers.unset_flag(Flags::N);
        self.registers.unset_flag(Flags::H);
        self.registers.set8(Register8::A, res;
    }

    //Right circular shift value stored in memory pointed to by RR
    fn rsb8_rrci(&mut self, pair: Register16) {
        let addr = self.registers.get16(pair);
        let val = memory.read(addr);
        let low = val & 0x01;
        let res = (val >> 1) + (low << 7);
        if low { self.registers.set_flag(Flags::C) } else { self.registers.unset_flag(Flags::C) }
        if res == 0 { self.registers.set_flag(Flags::Z) } else { self.registers.unset_flag(Flags::Z) }
        self.registers.unset_flag(Flags::N);
        self.registers.unset_flag(Flags::H);
        memory.write(addr, res);
    }

    //Left shift register A through carry
    fn rsb8_rra(&mut self) {
        let val = self.registers.get8(Targets::A);
        let low = val & 0x0F;
        let new_high = if self.registers.get_flag(Flags::C) { 1 } else { 0 };
        let res = (val >> 1) + (new_high << 7);
        if low { self.registers.set_flag(Flags::C) } else { self.registers.unset_flag(Flags::C) }
        self.registers.set8(Register8::A, res);
        self.registers.unset_flag(Flags::Z);
        self.registers.unset_flag(Flags::N);
        self.registers.unset_flag(Flags::H);
    }

    //Left shift arbitrary 8-bit register through carry
    fn rsb8_rr(&mut self, register: Register8) {
        let val = self.registers.get8(register);
        let low = val & 0x0F;
        let new_high = if self.registers.get_flag(Flags::C) { 1 } else { 0 };
        let res = (val >> 1) + (new_high << 7);
        if high { self.registers.set_flag(Flags::C) } else { self.registers.unset_flag(Flags::C) }
        self.registers.set8(register, res);
        if res == 0 { self.registers.unset_flag(Flags::Z) } else { self.registers.unset_flag(Flags::Z) }
        self.registers.unset_flag(Flags::N);
        self.registers.unset_flag(Flags::H);
    }

    //Left shift value stored in memory pointed to by RR through carry
    fn rsb8_rri(&mut self, pair: Register16) {
        let addr = self.registers.get16(pair);
        let val = memory.read(addr);
        let low = val & 0x0F;
        let new_high = if self.registers.get_flag(Flags::C) { 1 } else { 0 };
        let res = (val >> 1) + (new_high << 7);
        if high { self.registers.set_flag(Flags::C) } else { self.registers.unset_flag(Flags::C) }
        if res == 0 { self.registers.unset_flag(Flags::Z) } else { self.registers.unset_flag(Flags::Z) }
        self.registers.unset_flag(Flags::N);
        self.registers.unset_flag(Flags::H);
        memory.write(addr, res);
    }

    //Arithmetic shift left R
    fn rsb8_sla(&mut self, register: Register8) {
        let val = self.registers.get8(register);
        let carry = (val & 0xA0) >> 7;
        let low = val & 0x1;
        if carry == 1 { self.registers.set_flag(Flags::C) } else { self.registers.unset_flag(Flags::C) }
        let mut res = val << 1;
        if low == 1 { res = res | 0x1 } else { res = res & 0xF8 }
        self.registers.set8(register, res);
    }

    //Arithmetic shift left (RR)
    fn rsb8_slai(&mut self, memory: &mut Memory, pair: Register16) {
        let addr = self.registers.get16(pair);
        let val = memory.read(addr);
        let carry = (val & 0xA0) >> 7;
        let low = val & 0x1;
        if carry == 1 { self.registers.set_flag(Flags::C) } else { self.registers.unset_flag(Flags::C) }
        let mut res = val << 1;
        if low == 1 { res = res | 0x1 } else { res = res & 0xF8 }
        memory.write(addr, res);
    }

    //Arithmetic shift right R
    fn rsb8_sra(&mut self, register: Register8) {
        let val = self.registers.get8(register);
        let carry = val & 0x01;
        let high = val & 0xA0;
        if carry == 1 { self.registers.set_flag(Flags::C) } else { self.registers.unset_flag(Flags::C) }
        let mut res = val >> 1;
        if high == 0xA0 { res = res | 0xA0 } else { res = res & 0x7F }
        self.registers.set8(register, res);
    }

    //Arithmetic shift right (RR)
    fn rsb8_srai(&mut self, memory: &mut Memory, pair: Register16) {
        let addr = self.registers.get16(pair);
        let val = memory.read(addr);
        let carry = val & 0x01;
        let high = val & 0xA0;
        if carry == 1 { self.registers.set_flag(Flags::C) } else { self.registers.unset_flag(Flags::C) }
        let mut res = val >> 1;
        if high == 0xA0 { res = res | 0xA0 } else { res = res & 0x7F }
        memory.write(addr, res);
    }

    //Swap the upper 4 bits in R with the lower four
    fn rsb8_swap(&mut self, register: Register8) {
        let val = self.registers.get8(register);
        let low = (val & 0xF0) >> 4;
        let high = (val & 0x0F) << 4;
        let res = high + low;
        self.registers.set8(register, res);
    }

    //Swap the upper 4 bits in (RR) with the lower four
    fn rsb8_swapi(&mut self, memory: &mut Memory, pair: Register16) {
        let addr = self.registers.get16(pair);
        let val = memory.read(addr);
        let low = (val & 0xF0) >> 4;
        let high = (val & 0x0F) << 4;
        let res = high + low;
        memory.write(addr, res);
    }

    //Logical shift right R
    fn rsb8_srl(&mut self, register: Register8) {
        let val = self.registers.get8(register);
        let carry = val & 0x01;
        if carry == 1 { self.registers.set_flag(Flags::C) } else { self.registers.unset_flag(Flags::C) }
        let mut res = val >> 1;
        self.registers.set8(register, res);
    }

    //Logical shift right (RR)
    fn rsb8_srli(&mut self, memory: &mut Memory, pair: Register16) {
        let addr = self.registers.get16(pair);
        let val = memory.read(addr);
        let carry = val & 0x01;
        if carry == 1 { self.registers.set_flag(Flags::C) } else { self.registers.unset_flag(Flags::C) }
        let mut res = val >> 1;
        memory.write(addr, res);
    }

    //Test u3 in R, set Z if bit not set
    fn rsb8_bit(&mut self, bit: u8, register: Register8) {
        if bit > 7 { return }
        let val = self.registers.get8(register);
        let set = (val >> bit) & 0x1;
        if set { self.registers.unset_flag(Flags::Z) } else { self.registers.set_flag(Flags::Z) }
        self.registers.unset_flag(Flags::N);
        self.registers.set_flag(Flags::H);
    }

    //Test u3 in (RR), set Z if bit not set
    fn rsb8_biti(&mut self, memory: &mut Memory, bit: u8, pair: Register16) {
        if bit > 7 { return }
        let addr = self.registers.get16(pair);
        let val = memory.read(addr);
        let set = (val >> bit) & 0x1;
        if set { self.registers.unset_flag(Flags::Z) } else { self.registers.set_flag(Flags::Z) }
        self.registers.unset_flag(Flags::N);
        self.registers.set_flag(Flags::H);
    }

    //Set bit position in R to 0
    fn rsb8_res(&mut self, bit: u8, register: Register8) {
        if bit > 7 { return }
        let mut mask = 0xFF ^ (0x1 << bit);
        let val = self.registers.get8(register);
        let res = val & mask;
        self.registers.set8(register, res);
    }

    //Set bit position in (RR) to 0
    fn rsb8_resi(&mut self, memory: &mut Memory, bit: u8, pair: Register16) {
        if bit > 7 { return }
        let addr = self.registers.get16(pair);
        let mut mask = 0xFF ^ (0x1 << bit);
        let val = memory.read(addr);
        let res = val & mask;
        memory.write(addr, res);
    }

    //Set bit position in R to 1
    fn rsb8_set(&mut self, bit: u8, register: Register8) {
        if bit > 7 { return }
        let mut mask = 0x1 << bit;
        let val = self.registers.get8(register);
        let res = val | mask;
        self.registers.set8(register, res);
    }

    //Set bit position in  to 1
    fn rsb8_seti(&mut self, memory: &mut Memory, bit: u8, pair: Register16) {
        if bit > 7 { return }
        let addr = self.registers.get16(pair);
        let mut mask = 0x1 << bit;
        let val = memory.read(addr);
        let res = val | mask;
        memory.write(addr, res);
    }


    /*
        16-bit arithmetic / logic
        * INC RR
        * DEC RR
        * INC SP
        * DEC SP
        * ADD RR1, RR2
        * ADD RR1, SP
        * ADD SP, i8
        * LD HL, SP+i8
     */
    fn alu16_inc(&mut self, cycle: u32, rhigh: Register8, rlow: Register8) {
        if cycle == 4 {
            let val = self.registers.get8(rlow);
            let res = val.overflowing_add(1);
            self.registers.set8(rlow, res.0);
            if !res.1 {
                self.instr_state = None;
            }
        } else if cycle == 8 {
            let val = self.registers.get8(rhigh);
            let res = val.overflowing_add(1);
            self.registers.set8(rhigh, res.0);
            self.instr_state = None;
        }
    }

    fn alu16_incsp(&mut self) {
        let res = self.sp.overflowing_add(1);
        self.sp = res.0;
    }

    fn alu16_dec(&mut self, register: Register16) {
        let val = self.registers.get16(register);
        let res = val.overflowing_sub(1);
        self.registers.set16(register, res.0);
    }

    fn alu16_decsp(&mut self) {
        let res = self.sp.overflowing_sub(1);
        self.sp = res.0;
    }

    fn alu16_add(&mut self, pair1: Register16, pair2: Register16) {
        let v1 = self.registers.get16(pair1);
        let v2 = self.registers.get16(pair2);
        let res = v1.overflowing_add(v2);
        if (v1 & 0x0F00) + (v2 & 0x0F00) > 0x0F00 { self.registers.set_flag(Flags::H) } else { self.registers.unset_flag(Flags::H) }
        if res.1 { self.registers.set_flag(Flags::C) } else { self.registers.unset_flag(Flags::C) }
        self.registers.unset_flag(Flags::N);
        self.registers.set16(pair1, res.0);
    }

    fn alu16_add_rr_sp(&mut self, pair: Register16) {
        let val = self.registers.get16(pair);
        let res = val.overflowing_add(self.sp);
        if (val & 0x0F00) + (self.sp & 0x0F00) > 0x0F00 { self.registers.set_flag(Flags::H) } else { self.registers.unset_flag(Flags::H) }
        if res.1 { self.registers.set_flag(Flags::C) } else { self.registers.unset_flag(Flags::C) }
        self.registers.unset_flag(Flags::N);
        self.registers.set16(pair, res.0);
    }

    fn alu16_add_spimm(&mut self, memory: &mut Memory) {
        let i8 = self.fetch_u8_immdiate(memory) as i8 as i16 as u16;
        let res = self.sp.overflowing_add(i8);
        if (self.sp & 0x0F00) + (i8 & 0x0F00) > 0x0F00 { self.registers.set_flag(Flags::H) } else { self.registers.unset_flag(Flags::H) }
        if res.1 { self.registers.set_flag(Flags::C) } else { self.registers.unset_flag(Flags::C) }
        self.registers.unset_flag(Flags::N);
        self.registers.unset_flag(Flags::Z);
        self.sp = res.0;
    }

    fn alu16_ld_spimm(&mut self, memory: &mut Memory, pair: Register16) {
        let i8 = self.fetch_u8_immdiate(memory) as i8 as i16 as u16;
        let res = self.sp.overflowing_add(i8);
        if (self.sp & 0x0F00) + (i8 & 0x0F00) > 0x0F00 { self.registers.set_flag(Flags::H) } else { self.registers.unset_flag(Flags::H) }
        if res.1 { self.registers.set_flag(Flags::C) } else { self.registers.unset_flag(Flags::C) }
        self.registers.unset_flag(Flags::N);
        self.registers.unset_flag(Flags::Z);
        self.registers.set16(pair, res.0);
    }

    /*
        CPU Control / Misc
    */
    fn ret(&mut self, memory: &mut Memory) {
        let mut pc = memory.read(self.sp + 1) as u16;
        pc += (memory.read(self.sp + 2) as u16) << 8;
        self.pc = pc;
        self.sp += 2;
    }

    fn rst(&mut self, memory: &mut Memory, vec: u16) -> bool {
        memory.write(self.sp, (self.pc >> 8) as u8);
        memory.write(self.sp - 1, (self.pc & 0xFF) as u8);
        self.sp -= 2;
        update_pc = false;
        self.pc = vec;
        return false;
    }

    fn call(&mut self, memory: &mut Memory, addr: u16) -> bool {
        memory.write(self.sp, (self.pc >> 8) as u8);
        memory.write(self.sp - 1, (self.pc & 0xFF) as u8);
        self.sp -= 2;
        self.pc = addr;
        return false;
    }

    fn jr(&mut self, memory: &mut Memory, conditions: Vec<Flags>) -> u16 {
        let mut jump = true;
        let mut cycles = 0;
        for condition in conditions {
            if !self.registers.get_flag(condition) { jump = false }
        }
        if jump {
            cycles += 4;
            let e8 = memory.read(self.pc + 1) as i8;
            self.pc = ((self.pc as u32 as i32)  + (e8 as i32)) as u16;
        }
        return cycles;
    }
}