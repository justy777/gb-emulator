use crate::cartridge::{RAM_BANK_SIZE, ROM_BANK_SIZE};
use std::cmp::max;

pub trait MemoryBankController {
    fn read_rom_bank0(&self, addr: u16) -> u8;
    fn read_rom_bank1(&self, addr: u16) -> u8;
    fn write_register(&mut self, addr: u16, value: u8);
    fn read_ram_bank(&self, addr: u16) -> u8;
    fn write_ram_bank(&mut self, addr: u16, value: u8);
}

pub struct NoMBC {
    rom: Vec<u8>,
    ram: Option<Vec<u8>>,
}

impl NoMBC {
    pub const fn new(rom: Vec<u8>, ram: Option<Vec<u8>>) -> Self {
        Self { rom, ram }
    }
}

impl MemoryBankController for NoMBC {
    fn read_rom_bank0(&self, addr: u16) -> u8 {
        let index = addr as usize;
        self.rom[index]
    }

    fn read_rom_bank1(&self, addr: u16) -> u8 {
        let index = ROM_BANK_SIZE + (addr as usize);
        self.rom[index]
    }

    fn write_register(&mut self, _addr: u16, _value: u8) {
        panic!("Cannot write to Read-Only Memory (ROM) on cartridge.");
    }

    fn read_ram_bank(&self, addr: u16) -> u8 {
        if let Some(ram) = &self.ram {
            let index = addr as usize;
            ram[index]
        } else {
            panic!("Unable to read from cartridge RAM. No RAM included in cartridge.");
        }
    }

    fn write_ram_bank(&mut self, addr: u16, value: u8) {
        if let Some(ram) = &mut self.ram {
            let index = addr as usize;
            ram[index] = value;
        } else {
            panic!("Unable to write to cartridge RAM. No RAM included in cartridge.")
        }
    }
}

pub struct MBC1 {
    rom: Vec<u8>,
    ram: Option<Vec<u8>>,
    rom_banks: usize,
    ram_banks: usize,
    ram_enabled: bool,
    // 5 bits used
    rom_bank_number: u8,
    // 2 bits used
    ram_bank_number: u8,
    banking_mode: bool,
}

impl MBC1 {
    pub const fn new(
        rom: Vec<u8>,
        ram: Option<Vec<u8>>,
        rom_banks: usize,
        ram_banks: usize,
    ) -> Self {
        Self {
            rom,
            ram,
            rom_banks,
            ram_banks,
            ram_enabled: false,
            rom_bank_number: 1,
            ram_bank_number: 0,
            banking_mode: false,
        }
    }
}

impl MemoryBankController for MBC1 {
    fn read_rom_bank0(&self, addr: u16) -> u8 {
        let bank = if self.banking_mode {
            self.ram_bank_number << 5
        } else {
            0
        };
        let bank = truncate_bank(bank as usize, self.rom_banks);

        let index = (ROM_BANK_SIZE * bank) + (addr as usize);
        self.rom[index]
    }

    fn read_rom_bank1(&self, addr: u16) -> u8 {
        let bank = (self.ram_bank_number << 5) | max(self.rom_bank_number, 1);
        let bank = truncate_bank(bank as usize, self.rom_banks);

        let index = (ROM_BANK_SIZE * bank) + (addr as usize);
        self.rom[index]
    }

    fn write_register(&mut self, addr: u16, value: u8) {
        match addr {
            0x0000..=0x1FFF => self.ram_enabled = value & 0xF == 0xA,
            0x2000..=0x3FFF => self.rom_bank_number = value & 0x1F,
            0x4000..=0x5FFF => self.ram_bank_number = value & 0x3,
            0x6000..=0x7FFF => self.banking_mode = value & 0x1 == 0x1,
            _ => {}
        }
    }

