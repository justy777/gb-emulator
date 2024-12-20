mod mbc;
mod metadata;

use crate::cartridge::mbc::{MemoryBankController, NoMBC, MBC1, MBC3, MBC5};
use crate::cartridge::metadata::Metadata;

const ROM_BANK_SIZE: usize = 16 * 1024;
const RAM_BANK_SIZE: usize = 8 * 1024;

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
            let vec = vec![0; capacity];
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

    pub(crate) fn read_rom_bank0(&self, addr: u16) -> u8 {
        let offset = ROM_BANK_SIZE * self.mbc.get_rom_bank0();
        self.rom[(addr as usize) + offset]
    }

    pub(crate) fn read_rom_bank1(&self, addr: u16) -> u8 {
        let offset = ROM_BANK_SIZE * self.mbc.get_rom_bank1();
        self.rom[(addr as usize) + offset]
    }

    pub(crate) fn write_rom(&mut self, addr: u16, value: u8) {
        self.mbc.write_registers(addr, value);
    }

    pub(crate) fn read_ram(&self, addr: u16) -> u8 {
        if !self.mbc.is_ram_enabled() {
            return 0xFF;
        }

        if let Some(ram) = &self.ram {
            let offset = RAM_BANK_SIZE * self.mbc.get_ram_bank();
            ram[(addr as usize) + offset]
        } else {
            panic!("Unable to read from cartridge RAM. No RAM included in cartridge.");
        }
    }

    pub(crate) fn write_ram(&mut self, addr: u16, value: u8) {
        if !self.mbc.is_ram_enabled() {
            return;
        }

        if let Some(ram) = &mut self.ram {
            let offset = RAM_BANK_SIZE * self.mbc.get_ram_bank();
            ram[(addr as usize) + offset] = value;
        } else {
            panic!("Unable to write to cartridge RAM. No RAM included in cartridge.")
        }
    }

    #[must_use]
    pub fn get_title(&self) -> &str {
        &self.metadata.title
    }

    #[must_use]
    pub const fn get_rom_size(&self) -> usize {
        ROM_BANK_SIZE * self.get_rom_bank_count()
    }

    pub(crate) const fn get_rom_bank_count(&self) -> usize {
        self.metadata.rom_bank_count
    }

    #[must_use]
    pub const fn get_ram_size(&self) -> usize {
        RAM_BANK_SIZE * self.get_ram_bank_count()
    }

    pub(crate) const fn get_ram_bank_count(&self) -> usize {
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
