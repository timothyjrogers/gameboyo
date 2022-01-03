pub struct InterruptEnable {
    bits: u8,
}

impl InterruptEnable {
    pub fn new() -> Self {
        Self { bits: 0 }
    }

    pub fn set(&mut self, bit: u8) {
        let mask: u8 = 0b00000001;
        self.bits = self.bits | (mask << bit)
    }

    pub fn reset(&mut self, bit: u8) {
        let mask: u8 = 0b11111110;
        self.bits = self.bits & mask.rotate_left(bit)
    }

    pub fn read(&self) -> u8 {
        self.bits
    }

    pub fn read_bit(&self, bit: u8) -> u8 {
        return self.bits & (0b00000001 << bit) >> bit;
    }
}