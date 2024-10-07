use crate::util::bits_needed;

pub trait MemoryBankController {
    fn get_rom_bank0(&self) -> u32;
    fn get_rom_bank1(&self) -> u32;
    fn get_ram_bank(&self) -> u32;
    fn is_ram_enabled(&self) -> bool;
    fn write_registers(&mut self, address: u16, value: u8);
}

pub struct NoMBC {}

impl NoMBC {
    pub const fn new() -> Self {
        Self {}
    }
}

impl MemoryBankController for NoMBC {
    fn get_rom_bank0(&self) -> u32 {
        0
    }

    fn get_rom_bank1(&self) -> u32 {
        1
    }

    fn get_ram_bank(&self) -> u32 {
        0
    }

    fn is_ram_enabled(&self) -> bool {
        true
    }

    fn write_registers(&mut self, _address: u16, _value: u8) {
        panic!("Cannot write to Read-Only Memory (ROM) on cartridge.");
    }
}

pub struct MBC1 {
    ram_enabled: bool,
    rom_bank_number: u8,
    rom_bank_max: u32,
    ram_bank_number: u8,
    ram_bank_max: u32,
    banking_mode: bool,
}

impl MBC1 {
    pub const fn new(rom_bank_max: u32, ram_bank_max: u32) -> Self {
        Self {
            ram_enabled: false,
            rom_bank_number: 0,
            rom_bank_max,
            ram_bank_number: 0,
            ram_bank_max,
            banking_mode: false,
        }
    }
}

impl MemoryBankController for MBC1 {
    fn get_rom_bank0(&self) -> u32 {
        if self.banking_mode {
            let max_bits = bits_needed(self.rom_bank_max as usize);
            ((self.ram_bank_number << 5) & u8::MAX >> (8 - max_bits)) as u32
        } else {
            0
        }
    }

    fn get_rom_bank1(&self) -> u32 {
        let max_bits = bits_needed(self.rom_bank_max as usize);
        let value = (((self.ram_bank_number << 5) | self.rom_bank_number)
            & (u8::MAX >> (8 - max_bits))) as u32;
        if self.rom_bank_number == 0 {
            value + 1
        } else {
            value
        }
    }

    fn get_ram_bank(&self) -> u32 {
        if self.banking_mode {
            let max_bits = bits_needed(self.ram_bank_max as usize);
            (self.ram_bank_number & u8::MAX >> (8 - max_bits)) as u32
        } else {
            0
        }
    }

    fn is_ram_enabled(&self) -> bool {
        self.ram_enabled
    }

    fn write_registers(&mut self, address: u16, value: u8) {
        match address {
            0x0000..=0x1FFF => {
                self.ram_enabled = value & 0xF == 0xA;
            }
            0x2000..=0x3FFF => {
                self.rom_bank_number = value & 0x1F;
            }
            0x4000..=0x5FFF => {
                self.ram_bank_number = value & 0x3;
            }
            0x6000..=0x7FFF => {
                self.banking_mode = value & 0x1 == 0x1;
            }
            _ => panic!("Address {address:#X} not mapped in Memory Bank Controller."),
        }
    }
}

// TODO: add real-time clock (RTC) support
pub struct MBC3 {
    ram_enabled: bool,
    rom_bank_number: u8,
    ram_bank_number: u8,
}

impl MBC3 {
    pub const fn new() -> Self {
        Self {
            ram_enabled: false,
            rom_bank_number: 0,
            ram_bank_number: 0,
        }
    }
}

impl MemoryBankController for MBC3 {
    fn get_rom_bank0(&self) -> u32 {
        0
    }

    fn get_rom_bank1(&self) -> u32 {
        self.rom_bank_number as u32
    }

    fn get_ram_bank(&self) -> u32 {
        self.ram_bank_number as u32
    }

    fn is_ram_enabled(&self) -> bool {
        self.ram_enabled
    }

    fn write_registers(&mut self, address: u16, value: u8) {
        match address {
            0x0000..=0x1FFF => {
                if value & 0xF == 0xA {
                    self.ram_enabled = true;
                } else if value & 0xF == 0 {
                    self.ram_enabled = false;
                }
            }
            0x2000..=0x3FFF => {
                self.rom_bank_number = value & 0x1F;
            }
            0x4000..=0x5FFF => {
                self.ram_bank_number = value & 0x3;
            }
            _ => panic!("Address {address:#X} not mapped in Memory Bank Controller."),
        }
    }
}

pub struct MBC5 {
    ram_enabled: bool,
    rom_bank_number: u8,
    rom_bank_number2: u8,
    ram_bank_number: u8,
}

impl MBC5 {
    pub const fn new() -> Self {
        Self {
            ram_enabled: false,
            rom_bank_number: 0,
            rom_bank_number2: 0,
            ram_bank_number: 0,
        }
    }
}

impl MemoryBankController for MBC5 {
    fn get_rom_bank0(&self) -> u32 {
        0
    }

    fn get_rom_bank1(&self) -> u32 {
        (((self.rom_bank_number2 as u16) << 8) | (self.rom_bank_number as u16)) as u32
    }

    fn get_ram_bank(&self) -> u32 {
        self.ram_bank_number as u32
    }

    fn is_ram_enabled(&self) -> bool {
        self.ram_enabled
    }

    fn write_registers(&mut self, address: u16, value: u8) {
        match address {
            0x0000..=0x1FFF => {
                if value & 0xF == 0xA {
                    self.ram_enabled = true;
                } else if value & 0xF == 0 {
                    self.ram_enabled = false;
                }
            }
            0x2000..=0x2FFF => {
                self.rom_bank_number = value;
            }
            0x3000..=0x3FFF => {
                self.rom_bank_number2 = value & 0x1;
            }
            0x4000..=0x5FFF => {
                self.ram_bank_number = value & 0xF;
            }
            _ => panic!("Address {address:#X} not mapped in Memory Bank Controller."),
        }
    }
}