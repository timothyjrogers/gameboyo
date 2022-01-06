pub struct Register {
    high: u8,
    low: u8,
}

impl Register {
    pub fn new(data: u16) -> Self {
        Self {
            high: (data >> 8) as u8,
            low: (data & 0x00FF) as u8,
        }
    }

    pub fn write(&mut self, data: u16) {
        self.high = (data >> 8) as u8;
        self.low = (data & 0x00FF) as u8;
    }

    pub fn write_high(&mut self, data: u8) {
        self.high = data;
    }

    pub fn write_low(&mut self, data: u8) {
        self.low = data;
    }

    pub fn read(&self) -> u16 {
        let mut val: u16 = self.high as u16;
        val = val << 8;
        val += self.low as u16;
        return val;
    }

    pub fn read_high(&self) -> u8 {
        return self.high;
    }

    pub fn read_low(&self) -> u8 {
        return self.low;
    }
}

pub enum Interrupts {
    VerticalBlanking,
    LcdStat,
    Timer,
    Serial,
    Joypad,
}

pub struct InterruptEnable {
    bits: u8,
}

impl InterruptEnable {
    pub fn new() -> Self {
        Self { bits: 0 }
    }

    
}


/*

 */
pub struct InterruptFlags {
    bits: u8,
}

impl InterruptFlags {
    pub fn new() -> Self {
        Self { bits: 0b11100000 }
    }

    pub fn set(&mut self, bit: u8) {
        if bit > 4 { return }
        let mask: u8 = 0b00000001;
        self.bits = self.bits | (mask << bit);
    }

    pub fn reset(&mut self, bit: u8) {
        if bit > 4 { return }
        let mask: u8 = 0b11111110;
        self.bits = self.bits & mask.rotate_left(bit.into());
    }

    pub fn read(&self) -> u8 {
        self.bits
    }

    pub fn read_bit(&self, bit: u8) -> u8 {
        if bit > 4 { return 1 }
        return self.bits & (0b00000001 << bit) >> bit;
    }

    pub fn interrupt_set(&self) -> bool {
        return self.bits > 0;
    }
}