use crate::bits;
use crate::error::TryFromUintError;
use crate::util::bit;
use bitflags::bitflags;

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

#[derive(Debug, Clone, Copy)]
pub struct Timer {
    // DIV
    pub divider: u8,
    // TIMA
    pub counter: u8,
    // TMA
    pub modulo: u8,
    // TAC
    pub control: TimerControl,
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
