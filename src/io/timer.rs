use crate::bits;
use crate::error::TryFromUintError;
use crate::util::bit;
use bitflags::bitflags;

const MEM_DIVIDER_REGISTER: u16 = 0xFF04;
const MEM_TIMER_COUNTER: u16 = 0xFF05;
const MEM_TIMER_MODULO: u16 = 0xFF06;
const MEM_TIMER_CONTROL: u16 = 0xFF07;

// Measured in Hz
const CLOCK_SPEED: u32 = 4_194_304;

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
    divider: u8,
    // TIMA
    counter: u8,
    // TMA
    modulo: u8,
    // TAC
    control: TimerControl,
    // TODO: implement cycles and ticks
}

impl Timer {
    pub const fn new() -> Self {
        Self {
            divider: 0,
            counter: 0,
            modulo: 0,
            control: TimerControl::empty(),
        }
    }

    pub const fn read_byte(&self, address: u16) -> u8 {
        match address {
            MEM_DIVIDER_REGISTER => self.divider,
            MEM_TIMER_COUNTER => self.counter,
            MEM_TIMER_MODULO => self.modulo,
            MEM_TIMER_CONTROL => self.control.bits(),
            _ => unreachable!(),
        }
    }

    pub fn write_byte(&mut self, address: u16, value: u8) {
        match address {
            MEM_DIVIDER_REGISTER => self.divider = value,
            MEM_TIMER_COUNTER => self.counter = value,
            MEM_TIMER_MODULO => self.modulo = value,
            MEM_TIMER_CONTROL => self.control = TimerControl::from_bits_truncate(value),
            _ => unreachable!(),
        }
    }
}

enum Clock {
    Zero,
    One,
    Two,
    Three,
}

impl Clock {
    const fn increment_every(&self) -> u32 {
        match self {
            Self::Zero => 256,
            Self::One => 4,
            Self::Two => 16,
            Self::Three => 64,
        }
    }

    const fn frequency(&self) -> u32 {
        CLOCK_SPEED / (self.increment_every() * 4)
    }
}

impl From<Clock> for u8 {
    fn from(clock: Clock) -> Self {
        match clock {
            Clock::Zero => 0b00,
            Clock::One => 0b01,
            Clock::Two => 0b10,
            Clock::Three => 0b11,
        }
    }
}

impl TryFrom<u8> for Clock {
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
