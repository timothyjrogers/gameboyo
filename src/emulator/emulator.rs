use crate::emulator::constants;
use crate::emulator::memory::memory::Memory;
use crate::emulator::cpu::cpu::CPU;
use crate::emulator::timer::timer::Timer;
use crate::emulator::joypad::joypad::Joypad;
use crate::emulator::video::video::VideoController;
use crate::emulator::cpu::registers::Interrupt;
use crate::emulator::cpu::cpu::CpuState;

pub struct Emulator {
    memory: Memory,
    cpu: CPU,
    timer: Timer,
    joypad: Joypad,
    video: VideoController,
    timer_state: TimerState,
    interrupt_state: InterruptState,
}

enum TimerState {
    Normal,
    InterruptReady,
}

enum InterruptState {
    Ready,
    Nop(Interrupt),
    PushPc(Interrupt),
    WaitPc(Interrupt),
    LoadVector(Interrupt),
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
            timer_state: TimerState::Nil,
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
        Next, fetch / decode / execute from memory[PC]
     */
    pub fn tick(&mut self) {
        //Tick the system internal timer (and thereby DIV). If TIMA overflows, set IF for timer overflow
        match &self.timer_state {
            TimerState::InterruptReady => {
                self.timer.set_tima();
                self.timer_state = TimerState::Normal;
                if self.timer.read_tima() == 0 { self.cpu.enable_interrupt(Interrupt::Timer); }
            },
            TimerState::Normal => {
                for _ in 0..4 {
                    if self.timer.tick() {
                        self.timer_state = TimerState::InterruptReady;
                        self.timer.reset_tima();
                        break;
                    }
                }
            }
        }
        //TODO -- Check Joypad. State to be passed in from Iced

        //check interrupts, transfer control via ISR if necessary
        if self.cpu.state == CpuState::Ready {
            match &self.interrupt_state {
                InterruptState::Ready => {
                    match self.cpu.get_interrupt() {
                        Some(x) => {
                            self.interrupt_state = InterruptState::Nop(x);
                            self.cpu.reset_interrupt_flag(x);
                            self.cpu.reset_ime();
                            return;
                        },
                        None => ()
                    }
                },
                InterruptState::Nop(x) => {
                    self.interrupt_state = InterruptState::PushPc(*x);
                    return;
                },
                InterruptState::PushPc(x) => {
                    self.cpu.push_pc(&mut self.memory);
                    self.interrupt_state = InterruptState::WaitPc(*x);
                    return;
                },
                InterruptState::WaitPc(x) => {
                    self.interrupt_state = InterruptState::LoadVector(*x);
                    return;
                },
                InterruptState::LoadVector(i) => {
                    self.cpu.load_vector(*x);
                    self.interrupt_state = InterruptState::Ready;
                    return;
                }
            }
        }
        //Tick the CPU on each clock, pass exact cycle position into CPU
        self.cpu.tick(&mut self.memory);
        //increment cycle position (4 clock ticks per machine cycle)
        self.cycle = if self.cycle == 3 { 0 } else { self.cycle + 1};
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