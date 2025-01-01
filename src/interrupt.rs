use std::slice::Iter;

const PC_VBLANK_HANDLER: u16 = 0x40;
const PC_STAT_HANDLER: u16 = 0x48;
const PC_TIMER_HANDLER: u16 = 0x50;
const PC_SERIAL_HANDLER: u16 = 0x58;
const PC_JOYPAD_HANDLER: u16 = 0x60;

#[derive(Debug, Copy, Clone)]
pub enum Interrupt {
    VBlank = 0b0000_0001,
    Stat = 0b0000_0010,
    Timer = 0b0000_0100,
    Serial = 0b0000_1000,
    Joypad = 0b0001_0000,
}

impl Interrupt {
    pub fn iter() -> Iter<'static, Self> {
        // Ordered from highest to lowest priority
        [
            Self::VBlank,
            Self::Stat,
            Self::Timer,
            Self::Serial,
            Self::Joypad,
        ]
        .iter()
    }

    pub const fn handler_addr(self) -> u16 {
        match self {
            Self::VBlank => PC_VBLANK_HANDLER,
            Self::Stat => PC_STAT_HANDLER,
            Self::Timer => PC_TIMER_HANDLER,
            Self::Serial => PC_SERIAL_HANDLER,
            Self::Joypad => PC_JOYPAD_HANDLER,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct InterruptFlags(u8);

impl InterruptFlags {
    const UNUSED: u8 = 0b1110_0000;

    pub const fn from_bits(bits: u8) -> Self {
        Self(bits | Self::UNUSED)
    }

    pub const fn from_interrupt(interrupt: Interrupt) -> Self {
        Self::from_bits(interrupt as u8)
    }

    pub const fn bits(self) -> u8 {
        self.0
    }

    pub fn set(&mut self, interrupt: Interrupt, enable: bool) {
        let bits = interrupt as u8;
        if enable {
            self.0 |= bits;
        } else {
            self.0 &= !bits;
        }
        self.0 |= Self::UNUSED;
    }

    pub const fn contains(self, interrupt: Interrupt) -> bool {
        let bits = interrupt as u8;
        (self.0 & bits) == bits
    }
}

#[derive(Debug, Copy, Clone)]
pub struct InterruptEnable(u8);

impl InterruptEnable {
    pub const fn empty() -> Self {
        Self::from_bits(0)
    }

    pub const fn from_bits(bits: u8) -> Self {
        Self(bits)
    }

    pub const fn bits(self) -> u8 {
        self.0
    }

    pub const fn contains(self, interrupt: Interrupt) -> bool {
        let bits = interrupt as u8;
        (self.0 & bits) == bits
    }
}
