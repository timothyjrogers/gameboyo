pub struct DIV {
    high: u8,
    low: u8,
}

impl DIV {
    pub fn new(data: u16) -> Self {
        Self {
            high: data >> 8,
            low: data && 0x00FF,
        }
    }

    pub fn write(&mut self) {
        self.high = 0;
        self.low = 0;
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

pub struct TIMA {
    bits: u8,
    tma: u8,
}

impl TIMA {
    pub fn new() -> Self {
        Self { bits: 0, tma: 0 }
    }

    pub fn increment(&mut self) -> bool {
        match self.bits.checked_add(1) {
            Some(x) => {
                self.bits += 1;
                return false;
            },
            None => {
                self.bits = self.tma;
                return true;
            }
        }
    }

    pub fn reset(&mut self) {
        self.bits = 0;
    }

    pub fn set_tma(&mut self, val: u8) {
        self.tma = val;
    }
}

pub struct TAC {
    bits: u8,
}

impl TAC {
    pub fn new() -> Self {
        Self { bits: 0 }
    }

    pub fn enabled(&self) -> u8 {
        return self.bits & 0b00000001;
    }

    pub fn set_enabled(&mut self, val: bool) {
        if val {
            self.bits = self.bits | 0b00000001;
        } else {
            self.bits = self.bits & 0b11111110;
        }
    }

    pub fn freq(&self) -> u8 {
        return (self.bits & 0b00000110) >> 2;
    }

    pub fn set_freq(&mut self, val: u8) {
        if val == 0 {
            self.bits = self.bits | 0b00000000;
        } else if val == 1 {
            self.bits = self.bits | 0b00000010;
        } else if val == 2 {
            self.bits = self.bits | 0b00000100;
        } else if val == 3 {
            self.bits = self.bits | 0b00000110;
        }
    }
}