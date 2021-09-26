use crate::emulator::constants;

pub struct Emulator {
    registers: Registers,
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

impl Emulator {
    pub fn new(path: String) {
        let rom = std::fs::read(path).unwrap();
        let mut data: Vec<u8> = vec![];
        for (pos, e) in rom.iter().enumerate() {
            data.push(*e);
        }
        #[cfg(debug_assertions)]
            {
                println!("--Dumping Cartridge Header--");
                println!("--Entry Point--");
                println!("0x0100: {:#04x}", data[0x0100]);
                println!("0x0101: {:#04x}", data[0x0101]);
                println!("0x0102: {:#04x}", data[0x0102]);
                println!("0x0103: {:#04x}", data[0x0103]);
                println!("--Nintendo Logo--");
                for i in 0x0104..=0x0133 {
                    println!("{:#06x}: {:#04x}", i, data[i]);
                }
                println!("--Title--"); //Note: Title, manufacturer code, CGB Flags are split out regardless of cartridge format
                for i in 0x0134..=0x013E {
                    println!("{:#06x}: {}", i, data[i] as char);
                }
                println!("--Manufacturer Code--");
                for i in 0x013F..=0x0142 {
                    println!("{:#06x}: {:#04x}", i, data[i]);
                }
                println!("--CGB Flag--");
                println!("0x0143: {:#04x}", data[0x0143]);
                println!("--New Licensee Code--");
                println!("0x0144: {:#04x}", data[0x0144]);
                println!("0x0145: {:#04x}", data[0x0145]);
                println!("--SGB Flag--");
                println!("0x0146: {:#04x}", data[0x0146]);
                println!("--Cartridge Type--");
                println!("0x0147: {:#04x}", data[0x0147]);
                println!("--ROM Size--");
                println!("0x0148: {:#04x}", data[0x0148]);
                println!("--RAM Size--");
                println!("0x0149: {:#04x}", data[0x0149]);
                println!("--Destination Code--");
                println!("0x014A: {:#04x}", data[0x014A]);
                println!("--Old Licensee Code--");
                println!("0x014B: {:#04x}", data[0x014B]);
                println!("--Mask ROM Version Number--");
                println!("0x014C: {:#04x}", data[0x014C]);
                println!("--Header Checksum--");
                println!("0x014D: {:#04x}", data[0x014D]);
                println!("--Global Checksum--");
                println!("0x014E: {:#04x}", data[0x014E]);
                println!("0x014F: {:#04x}", data[0x014F]);
            }
    }
}