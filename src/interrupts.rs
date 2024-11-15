use crate::util::bit;
use bitflags::bitflags;

const PC_VBLANK_HANDLER: u16 = 0x40;
const PC_STAT_HANDLER: u16 = 0x48;
const PC_TIMER_HANDLER: u16 = 0x50;
const PC_SERIAL_HANDLER: u16 = 0x58;
const PC_JOYPAD_HANDLER: u16 = 0x60;

bitflags! {
    #[repr(transparent)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct InterruptFlags: u8 {
        const VBLANK = bit(0);
        const STAT = bit(1);
        const TIMER = bit(2);
        const SERIAL = bit(3);
        const JOYPAD = bit(4);
    }
}

impl InterruptFlags {
    pub(crate) fn handler(self) -> u16 {
        match self {
            InterruptFlags::VBLANK => PC_VBLANK_HANDLER,
            InterruptFlags::STAT => PC_STAT_HANDLER,
            InterruptFlags::TIMER => PC_TIMER_HANDLER,
            InterruptFlags::SERIAL => PC_SERIAL_HANDLER,
            InterruptFlags::JOYPAD => PC_JOYPAD_HANDLER,
            _ => unreachable!(),
        }
    }
}
