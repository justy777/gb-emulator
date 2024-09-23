#[derive(Clone)]
pub struct Cartridge {
    data: Vec<u8>,
    // TODO: implement memory bank controller
}

impl Cartridge {
    #[must_use]
    pub const fn new(data: Vec<u8>) -> Self {
        Self { data }
    }

    pub(crate) fn read_rom(&self, address: u16) -> u8 {
        self.data[address as usize]
    }

    pub(crate) fn read_ram(&self, _address: u16) -> u8 {
        // TODO: implement cartridge RAM
        0
    }

    pub(crate) fn write_ram(&self, _address: u16, _value: u8) {
        // TODO: implement cartridge RAM
    }
}
