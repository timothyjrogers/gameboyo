/*
Implementation for the "No MBC" case: https://gbdev.io/pandocs/nombc.html
*/
use crate::emulator::constants;
use crate::emulator::mbc::MemoryBankController;

pub struct MBC0 {
    rom: [u8; constants::SIXTEEN_KB],
    ram: [u8; constants::EXTERNAL_RAM_SIZE],
}

impl MBC0 {
    pub fn new() -> Self {
        Self {
            rom: [0; constants::SIXTEEN_KB],
            ram: [0; constants::EXTERNAL_RAM_SIZE],
        }
    }
}

impl MemoryBankController for MBC0 {
    fn read(&self, addr: u16) -> u8 {
        match addr as usize {
            constants::SWITCHABLE_ROM_START..=constants::SWITCHABLE_ROM_END => self.rom[addr as usize - constants::SIXTEEN_KB],
            constants::EXTERNAL_RAM_START..=constants::EXTERNAL_RAM_END => self.ram[addr as usize],
            _ => panic!("Unreachable memory")
        }
    }

    fn write(&mut self, addr: u16, data: u8) {
        match addr as usize {
            constants::SWITCHABLE_ROM_START..=constants::SWITCHABLE_ROM_END => self.rom[addr as usize - constants::SIXTEEN_KB] = data,
            constants::EXTERNAL_RAM_START..=constants::EXTERNAL_RAM_END => self.ram[addr as usize] = data,
            _ => panic!("Unreachable memory")
        }  }
}