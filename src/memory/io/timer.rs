use crate::bits;
use crate::error::TryFromUintError;
use crate::memory::io::interrupts::InterruptFlags;
use crate::util::bit;
use bitflags::bitflags;

const MEM_DIVIDER_REGISTER: u16 = 0xFF04;
const MEM_TIMER_COUNTER: u16 = 0xFF05;
const MEM_TIMER_MODULO: u16 = 0xFF06;
const MEM_TIMER_CONTROL: u16 = 0xFF07;

bitflags! {
    #[repr(transparent)]
    #[derive(Debug, Clone, Copy)]
    pub struct TimerControl: u8 {
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
    // TODO: implement cycles and ticks
    last_edge: bool,
}

impl Timer {
    pub const fn new() -> Self {
        Self {
            divider: 0,
            counter: 0,
            modulo: 0,
            control: TimerControl::empty(),
            last_edge: false,
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
            MEM_TIMER_COUNTER => self.counter = value,
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
            let bit_set = self.divider & clock_frequency.increment_every()
                == clock_frequency.increment_every();

            let enabled = self.control.contains(TimerControl::ENABLE);
            let current_edge = bit_set && enabled;

            if self.last_edge && !current_edge {
                if self.counter == 255 {
                    self.counter = self.modulo;
                    interrupt_flag.set(InterruptFlags::TIMER, true);
                } else {
                    self.counter += 1;
                }
            }
            self.last_edge = current_edge;
        }
    }
}

enum ClockFrequency {
    Zero,
    One,
    Two,
    Three,
}

impl ClockFrequency {
    const fn increment_every(&self) -> u16 {
        match self {
            Self::Zero => 512,
            Self::One => 8,
            Self::Two => 32,
            Self::Three => 128,
        }
    }
}

impl From<ClockFrequency> for u8 {
    fn from(clock: ClockFrequency) -> Self {
        match clock {
            ClockFrequency::Zero => 0b00,
            ClockFrequency::One => 0b01,
            ClockFrequency::Two => 0b10,
            ClockFrequency::Three => 0b11,
        }
    }
}

impl TryFrom<u8> for ClockFrequency {
    type Error = TryFromUintError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0b00 => Ok(Self::Zero),
            0b01 => Ok(Self::One),
            0b10 => Ok(Self::Two),
            0b11 => Ok(Self::Three),
            _ => Err(TryFromUintError(())),
        }
    }
}
