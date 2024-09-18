#[derive(Clone)]
pub struct MemoryBus {
    memory: [u8; 0xFFFF + 1],
}

impl MemoryBus {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            memory: [0; 0xFFFF + 1],
        }
    }

    pub(crate) const fn read_byte(&self, address: u16) -> u8 {
        self.memory[address as usize]
    }

    pub(crate) fn write_byte(&mut self, address: u16, value: u8) {
        self.memory[address as usize] = value;
    }
}

impl Default for MemoryBus {
    fn default() -> Self {
        Self::new()
    }
}
