pub struct Register {
    high: u8,
    low: u8,
}

impl Register {
    pub fn new(data: u16) -> Self {
        Self {
            high: data >> 8,
            low: data && 0x00FF,
        }
    }

    pub fn write(&mut self, data: u16) {
        self.high = data >> 8;
        self.low = data && 0x00FF;
    }

    pub fn write_high(&mut self, data: u8) {
        self.high = data;
    }

    pub fn write_low(&mut self, data: u8) {
        self.low = data;
    }

    pub fn read(&self) -> u16 {
        let val: u16 = 0;
        val += self.high;
        val = val << 8;
        val += self.low;
        return val;
    }

    pub fn read_high(&self) -> u8 {
        return self.high;
    }

    pub fn read_low(&self) -> u8 {
        return self.low;
    }
}