const CART_TITLE_START: usize = 0x134;
const CART_TITLE_END: usize = 0x143;
const CART_CGB_FLAG: usize = 0x143;
const CART_CARTRIDGE_TYPE: usize = 0x147;
const CART_ROM_SIZE: usize = 0x148;
const CART_RAM_SIZE: usize = 0x149;
const CART_DESTINATION_CODE: usize = 0x14A;
const CART_VERSION_NUMBER: usize = 0x14C;
const CART_HEADER_CHECKSUM: usize = 0x14D;
const CART_GLOBAL_CHECKSUM1: usize = 0x14E;
const CART_GLOBAL_CHECKSUM2: usize = 0x14F;

const CARTRIDGE_TYPE_HAS_RAM: &[u8] = &[
    0x02, 0x03, 0x08, 0x09, 0x0C, 0x0D, 0x10, 0x12, 0x13, 0x1A, 0x1B, 0x1D, 0x1E, 0x22, 0xFF,
];
const CARTRIDGE_TYPE_HAS_BATTERY: &[u8] = &[
    0x03, 0x06, 0x09, 0x0D, 0x0F, 0x10, 0x13, 0x1B, 0x1E, 0x22, 0xFF,
];
const CARTRIDGE_TYPE_SUPPORTS_RTC: &[u8] = &[0x0F, 0x10];
const CARTRIDGE_TYPE_SUPPORTS_RUMBLE: &[u8] = &[0x1C, 0x1D, 0x1E, 0x22];

const ROM_BANK_SIZE: usize = 16 * 1024;
const RAM_BANK_SIZE: usize = 8 * 1024;

#[derive(Debug)]
pub struct Metadata {
    title: String,
    cgb_flag: u8,
    cartridge_type: u8,
    pub mbc_number: u8,
    rom_banks: usize,
    ram_banks: usize,
    destination_code: u8,
    version_number: u8,
    expected_header_checksum: u8,
    actual_header_checksum: u8,
    expected_global_checksum: u16,
    actual_global_checksum: u16,
}

impl Metadata {
    pub fn new(rom: &[u8]) -> Self {
        let title = rom[CART_TITLE_START..=CART_TITLE_END]
            .iter()
            .map(|byte| char::from(*byte))
            .filter(char::is_ascii)
            .collect();

        let cgb_flag = rom[CART_CGB_FLAG];

        let cartridge_type = rom[CART_CARTRIDGE_TYPE];

        let mbc_number = match cartridge_type {
            0x00 | 0x08 | 0x09 => 0,
            0x01..=0x03 => 1,
            0x0F..=0x13 => 3,
            0x19..=0x1E => 5,
            val => panic!("Memory bank controller for {val:#X} not implemented"),
        };

        let rom_banks = match rom[CART_ROM_SIZE] {
            n @ 0x00..=0x08 => 1 << (n + 1),
            val => panic!("Invalid value {val:#X} for ROM size in cartridge header."),
        };

        let ram_banks = match rom[CART_RAM_SIZE] {
            0x00 => 0,
            0x02 => 1,
            0x03 => 4,
            0x04 => 16,
            0x05 => 8,
            val => panic!("Invalid value {val:#02X} for RAM size in cartridge header."),
        };

        let destination_code = rom[CART_DESTINATION_CODE];

        let version_number = rom[CART_VERSION_NUMBER];

        let expected_header_checksum = rom[CART_HEADER_CHECKSUM];

        let actual_header_checksum = calculate_header_checksum(rom);

        let expected_global_checksum =
            u16::from_be_bytes([rom[CART_GLOBAL_CHECKSUM1], rom[CART_GLOBAL_CHECKSUM2]]);

        let actual_global_checksum = calculate_global_checksum(rom);

        Self {
            title,
            cgb_flag,
            cartridge_type,
            mbc_number,
            rom_banks,
            ram_banks,
            destination_code,
            version_number,
            expected_header_checksum,
            actual_header_checksum,
            expected_global_checksum,
            actual_global_checksum,
        }
    }

    pub fn verify_header_checksum(&self) -> Result<(), String> {
        let expected = self.expected_header_checksum;
        let actual = self.actual_header_checksum;
        if actual == expected {
            Ok(())
        } else {
            Err(format!(
                "Header verification failed (expected {expected:#04X}, found {actual:#04X})"
            ))
        }
    }

    pub fn verify_global_checksum(&self) -> Result<(), String> {
        let expected = self.expected_global_checksum;
        let actual = self.actual_global_checksum;
        if actual == expected {
            Ok(())
        } else {
            Err(format!(
                "Global verification failed (expected {expected:#04X}, found {actual:#04X})"
            ))
        }
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub const fn cgb_flag(&self) -> u8 {
        self.cgb_flag
    }

    pub const fn cartridge_type(&self) -> u8 {
        self.cartridge_type
    }

    pub fn has_ram(&self) -> bool {
        CARTRIDGE_TYPE_HAS_RAM.contains(&self.cartridge_type)
    }

    pub fn has_battery(&self) -> bool {
        CARTRIDGE_TYPE_HAS_BATTERY.contains(&self.cartridge_type)
    }

    pub fn supports_rtc(&self) -> bool {
        CARTRIDGE_TYPE_SUPPORTS_RTC.contains(&self.cartridge_type)
    }

    pub fn supports_rumble(&self) -> bool {
        CARTRIDGE_TYPE_SUPPORTS_RUMBLE.contains(&self.cartridge_type)
    }

    pub const fn rom_banks(&self) -> usize {
        self.rom_banks
    }

    pub const fn ram_banks(&self) -> usize {
        self.ram_banks
    }

    pub const fn rom_size(&self) -> usize {
        self.rom_banks * ROM_BANK_SIZE
    }

    pub const fn ram_size(&self) -> usize {
        self.ram_banks * RAM_BANK_SIZE
    }

    pub const fn destination_code(&self) -> u8 {
        self.destination_code
    }

    pub const fn version_number(&self) -> u8 {
        self.version_number
    }
}

fn calculate_header_checksum(rom: &[u8]) -> u8 {
    let mut checksum: u8 = 0;
    for byte in &rom[CART_TITLE_START..=CART_VERSION_NUMBER] {
        checksum = checksum.wrapping_sub(*byte).wrapping_sub(1);
    }
    checksum
}

fn calculate_global_checksum(rom: &[u8]) -> u16 {
    let mut checksum: u16 = 0;
    for (addr, byte) in rom.iter().enumerate() {
        if addr != CART_GLOBAL_CHECKSUM1 && addr != CART_GLOBAL_CHECKSUM2 {
            checksum = checksum.wrapping_add(*byte as u16);
        }
    }
    checksum
}
