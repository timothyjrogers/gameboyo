pub struct InterruptFlags {
    bits: u8,
}

impl InterruptFlags {
    pub fn new() -> Self {
        Self { bits: 0 }
    }

    pub fn set(&mut self, bit: u8) {
        if bit > 4 { return }
        let mask: u8 = 0b00000001;
        self.bits = self.bits | (mask << bit)
    }

    pub fn reset(&mut self, bit: u8) {
        if bit > 4 { return }
        let mask: u8 = 0b11111110;
        self.bits = self.bits & mask.rotate_left(bit)
    }

    pub fn read(&self) -> u8 {
        self.bits
    }

    pub fn read_bit(&self, bit: u8) -> u8 {
        if bit > 4 { return 1 }
        return self.bits & (0b00000001 << bit) >> bit;
    }
}