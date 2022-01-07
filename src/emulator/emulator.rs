use crate::emulator::constants;
use crate::emulator::memory::memory::Memory;
use crate::emulator::cpu::cpu::CPU;
use crate::emulator::timer::timer::Timer;
use crate::emulator::joypad::joypad::Joypad;
use crate::emulator::video::video::VideoController;
use crate::emulator::cpu::registers::Interrupt;

pub struct Emulator {
    memory: Memory,
    cpu: CPU,
    timer: Timer,
    joypad: Joypad,
    video: VideoController,
    timer_state;
}

enum TimerState {
    Steady,
    InterruptReady,
}

pub enum Platform {
    DMG,
    GBC
}

impl Emulator {
    pub fn new(path: String) -> Self {
        let platform: Platform;
        if path.ends_with(".gb") {
            platform = Platform::DMG;
        } else if path.ends_with((".gbc")) {
            platform = Platform::GBC;
        } else {
            panic!("Unrecognized file type, please provide .gb or .gbc file");
        }
        let memory = Memory::new(path);
        let cpu = CPU::new(&platform);
        let timer = Timer::new(&platform);
        let joypad = Joypad::new();
        let video = VideoController::new();
        Self {
            memory,
            cpu,
            timer,
            joypad,
            video,
        }
    }

    /*
        CPU needs reference to Memory, Timer, Video Controller to read/write values
     */
    pub fn tick(&mut self) {
        match self.timer_state {
            TimerState::InterruptReady => {
                self.cpu.enable_interrupt(Interrupt::Timer);
                self.timer.set_tima();
                self.timer_state = TimerState::Steady;
            },
            _ => ()
        }
        if self.timer.tick() { self.timer_state = TimerState::InterruptReady }

        //check interrupts, process if necessary
        if self.cpu.interrupt_ready() {
            self.cpu.setup_interrupts(&mut self.memory);
        }
        //fetch instruction
    }

    /*
    pub fn validate_logo(&self) -> bool {
        let mut valid = true;
        for i in constants::LOGO_START..=constants::LOGO_END {
            if self.memory.read(i as u16) != constants::NINTENDO_LOGO[i - constants::LOGO_START] {
                valid = false;
            }
        }
        println!("Logo validation = {}", valid);
        return valid;
    }
     */
}