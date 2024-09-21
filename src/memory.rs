use bitflags::bitflags;

const MEM_JOYPAD: u16 = 0xFF00;
const MEM_INTERRUPT_FLAG: u16 = 0xFF0F;
const MEM_INTERRUPT_ENABLE: u16 = 0xFFFF;

const MEM_TOTAL_SIZE: usize = 0xFFFF + 1;

bitflags! {
    #[repr(transparent)]
    #[derive(Debug, Clone, Copy)]
    pub(crate) struct InterruptFlags: u8 {
        const VBLANK = 0b0000_0001;
        const LCD = 0b0000_0010;
        const TIMER = 0b0000_0100;
        const SERIAL = 0b0000_1000;
        const JOYPAD = 0b0001_0000;
    }
}

#[derive(Clone)]
pub struct AddressBus {
    memory: [u8; MEM_TOTAL_SIZE],
}

impl AddressBus {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            memory: [0; MEM_TOTAL_SIZE],
        }
    }

    pub(crate) const fn read_byte(&self, address: u16) -> u8 {
        self.memory[address as usize]
    }

    pub(crate) fn write_byte(&mut self, address: u16, value: u8) {
        self.memory[address as usize] = value;
    }

    pub(crate) const fn read_joypad(&self) -> u8 {
        self.read_byte(MEM_JOYPAD)
    }

    pub(crate) const fn read_interrupt_flag(&self) -> InterruptFlags {
        let byte = self.read_byte(MEM_INTERRUPT_FLAG);
        InterruptFlags::from_bits_truncate(byte)
    }

    pub(crate) fn write_interrupt_flag(&mut self, value: InterruptFlags) {
        self.write_byte(MEM_INTERRUPT_FLAG, value.bits());
    }

    pub(crate) const fn read_interrupt_enable(&self) -> InterruptFlags {
        let byte = self.read_byte(MEM_INTERRUPT_ENABLE);
        InterruptFlags::from_bits_truncate(byte)
    }
}

impl Default for AddressBus {
    fn default() -> Self {
        Self::new()
    }
}
