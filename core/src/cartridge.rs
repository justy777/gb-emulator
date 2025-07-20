mod mbc;
mod metadata;

use crate::cartridge::mbc::{MBC1, MBC2, MBC3, MBC5, MemoryBankController, NoMBC};
use crate::cartridge::metadata::{Metadata, MetadataError};

const ROM_BANK_SIZE: usize = 16 * 1024;
const RAM_BANK_SIZE: usize = 8 * 1024;

#[derive(Debug)]
pub enum CartridgeError {
    TooSmall,
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
            Self::TooSmall => {
                write!(f, "Cartridge is too small; should be at least 32 KiB")
            }
            Self::Metadata(err) => write!(f, "Bad cartridge header: {err}"),
        }
    }
}

impl std::error::Error for CartridgeError {}

// TODO: add support for save files
pub struct Cartridge {
    mbc: Box<dyn MemoryBankController>,
    metadata: Metadata,
}

impl Cartridge {
    pub fn new(mut rom: Vec<u8>) -> Result<Self, CartridgeError> {
        if rom.len() < (2 * ROM_BANK_SIZE) {
            return Err(CartridgeError::TooSmall);
        }

        if !(rom.len() / ROM_BANK_SIZE).is_power_of_two() {
            // Resize rom to proper size
            let len = (rom.len() / ROM_BANK_SIZE).next_power_of_two() * ROM_BANK_SIZE;
            rom.resize(len, 0xFF);
        }

        let metadata = Metadata::new(&rom)?;

        let ram = if metadata.has_ram() && metadata.ram_size() > 0 {
            let capacity = metadata.ram_size();
            let vec = vec![0; capacity];
            Some(vec)
        } else {
            None
        };

        let mbc: Box<dyn MemoryBankController> = match metadata.mbc_number {
            0 => Box::new(NoMBC::new(rom, ram)),
            1 => Box::new(MBC1::new(
                rom,
                ram,
                metadata.rom_banks(),
                metadata.ram_banks(),
            )),
            2 => Box::new(MBC2::new(rom, metadata.rom_banks())),
            3 => Box::new(MBC3::new(
                rom,
                ram,
                metadata.rom_banks(),
                metadata.ram_banks(),
            )),
            5 => Box::new(MBC5::new(
                rom,
                ram,
                metadata.rom_banks(),
                metadata.ram_banks(),
            )),
            _ => unreachable!(),
        };

        Ok(Self { mbc, metadata })
    }

    pub(crate) fn read_rom_bank0(&self, addr: u16) -> u8 {
        self.mbc.read_rom_bank0(addr)
    }

    pub(crate) fn read_rom_bank1(&self, addr: u16) -> u8 {
        self.mbc.read_rom_bank1(addr)
    }

    pub(crate) fn write_mbc_register(&mut self, addr: u16, value: u8) {
        self.mbc.write_register(addr, value);
    }

    pub(crate) fn read_ram_bank(&self, addr: u16) -> u8 {
        self.mbc.read_ram_bank(addr)
    }

    pub(crate) fn write_ram_bank(&mut self, addr: u16, value: u8) {
        self.mbc.write_ram_bank(addr, value);
    }

    #[must_use]
    pub const fn metadata(&self) -> &Metadata {
        &self.metadata
    }
}
