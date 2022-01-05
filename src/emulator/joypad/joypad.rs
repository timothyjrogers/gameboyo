pub struct Joypad {
    p1: u8,
}

impl Joypad {
    pub fn new() -> Self {
        Self { p1: 0b11000000 }
    }

    pub fn set_bit(&mut self, bit: u8) {
        self.p1 = 0b11000000 | (0b11111111 & (0b00000001 << bit));
    }

    pub fn get_bit(&self, bit: u8) -> u8 {
        return (self.p1 & (0b00000001 << bit)) >> bit;
    }
}