    fn read_ram_bank(&self, addr: u16) -> u8 {
        if !self.ram_enabled {
            return 0xFF;
        }

        let bank = if self.banking_mode {
            self.ram_bank_number
        } else {
            0
        };
        let bank = truncate_bank(bank as usize, self.ram_banks);

        if let Some(ram) = &self.ram {
            let index = (RAM_BANK_SIZE * bank) + (addr as usize);
            ram[index]
        } else {
            panic!("Unable to read from cartridge RAM. No RAM included in cartridge.");
        }
    }

    fn write_ram_bank(&mut self, addr: u16, value: u8) {
        if !self.ram_enabled {
            return;
        }

        let bank = if self.banking_mode {
            self.ram_bank_number
        } else {
            0
        };
        let bank = truncate_bank(bank as usize, self.ram_banks);

        if let Some(ram) = &mut self.ram {
            let index = (RAM_BANK_SIZE * bank) + (addr as usize);
            ram[index] = value;
        } else {
            panic!("Unable to write to cartridge RAM. No RAM included in cartridge.")
        }
    }
}

pub struct MBC2 {
    rom: Vec<u8>,
    ram: Vec<u8>,
    rom_banks: usize,
    ram_enabled: bool,
    rom_bank_number: u8,
}

impl MBC2 {
    const RAM_SIZE: usize = 512;

    pub fn new(rom: Vec<u8>, rom_banks: usize) -> Self {
        Self {
            rom,
            ram: vec![0xFF; Self::RAM_SIZE],
            rom_banks,
            ram_enabled: false,
            rom_bank_number: 1,
        }
    }
}

impl MemoryBankController for MBC2 {
    fn read_rom_bank0(&self, addr: u16) -> u8 {
        let index = addr as usize;
        self.rom[index]
    }

    fn read_rom_bank1(&self, addr: u16) -> u8 {
        let bank = max(self.rom_bank_number, 1);
        let bank = truncate_bank(bank as usize, self.rom_banks);

        let index = (ROM_BANK_SIZE * bank) + (addr as usize);
        self.rom[index]
    }

    fn write_register(&mut self, addr: u16, value: u8) {
        if let 0x0000..=0x3FFF = addr {
            if addr & 0x0100 == 0 {
                self.ram_enabled = value & 0x0F == 0x0A;
            } else {
                self.rom_bank_number = value & 0x0F;
            }
        }
    }

    fn read_ram_bank(&self, addr: u16) -> u8 {
        if !self.ram_enabled {
            return 0xFF;
        }

        let index = addr as usize % Self::RAM_SIZE;
        self.ram[index]
    }

    fn write_ram_bank(&mut self, addr: u16, value: u8) {
        if !self.ram_enabled {
            return;
        }

        let index = addr as usize % Self::RAM_SIZE;
        self.ram[index] = value | 0xF0;
    }
}

// TODO: add real-time clock (RTC) support
pub struct MBC3 {
    rom: Vec<u8>,
    ram: Option<Vec<u8>>,
    rom_banks: usize,
    ram_banks: usize,
    ram_enabled: bool,
    // 7 bits used
    rom_bank_number: u8,
    // 2 bits used
    ram_bank_number: u8,
}

impl MBC3 {
    pub const fn new(
        rom: Vec<u8>,
        ram: Option<Vec<u8>>,
        ram_banks: usize,
        rom_banks: usize,
    ) -> Self {
        Self {
            rom,
            ram,
            rom_banks,
            ram_banks,
            ram_enabled: false,
            rom_bank_number: 1,
            ram_bank_number: 0,
        }
    }
}

impl MemoryBankController for MBC3 {
    fn read_rom_bank0(&self, addr: u16) -> u8 {
        let index = addr as usize;
        self.rom[index]
    }

    fn read_rom_bank1(&self, addr: u16) -> u8 {
        let bank = max(self.rom_bank_number, 1);
        let bank = truncate_bank(bank as usize, self.rom_banks);

        let index = (ROM_BANK_SIZE * bank) + (addr as usize);
        self.rom[index]
    }

    fn write_register(&mut self, addr: u16, value: u8) {
        match addr {
            0x0000..=0x1FFF => self.ram_enabled = value & 0xF == 0xA,
            0x2000..=0x3FFF => self.rom_bank_number = value & 0x7F,
            0x4000..=0x5FFF => self.ram_bank_number = value & 0x3,
            _ => {}
        }
    }

