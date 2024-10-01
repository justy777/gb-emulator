use crate::util::DataSize;

const ROM_BANK_SIZE: DataSize = DataSize::from_kilobytes(16);
const RAM_BANK_SIZE: DataSize = DataSize::from_kilobytes(8);

const CART_ROM_SIZE: usize = 0x148;
const CART_RAM_SIZE: usize = 0x149;
const CART_HEADER_CHECKSUM: usize = 0x14D;
const CART_GLOBAL_CHECKSUM1: usize = 0x14E;
const CART_GLOBAL_CHECKSUM2: usize = 0x14F;

#[derive(Clone)]
pub struct Cartridge {
    rom: Vec<u8>,
    // TODO: implement memory bank controller
}

impl Cartridge {
    #[must_use]
    pub const fn new(rom: Vec<u8>) -> Self {
        Self { rom }
    }

    pub(crate) fn read_rom(&self, address: u16) -> u8 {
        self.rom[address as usize]
    }

    pub(crate) fn read_ram(&self, _address: u16) -> u8 {
        // TODO: implement cartridge RAM
        0
    }

    pub(crate) fn write_ram(&self, _address: u16, _value: u8) {
        // TODO: implement cartridge RAM
    }

    pub fn get_title(&self) -> String {
        self.rom[0x134..=0x143]
            .iter()
            .map(|byte| char::from(*byte))
            .filter(char::is_ascii)
            .collect()
    }

    #[must_use]
    pub fn get_rom_size(&self) -> DataSize {
        ROM_BANK_SIZE * self.get_rom_bank_count()
    }

    pub(crate) fn get_rom_bank_count(&self) -> u32 {
        let value = self.rom[CART_ROM_SIZE];
        2_u32.pow((value + 1) as u32)
    }

    #[must_use]
    pub fn get_ram_size(&self) -> DataSize {
        RAM_BANK_SIZE * self.get_ram_bank_count()
    }

    pub(crate) fn get_ram_bank_count(&self) -> u32 {
        let value = self.rom[CART_RAM_SIZE];
        match value {
            0x00 => 0,
            0x02 => 1,
            0x03 => 4,
            0x04 => 16,
            0x05 => 8,
            _ => panic!("RAM size value not mapped {value:#02X}"),
        }
    }

    #[must_use]
    pub fn get_header_checksum(&self) -> u8 {
        self.rom[CART_HEADER_CHECKSUM]
    }

    #[must_use]
    pub fn calculate_header_checksum(&self) -> u8 {
        let mut checksum: u8 = 0;
        for byte in &self.rom[0x0134..=0x14C] {
            checksum = checksum.wrapping_sub(*byte).wrapping_sub(1);
        }
        checksum
    }

    #[must_use]
    pub fn get_global_checksum(&self) -> u16 {
        u16::from_be_bytes([
            self.rom[CART_GLOBAL_CHECKSUM1],
            self.rom[CART_GLOBAL_CHECKSUM2],
        ])
    }

    #[must_use]
    pub fn calculate_global_checksum(&self) -> u16 {
        let mut checksum: u16 = 0;
        for (address, byte) in self.rom.iter().enumerate() {
            if address != CART_GLOBAL_CHECKSUM1 && address != CART_GLOBAL_CHECKSUM2 {
                checksum = checksum.wrapping_add(*byte as u16);
            }
        }
        checksum
    }
}
