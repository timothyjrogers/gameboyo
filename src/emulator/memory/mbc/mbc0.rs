/*
Implementation for the "No MBC" case: https://gbdev.io/pandocs/nombc.html
*/
use crate::emulator::constants;
use crate::emulator::memory::mbc::MemoryBankController;

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

    //little-endian (least significant byte first)
    fn read_double(&self, addr: u16) -> u16 {
        match addr as usize {
            constants::SWITCHABLE_ROM_START..=constants::SWITCHABLE_ROM_END => {
                let low = self.rom[addr as usize - constants::SIXTEEN_KB];
                let high = self.rom[(addr + 1) as usize - constants::SIXTEEN_KB];
                return ((high as u16) << 8) + low as u16;
            },
            constants::EXTERNAL_RAM_START..=constants::EXTERNAL_RAM_END => {
                let low = self.rom[addr as usize];
                let high = self.rom[(addr + 1) as usize];
                return ((high as u16) << 8) + low as u16;
            },
            _ => panic!("Unreachable memory")
        }
    }

    fn write(&mut self, addr: u16, data: u8) {
        match addr as usize {
            constants::SWITCHABLE_ROM_START..=constants::SWITCHABLE_ROM_END => self.rom[addr as usize - constants::SIXTEEN_KB] = data,
            constants::EXTERNAL_RAM_START..=constants::EXTERNAL_RAM_END => self.ram[addr as usize] = data,
            _ => panic!("Unreachable memory")
        }
    }

    //little-endian (least significant byte first)
    fn write_double(&mut self, addr: u16, data: u16) {
        match addr as usize {
            constants::SWITCHABLE_ROM_START..=constants::SWITCHABLE_ROM_END => {
                self.rom[addr as usize - constants::SIXTEEN_KB] = (data & 0x00FF) as u8;
                self.rom[(addr + 1) as usize - constants::SIXTEEN_KB] = (data >> 8) as u8;
            },
            constants::EXTERNAL_RAM_START..=constants::EXTERNAL_RAM_END => {
                self.rom[addr as usize] = (data & 0x00FF) as u8;
                self.rom[(addr + 1) as usize] = (data >> 8) as u8;
            },
            _ => panic!("Unreachable memory")
        }
    }
}