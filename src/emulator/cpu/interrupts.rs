#[derive(Clone, Copy)]
pub enum Interrupt {
    VerticalBlanking,
    LcdStat,
    Timer,
    Serial,
    Joypad,
}

enum InterruptState {
    Disabled,
    Enabled,
    Pending(u32),
}

pub struct InterruptRegisters {
    enable: u8,
    flags: u8,
    ime: bool,
    state: InterruptState,
}

impl InterruptRegisters {
    pub fn new() -> Self {
        Self {
            enable: 0b00000000,
            flags: 0b11100000,
            ime: true,
            state: InterruptState::Enabled,
        }
    }

    pub fn enabled(&self, interrupt: Interrupt) -> bool {
        let mut mask;
        match interrupt {
            Interrupt::VerticalBlanking => mask = 0b00000001,
            Interrupt::LcdStat => mask = 0b00000010,
            Interrupt::Timer => mask = 0b00000100,
            Interrupt::Serial => mask = 0b00001000,
            Interrupt::Joypad => mask = 0b00010000,
        }
        return (self.enable & mask) > 0;
    }

    pub fn enable(&mut self, interrupt: Interrupt) {
        let mut mask;
        match interrupt {
            Interrupt::VerticalBlanking => mask = 0b00000001,
            Interrupt::LcdStat => mask = 0b00000010,
            Interrupt::Timer => mask = 0b00000100,
            Interrupt::Serial => mask = 0b00001000,
            Interrupt::Joypad => mask = 0b00010000,
        }
        self.enable = self.enable | mask;
    }

    pub fn disable(&mut self, interrupt: Interrupt) {
        let mut mask;
        match interrupt {
            Interrupt::VerticalBlanking => mask = 0b11111110,
            Interrupt::LcdStat => mask = 0b11111101,
            Interrupt::Timer => mask = 0b11111011,
            Interrupt::Serial => mask = 0b11110111,
            Interrupt::Joypad => mask = 0b11101111,
        }
        self.enable = self.enable & mask;
    }

    pub fn get_flag(&self, interrupt: Interrupt) -> bool {
        let mut mask;
        match interrupt {
            Interrupt::VerticalBlanking => mask = 0b00000001,
            Interrupt::LcdStat => mask = 0b00000010,
            Interrupt::Timer => mask = 0b00000100,
            Interrupt::Serial => mask = 0b00001000,
            Interrupt::Joypad => mask = 0b00010000,
        }
        return (self.bits & mask) > 0;
    }

    pub fn set_flag(&mut self, interrupt: Interrupt) {
        let mut mask;
        match interrupt {
            Interrupt::VerticalBlanking => mask = 0b00000001,
            Interrupt::LcdStat => mask = 0b00000010,
            Interrupt::Timer => mask = 0b00000100,
            Interrupt::Serial => mask = 0b00001000,
            Interrupt::Joypad => mask = 0b00010000,
        }
        self.bits = self.bits | mask;
    }

    pub fn reset_flag(&mut self, interrupt: Interrupt) {
        let mut mask;
        match interrupt {
            Interrupt::VerticalBlanking => mask = 0b11111110,
            Interrupt::LcdStat => mask = 0b11111101,
            Interrupt::Timer => mask = 0b11111011,
            Interrupt::Serial => mask = 0b11110111,
            Interrupt::Joypad => mask = 0b11101111,
        }
        self.bits = self.bits & mask;
    }

    pub fn enabled_any(&self) -> bool {
        return self.bits & 0b00011111 > 0;
    }

    pub fn get_ime(&self) -> bool {
        return self.ime;
    }

    pub fn set_ime(&mut self) {
        self.ime = true;
        self.state = InterruptState::Enabled;
    }

    pub fn reset_ime(&mut self) {
        self.ime = false;
        self.state = InterruptState::Disabled;
    }

    pub fn check_interrupt(&self, interrupt: Interrupt) -> bool {
        return self.ime && self.enabled(interrupt) && self.get_flag(interrupt);
    }

    pub fn ei(&mut self) {
        self.state = InterruptState::Pending(0);
    }

    pub fn check_ei(&mut self) {
        match &self.state {
            InterruptStatePending(x) => {
                if *x == 0 {
                    self.state = InterruptState::Pending(1);
                } else {
                    self.set_ime;
                }
            }
        }
    }
}