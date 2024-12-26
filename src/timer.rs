use crate::interrupts::{Interrupt, InterruptFlags};

const MEM_DIV: u16 = 0xFF04;
const MEM_TIMA: u16 = 0xFF05;
const MEM_TMA: u16 = 0xFF06;
const MEM_TAC: u16 = 0xFF07;

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
    interrupt_signal: bool,
    // Used to delay overflow until the next cycle
    overflow_delay_counter: Option<u8>,
}

impl Timer {
    pub const fn new() -> Self {
        Self {
            // TODO: between 0x2C and 0x3F
            system_counter: (0xAB << 6) + 0x2C,
            counter: 0,
            modulo: 0,
            control: TimerControl::empty(),
            interrupt_signal: false,
            overflow_delay_counter: None,
        }
    }

    pub const fn read_byte(&self, addr: u16) -> u8 {
        match addr {
            #[allow(clippy::cast_possible_truncation)]
            MEM_DIV => (self.system_counter >> 6) as u8,
            MEM_TIMA => self.counter,
            MEM_TMA => self.modulo,
            MEM_TAC => self.control.bits(),
            _ => unreachable!(),
        }
    }

    pub fn write_byte(&mut self, addr: u16, value: u8) {
        match addr {
            MEM_DIV => self.system_counter = 0,
            MEM_TIMA => {
                self.counter = value;
                self.overflow_delay_counter = None;
            }
            MEM_TMA => self.modulo = value,
            MEM_TAC => self.control = TimerControl::from_bits(value),
            _ => unreachable!(),
        }
    }

    pub fn tick(&mut self, interrupt_flags: &mut InterruptFlags) {
        self.system_counter = self.system_counter.wrapping_add(1);

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
            interrupt_flags.set(Interrupt::Timer, true);
            self.overflow_delay_counter = None;
        }
    }

    fn counter_bit(&self) -> bool {
        (self.system_counter & self.control.counter_mask()) != 0
    }
}
