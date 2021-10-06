use crate::emulator::constants;
use crate::emulator::mbc::*;
use crate::emulator::constants::{FOUR_KB, ONBOARD_ROM_END};

pub struct Memory {
    onboard_rom: [u8; constants::SIXTEEN_KB],
    memory_bank_controller: Box<MemoryBankController>,
    vram: [[u8; constants::EIGHT_KB]; 2],
    vram_active_bank: usize,
    onboard_wram: [u8; constants::FOUR_KB],
    switchable_wram: [[u8; constants::FOUR_KB]; 7],
    wram_active_bank: usize,
    oam: [u8; 0x9F],
    io_reg: [u8; 0x7F],
    hram: [u8; 0x8E],
    ie_reg: u8,
}

impl Memory {
    pub fn new(path: String) -> Self {
        let rom_data = std::fs::read(path).unwrap();
        println!("{}", rom_data.len());
        let cartridge_type = rom_data[constants::CARTRIDGE_TYPE];
        let mbc = match cartridge_type {
            0x00 => Box::new(mbc0::MBC0::new()),
            _ => panic!("Unsupported cartridge type")
        };
        let mut mem = Self {
            onboard_rom: [0; constants::ROM_BANK_SIZE],
            memory_bank_controller: mbc,
            vram: [[0; constants::EIGHT_KB]; 2],
            vram_active_bank: 0,
            onboard_wram: [0; constants::FOUR_KB],
            switchable_wram: [[0; constants::FOUR_KB]; 7],
            wram_active_bank: 0,
            oam: [0; 0x9F],
            io_reg: [0; 0x7F],
            hram: [0; 0x8E],
            ie_reg: 0,
        };
        for i in constants::ONBOARD_ROM_START..=ONBOARD_ROM_END {
            mem.onboard_rom[i] = rom_data[i];
        }
        //TODO -- currently only supports MBC0
        for i in constants::SWITCHABLE_ROM_START..=constants::SWITCHABLE_ROM_END {
            mem.memory_bank_controller.write(i as u16, rom_data[i]);
        }
        //TODO -- debug only
        //mem.dump_rom();
        return mem;
    }

    pub fn read(&self, addr: u16) -> u8 {
        let byte: u8 = match addr as usize {
            constants::ONBOARD_ROM_START..=constants::ONBOARD_ROM_END => self.onboard_rom[addr as usize],
            constants::SWITCHABLE_ROM_START..=constants::SWITCHABLE_ROM_END => self.memory_bank_controller.read(addr),
            constants::SWITCHABLE_VRAM_START..=constants::SWITCHABLE_VRAM_END => self.vram[self.vram_active_bank][addr as usize - constants::SWITCHABLE_VRAM_START],
            constants::EXTERNAL_RAM_START..=constants::EXTERNAL_RAM_END => self.memory_bank_controller.read(addr),
            constants::ONBOARD_WRAM_START..=constants::ONBOARD_WRAM_END => self.onboard_wram[addr as usize - constants::ONBOARD_WRAM_START],
            constants::SWITCHABLE_WRAM_START..=constants::SWITCHABLE_WRAM_END => self.switchable_wram[self.wram_active_bank][addr as usize - constants::SWITCHABLE_WRAM_START],
            constants::ECHO_RAM_LOW_START..=constants::ECHO_RAM_LOW_END => self.onboard_wram[addr as usize - constants::ONBOARD_WRAM_START],
            constants::ECHO_RAM_HIGH_START..=constants::ECHO_RAM_HIGH_END => self.memory_bank_controller.read(addr),
            constants::OAM_START..=constants::OAM_END => self.oam[addr as usize - constants::OAM_START],
            constants::IO_REG_START..=constants::IO_REG_END => self.io_reg[addr as usize - constants::IO_REG_START],
            constants::HRAM_START..=constants::HRAM_END => self.hram[addr as usize - constants::HRAM_START],
            constants::IE_REGISTER => self.ie_reg,
            _ => panic!("Unreachable address")
        };
        return byte;
    }

    pub fn write(&mut self, addr: u16, data: u8) {
        match addr as usize {
            constants::ONBOARD_ROM_START..=constants::ONBOARD_ROM_END => self.onboard_rom[addr as usize] = data,
            constants::SWITCHABLE_ROM_START..=constants::SWITCHABLE_ROM_END => self.memory_bank_controller.write(addr, data),
            constants::SWITCHABLE_VRAM_START..=constants::SWITCHABLE_VRAM_END => self.vram[self.vram_active_bank][addr as usize - constants::SWITCHABLE_VRAM_START] = data,
            constants::EXTERNAL_RAM_START..=constants::EXTERNAL_RAM_END => self.memory_bank_controller.write(addr, data),
            constants::ONBOARD_WRAM_START..=constants::ONBOARD_WRAM_END => self.onboard_wram[addr as usize - constants::ONBOARD_WRAM_START] = data,
            constants::SWITCHABLE_WRAM_START..=constants::SWITCHABLE_WRAM_END => self.switchable_wram[self.wram_active_bank][addr as usize - constants::SWITCHABLE_WRAM_START] = data,
            constants::ECHO_RAM_LOW_START..=constants::ECHO_RAM_LOW_END => self.onboard_wram[addr as usize - constants::ONBOARD_WRAM_START] = data,
            constants::ECHO_RAM_HIGH_START..=constants::ECHO_RAM_HIGH_END => self.memory_bank_controller.write(addr, data),
            constants::OAM_START..=constants::OAM_END => self.oam[addr as usize - constants::OAM_START] = data,
            constants::IO_REG_START..=constants::IO_REG_END => self.io_reg[addr as usize - constants::IO_REG_START] = data,
            constants::HRAM_START..=constants::HRAM_END => self.hram[addr as usize - constants::HRAM_START] = data,
            constants::IE_REGISTER => self.ie_reg = data,
            _ => panic!("Unreachable address")
        };
    }

    fn dump_rom(&self) {
        let pc = 0x0100;
        for i in 0x0100..=constants::SIXTEEN_KB {
            println!("{:#04x}", self.read(i as u16));
        }
    }
}