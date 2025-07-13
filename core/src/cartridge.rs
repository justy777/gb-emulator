mod mbc;
mod metadata;

use crate::cartridge::mbc::{MBC1, MBC3, MBC5, MemoryBankController, NoMBC};
use crate::cartridge::metadata::{Metadata, MetadataError};

const ROM_BANK_SIZE: usize = 16 * 1024;
const RAM_BANK_SIZE: usize = 8 * 1024;

#[derive(Debug)]
pub enum CartridgeError {
    NotDivisibleIntoBanks,
    Metadata(MetadataError),
}

impl From<MetadataError> for CartridgeError {
    fn from(err: MetadataError) -> Self {
        Self::Metadata(err)
    }
}

impl std::fmt::Display for CartridgeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NotDivisibleIntoBanks => {
                write!(f, "Cartridge is not divisible into 16 KiB banks")
            }
            Self::Metadata(err) => write!(f, "Bad cartridge header: {err}"),
        }
    }
}

impl std::error::Error for CartridgeError {}

// TODO: add support for save files
pub struct Cartridge {
    rom: Vec<u8>,
    ram: Option<Vec<u8>>,
    mbc: Box<dyn MemoryBankController>,
    metadata: Metadata,
}

impl Cartridge {
    pub fn new(rom: Vec<u8>) -> Result<Self, CartridgeError> {
        if rom.is_empty() || rom.len() % ROM_BANK_SIZE != 0 {
            return Err(CartridgeError::NotDivisibleIntoBanks);
        }

        let metadata = Metadata::new(&rom)?;

        let mbc: Box<dyn MemoryBankController> = match metadata.mbc_number {
            0 => Box::new(NoMBC::new()),
            1 => Box::new(MBC1::new(metadata.rom_banks(), metadata.ram_banks())),
            3 => Box::new(MBC3::new(metadata.rom_banks(), metadata.ram_banks())),
            5 => Box::new(MBC5::new(metadata.rom_banks(), metadata.ram_banks())),
            _ => unreachable!(),
        };

        let ram = if metadata.has_ram() && metadata.ram_size() > 0 {
            let capacity = metadata.ram_size();
            let vec = vec![0; capacity];
            Some(vec)
        } else {
            None
        };

        Ok(Self {
            rom,
            ram,
            mbc,
            metadata,
        })
    }

    pub(crate) fn read_rom_bank0(&self, addr: u16) -> u8 {
        let index = (ROM_BANK_SIZE * self.mbc.rom_bank0()) + (addr as usize);
        self.rom[index]
    }

    pub(crate) fn read_rom_bank1(&self, addr: u16) -> u8 {
        let index = (ROM_BANK_SIZE * self.mbc.rom_bank1()) + (addr as usize);
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
            let index = (RAM_BANK_SIZE * self.mbc.ram_bank()) + (addr as usize);
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
            let index = (RAM_BANK_SIZE * self.mbc.ram_bank()) + (addr as usize);
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
