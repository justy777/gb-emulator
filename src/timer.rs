use crate::bits;
use crate::error::TryFromUintError;
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

#[derive(Debug, Clone)]
pub struct Timer {
    // DIV
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
            divider: 0,
            counter: 0,
            modulo: 0,
            control: TimerControl::empty(),
            interrupt_signal: false,
            overflow_delay_counter: None,
        }
    }

    pub const fn read_byte(&self, address: u16) -> u8 {
        match address {
            MEM_DIVIDER_REGISTER => (self.divider >> 8) as u8,
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
            MEM_TIMER_CONTROL => self.control = TimerControl::from_bits_truncate(value),
            _ => unreachable!(),
        }
    }

    pub fn tick(&mut self, cycles: usize, interrupt_flag: &mut InterruptFlags) {
        for _ in 0..cycles {
            self.divider = self.divider.wrapping_add(1);

            let clock_select = self.control.intersection(TimerControl::CLOCK_SELECT);
            let clock_frequency = ClockFrequency::try_from(clock_select.bits()).unwrap();
            let divider_mask = clock_frequency.divider_mask();
            let bit_set = self.divider & divider_mask == divider_mask;

            let enabled = self.control.contains(TimerControl::ENABLE);
            let new_signal = bit_set && enabled;

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
}

enum ClockFrequency {
    // 256 M-cycles
    Zero = 0b00,
    // 4 M-cycles
    One = 0b01,
    // 16 M-cycles
    Two = 0b10,
    // 64 M-cycles
    Three = 0b11,
}

impl ClockFrequency {
    const fn divider_mask(&self) -> u16 {
        let increment_every = match self {
            Self::Zero => 256,
            Self::One => 4,
            Self::Two => 16,
            Self::Three => 64,
        };
        increment_every * 4 / 2
    }
}

impl From<ClockFrequency> for u8 {
    fn from(clock: ClockFrequency) -> Self {
        clock as Self
    }
}

impl TryFrom<u8> for ClockFrequency {
    type Error = TryFromUintError;
    fn try_from(byte: u8) -> Result<Self, Self::Error> {
        match byte {
            0b00 => Ok(Self::Zero),
            0b01 => Ok(Self::One),
            0b10 => Ok(Self::Two),
            0b11 => Ok(Self::Three),
            _ => Err(TryFromUintError(())),
        }
    }
}
