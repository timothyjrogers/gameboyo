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
    timer_state: TimerState,
    interrupt_state: InterruptState,
    control_state: ControlState,
    cycle: u32,
}

enum TimerState {
    Nil,
    InterruptReady,
}

enum InterruptState {
    Nil,
    InProgress((Interrupt, u32)),
}

pub enum Platform {
    DMG,
    GBC
}

enum ControlState {
    Ready,
    Cpu,
    Isr,
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
            control_state: ControlState::Ready,
            cycle: 0,
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
        //Tick the system internal timer (and thereby DIV). If TIMA overflows, set IF for timer overflow
        match &self.timer_state {
            TimerState::InterruptReady => {
                if self.cycle == 0 {
                    self.cpu.enable_interrupt(Interrupt::Timer);
                    self.timer.set_tima();
                    self.timer_state = TimerState::Nil;
                }
            },
            _ => ()
        }
        if self.timer.tick() { self.timer_state = TimerState::InterruptReady }
        //TODO -- Check Joypad. State to be passed in from Iced

        //check interrupts, transfer control via ISR if necessary
        if self.cycle == 0 {
            match &self.interrupt_state {
                InterruptState::Nil => {
                    match self.cpu.get_interrupt() {
                        Some(x) => {
                            self.interrupt_state = InterruptState::InProgress((*x, 0));
                            self.cycle = if self.cycle == 3 { 0 } else { self.cycle + 1};
                            return;
                        },
                        None => ()
                    }
                },
                InterruptState::InProgress(c) => {
                    if *c.1 < 8 || (*c.1 > 8 && *c.1 < 16) || (*c.1 > 16 && *c.1 < 20) {
                        self.interrupt_state = (*c.0, *c.1 + 1);
                    } else if *c.1 == 8 {
                        self.cpu.setup_interrupts(&mut self.memory);
                        self.interrupt_state = (*c.0, *c.1 + 1);
                    } else if *c.1 == 16 {
                        match *c.0 {
                            Interrupt::VerticalBlanking => self.cpu.set_pc(constants::INT_VBL),
                            Interrupt::LcdStat => self.cpu.set_pc(constants::INT_STAT),
                            Interrupt::Timer => self.cpu.set_pc(constants::INT_TIMER),
                            Interrupt::Serial => self.cpu.set_pc(constants::INT_SERIAL),
                            Interrupt::Joypad => self.cpu.set_pc(constants::INT_JOYPAD),
                        }
                        self.interrupt_state = (*c.0, *c.1 + 1);
                    } else {
                        self.interrupt_state = InterruptState::Nil;
                    }
                    self.cycle = if self.cycle == 3 { 0 } else { self.cycle + 1};
                    return;
                }
            }
        }
        //Tick the CPU on each clock, pass exact cycle position into CPU
        //self.cpu.tick(self.cycle)
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