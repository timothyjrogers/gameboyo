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
    intermediate: u16,
    prefix: bool,
}

impl InstructionState {
    fn new(instruction: u16) -> Self {
        Self {
            cycle: 1,
            instruction,
            intermediate: 0,
            prefix: false,
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
        //TODO check EI in interrupts
        //TODO check for requested interrupts
        match &self.state {
            CpuState::Ready => {
                //let mut instr = memory.read(self.pc);             // fetch instruction at [PC]
                let mut instr: u8 = 0;
                let mut cycle: u32 = 0;
                match &mut self.instr_state {
                    Some(x) => {
                        if 1 <= *x.cycle && *x.cycle < 4 || (4 <= *x.cycle && *x.cycle < 8 && *x.prefix) {
                            *x.cycle = *x.cycle + 1;
                            return;
                        }
                        if *x.cycle == 8 && *x.prefix {
                            self.pc = self.pc + 1;
                            *x.instruction = memory.read(self.pc);
                        }
                        instr = *x.instruction;
                        *x.cycle = *x.cycle + 1;
                        cycle = *x.cycle;
                    },
                    None => {
                        instr = memory.read(self.pc);
                        self.instr_state = Some(InstructionState::new(instr as u16));
                        return;
                    }
                }
                match instr {
                    0xCB  => self.state = CpuState::Wait(4),                                                                       //TODO PREFIX CB
                    0x00 => self.nop(cycle),                                                                                        //NOP
                    0x01 => self.ld_rr_u16(cycle, memory, Register8::B, Register8::C),                                   //LD BC,u16
                    0x02 => self.sti_rr_r(cycle, memory, Register16::BC, Register8::A),                             //LD (BC),A
                    0x03 => self.inc_rr(cycle, Register16::BC),                                                                 //INC BC
                    0x04 => self.inc_r(cycle,Register8::B),                                                                 //INC B
                    0x05 => self.dec_r(cycle,Register8::B),                                                                 //DEC B
                    0x06 => self.ld_r_u8(cycle, memory, Register8::B),                                                      //LD B,u8
                    0x07 => self.rlca(cycle),                                                                                      //RLCA
                    0x08 => self.sti_u16_sp(cycle, memory),                                                                         //LD (u16),SP
                    0x09 => self.add_rr_rr(cycle, Register16::HL, Register16::BC),                                       //ADD HL,BC
                    0x0A => self.ldi_r_rr(cycle, memory, Register8::A, Register16::BC),                              //LD A,(BC)
                    0x0B => self.dec_rr(cycle, Register16::BC),                                                               //DEC BC
                    0x0C => self.inc_r(cycle, Register8::C),                                                                  //INC C
                    0x0D => self.dec_r(cycle, Register8::C),                                                                  //DEC C
                    0x0E => self.ld_r_u8(cycle, memory, Register8::C),                                                        //LD C,u8
                    0x0F => self.rrca(cycle),                                                                                    //RRCA
                    0x10 => {                                                                                                    //TODO STOP
                        self.state = CpuState::Wait(4);
                    },
                    0x11 => self.ld_rr_u16(cycle, memory, Register8::D, Register8::E),                                 //LD DE,u16
                    0x12 => self.sti_rr_r(cycle, memory, Register16::DE, Register8::A),                            //LD (DE),A
                    0x13 => self.inc_rr(cycle, Register16::DE),                                                                //INC DE
                    0x14 => self.inc_r(cycle, Register8::D),                                                               //INC D
                    0x15 => self.dec_r(cycle, Register8::D),                                                                //DEC D
                    0x16 => self.ld_r_u8(cycle, memory, Register8::D),                                                       //LD D,u8
                    0x17 => self.rla(cycle),                                                                                       //RLA
                    0x18 => self.jr_i8(cycle, memory, vec![]),                                                            //JR i8
                    0x19 => self.add_rr_rr(cycle, Register16::HL, Register16::DE), //ADD HL,DE
                    0x1A => self.ldi_r_rr(cycle, memory, Register8::A, Register16::DE),                               //LD A,(DE)
                    0x1B => self.dec_rr(cycle, Register16::DE),                                                               //DEC DE
                    0x1C => self.inc_r(cycle, Register8::E),                                                                  //INC E
                    0x1D => self.dec_r(cycle, Register8::E),                                                                  //DEC E
                    0x1E => self.ld_r_u8(cycle, memory, Register8::E),                                                           //LD E,u8
                    0x1F => self.rra(cycle),                                                                                    //RRA
                    0x20 => self.jr_i8(cycle, memory, vec![Flags::N, Flags::Z]),                                       //JR NZ,e8
                    0x21 => self.ld_rr_u16(cycle, memory, Register8::H, Register8::L),                                //LD HL,u16
                    0x22 => self.sti_rr_r_inc(cycle, memory, Register16::HL, Register8::A),                      //LD (HL+), A
                    0x23 => self.inc_rr(cycle, Register16::HL),                                                               //INC HL
                    0x24 => self.inc_r(cycle, Register8::H),                                                                  //INC H
                    0x25 => self.dec_r(cycle, Register8::H),                                                                  //DEC H
                    0x26 => self.ld_r_u8(cycle, memory, Register8::H),                                                           //LD H,u8
                    0x27 => self.daa(cycle),                                                                                    //DAA
                    0x28 => self.jr_i8(cycle, memory, vec![Flags::Z]),                                                 //JR Z,e8
                    0x29 => self.add_rr_rr(cycle, Register16::HL, Register16::HL),                                  //ADD HL,HL
                    0x2A => self.ldi_r_rr_inc(cycle, memory, Register8::A, Register16::HL),                      //LD A, (HL+)
                    0x2B => self.dec_rr(cycle, Register16::HL),                                                         //DEC HL
                    0x2C => self.inc_r(cycle, Register8::L),                                                            //INC L
                    0x2D => self.dec_r(cycle, Targets::L),                                                              //DEC L
                    0x2E => self.ld_r_u8(cycle, memory, Register8::L),                                                 //LD L,u8
                    0x2F => self.cpl(cycle),                                                                                         //CPL
                    0x30 => self.jr_i8(cycle, memory, vec![Flags::N, Flags::C]),                                       //JR NC,e8
                    0x31 => self.ld_sp_u16(cycle, memory),                                                                      //LD SP,u16
                    0x32 => self.sti_rr_r_dec(cycle, memory, Register16::HL, Register8::A),                      //LD (HL-), A
                    0x33 => self.inc_sp(cycle),                                                                                 //INC SP
                    0x34 => self.inci_rr(cycle, memory, Register16::HL),                                                         //INC (HL)
                    0x35 => self.deci_rr(cycle, memory, Register16::HL),                                                         //DEC (HL)
                    0x36 => self.sti_rr_u8(cycle, memory, Register16::HL),                                                    //LD (HL), u8
                    0x37 => self.scf(cycle),                                                                                   //SCF
                    0x38 => self.jr_i8(cycle, memory, vec![Flags::C]),                                                 //JR C,e8
                    0x39 => self.add_rr_sp(cycle,Register16::HL),                                                                 //ADD HL,SP TODO
                    0x3A => self.ldi_r_rr_dec(cycle, memory, Register8::A, Register16::HL),                      //LD A, (HL-)
                    0x3B => self.dec_sp(cycle),                                                                                 //DEC SP
                    0x3C => self.inc_r(cycle, Register8::A),                                                             //INC A
                    0x3D => self.dec_r(cycle, Register8::A),                                                             //DEC A
                    0x3E => self.ld_r_u8(cycle, memory, Register8::A),                                                   //LD A,u8
                    0x3F => self.ccf(cycle),                                                                                    //CCF
                    0x40 => self.ld_r_r(cycle, Register8::B, Register8::B),                                                     //LD B, B
                    0x41 => self.ld_r_r(cycle, Register8::B, Register8::C),                                                     //LD B, C
                    0x42 => self.ld_r_r(cycle, Register8::B, Register8::D),                                                     //LD B, D
                    0x43 => self.ld_r_r(cycle, Register8::B, Register8::E),                                                     //LD B, E
                    0x44 => self.ld_r_r(cycle, Register8::B, Register8::H),                                                     //LD B, H
                    0x45 => self.ld_r_r(cycle, Register8::B, Register8::L),                                                     //LD B, L
                    0x46 => self.ldi_r_rr(cycle, memory, Register8::B, Register16::HL),                          //LD B, (HL)
                    0x47 => self.ld_r_r(cycle, Register8::B, Register8::A),                                                     //LD B, A
                    0x48 => self.ld_r_r(cycle, Register8::C, Register8::B),                                                     //LD C, B
                    0x49 => self.ld_r_r(cycle, Register8::C, Register8::C),                                                     //LD C, C
                    0x4A => self.ld_r_r(cycle, Register8::C, Register8::D),                                                     //LD C, D
                    0x4B => self.ld_r_r(cycle, Register8::C, Register8::E),                                                     //LD C, E
                    0x4C => self.ld_r_r(cycle, Register8::C, Register8::H),                                                     //LD C, H
                    0x4D => self.ld_r_r(cycle, Register8::C, Register8::L),                                                     //LD C, L
                    0x4E => self.ldi_r_rr(cycle, memory, Register8::C, Register16::HL),                                   //LD C, (HL)
                    0x4F => self.ld_r_r(cycle, Register8::C, Register8::A),                                                     //LD C, A
                    0x50 => self.ld_r_r(cycle, Register8::D, Register8::B),                                                     //LD D, B
                    0x51 => self.ld_r_r(cycle, Register8::D, Register8::C),                                                     //LD D, C
                    0x52 => self.ld_r_r(cycle, Register8::D, Register8::D),                                                     //LD D, D
                    0x53 => self.ld_r_r(cycle, Register8::D, Register8::E),                                                     //LD D, E
                    0x54 => self.ld_r_r(cycle, Register8::D, Register8::H),                                                     //LD D, H
                    0x55 => self.ld_r_r(cycle, Register8::D, Register8::L),                                                     //LD D, L
                    0x56 => self.ldi_r_rr(cycle, memory, Register8::D, Register16::HL),                                   //LD D, (HL)
                    0x57 => self.ld_r_r(cycle, Register8::D, Register8::A),                                                     //LD D, A
                    0x58 => self.ld_r_r(cycle, Register8::E, Register8::B),                                                     //LD E, B
                    0x59 => self.ld_r_r(cycle, Register8::E, Register8::C),                                                     //LD E, C
                    0x5A => self.ld_r_r(cycle, Register8::E, Register8::D),                                                     //LD E, D
                    0x5B => self.ld_r_r(cycle, Register8::E, Register8::E),                                                     //LD E, E
                    0x5C => self.ld_r_r(cycle, Register8::E, Register8::H),                                                     //LD E, H
                    0x5D => self.ld_r_r(cycle, Register8::E, Register8::L),                                                     //LD E, L
                    0x5E => self.ldi_r_rr(cycle, memory, Register8::E, Register16::HL),                                   //LD E, (HL)
                    0x5F => self.ld_r_r(cycle, Register8::E, Register8::A),                                                      //LD E, A
                    0x60 => self.ld_r_r(cycle, Register8::H, Register8::B),                                                     //LD H, B
                    0x61 => self.ld_r_r(cycle, Register8::H, Register8::C),                                                     //LD H, C
                    0x62 => self.ld_r_r(cycle, Register8::H, Register8::D),                                                     //LD H, D
                    0x63 => self.ld_r_r(cycle, Register8::H, Register8::E),                                                     //LD H, E
                    0x64 => self.ld_r_r(cycle, Register8::H, Register8::H),                                                     //LD H, H
                    0x65 => self.ld_r_r(cycle, Register8::H, Register8::L),                                                     //LD H, L
                    0x66 => self.ldi_r_rr(cycle, memory, Register8::H, Register16::HL),                                  //LD H, (HL)
                    0x67 => self.ld_r_r(cycle, Register8::H, Register8::A),                                                     //LD H, A
                    0x68 => self.ld_r_r(cycle, Register8::L, Register8::B),                                                     //LD L, B
                    0x69 => self.ld_r_r(cycle, Register8::L, Register8::C),                                                     //LD L, C
                    0x6A => self.ld_r_r(cycle, Register8::L, Register8::D),                                                     //LD L, D
                    0x6B => self.ld_r_r(cycle, Register8::L, Register8::E),                                                     //LD L, E
                    0x6C => self.ld_r_r(cycle, Register8::L, Register8::H),                                                     //LD L, H
                    0x6D => self.ld_r_r(cycle, Register8::L, Register8::L),                                                     //LD L, L
                    0x6E => self.ldi_r_rr(cycle, memory, Register8::L, Register16::HL),                                   //LD L, (HL)
                    0x6F=> self.ld_r_r(cycle, Register8::L, Register8::A),                                                      //LD L, A
                    0x70 => self.sti_rr_r(cycle, memory, Register16::HL, Register8::B),                                  //LD (HL), B
                    0x71 => self.sti_rr_r(cycle, memory, Register16::HL, Register8::C),                                  //LD (HL), C
                    0x72 => self.sti_rr_r(cycle, memory, Register16::HL, Register8::D),                                  //LD (HL), D
                    0x73 => self.sti_rr_r(cycle, memory, Register16::HL, Register8::E),                                  //LD (HL), E
                    0x74 => self.sti_rr_r(cycle, memory, Register16::HL, Register8::H),                                  //LD (HL), H
                    0x75 => self.sti_rr_r(cycle, memory, Register16::HL, Register8::L),                                  //LD (HL), L
                    0x76 => {                                                                                                   //HALT TODO
                        CpuState::Wait(4)
                    },
                    0x77 => self.sti_rr_r(cycle, memory, Register16::HL, Register8::A),                                  //LD (HL), A
                    0x78 => self.ld_r_r(cycle, Register8::A, Register8::B),                                                     //LD A, B
                    0x79 => self.ld_r_r(cycle, Register8::A, Register8::C),                                                     //LD A, C
                    0x7A => self.ld_r_r(cycle, Register8::A, Register8::D),                                                     //LD A, D
                    0x7B => self.ld_r_r(cycle, Register8::A, Register8::E),                                                     //LD A, E
                    0x7C => self.ld_r_r(cycle, Register8::A, Register8::H),                                                     //LD A, H
                    0x7D => self.ld_r_r(cycle, Register8::A, Register8::L),                                                     //LD A, L
                    0x7E => self.ldi_r_rr(cycle, memory, Register8::A, Register16::HL),                                  //LD A, (HL)
                    0x7F=> self.ld_r_r(cycle, Register8::A, Register8::A),                                                      //LD A, A
                    0x80 => self.add_r_r(cycle, Register8::A, Register8::B),                                            //ADD A, B
                    0x81 => self.add_r_r(cycle, Register8::A, Register8::C),                                            //ADD A, C
                    0x82 => self.add_r_r(cycle, Register8::A, Register8::D),                                            //ADD A, D
                    0x83 => self.add_r_r(cycle, Register8::A, Register8::E),                                            //ADD A, E
                    0x84 => self.add_r_r(cycle, Register8::A, Register8::H),                                            //ADD A, H
                    0x85 => self.add_r_r(cycle, Register8::A, Register8::L),                                            //ADD A, L
                    0x86 => self.addi_r_rr(cycle, memory, Register8::A, Register16::HL),                                     //ADD A, (HL)
                    0x87 => self.add_r_r(cycle, Register8::A, Register8::A),                                            //ADD A, A
                    0x88 => self.adc_r_r(cycle, Register8::A, Register8::B),                                            //ADC A, B
                    0x89 => self.adc_r_r(cycle, Register8::A, Register8::C),                                            //ADC A, C
                    0x8A => self.adc_r_r(cycle, Register8::A, Register8::D),                                            //ADC A, D
                    0x8B => self.adc_r_r(cycle, Register8::A, Register8::E),                                            //ADC A, E
                    0x8C => self.adc_r_r(cycle, Register8::A, Register8::H),                                            //ADC A, H
                    0x8D => self.adc_r_r(cycle, Register8::A, Register8::L),                                            //ADC A, L
                    0x8E => self.adci_r_rr(cycle, memory, Register8::A, Register16::HL),                                     //ADC A, (HL)
                    0x8F => self.adc_r_r(cycle, Register8::A, Register8::A),                                            //ADC A, A
                    0x90 => self.sub_r_r(cycle, Register8::A, Register8::B),                                            //SUB A, B
                    0x91 => self.sub_r_r(cycle, Register8::A, Register8::C),                                            //SUB A, C
                    0x92 => self.sub_r_r(cycle, Register8::A, Register8::D),                                            //SUB A, D
                    0x93 => self.sub_r_r(cycle, Register8::A, Register8::E),                                            //SUB A, E
                    0x94 => self.sub_r_r(cycle, Register8::A, Register8::H),                                            //SUB A, H
                    0x95 => self.sub_r_r(cycle, Register8::A, Register8::L),                                            //SUB A, L
                    0x96 => self.sub_r_rr(cycle, memory, Register8::A, Register16::HL),                                     //SUB A, (HL)
                    0x97 => self.sub_r_r(cycle, Register8::A, Register8::A),                                            //SUB A, A
                    0x98 => self.sbc_r_r(cycle, Register8::A, Register8::B),                                                           //SBC A, B
                    0x99 => self.sbc_r_r(cycle, Register8::A, Register8::C),                                                           //SBC A, C
                    0x9A => self.sbc_r_r(cycle, Register8::A, Register8::D),                                                           //SBC A, D
                    0x9B => self.sbc_r_r(cycle, Register8::A, Register8::E),                                                           //SBC A, E
                    0x9C => self.sbc_r_r(cycle, Register8::A, Register8::H),                                                           //SBC A, H
                    0x9D => self.sbc_r_r(cycle, Register8::A, Register8::L),                                                           //SBC A, L
                    0x9E => self.sbci_r_rr(cycle, memory, Register8::A, Register16::HL),                                                //SBC A, (HL)
                    0x9F => self.sbc_r_r(cycle, Register8::A, Register8::A),                                                           //SBC A, A
                    0xA0 => self.and_r_r(cycle, Register8::A, Register8::B),                                            //AND A, B
                    0xA1 => self.and_r_r(cycle, Register8::A, Register8::C),                                            //AND A, C
                    0xA2 => self.and_r_r(cycle, Register8::A, Register8::D),                                            //AND A, D
                    0xA3 => self.and_r_r(cycle, Register8::A, Register8::E),                                            //AND A, E
                    0xA4 => self.and_r_r(cycle, Register8::A, Register8::H),                                            //AND A, H
                    0xA5 => self.and_r_r(cycle, Register8::A, Register8::L),                                            //AND A, L
                    0xA6 => self.andi_r_rr(cycle, memory, Register8::A, Register16::HL),                                     //AND A, (HL)
                    0xA7 => self.and_r_r(cycle, Register8::A, Register8::A),                                            //AND A, A
                    0xA8 => self.xor_r_r(cycle, Register8::A, Register8::B),                                            //XOR A, B
                    0xA9 => self.xor_r_r(cycle, Register8::A, Register8::C),                                            //XOR A, C
                    0xAA => self.xor_r_r(cycle, Register8::A, Register8::D),                                            //XOR A, D
                    0xAB => self.xor_r_r(cycle, Register8::A, Register8::E),                                            //XOR A, E
                    0xAC => self.xor_r_r(cycle, Register8::A, Register8::H),                                            //XOR A, H
                    0xAD => self.xor_r_r(cycle, Register8::A, Register8::L),                                            //XOR A, L
                    0xAE => self.xori_r_rr(cycle, memory, Register8::A, Register16::HL),                                     //XOR A, (HL)
                    0xAF => self.xor_r_r(cycle, Register8::A, Register8::A),                                            //XOR A, A
                    0xB0 => self.or_r_r(cycle, Register8::A, Register8::B),                                             //OR A, B
                    0xB1 => self.or_r_r(cycle, Register8::A, Register8::C),                                             //OR A, C
                    0xB2 => self.or_r_r(cycle, Register8::A, Register8::D),                                             //OR A, D
                    0xB3 => self.or_r_r(cycle, Register8::A, Register8::E),                                             //OR A, E
                    0xB4 => self.or_r_r(cycle, Register8::A, Register8::H),                                             //OR A, H
                    0xB5 => self.or_r_r(cycle, Register8::A, Register8::L),                                             //OR A, L
                    0xB6 => self.ori_r_rr(cycle, memory, Register8::A, Register16::HL),                                      //OR A, (HL)
                    0xB7 => self.or_r_r(cycle, Register8::A, Register8::A),                                             //OR A, A
                    0xB8 => self.cp_r_r(cycle, Register8::A, Register8::B),                                             //CP A, B
                    0xB9 => self.cp_r_r(cycle, Register8::A, Register8::C),                                             //CP A, C
                    0xBA => self.cp_r_r(cycle, Register8::A, Register8::D),                                             //CP A, D
                    0xBB => self.cp_r_r(cycle, Register8::A, Register8::E),                                             //CP A, E
                    0xBC => self.cp_r_r(cycle, Register8::A, Register8::H),                                             //CP A, H
                    0xBD => self.cp_r_r(cycle, Register8::A, Register8::L),                                             //CP A, L
                    0xBE => self.cpi_r_rr(cycle, memory, Register8::A, Register16::HL),                                      //CP A, (HL)
                    0xBF => self.cp_r_r(cycle, Register8::A, Register8::A),                                             //CP A, A
                    0xC0 => self.ret(cycle, memory, vec![Flags::N, Flags::Z]),                                         //RET NZ
                    0xC1 => self.pop_rr(cycle, memory, Register16::BC),                                                  //POP BC
                    0xC2 => self.jp_u16(cycle, memory, vec![Flags::N, Flags::Z]),                                       //JP NZ, u16
                    0xC3 => self.jp_u16(cycle, memory, vec![]),                                                         //JP u16
                    0xC4 => self.call(cycle, memory,vec![Flags::N, Flags::Z]),                                          //CALL NZ,u16
                    0xC5 => self.push_rr(cycle, memory, Register16::BC),                                                    //PUSH BC
                    0xC6 => self.add_r_u8(cycle, memory, Register8::A),                                                         //ADD A, u8
                    0xC7 => self.rst(cycle, memory, 0x00),                                                                  //RST 00
                    0xC8 => self.ret(cycle, memory, vec![Flags::Z]),                                                    //RET Z
                    0xC9 => self.ret(cycle, memory, vec![]),                                                            //RET
                    0xCA => self.jp_u16(cycle, memory, vec![Flags::Z]),                                                          //JP Z, u16
                    0xCC => self.call(cycle, memory, vec![Flags::Z]),                                                   //CALL Z, u16
                    0xCD => self.call(cycle, memory, vec![]),                                                             //CALL u16
                    0xCE => self.adc_r_u8(cycle, memory, Targets::A),                                                           //ADC A, u8
                    0xCF => self.rst(cycle, memory, 0x08),                                                                  //RST 08h
                    0xD0 => self.ret(cycle, memory, vec![Flags::N, Flags::C])  ,                                        //RET NC
                    0xD1 => self.pop_rr(cycle, memory, Register16::DE),                                                   //POP DE
                    0xD2 => self.jp_u16(cycle, memory, vec![Flags::N, Flags::C]),                                      //JP NC, u16
                    0xD4 => self.call(cycle, memory, vec![Flags::N, Flags::C]),                                        //CALL NC,u16
                    0xD5 => self.push_rr(cycle, memory, Register16::DE),                                                 //PUSH DE
                    0xD6 => self.sub_r_u8(cycle, memory, Targets::A),                                                           //SUB A, u8
                    0xD7 => self.rst(cycle, memory, 0x10),                                                                  //RST 10h
                    0xD8 => self.ret(cycle, memory, vec![Flags::C])                                                    //RET C
                    0xD9 => self.reti(cycle, memory),                                                                          //RETI
                    0xDA => self.jp_u16(cycle, memory, vec![Flags::C]),                                               //JP C, u16
                    0xDC => self.call(cycle, memory, vec![Flags::C]),                                                //CALL C, u16
                    0xDE => self.sbc_r_u8(cycle, memory, Register8::A),                                                      //SBC A, u8
                    0xDF => self.rst(cycle, memory, 0x18),                                                             //RS 18h
                    0xE0 => self.st_extended_u8(cycle, memory, Register8::A),                                   //LD (FF00+u8), A
                    0xE1 => self.pop_rr(cycle, memory, Register16::HL),                                                       //POP HL
                    0xE2 => self.st_extended_r(cycle, memory, Register8::A),                                  //LD (FF00+C), A
                    0xE5 => self.push_rr(cycle, memory, Register16::HL),                                                      //PUSH HL
                    0xE6 => self.and_r_u8(cycle, memory, Register8::A),                                                      //AND A, u8
                    0xE7 => self.rst(cycle, memory, 0x20),                                                             //RST 20h
                    0xE8 => self.add_sp_i8(cycle, memory),                                                                       //ADD SP, i8
                    0xE9 => self.jp_rr(cycle, Register16::HL),                                                     //JP HL
                    0xEA => self.sti_u16_r(cycle, memory, Register8::A),                                                           //LD (u16), A
                    0xEE => self.xor_r_u8(cycle, memory, Register8::A),                                                      //XOR A, u8
                    0xEF => self.rst(cycle, memory, 0x28),                                                             //RST 28h
                    0xF0 => self.ld_extended_u8(cycle, memory, Register8::A),                                   //LD A, (FF00+u8)
                    0xF1 => self.pop_rr(cycle, memory, Register16::AF),                                                       //POP AF
                    0xF2 => self.ld_extended_r(cycle, memory, Register8::A),                                  //LD A, (FF00+C)
                    0xF3 => self.di(cycle),                                                                         //DI (Disable interrupts)
                    0xF5 => self.push_rr(cycle, memory, Register16::AF),                                                      //PUSH AF
                    0xF6 => self.or_r_u8(cycle, memory, Register8::A),                                                       //OR A, u8
                    0xF7 => self.rst(cycle, memory, 0x30),                                                             //RST 30h
                    0xF8 => self.ld_rr_spi8(cycle, memory, Register16::HL),                                                    //LD HL, SP+i8
                    0xF9 => self.ld_sp_rr(cycle, Register16::HL),                                                   //LD SP, HL
                    0xFA => self.ldi_r_u16(cycle, memory, Register8::A),                                                       //LD A, (u16)
                    0xFB => self.ei(cycle),                                                                           //EI (Enable interrupts)
                    0xFE => self.cp_r_u8(cycle, memory, Register8::A),                                                       //CP A, u8
                    0xFF => self.rst(cycle, memory, 0x38),                                                             //RST 38h
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

    pub fn nop(&mut self, cycle: u32) {
        if cycle == 4 {
             self.instr_state = None;
        }
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
    fn ld_r_u8(&mut self, cycle: u32, memory: &mut Memory, register: Register8) {
        if cycle == 8 {
            let val = self.fetch_u8_immdiate(memory);
            self.registers.set8(register, val);
            self.instr_state = None;
        }
    }

    //ldi R1, (R2)
    fn ldi_r_rr(&mut self, cycle: u32, memory: &mut Memory, register1: Register8, register2: Register16) {
        if cycle == 8 {
            let addr = self.registers.get16(register2);
            let val = memory.read(addr);
            self.registers.set8(register1, val);
            self.instr_state = None;
        }
    }

    //ldi R1, (R2+)
    fn ldi_r_rr_inc(&mut self, cycle: u32, memory: &mut Memory, register1: Register8, register2: Register16) {
       if cycle == 8 {
           let addr = self.registers.get16(register2);
           let val = memory.read(addr);
           self.registers.set8(register1, val);
           let res = addr.overflowing_add(1);
           self.registers.set16(register2, res.0);
           self.instr_state = None;
        }
    }

    //ldi R1, (R2-)
    fn ldi_r_rr_dec(&mut self, cycle: u32, memory: &mut Memory, register1: Register8, register2: Register16) {
        if cycle == 8 {
            let addr = self.registers.get16(register2);
            let val = memory.read(addr);
            self.registers.set8(register1, val);
            let res = addr.overflowing_sub(1);
            self.registers.set16(register2, res.0);
            self.instr_state = None;
        }
    }

    //ldi R, (u16)
    fn ldi_r_u16(&mut self, cycle: u32, memory: &mut Memory, register: Register8) {
        let mut state = match &self.instr_state {
            Some(x) => x,
            None => panic!("Invalid state"),
        };
        if cycle == 8 {
            *state.intermediate = memory.read(self.pc + 1) as u16;
        } else if cycle == 12 {
            *state.intermediate = *state.intermediate + ((memory.read(self.pc + 2) as u16) << 8);
        } else if cycle == 16 {
            self.registers.set8(register, memory.read(*state.intermediate));
            self.instr_state = None;
        }
    }

    //st (u16), R
    fn sti_u16_r(&mut self, cycle: u32, memory: &mut Memory, register: Register8) {
        let mut state = match &self.instr_state {
            Some(x) => x,
            None => panic!("Invalid state"),
        };
        if cycle == 8 {
            *state.intermediate = memory.read(self.pc + 1) as u16;
        } else if cycle == 12 {
            *state.intermediate = *state.intermediate + ((memory.read(self.pc + 2) as u16) << 8);
        } else if cycle == 16 {
            memory.write(*state.intermediate, self.registers.get8(register));
            self.instr_state = None;
        }
    }

    //ST A, (FF00+u8)
    fn st_extended_u8(&mut self, cycle: u32, memory: &mut Memory, register: Register8) {
        if cycle == 12 {
            let offset = memory.read(self.pc + 1);
            let addr: u16 = 0xFF00 + offset as u16;
            memory.write(addr, self.registers.get8(register));
            self.instr_state = None;
        }
    }

    //ST A, (FF00+C)
    fn st_extended_r(&mut self, cycle: u32, memory: &mut Memory, register: Register8) {
        if cycle == 8 {
            let offset = self.registers.get8(Register8::C);
            let addr: u16 = 0xFF00 + offset as u16;
            memory.write(addr, self.registers.get8(register));
            self.instr_state = None;
        }
    }

    //LD A, (FF00+u8)
    fn ld_extended_u8(&mut self, cycle: u32, memory: &mut Memory, register: Register8) {
        if cycle == 12 {
            let offset = memory.read(self.pc + 1);
            let addr: u16 = 0xFF00 + offset as u16;
            let val = memory.read(addr);
            self.registers.set8(register, val);
            self.instr_state = None;
        }
    }

    //LD A, (FF00+C)
    fn ld_extended_r(&mut self, cycle: u32, memory: &mut Memory, register: Register8) {
        if cycle == 8 {
            let offset = self.registers.get8(Register8::C);
            let addr: u16 = 0xFF00 + offset as u16;
            let val = memory.read(addr);
            self.registers.set8(register, val);
            self.instr_state = None;
        }
    }

    //sti (R1), R2
    fn sti_rr_r(&mut self, cycle: u32, memory: &mut Memory, register1: Register16, register2: Register8) {
        if cycle == 8 {
            let addr = self.registers.get16(register1);
            let val = self.registers.get8(register2);
            memory.write(addr, val);
            self.instr_state = None;
        }
    }

    //sti (R1+), R2
    fn sti_rr_r_inc(&mut self, cycle: u32, memory: &mut Memory, register1: Register16, register2: Register8) {
        if cycle == 8 {
            let addr = self.registers.get16(register1);
            let val = self.registers.get8(register2);
            memory.write(addr, val);
            let res = addr.overflowing_add(1);
            self.registers.set16(register1, res.0);
            self.instr_state = None;
        }
    }

    //sti (R1-), R2
    fn sti_rr_r_dec(&mut self, cycle: u32, memory: &mut Memory, register1: Register16, register2: Register8) {
        if cycle == 8 {
            let addr = self.registers.get16(register1);
            let val = self.registers.get8(register2);
            memory.write(addr, val);
            let res = addr.overflowing_sub(1);
            self.registers.set16(register1, res.0);
            self.instr_state = None;
        }
    }

    //sti (R), u8
    fn sti_rr_u8(&mut self, cycle: u32, memory: &mut Memory, register: Register16) {
        if cycle == 12 {
            let addr = self.registers.get16(register);
            let val = memory.read(self.pc + 1);
            memory.write(addr, val);
            self.instr_state = None;
        }
    }

    //mv R1, R2
    fn ld_r_r(&mut self, cycle: u32, dst: Register8, src: Register8) {
        if cycle == 4 {
            self.registers.set8(dst, self.registers.get8(src));
            self.instr_state = None;
        }
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
    fn ld_rr_u16(&mut self, cycle: u32, memory: &mut Memory, rhigh: Register8, rlow: Register8) {
        if cycle == 8 {
            self.registers.set8(rlow, memory.read(self.pc));
            self.pc = self.pc + 1;
        } else if cycle == 12 {
            self.registers.set8(rhigh, memory.read(self.pc));
            self.pc = self.pc + 1;
            self.instr_state = None;
        }
    }

    fn ld_sp_u16(&mut self, cycle: u32, memory: &mut Memory) {
        if cycle == 8 {
            let val = memory.read(self.pc + 1);
            self.sp = self.sp & 0xFF00;
            self.sp = self.sp + val;
        } else if cycle == 12 {
            let val = memory.read(self.pc + 2);
            self.sp = self.sp & 0x00FF;
            self.sp = self.sp + ((val as u16) << 8);
            self.instr_state = None;
        }
    }

    //st (u16), RR
    fn lsm16_st(&mut self, cycle: u32, memory: &mut Memory, register: Register16) {
        let mut addr= self.fetch_u16_immediate(memory);
        let val = self.registers.get16(register);
        let low = (val & 0x00FF) as u8;
        let high = (val >> 8) as u8;
        memory.write(addr, low);
        memory.write(addr + 1, high);
    }

    //push RR
    fn push_rr(&mut self, cycle: u32, memory: &mut Memory, register: Register16) {
        if cycle == 12 {
            let val = self.registers.get(register) >> 8;
            self.sp = self.sp - 1;
            memory.write(self.sp, val);
        } else if cycle == 16 {
            let val = self.registers.get(register) >> 8;
            self.sp = self.sp - 1;
            memory.write(self.sp, val);
            self.instr_state = None;
        }
    }

    //pop RR
    fn pop_rr(&mut self, cycle: u32, memory: &mut Memory, register: Register16) {
        let rs = register.sub_registers();
        if cycle == 8 {
            self.registers.set8(rs.1,memory.read(self.sp));
            self.sp = self.sp + 1;
        } else if cycle == 12 {
            self.registers.set8(rs.0,memory.read(self.sp));
            self.sp = self.sp + 1;
            self.instr_state = None;
        }
    }

    //mv RR1, RR2
    fn lsm16_mv(&mut self, register1: Register16, register2: Register16) {
        self.registers.set16(register1, self.registers.get16(register2));
    }

    //store SP
    fn sti_u16_sp(&mut self, cycle: u32, memory: &mut Memory) {
        let mut state = match &self.instr_state {
            Some(x) => x,
            None => panic!("Invalid state"),
        };
        if cycle == 4 {
            *state.intermediate = self.fetch_u8_immdiate(memory);
            self.pc = self.pc + 1;
        } else if cycle == 8 {
            *state.intermediate = *state.intermediate + ((self.fetch_u8_immdiate(memory) as u16) << 8);
            self.pc = self.pc + 1;
        } else if cycle == 12 {
            memory.write(*state.intermediate, (self.sp & 0xFF) as u8);
        } else if cycle == 16 {
            memory.write(*state.intermediate, (self.sp >> 8) as u8);
        } else if cycle == 20 {
            self.instr_state = None;
        }
    }

    fn ld_sp_rr(&mut self, cycle: u32, register: Register16) {
        if cycle == 8 {
            self.sp = self.registers.get16(register);
            self.instr_state = None;
        }
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
    fn inc_r(&mut self, cycle: u32, register: Register8) {
        if cycle == 1 {
            let val = self.registers.get8(register);
            let res = val.overflowing_add(1);
            if res.0 == 0 { self.registers.set_flag(Flags::Z) } else { self.registers.unset_flag(Flags::Z) }
            if (val & 0xF) + 1 > 0xF { self.registers.set_flag(Flags::H) } else { self.registers.unset_flag(Flags::H) }
            self.registers.unset_flag(Flags::N);
            self.registers.set8(register, res.0);
        } else if cycle == 4 {
            self.instr_state = None;
        }
    }

    fn inci_rr(&mut self, cycle: u32, memory: &mut Memory, pair: Register16) {
        if cycle == 12 {
            let mut addr = self.registers.get16(pair);
            let val = memory.read(addr);
            let res = val.overflowing_add(1);
            if res.0 == 0 { self.registers.set_flag(Flags::Z) } else { self.registers.unset_flag(Flags::Z) }
            if (val & 0xF) + 1  > 0xF { self.registers.set_flag(Flags::H) } else { self.registers.unset_flag(Flags::H) }
            self.registers.unset_flag(Flags::N);
            memory.write(addr, res.0);
            self.instr_state = None;
        }
    }

    //Decrement 8-bit register and set ZNH accordingly
    fn dec_r(&mut self, cycle: u32, register: Register8) {
        if cycle == 1 {
            let val = self.registers.get8(register);
            let res = val.overflowing_sub(1);
            if res.0 == 0 { self.registers.set_flag(Flags::Z) } else { self.registers.unset_flag(Flags::Z) }
            if (val & 0xF) - 1 > 0xF { self.registers.set_flag(Flags::H) } else { self.registers.unset_flag(Flags::H) }
            self.registers.set_flag(Flags::N);
            self.registers.set8(register, res.0);
        } else if cycle == 4 {
            self.instr_state = None;
        }
    }

    fn deci_rr(&mut self, cycle: u32, memory: &mut Memory, pair: Register16) {
        if cycle == 12 {
            let mut addr = self.registers.get16(pair);
            let val = memory.read(addr);
            let res = val.overflowing_sub(1);
            if res.0 == 0 { self.registers.set_flag(Flags::Z) } else { self.registers.unset_flag(Flags::Z) }
            if (val & 0xF) - 1 > 0xF { self.registers.set_flag(Flags::H) } else { self.registers.unset_flag(Flags::H) }
            self.registers.set_flag(Flags::N);
            memory.write(addr, res.0);
            self.instr_state = None;
        }
    }

    //add r1, r2
    fn add_r_r(&mut self, cycle: u32, register1: Register8, register2: Register8) {
        if cycle == 4 {
            let v1 = self.registers.get8(register1);
            let v2 = self.registers.get8(register2);
            let res = v1.overflowing_add(v2);
            if res.0 == 0 { self.registers.set_flag(Flags::Z) } else { self.registers.unset_flag(Flags::Z) }
            if res.1 { self.registers.set_flag(Flags::C) } else { self.registers.unset_flag(Flags::C) }
            if (v1 & 0xF) + (v2 & 0xF) > 0xF { self.registers.set_flag(Flags::H) } else { self.registers.unset_flag(Flags::H) }
            self.registers.unset_flag(Flags::N);
            self.registers.set8(register1, res.0);
            self.instr_state = None;
        }
    }
    //add r, u8
    fn add_r_u8(&mut self, cycle: u32, memory: &mut Memory, register: Register8) {
        if cycle == 8 {
            let v1 = self.registers.get8(register1);
            let v2 = self.fetch_u8_immdiate(memory);
            let res = v1.overflowing_add(v2);
            if res.0 == 0 { self.registers.set_flag(Flags::Z) } else { self.registers.unset_flag(Flags::Z) }
            if res.1 { self.registers.set_flag(Flags::C) } else { self.registers.unset_flag(Flags::C) }
            if (v1 & 0xF) + (v2 & 0xF) > 0xF { self.registers.set_flag(Flags::H) } else { self.registers.unset_flag(Flags::H) }
            self.registers.unset_flag(Flags::N);
            self.registers.set8(register1, res.0);
            self.instr_state = None;
        }
    }

    //add r, (rr)
    fn addi_r_rr(&mut self, cycle: u32, memory: &mut Memory, register: Register8, pair: Register16) {
        if cycle == 8 {
            let v1 = self.registers.get8(register);
            let addr = self.registers.get16(pair);
            let v2 = memory.read(addr);
            let res = v1.overflowing_add(v2);
            if res.0 == 0 { self.registers.set_flag(Flags::Z) } else { self.registers.unset_flag(Flags::Z) }
            if res.1 { self.registers.set_flag(Flags::C) } else { self.registers.unset_flag(Flags::C) }
            if (v1 & 0xF) + (v2 & 0xF) > 0xF { self.registers.set_flag(Flags::H) } else { self.registers.unset_flag(Flags::H) }
            self.registers.unset_flag(Flags::N);
            self.registers.set8(register1, res.0);
            self.instr_state = None;
        }
    }

    //add r1, r2
    fn sub_r_r(&mut self, cycle: u32, register1: Register8, register2: Register8) {
        if cycle == 4 {
            let v1 = self.registers.get8(register1);
            let v2 = self.registers.get8(register2);
            let res = v1.overflowing_sub(v2);
            if res.0 == 0 { self.registers.set_flag(Flags::Z) } else { self.registers.unset_flag(Flags::Z) }
            if res.1 { self.registers.set_flag(Flags::C) } else { self.registers.unset_flag(Flags::C) }
            if (v1 & 0xF) - (v2 & 0xF) > 0xF { self.registers.set_flag(Flags::H) } else { self.registers.unset_flag(Flags::H) }
            self.registers.set_flag(Flags::N);
            self.registers.set8(register1, res.0);
            self.instr_state = None;
        }
    }
    //add r, u8
    fn sub_r_u8(&mut self, cycle: u32, memory: &mut Memory, register: Register8) {
        if cycle == 8 {
            let v1 = self.registers.get8(register1);
            let v2 = self.fetch_u8_immdiate(memory);
            let res = v1.overflowing_sub(v2);
            if res.0 == 0 { self.registers.set_flag(Flags::Z) } else { self.registers.unset_flag(Flags::Z) }
            if res.1 { self.registers.set_flag(Flags::C) } else { self.registers.unset_flag(Flags::C) }
            if (v1 & 0xF) - (v2 & 0xF) > 0xF { self.registers.set_flag(Flags::H) } else { self.registers.unset_flag(Flags::H) }
            self.registers.set_flag(Flags::N);
            self.registers.set8(register1, res.0);
            self.instr_state = None;
        }
    }
    //add r, (rr)
    fn sub_r_rr(&mut self, cycle: u32, memory: &mut Memory, register: Register8, pair: Register16) {
        if cycle == 8 {
            let v1 = self.registers.get8(register);
            let addr = self.registers.get16(pair);
            let v2 = memory.read(addr);
            let res = v1.overflowing_sub(v2);
            if res.0 == 0 { self.registers.set_flag(Flags::Z) } else { self.registers.unset_flag(Flags::Z) }
            if res.1 { self.registers.set_flag(Flags::C) } else { self.registers.unset_flag(Flags::C) }
            if (v1 & 0xF) - (v2 & 0xF) > 0xF { self.registers.set_flag(Flags::H) } else { self.registers.unset_flag(Flags::H) }
            self.registers.set_flag(Flags::N);
            self.registers.set8(register1, res.0);
            self.instr_state = None;
        }
    }

    //add r1, r2
    fn adc_r_r(&mut self, cycle: u32, register1: Register8, register2: Register8) {
        if cycle == 4 {
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
            self.instr_state = None;
        }
    }

    //add r, u8
    fn adc_r_u8(&mut self, cycle: u32, memory: &mut Memory, register: Register8) {
        if cycle == 8 {
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
            self.instr_state = None;
        }
    }

    //add r, (rr)
    fn adci_r_rr(&mut self, cycle: u32, memory: &mut Memory, register: Register8, pair: Register16) {
        if cycle == 8 {
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
            self.instr_state = None;
        }
    }

    //add r1, r2
    fn sbc_r_r(&mut self, cycle: u32, register1: Register8, register2: Register8) {
        if cycle == 4 {
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
            self.instr_state = None;
        }
    }

    //add r, u8
    fn sbc_r_u8(&mut self, cycle: u32, memory: &mut Memory, register: Register8) {
        if cycle == 8 {
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
            self.instr_state = None;
        }
    }

    //add r, (rr)
    fn sbci_r_rr(&mut self, cycle: u32, memory: &mut Memory, register: Register8, pair: Register16) {
        if cycle == 8 {
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
            self.instr_state = None;
        }
    }

    //add r1, r2
    fn and_r_r(&mut self, cycle: u32, register1: Register8, register2: Register8) {
        if cycle == 4 {
            let v1 = self.registers.get8(register1);
            let v2 = self.registers.get8(register2);
            let res = v1 & v2;
            if res == 0 { self.registers.set_flag(Flags::Z) } else { self.registers.unset_flag(Flags::Z) }
            self.registers.unset_flag(Flags::C);
            self.registers.set_flag(Flags::H);
            self.registers.unset_flag(Flags::N);
            self.registers.set8(register1, res.0);
            self.instr_state = None;
        }
    }

    //add r, u8
    fn and_r_u8(&mut self, cycle: u32, memory: &mut Memory, register: Register8) {
        if cycle == 8 {
            let v1 = self.registers.get8(register1);
            let v2 = self.fetch_u8_immdiate(memory);
            let res = v1 & v2;
            if res == 0 { self.registers.set_flag(Flags::Z) } else { self.registers.unset_flag(Flags::Z) }
            self.registers.unset_flag(Flags::C);
            self.registers.set_flag(Flags::H);
            self.registers.unset_flag(Flags::N);
            self.registers.set8(register1, res.0);
            self.instr_state = None;
        }
    }

    //add r, (rr)
    fn andi_r_rr(&mut self, cycle: u32, memory: &mut Memory, register: Register8, pair: Register16) {
        if cycle == 8 {
            let v1 = self.registers.get8(register);
            let addr = self.registers.get16(pair);
            let v2 = memory.read(addr);
            let res = v1 & v2;
            if res == 0 { self.registers.set_flag(Flags::Z) } else { self.registers.unset_flag(Flags::Z) }
            self.registers.unset_flag(Flags::C);
            self.registers.set_flag(Flags::H);
            self.registers.unset_flag(Flags::N);
            self.registers.set8(register1, res.0);
            self.instr_state = None;
        }
    }

    //add r1, r2
    fn xor_r_r(&mut self, cycle: u32, register1: Register8, register2: Register8) {
        if cycle == 4 {
            let v1 = self.registers.get8(register1);
            let v2 = self.registers.get8(register2);
            let res = v1 ^ v2;
            if res == 0 { self.registers.set_flag(Flags::Z) } else { self.registers.unset_flag(Flags::Z) }
            self.registers.unset_flag(Flags::C);
            self.registers.unset_flag(Flags::H);
            self.registers.unset_flag(Flags::N);
            self.registers.set8(register1, res.0);
            self.instr_state = None;
        }
    }

    //add r, u8
    fn xor_r_u8(&mut self, cycle: u32, memory: &mut Memory, register: Register8) {
        if cycle == 8 {
            let v1 = self.registers.get8(register1);
            let v2 = self.fetch_u8_immdiate(memory);
            let res = v1 ^ v2;
            if res == 0 { self.registers.set_flag(Flags::Z) } else { self.registers.unset_flag(Flags::Z) }
            self.registers.unset_flag(Flags::C);
            self.registers.unset_flag(Flags::H);
            self.registers.unset_flag(Flags::N);
            self.registers.set8(register1, res.0);
            self.instr_state = None;
        }
    }

    //add r, (rr)
    fn xori_r_rr(&mut self, cycle: u32, memory: &mut Memory, register: Register8, pair: Register16) {
        if cycle == 4 {
            let v1 = self.registers.get8(register);
            let addr = self.registers.get16(pair);
            let v2 = memory.read(addr);
            let res = v1 ^ v2;
            if res == 0 { self.registers.set_flag(Flags::Z) } else { self.registers.unset_flag(Flags::Z) }
            self.registers.unset_flag(Flags::C);
            self.registers.unset_flag(Flags::H);
            self.registers.unset_flag(Flags::N);
            self.registers.set8(register1, res.0);
            self.instr_state = None;
        }
    }

    //add r1, r2
    fn or_r_r(&mut self, cycle: u32, register1: Register8, register2: Register8) {
        if cycle == 4 {
            let v1 = self.registers.get8(register1);
            let v2 = self.registers.get8(register2);
            let res = v1 | v2;
            if res == 0 { self.registers.set_flag(Flags::Z) } else { self.registers.unset_flag(Flags::Z) }
            self.registers.unset_flag(Flags::C);
            self.registers.unset_flag(Flags::H);
            self.registers.unset_flag(Flags::N);
            self.registers.set8(register1, res.0);
            self.instr_state = None;
        }
    }

    //add r, u8
    fn or_r_u8(&mut self, cycle: u32, memory: &mut Memory, register: Register8) {
        if cycle == 8 {
            let v1 = self.registers.get8(register1);
            let v2 = self.fetch_u8_immdiate(memory);
            let res = v1 | v2;
            if res == 0 { self.registers.set_flag(Flags::Z) } else { self.registers.unset_flag(Flags::Z) }
            self.registers.unset_flag(Flags::C);
            self.registers.unset_flag(Flags::H);
            self.registers.unset_flag(Flags::N);
            self.registers.set8(register1, res.0);
            self.instr_state = None;
        }
    }

    //add r, (rr)
    fn ori_r_rr(&mut self, cycle: u32, memory: &mut Memory, register: Register8, pair: Register16) {
        if cycle == 8 {
            let v1 = self.registers.get8(register);
            let addr = self.registers.get16(pair);
            let v2 = memory.read(addr);
            let res = v1 | v2;
            if res == 0 { self.registers.set_flag(Flags::Z) } else { self.registers.unset_flag(Flags::Z) }
            self.registers.unset_flag(Flags::C);
            self.registers.unset_flag(Flags::H);
            self.registers.unset_flag(Flags::N);
            self.registers.set8(register1, res.0);
            self.instr_state = None;
        }
    }

    //add r1, r2
    fn cp_r_r(&mut self, cycle: u32, register1: Register8, register2: Register8) {
        if cycle == 4 {
            let v1 = self.registers.get8(register1);
            let v2 = self.registers.get8(register2);
            let res = v1.overflowing_sub(v2);
            if res.0 == 0 { self.registers.set_flag(Flags::Z) } else { self.registers.unset_flag(Flags::Z) }
            if res.1 { self.registers.set_flag(Flags::C) } else { self.registers.unset_flag(Flags::C) }
            if (v1 & 0xF) - (v2 & 0xF) > 0xF { self.registers.set_flag(Flags::H) } else { self.registers.unset_flag(Flags::H) }
            self.registers.set_flag(Flags::N);
            self.instr_state = None;
        }
    }
    //add r, u8
    fn cp_r_u8(&mut self, cycle: u32, memory: &mut Memory, register: Register8) {
        if cycle == 8 {
            let v1 = self.registers.get8(register1);
            let v2 = self.fetch_u8_immdiate(memory);
            let res = v1.overflowing_sub(v2);
            if res.0 == 0 { self.registers.set_flag(Flags::Z) } else { self.registers.unset_flag(Flags::Z) }
            if res.1 { self.registers.set_flag(Flags::C) } else { self.registers.unset_flag(Flags::C) }
            if (v1 & 0xF) - (v2 & 0xF) > 0xF { self.registers.set_flag(Flags::H) } else { self.registers.unset_flag(Flags::H) }
            self.registers.set_flag(Flags::N);
            self.instr_state = None;
        }
    }
    //add r, (rr)
    fn cpi_r_rr(&mut self, cycle: u32, memory: &mut Memory, register: Register8, pair: Register16) {
        if cycle == 8 {
            let v1 = self.registers.get8(register);
            let addr = self.registers.get16(pair);
            let v2 = memory.read(addr);
            let res = v1.overflowing_sub(v2);
            if res.0 == 0 { self.registers.set_flag(Flags::Z) } else { self.registers.unset_flag(Flags::Z) }
            if res.1 { self.registers.set_flag(Flags::C) } else { self.registers.unset_flag(Flags::C) }
            if (v1 & 0xF) - (v2 & 0xF) > 0xF { self.registers.set_flag(Flags::H) } else { self.registers.unset_flag(Flags::H) }
            self.registers.set_flag(Flags::N);
            self.instr_state = None;
        }
    }

    //DAA
    fn daa(&mut self, cycle: u32) {
        if cycle == 4 {
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
            self.instr_state = None;
        }
    }

    fn cpl(&mut self, cycle: u32) {
        if cycle == 4 {
            let val = self.registers.get8(Register8::A);
            self.registers.set8(Register8::A, val ^ 0xFF)
            self.instr_state = None;
        }
    }

    fn scf(&mut self, cycle: u32) {
        if cycle == 4 {
            self.registers.set_flag(Flags::C);
            self.registers.unset_flag(Flags::N);
            self.registers.unset_flag(Flags::H);
            self.instr_state = None;
        }
    }

    fn ccf(&mut self, cycle: u32) {
        if cycle == 4 {
            if self.registers.get_flag(Flags::C) {
                self.registers.unset_flag(Flags::C);
            } else {
                self.registers.set_flag(Flags::C);
            }
            self.instr_state = None;
        }
    }

    /*
        8-bit rotate/shift bits
            * RLCA
     */

    //Left circular shift register A
    fn rlca(&mut self, cycle: u32) {
        if cycle == 4 {
            let val = self.registers.get8(Targets::A);
            let high = (val & 0xA0) >> 7;
            let res = (val << 1) + high;
            if high { self.registers.set_flag(Flags::C) } else { self.registers.unset_flag(Flags::C) }
            self.registers.set8(Register8::A, res);
            self.registers.unset_flag(Flags::Z);
            self.registers.unset_flag(Flags::N);
            self.registers.unset_flag(Flags::H);
        }
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
    fn rla(&mut self, cycle: u32) {
        if cycle == 4 {
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
    fn rrca(&mut self, cycle: u32) {
        if cycle == 4 {
            let val = self.registers.get8(Targets::A);
            let low = val & 0x01;
            let res = (val >> 1) + (low << 7);
            if low { self.registers.set_flag(Flags::C) } else { self.registers.unset_flag(Flags::C) }
            self.registers.unset_flag(Flags::Z);
            self.registers.unset_flag(Flags::N);
            self.registers.unset_flag(Flags::H);
            self.registers.set8(Register8::A, res);
        }
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
    fn rra(&mut self, cycle: u32) {
        if cycle == 4 {
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
    fn inc_rr(&mut self, cycle: u32, rr: Register16) {
        let rs = rr.sub_registers();
        let mut state = match &self.instr_state {
            Some(x) => x,
            None => panic!("Invalid state"),
        };
        if cycle == 4 {
            let val = self.registers.get16(register);
            let res = val.overflowing_add(1);
            *state.intermediate = res.0;
            self.registers.set8(rs.1, (res.0 & 0xFF) as u8);
        } else if cycle == 8 {
            self.registers.set8(rs.0, (*state.intermediate >> 8) as u8);
            self.instr_state = None;
        }
    }

    fn inc_sp(&mut self, cycle: u32) {
        let mut state = match &self.instr_state {
            Some(x) => x,
            None => panic!("Invalid state"),
        };
        if cycle == 4 {
            let val = self.sp;
            let res = val.overflowing_add(1);
            *state.intermediate = res.0;
            self.sp = self.sp & 0xFF00;
            self.sp = self.sp + (res.0 & 0xFF);
        } else if cycle == 8 {
            self.sp = *state.intermediate;
            self.instr_state = None;
        }
    }

    fn alu16_incsp(&mut self) {
        let res = self.sp.overflowing_add(1);
        self.sp = res.0;
    }

    fn dec_rr(&mut self, cycle: u32, register: Register16) {
        let rs = rr.sub_registers();
        let mut state = match &self.instr_state {
            Some(x) => x,
            None => panic!("Invalid state"),
        };
        if cycle == 4 {
            let val = self.registers.get16(register);
            let res = val.overflowing_sub(1);
            *state.intermediate = res.0;
            self.registers.set8(rs.1, (res.0 & 0xFF) as u8);
        } else if cycle == 8 {
            self.registers.set8(rs.0, (*state.intermediate >> 8) as u8);
            self.instr_state = None;
        }
    }

    fn dec_sp(&mut self, cycle: u32) {
        let mut state = match &self.instr_state {
            Some(x) => x,
            None => panic!("Invalid state"),
        };
        if cycle == 4 {
            let val = self.sp;
            let res = val.overflowing_sub(1);
            *state.intermediate = res.0;
            self.sp = self.sp & 0xFF00;
            self.sp = self.sp + (res.0 & 0xFF);
        } else if cycle == 8 {
            self.sp = *state.intermediate;
            self.instr_state = None;
        }
    }

    fn alu16_decsp(&mut self) {
        let res = self.sp.overflowing_sub(1);
        self.sp = res.0;
    }

    fn add_rr_rr(&mut self, cycle: u32, pair1: Register16, pair2: Register16) {
        if cycle == 4 {
            let v1 = self.registers.get16(pair1);
            let v2 = self.registers.get16(pair2);
            let res = v1.overflowing_add(v2);
            if (v1 & 0x0F00) + (v2 & 0x0F00) > 0x0F00 { self.registers.set_flag(Flags::H) } else { self.registers.unset_flag(Flags::H) }
            if res.1 { self.registers.set_flag(Flags::C) } else { self.registers.unset_flag(Flags::C) }
            self.registers.unset_flag(Flags::N);
            self.registers.set16(pair1, res.0);
        } else if cycle == 8 {
            self.instr_state = None;
        }
    }

    fn add_rr_sp(&mut self, cycle: u32, register: Register16) {
        let mut state = match &self.instr_state {
            Some(x) => x,
            None => panic!("Invalid state"),
        };
        let rs = register.sub_registers();
        if cycle == 4 {
            let val = self.registers.get16(register);
            let res = val.overflowing_add(self.sp);
            if (val & 0x0F00) + (self.sp & 0x0F00) > 0x0F00 { self.registers.set_flag(Flags::H) } else { self.registers.unset_flag(Flags::H) }
            if res.1 { self.registers.set_flag(Flags::C) } else { self.registers.unset_flag(Flags::C) }
            self.registers.unset_flag(Flags::N);
            *state.intermediate = res.0;
            self.registers.set8(rs.1, (res.0 & 0xFF) as u8);
        }  else if cycle == 8 {
            self.registers.set8(rs.0, (*state.intermediate >> 8) as u8);
            self.instr_state = None;
        }
    }

    fn add_sp_i8(&mut self, cycle: u32, memory: &mut Memory) {
        let e8 = memory.read(self.pc + 1) as i8;
        let val = e8.abs() as u8;
        let res = if e8 >= 0 { self.sp.overflowing_add(val as u16) } else { self.sp.overflowing_sub(val as u16) };
        if (self.sp & 0x0F00) + (e8 & 0x0F00) > 0x0F00 { self.registers.set_flag(Flags::H) } else { self.registers.unset_flag(Flags::H) }
        if res.1 { self.registers.set_flag(Flags::C) } else { self.registers.unset_flag(Flags::C) }
        self.registers.unset_flag(Flags::N);
        self.registers.unset_flag(Flags::Z);
        self.sp = res.0;
    }

    //LD HL,SP+i8
    fn ld_rr_spi8(&mut self, cycle: u32, memory: &mut Memory, register: Register16) {
        if cycle == 12 {
            let e8 = memory.read(self.pc + 1) as i8;
            let val = e8.abs() as u8;
            let res = if e8 >= 0 { self.sp.overflowing_add(val as u16) } else { self.sp.overflowing_sub(val as u16) };
            if (self.sp & 0x0F00) + (e8 & 0x0F00) > 0x0F00 { self.registers.set_flag(Flags::H) } else { self.registers.unset_flag(Flags::H) }
            if res.1 { self.registers.set_flag(Flags::C) } else { self.registers.unset_flag(Flags::C) }
            self.registers.unset_flag(Flags::N);
            self.registers.unset_flag(Flags::Z);
            self.registers.set16(register, res.0);
        }
    }

    /*
        CPU Control / Misc
    */
    fn ret(&mut self, cycle: u32, memory: &mut Memory, conditions: Vec<Flags>) {
        let mut state = match &self.instr_state {
            Some(x) => x,
            None => panic!("Invalid state"),
        };
        if cycle == 8 {
            let mut jump = true;
            for condition in conditions {
                if !self.registers.get_flag(condition) { jump = false }
            }
            if !jump { self.instr_state = None; }
        } else if cycle == 12 {
            *state.intermediate = memory.read(self.sp);
            self.sp = self.sp + 1;
        } else if cycle == 16 {
            *state.intermediate = *state.intermediate + ((memory.read(self.sp) as u16) << 8);
            self.sp = self.sp + 1;
        } else if cycle == 20 {
            self.pc = *state.intermediate;
            self.instr_state = None;
        }
    }

    fn rst(&mut self, cycle: u32, memory: &mut Memory, vec: u16) {
        if cycle == 12 {
            self.sp = self.sp - 1;
            memory.write(self.sp, ((self.pc + 4) >> 8) as u8);
        } else if cycle == 16 {
            self.sp = self.sp - 1;
            memory.write(self.sp - 1, ((self.pc + 3) & 0xFF) as u8);
            self.pc = vec;
            self.instr_state = None;
        }
    }

    fn call(&mut self, cycle: u32, memory: &mut Memory, conditions: Vec<Flags>){
        let mut state = match &self.instr_state {
            Some(x) => x,
            None => panic!("Invalid state"),
        };
        if cycle == 8 {
            *state.intermediate = memory.read(self.pc + 1);
        } else if cycle == 12 {
            *state.intermediate  = *state.intermediate + ((memory.read(self.pc + 2) as u16) << 8);
            self.pc = self.pc + 2;
            let mut jump = true;
            for condition in conditions {
                if !self.registers.get_flag(condition) { jump = false }
            }
            if !jump { self.instr_state = None; }
        } else if cycle == 16 {
            self.sp = self.sp -1 ;
            memory.write(self.sp, ((self.pc + 4) >> 8) as u8);
        } else if cycle == 20 {
            self.sp = self.sp - 1;
            memory.write(self.sp - 1, ((self.pc + 3) & 0xFF) as u8);
        } else if cycle == 24 {
            self.instr_state = None;
        }
    }

    fn jr_i8(&mut self, cycle: u32, memory: &mut Memory, conditions: Vec<Flags>) {
        if cycle == 8 {
            let mut jump = true;
            for condition in conditions {
                if !self.registers.get_flag(condition) { jump = false }
            }
            if !jump { self.instr_state = None; }
        } else if cycle == 12 {
            //let e8 = self.fetch_u8_immdiate(memory) as i8;
            let e8 = memory.read(self.pc + 1) as i8;
            let val = e8.abs() as u8;
            let res = if e8 >= 0 { self.pc.overflowing_add(val as u16) } else { self.pc.overflowing_sub(val as u16) };
            self.pc = res.0;
            self.instr_state = None;
        }
    }

    fn jp_u16(&mut self, cycle: u32, memory: &mut Memory, conditions: Vec<Flags>) {
        let mut state = match &self.instr_state {
            Some(x) => x,
            None => panic!("Invalid state"),
        };
        if cycle == 8 {
            *state.intermediate = memory.read(self.pc + 1);
        } else if cycle == 12 {
            *state.intermediate  = *state.intermediate + ((memory.read(self.pc + 2) as u16) << 8);
            let mut jump = true;
            for condition in conditions {
                if !self.registers.get_flag(condition) { jump = false }
            }
            if !jump { self.instr_state = None; }
        } else if cycle == 16 {
            self.pc = *state.intermediate;
            self.instr_state = None;
        }
    }

    fn jp_rr(&mut self, cycle: u32, register: Register16) {
        if cycle == 4 {
            let val = self.registers.get16(register);
            self.pc = val;
            self.instr_state = None;
        }
    }

    fn di(&mut self, cycle: u32) {
        if cycle == 4 {
            self.interrupts.reset_ime();
            self.instr_state = None;
        }
    }

    fn ei(&mut self, cycle: u32) {
        if cycle == 4 {
            self.interrupts.ei();
        }
    }

    fn reti(&mut self, cycle: u32, memory: &mut Memory) {
        let mut state = match &self.instr_state {
            Some(x) => x,
            None => panic!("Invalid state"),
        };
        if cycle == 8 {
            *state.intermediate = memory.read(self.sp);
            self.sp = self.sp + 1;
        } else if cycle == 12 {
            *state.intermediate = *state.intermediate + ((memory.read(self.sp) as u16) << 8);
            self.sp = self.sp + 1;
        } else if cycle == 16 {
            self.interrupts.set_ime();
            self.pc = *state.intermediate;
            self.instr_state = None;
        }
    }

    fn prefix(&mut self, cycle: u32) {
        let mut state = match &self.instr_state {
            Some(x) => x,
            None => panic!("Invalid state"),
        };
        *state.prefix = true;
    }
}