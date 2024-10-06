use crate::util::{bits_needed, DataSize};

const ROM_BANK_SIZE: DataSize = DataSize::from_kilobytes(16);
const RAM_BANK_SIZE: DataSize = DataSize::from_kilobytes(8);

const CART_ROM_SIZE: usize = 0x148;
const CART_RAM_SIZE: usize = 0x149;
const CART_HEADER_CHECKSUM: usize = 0x14D;
const CART_GLOBAL_CHECKSUM1: usize = 0x14E;
const CART_GLOBAL_CHECKSUM2: usize = 0x14F;

pub struct Cartridge {
    rom: Vec<u8>,
    ram: Option<Vec<u8>>,
    mbc: Box<dyn MemoryBankController>,
}

impl Cartridge {
    #[must_use]
    pub fn new(rom: Vec<u8>) -> Self {
        // TODO: create constructor that sets memory bank controller
        Self {
            rom,
            ram: None,
            mbc: Box::new(NoMbc {}),
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

        assert!(
            value < 9,
            "Invalid value {value:#X} for ROM size in cartridge header."
        );

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
            _ => panic!("Invalid value {value:#02X} for RAM size in cartridge header."),
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

trait MemoryBankController {
    fn get_rom_bank0(&self) -> u32;
    fn get_rom_bank1(&self) -> u32;
    fn get_ram_bank(&self) -> u32;
    fn is_ram_enabled(&self) -> bool;
    fn write_registers(&mut self, address: u16, value: u8);
}

struct NoMbc {}

impl MemoryBankController for NoMbc {
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

struct Mbc1 {
    ram_enabled: bool,
    rom_bank_number: u8,
    rom_bank_max: u8,
    ram_bank_number: u8,
    ram_bank_max: u8,
    banking_mode: bool,
}

impl Mbc1 {
    const fn new(rom_bank_max: u8, ram_bank_max: u8) -> Self {
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

impl MemoryBankController for Mbc1 {
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
struct Mbc3 {
    ram_enabled: bool,
    rom_bank_number: u8,
    ram_bank_number: u8,
}

impl Mbc3 {
    const fn new() -> Self {
        Self {
            ram_enabled: false,
            rom_bank_number: 0,
            ram_bank_number: 0,
        }
    }
}

impl MemoryBankController for Mbc3 {
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

struct Mbc5 {
    ram_enabled: bool,
    rom_bank_number: u8,
    rom_bank_number2: u8,
    ram_bank_number: u8,
}

impl Mbc5 {
    const fn new() -> Self {
        Self {
            ram_enabled: false,
            rom_bank_number: 0,
            rom_bank_number2: 0,
            ram_bank_number: 0,
        }
    }
}

impl MemoryBankController for Mbc5 {
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
