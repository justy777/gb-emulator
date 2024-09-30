#[derive(Debug, Clone)]
pub struct Audio {
    // TODO: replace stubs with real data structure
    data: [u8; 23],
}

impl Audio {
    pub const fn new() -> Self {
        Self { data: [0; 23] }
    }

    pub const fn read_byte(&self, address: u16) -> u8 {
        let index = (address - 0xFF10) as usize;
        self.data[index]
    }

    pub fn write_byte(&mut self, address: u16, value: u8) {
        let index = (address - 0xFF10) as usize;
        self.data[index] = value;
    }
}