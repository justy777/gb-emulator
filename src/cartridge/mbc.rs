pub trait MemoryBankController {
    fn get_rom_bank0(&self) -> usize;
    fn get_rom_bank1(&self) -> usize;
    fn get_ram_bank(&self) -> usize;
    fn is_ram_enabled(&self) -> bool;
    fn write_register(&mut self, addr: u16, value: u8);
}

pub struct NoMBC {}

impl NoMBC {
    pub const fn new() -> Self {
        Self {}
    }
}

impl MemoryBankController for NoMBC {
    fn get_rom_bank0(&self) -> usize {
        0
    }

    fn get_rom_bank1(&self) -> usize {
        1
    }

    fn get_ram_bank(&self) -> usize {
        0
    }

    fn is_ram_enabled(&self) -> bool {
        true
    }

    fn write_register(&mut self, _addr: u16, _value: u8) {
        panic!("Cannot write to Read-Only Memory (ROM) on cartridge.");
    }
}

pub struct MBC1 {
    ram_enabled: bool,
    // 5 bits used
    rom_bank_number: u8,
    // 2 bits used
    ram_bank_number: u8,
    banking_mode: bool,
    rom_banks: usize,
    ram_banks: usize,
}

impl MBC1 {
    pub const fn new(rom_banks: usize, ram_banks: usize) -> Self {
        Self {
            ram_enabled: false,
            rom_bank_number: 1,
            ram_bank_number: 0,
            banking_mode: false,
            rom_banks,
            ram_banks,
        }
    }
}

impl MemoryBankController for MBC1 {
    fn get_rom_bank0(&self) -> usize {
        let bank = if self.banking_mode {
            self.ram_bank_number << 5
        } else {
            0
        };
        truncate_bank(bank as usize, self.rom_banks)
    }

    fn get_rom_bank1(&self) -> usize {
        let mut bank = self.ram_bank_number << 5 | self.rom_bank_number;

        if self.rom_bank_number == 0 {
            bank += 1;
        }

        truncate_bank(bank as usize, self.rom_banks)
    }

    fn get_ram_bank(&self) -> usize {
        let bank = if self.banking_mode {
            self.ram_bank_number
        } else {
            0
        };
        truncate_bank(bank as usize, self.ram_banks)
    }

    fn is_ram_enabled(&self) -> bool {
        self.ram_enabled
    }

    fn write_register(&mut self, addr: u16, value: u8) {
        match addr {
            0x0000..=0x1FFF => self.ram_enabled = value & 0xF == 0xA,
            0x2000..=0x3FFF => self.rom_bank_number = value & 0x1F,
            0x4000..=0x5FFF => self.ram_bank_number = value & 0x3,
            0x6000..=0x7FFF => self.banking_mode = value & 0x1 == 0x1,
            _ => panic!("Address {addr:#X} not mapped in Memory Bank Controller."),
        }
    }
}

// TODO: add real-time clock (RTC) support
pub struct MBC3 {
    ram_enabled: bool,
    // 7 bits used
    rom_bank_number: u8,
    // 2 bits used
    ram_bank_number: u8,
    ram_banks: usize,
    rom_banks: usize,
}

impl MBC3 {
    pub const fn new(ram_banks: usize, rom_banks: usize) -> Self {
        Self {
            ram_enabled: false,
            rom_bank_number: 1,
            ram_bank_number: 0,
            ram_banks,
            rom_banks,
        }
    }
}

impl MemoryBankController for MBC3 {
    fn get_rom_bank0(&self) -> usize {
        0
    }

    fn get_rom_bank1(&self) -> usize {
        let bank = if self.rom_bank_number == 0 {
            1
        } else {
            self.rom_bank_number
        };
        truncate_bank(bank as usize, self.rom_banks)
    }

    fn get_ram_bank(&self) -> usize {
        let bank = self.ram_bank_number;
        truncate_bank(bank as usize, self.ram_banks)
    }

    fn is_ram_enabled(&self) -> bool {
        self.ram_enabled
    }

    fn write_register(&mut self, addr: u16, value: u8) {
        match addr {
            0x0000..=0x1FFF => self.ram_enabled = value & 0xF == 0xA,
            0x2000..=0x3FFF => self.rom_bank_number = value & 0x7F,
            0x4000..=0x5FFF => self.ram_bank_number = value & 0x3,
            _ => panic!("Address {addr:#X} not mapped in Memory Bank Controller."),
        }
    }
}

// TODO: add rumble support
pub struct MBC5 {
    ram_enabled: bool,
    // 8 bits used
    rom_bank_number: u8,
    // 1 bit used
    rom_bank_number2: u8,
    // 4 bits used
    ram_bank_number: u8,
    rom_banks: usize,
    ram_banks: usize,
}

impl MBC5 {
    pub const fn new(rom_banks: usize, ram_banks: usize) -> Self {
        Self {
            ram_enabled: false,
            rom_bank_number: 1,
            rom_bank_number2: 0,
            ram_bank_number: 0,
            rom_banks,
            ram_banks,
        }
    }
}

impl MemoryBankController for MBC5 {
    fn get_rom_bank0(&self) -> usize {
        0
    }

    fn get_rom_bank1(&self) -> usize {
        let bank = ((self.rom_bank_number2 as u16) << 8) | (self.rom_bank_number as u16);
        truncate_bank(bank as usize, self.rom_banks)
    }

    fn get_ram_bank(&self) -> usize {
        let bank = self.ram_bank_number;
        truncate_bank(bank as usize, self.ram_banks)
    }

    fn is_ram_enabled(&self) -> bool {
        self.ram_enabled
    }

    fn write_register(&mut self, addr: u16, value: u8) {
        match addr {
            0x0000..=0x1FFF => self.ram_enabled = value & 0xF == 0xA,
            0x2000..=0x2FFF => self.rom_bank_number = value,
            0x3000..=0x3FFF => self.rom_bank_number2 = value & 0x1,
            0x4000..=0x5FFF => self.ram_bank_number = value & 0xF,
            _ => panic!("Address {addr:#X} not mapped in Memory Bank Controller."),
        }
    }
}

fn truncate_bank(bank: usize, available_banks: usize) -> usize {
    let leading_zeroes = available_banks.saturating_sub(1).leading_zeros();
    let mask = usize::MAX.checked_shr(leading_zeroes).unwrap_or(0);
    bank & mask
}
