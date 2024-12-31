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
            1 => Box::new(MBC1::new(
                metadata.get_rom_banks(),
                metadata.get_ram_banks(),
            )),
            3 => Box::new(MBC3::new(
                metadata.get_rom_banks(),
                metadata.get_ram_banks(),
            )),
            5 => Box::new(MBC5::new(
                metadata.get_rom_banks(),
                metadata.get_ram_banks(),
            )),
            _ => unreachable!(),
        };

        let ram = if metadata.has_ram() {
            let capacity = RAM_BANK_SIZE * metadata.get_ram_banks();
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
        let index = (ROM_BANK_SIZE * self.mbc.get_rom_bank0()) + (addr as usize);
        self.rom[index]
    }

    pub(crate) fn read_rom_bank1(&self, addr: u16) -> u8 {
        let index = (ROM_BANK_SIZE * self.mbc.get_rom_bank1()) + (addr as usize);
        self.rom[index]
    }

    pub(crate) fn write_mbc_register(&mut self, addr: u16, value: u8) {
        self.mbc.write_register(addr, value);
    }

    pub(crate) fn read_ram_bank(&self, addr: u16) -> u8 {
        if !self.mbc.is_ram_enabled() {
            return 0xFF;
        }

        if let Some(ram) = &self.ram {
            let index = (RAM_BANK_SIZE * self.mbc.get_ram_bank()) + (addr as usize);
            ram[index]
        } else {
            panic!("Unable to read from cartridge RAM. No RAM included in cartridge.");
        }
    }

    pub(crate) fn write_ram_bank(&mut self, addr: u16, value: u8) {
        if !self.mbc.is_ram_enabled() {
            return;
        }

        if let Some(ram) = &mut self.ram {
            let index = (RAM_BANK_SIZE * self.mbc.get_ram_bank()) + (addr as usize);
            ram[index] = value;
        } else {
            panic!("Unable to write to cartridge RAM. No RAM included in cartridge.")
        }
    }

    #[must_use]
    pub const fn metadata(&self) -> &Metadata {
        &self.metadata
    }
}