    fn read_ram_bank(&self, addr: u16) -> u8 {
        if !self.ram_enabled {
            return 0xFF;
        }

        let bank = truncate_bank(self.ram_bank_number as usize, self.ram_banks);

        if let Some(ram) = &self.ram {
            let index = (RAM_BANK_SIZE * bank) + (addr as usize);
            ram[index]
        } else {
            panic!("Unable to read from cartridge RAM. No RAM included in cartridge.");
        }
    }

    fn write_ram_bank(&mut self, addr: u16, value: u8) {
        if !self.ram_enabled {
            return;
        }

        let bank = truncate_bank(self.ram_bank_number as usize, self.ram_banks);

        if let Some(ram) = &mut self.ram {
            let index = (RAM_BANK_SIZE * bank) + (addr as usize);
            ram[index] = value;
        } else {
            panic!("Unable to write to cartridge RAM. No RAM included in cartridge.")
        }
    }
}

// TODO: add rumble support
pub struct MBC5 {
    rom: Vec<u8>,
    ram: Option<Vec<u8>>,
    rom_banks: usize,
    ram_banks: usize,
    ram_enabled: bool,
    // 8 bits used
    rom_bank_number: u8,
    // 1 bit used
    rom_bank_number2: u8,
    // 4 bits used
    ram_bank_number: u8,
}

impl MBC5 {
    pub const fn new(
        rom: Vec<u8>,
        ram: Option<Vec<u8>>,
        rom_banks: usize,
        ram_banks: usize,
    ) -> Self {
        Self {
            rom,
            ram,
            rom_banks,
            ram_banks,
            ram_enabled: false,
            rom_bank_number: 1,
            rom_bank_number2: 0,
            ram_bank_number: 0,
        }
    }
}

impl MemoryBankController for MBC5 {
    fn read_rom_bank0(&self, addr: u16) -> u8 {
        let index = addr as usize;
        self.rom[index]
    }

    fn read_rom_bank1(&self, addr: u16) -> u8 {
        let bank = ((self.rom_bank_number2 as u16) << 8) | (self.rom_bank_number as u16);
        let bank = truncate_bank(bank as usize, self.rom_banks);

        let index = (ROM_BANK_SIZE * bank) + (addr as usize);
        self.rom[index]
    }

    fn write_register(&mut self, addr: u16, value: u8) {
        match addr {
            0x0000..=0x1FFF => self.ram_enabled = value & 0xF == 0xA,
            0x2000..=0x2FFF => self.rom_bank_number = value,
            0x3000..=0x3FFF => self.rom_bank_number2 = value & 0x1,
            0x4000..=0x5FFF => self.ram_bank_number = value & 0xF,
            _ => {}
        }
    }

    fn read_ram_bank(&self, addr: u16) -> u8 {
        if !self.ram_enabled {
            return 0xFF;
        }

        let bank = truncate_bank(self.ram_bank_number as usize, self.ram_banks);

        if let Some(ram) = &self.ram {
            let index = (RAM_BANK_SIZE * bank) + (addr as usize);
            ram[index]
        } else {
            panic!("Unable to read from cartridge RAM. No RAM included in cartridge.");
        }
    }

    fn write_ram_bank(&mut self, addr: u16, value: u8) {
        if !self.ram_enabled {
            return;
        }

        let bank = truncate_bank(self.ram_bank_number as usize, self.ram_banks);

        if let Some(ram) = &mut self.ram {
            let index = (RAM_BANK_SIZE * bank) + (addr as usize);
            ram[index] = value;
        } else {
            panic!("Unable to write to cartridge RAM. No RAM included in cartridge.")
        }
    }
}

fn truncate_bank(bank: usize, available_banks: usize) -> usize {
    let leading_zeroes = available_banks.saturating_sub(1).leading_zeros();
    let mask = usize::MAX.checked_shr(leading_zeroes).unwrap_or(0);
    bank & mask
}
