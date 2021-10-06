pub mod mbc0;

pub trait MemoryBankController {
    //fn init(&self, data: Vec<u8>);
    fn read(&self, addr: u16) -> u8;
    fn write(&mut self, addr:u16, data: u8);
}