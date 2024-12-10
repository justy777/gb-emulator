use std::ops::BitAnd;

const PC_VBLANK_HANDLER: u16 = 0x40;
const PC_STAT_HANDLER: u16 = 0x48;
const PC_TIMER_HANDLER: u16 = 0x50;
const PC_SERIAL_HANDLER: u16 = 0x58;
const PC_JOYPAD_HANDLER: u16 = 0x60;

#[derive(Debug, Clone, Copy)]
pub struct InterruptFlags(u8);

impl InterruptFlags {
    pub const VBLANK: u8 = 0b0000_0001;
    pub const STAT: u8 = 0b0000_0010;
    pub const TIMER: u8 = 0b0000_0100;
    pub const SERIAL: u8 = 0b0000_1000;
    pub const JOYPAD: u8 = 0b0001_0000;
    const UNUSED: u8 = 0b1110_0000;

    pub const fn empty() -> Self {
        Self::from_bits(0)
    }

    pub const fn from_bits(bits: u8) -> Self {
        Self(bits | Self::UNUSED)
    }

    pub const fn flags() -> [Self; 5] {
        // Ordered from highest to lowest priority
        [
            Self::from_bits(Self::VBLANK),
            Self::from_bits(Self::STAT),
            Self::from_bits(Self::TIMER),
            Self::from_bits(Self::SERIAL),
            Self::from_bits(Self::JOYPAD),
        ]
    }

    pub const fn bits(self) -> u8 {
        self.0
    }

    pub fn set(&mut self, bits: u8, enable: bool) {
        if enable {
            self.0 |= bits;
        } else {
            self.0 &= !bits;
        }
        self.0 |= Self::UNUSED;
    }

    pub const fn contains(self, bits: u8) -> bool {
        (self.0 & bits) == bits
    }

    pub(crate) fn handler(self) -> u16 {
        match self.0 {
            Self::VBLANK => PC_VBLANK_HANDLER,
            Self::STAT => PC_STAT_HANDLER,
            Self::TIMER => PC_TIMER_HANDLER,
            Self::SERIAL => PC_SERIAL_HANDLER,
            Self::JOYPAD => PC_JOYPAD_HANDLER,
            _ => unreachable!(),
        }
    }
}

impl BitAnd for InterruptFlags {
    type Output = Self;

    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
