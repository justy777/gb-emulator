use crate::interrupts::InterruptFlags;
use crate::util::Delay;

const MEM_DIVIDER_REGISTER: u16 = 0xFF04;
const MEM_TIMER_COUNTER: u16 = 0xFF05;
const MEM_TIMER_MODULO: u16 = 0xFF06;
const MEM_TIMER_CONTROL: u16 = 0xFF07;

#[derive(Debug, Clone, Copy)]
struct TimerControl(u8);

impl TimerControl {
    const ENABLE: u8 = 0b0000_0100;
    const CLOCK_SELECT: u8 = 0b0000_0011;
    const UNUSED: u8 = 0b1111_1000;

    const fn empty() -> Self {
        Self::from_bits(0)
    }

    const fn from_bits(bits: u8) -> Self {
        Self(bits | Self::UNUSED)
    }

    const fn bits(self) -> u8 {
        self.0
    }

    const fn is_enabled(self) -> bool {
        (self.0 & Self::ENABLE) == Self::ENABLE
    }

    fn counter_mask(self) -> u16 {
        match self.0 & Self::CLOCK_SELECT {
            0b00 => 128,
            0b01 => 2,
            0b10 => 8,
            0b11 => 32,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Timer {
    // DIV
    // Note: only uses 14 bits
    system_counter: u16,
    // TIMA
    counter: u8,
    // TMA
    modulo: u8,
    // TAC
    control: TimerControl,
    // Used to check for falling edge
    tick_signal: bool,
    overflow: Delay<bool>,
}

impl Timer {
    pub const fn new() -> Self {
        Self {
            // TODO: between 0x2C and 0x3F
            system_counter: (0xAB << 6) + 0x2C,
            counter: 0,
            modulo: 0,
            control: TimerControl::empty(),
            tick_signal: false,
            overflow: Delay::new(false),
        }
    }

    pub const fn read_byte(&self, address: u16) -> u8 {
        match address {
            #[allow(clippy::cast_possible_truncation)]
            MEM_DIVIDER_REGISTER => (self.system_counter >> 6) as u8,
            MEM_TIMER_COUNTER => self.counter,
            MEM_TIMER_MODULO => self.modulo,
            MEM_TIMER_CONTROL => self.control.bits(),
            _ => unreachable!(),
        }
    }

    pub fn write_byte(&mut self, address: u16, value: u8, interrupt_flag: &mut InterruptFlags) {
        match address {
            MEM_DIVIDER_REGISTER => self.reset(interrupt_flag),
            MEM_TIMER_COUNTER => {
                self.counter = value;
                self.overflow = Delay::new(false);
            }
            MEM_TIMER_MODULO => {
                self.modulo = value;
            }
            MEM_TIMER_CONTROL => {
                self.control = TimerControl::from_bits(value);
                self.tick(interrupt_flag);
            }
            _ => unreachable!(),
        }
    }

    pub fn reset(&mut self, interrupt_flag: &mut InterruptFlags) {
        self.system_counter = 0;
        self.tick(interrupt_flag);
    }

    pub fn increment(&mut self, interrupt_flag: &mut InterruptFlags) {
        self.system_counter = self.system_counter.wrapping_add(1);
        self.tick(interrupt_flag);
    }

    fn tick(&mut self, interrupt_flag: &mut InterruptFlags) {
        if *self.overflow.get_and_advance() {
            self.overflow = Delay::new(false);
            self.counter = self.modulo;
            interrupt_flag.set(InterruptFlags::TIMER, true);
        }

        let new_tick_signal = self.control.is_enabled() && self.counter_bit();
        let falling_edge = self.tick_signal && !new_tick_signal;
        self.tick_signal = new_tick_signal;

        if falling_edge {
            let (counter, overflow) = self.counter.overflowing_add(1);
            self.counter = counter;
            if overflow {
                self.overflow.set_delay(true, 1);
            }
        }
    }

    fn counter_bit(&self) -> bool {
        (self.system_counter & self.control.counter_mask()) != 0
    }
}
