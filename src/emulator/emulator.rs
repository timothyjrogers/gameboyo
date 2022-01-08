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
    timer_state,
    interrupt_state: InterruptState,
}

enum TimerState {
    Nil,
    InterruptReady,
}

enum InterruptState {
    Nil,
    Nop1,
    Nop2,
    PushPC1,
    PushPC2,
    SetPC,
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
            timer_state: Timer::Nil,
            interrupt_state: InterruptState::Nil,
        }
    }

    /*
        CPU needs reference to Memory, Timer, Video Controller to read/write values

        First tick the timer module. If TIMA overflows, set the timer state so an interrupt will be set on the NEXT machine cycle.
        Next, check if any interrupt flags are set for interrupts that are enabled. If so:
            - Get highest-priority (lowest address) Interrupt vector
            - Run interrupt handler (typically disables IME while handling):
                - (2 machine cycles) NOP
                - (2 machine cycles) Push current PC to stack
                - (1 machine cycle) Set PC to Interrupt vector
            - Loop until all enabled interrupt flags are cleared
        Next, fetch / decode / execute from memory[PC]
     */
    pub fn tick(&mut self) {
        match &self.timer_state {
            TimerState::InterruptReady => {
                self.cpu.enable_interrupt(Interrupt::Timer);
                self.timer.set_tima();
                self.timer_state = TimerState::Nil;
            },
            _ => ()
        }
        if self.timer.tick() { self.timer_state = TimerState::InterruptReady }

        //check interrupts, process if necessary
        match &self.interrupt_state {
            InterruptState::Nil => {
                if self.cpu.interrupt_ready() {
                    self.cpu.setup_interrupts(&mut self.memory);
                }
            },
            InterruptState::Nop1 => {
                self.interrupt_state = InterruptState::Nop1;
                return;
            },
            InterruptState::Nop2 => {
                self.interrupt_state = InterruptState::PushPC1;
                return;
            },
            InterruptState::PushPC1=> {

            }
            InterruptState::PushPC2 => {},
            InterruptState::SetPC => {},
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