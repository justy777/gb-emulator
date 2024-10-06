mod mbc;
mod metadata;

use crate::cartridge::mbc::{MemoryBankController, NoMBC, MBC1, MBC3, MBC5};
use crate::cartridge::metadata::Metadata;
use crate::util::DataSize;

const ROM_BANK_SIZE: DataSize = DataSize::from_kilobytes(16);
const RAM_BANK_SIZE: DataSize = DataSize::from_kilobytes(8);

// TODO: add support for save files
pub struct Cartridge {
    rom: Vec<u8>,
    ram: Option<Vec<u8>>,
    mbc: Box<dyn MemoryBankController>,
    metadata: Metadata,
}

impl Cartridge {
    #[must_use]
    pub fn new(rom: Vec<u8>) -> Self {
        let metadata = Metadata::new(&rom);

        let mbc: Box<dyn MemoryBankController> = match metadata.mbc_number {
            0 => Box::new(NoMBC::new()),
            1 => Box::new(MBC1::new(metadata.rom_bank_count, metadata.rom_bank_count)),
            3 => Box::new(MBC3::new()),
            5 => Box::new(MBC5::new()),
            _ => unreachable!(),
        };

        let ram = if metadata.has_ram {
            let capacity = RAM_BANK_SIZE * metadata.ram_bank_count;
            let vec = Vec::with_capacity(capacity.as_bytes());
            Some(vec)
        } else {
            None
        };

        Self {
            rom,
            ram,
            mbc,
            metadata,
        }
    }

    pub(crate) fn read_rom_bank0(&self, address: u16) -> u8 {
        let offset = ROM_BANK_SIZE * self.mbc.get_rom_bank0();
        self.rom[(address as usize) + offset.as_bytes()]
    }

    pub(crate) fn read_rom_bank1(&self, address: u16) -> u8 {
        let offset = ROM_BANK_SIZE * self.mbc.get_rom_bank1();
        self.rom[(address as usize) + offset.as_bytes()]
    }

    pub(crate) fn write_rom(&mut self, address: u16, value: u8) {
        self.mbc.write_registers(address, value);
    }

    pub(crate) fn read_ram(&self, address: u16) -> u8 {
        if !self.mbc.is_ram_enabled() {
            return 0xFF;
        }

        if let Some(ram) = &self.ram {
            let offset = RAM_BANK_SIZE * self.mbc.get_ram_bank();
            ram[(address as usize) + offset.as_bytes()]
        } else {
            panic!("Unable to read from cartridge RAM. No RAM included in cartridge.");
        }
    }

    pub(crate) fn write_ram(&mut self, address: u16, value: u8) {
        if !self.mbc.is_ram_enabled() {
            return;
        }

        if let Some(ram) = &mut self.ram {
            let offset = RAM_BANK_SIZE * self.mbc.get_ram_bank();
            ram[(address as usize) + offset.as_bytes()] = value;
        } else {
            panic!("Unable to write to cartridge RAM. No RAM included in cartridge.")
        }
    }

    #[must_use]
    pub fn get_title(&self) -> &str {
        &self.metadata.title
    }

    #[must_use]
    pub fn get_rom_size(&self) -> DataSize {
        ROM_BANK_SIZE * self.get_rom_bank_count()
    }

    pub(crate) const fn get_rom_bank_count(&self) -> u32 {
        self.metadata.rom_bank_count
    }

    #[must_use]
    pub fn get_ram_size(&self) -> DataSize {
        RAM_BANK_SIZE * self.get_ram_bank_count()
    }

    pub(crate) const fn get_ram_bank_count(&self) -> u32 {
        self.metadata.ram_bank_count
    }

    #[must_use]
    pub const fn passed_header_check(&self) -> bool {
        self.metadata.passed_header_check
    }

    #[must_use]
    pub const fn passed_global_check(&self) -> bool {
        self.metadata.passed_global_check
    }
}
