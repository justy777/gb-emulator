use crate::util::bit;
use bitflags::bitflags;

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
