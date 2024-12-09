use crate::bits;
use crate::interrupts::InterruptFlags;
use crate::util::bit;
use bitflags::bitflags;

const MEM_DIVIDER_REGISTER: u16 = 0xFF04;
const MEM_TIMER_COUNTER: u16 = 0xFF05;
const MEM_TIMER_MODULO: u16 = 0xFF06;
const MEM_TIMER_CONTROL: u16 = 0xFF07;

bitflags! {
    #[repr(transparent)]
    #[derive(Debug, Clone, Copy)]
    struct TimerControl: u8 {
        const ENABLE = bit(2);
        const CLOCK_SELECT = bits![0, 1];
    }
}

impl TimerControl {
    pub const fn unknown() -> Self {
        // 0xF8
        Self::from_bits_retain(0b1111_1000)
    }
}

impl TimerControl {
    const fn is_enabled(self) -> bool {
        self.contains(Self::ENABLE)
    }

    fn counter_mask(self) -> u16 {
        match self.bits() & Self::CLOCK_SELECT.bits() {
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
    divider: u16,
    // TIMA
    counter: u8,
    // TMA
    modulo: u8,
    // TAC
    control: TimerControl,
    // Used to check for falling edge
    interrupt_signal: bool,
    // Used to delay overflow until the next cycle
    overflow_delay_counter: Option<u8>,
}

impl Timer {
    pub const fn new() -> Self {
        Self {
            // TODO: between 0x2C and 0x3F
            divider: (0xAB << 6) + 0x2C,
            counter: 0,
            modulo: 0,
            control: TimerControl::unknown(),
            interrupt_signal: false,
            overflow_delay_counter: None,
        }
    }

    pub const fn read_byte(&self, address: u16) -> u8 {
        match address {
            MEM_DIVIDER_REGISTER => (self.divider >> 6) as u8,
            MEM_TIMER_COUNTER => self.counter,
            MEM_TIMER_MODULO => self.modulo,
            MEM_TIMER_CONTROL => self.control.bits(),
            _ => unreachable!(),
        }
    }

    pub fn write_byte(&mut self, address: u16, value: u8) {
        match address {
            MEM_DIVIDER_REGISTER => self.divider = 0,
            MEM_TIMER_COUNTER => {
                self.counter = value;
                self.overflow_delay_counter = None;
            }
            MEM_TIMER_MODULO => self.modulo = value,
            MEM_TIMER_CONTROL => self.control = TimerControl::from_bits_retain(value),
            _ => unreachable!(),
        }
    }

    pub fn tick(&mut self, cycles: usize, interrupt_flag: &mut InterruptFlags) {
        for _ in 0..cycles {
            self.divider = self.divider.wrapping_add(1);

            let new_signal = self.counter_bit() && self.control.is_enabled();

            if self.interrupt_signal && !new_signal {
                if self.counter == 255 {
                    self.counter = 0;
                    self.overflow_delay_counter = Some(2);
                } else {
                    self.counter += 1;
                }
            }
            self.interrupt_signal = new_signal;

            // Checks for next cycle after overflow occurs
            self.overflow_delay_counter = self.overflow_delay_counter.map(|n| n - 1);
            if self.overflow_delay_counter.is_some_and(|n| n == 0) {
                self.counter = self.modulo;
                interrupt_flag.set(InterruptFlags::TIMER, true);
                self.overflow_delay_counter = None;
            }
        }
    }

    fn counter_bit(&self) -> bool {
        (self.divider & self.control.counter_mask()) != 0
    }
}
