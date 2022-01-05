pub mod mbc0;

pub trait MemoryBankController {
    //fn init(&self, data: Vec<u8>);
    fn read(&self, addr: u16) -> u8;
    fn read_double(&self, addr: u16) -> u16;
    fn write(&mut self, addr:u16, data: u8);
    fn write_double(&mut self, addr:u16, data: u16);
}