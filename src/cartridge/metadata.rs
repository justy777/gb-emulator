const CART_TITLE_START: usize = 0x134;
const CART_TITLE_END: usize = 0x143;
const CART_CARTRIDGE_TYPE: usize = 0x147;
const CART_ROM_SIZE: usize = 0x148;
const CART_RAM_SIZE: usize = 0x149;
const CART_HEADER_CHECKSUM: usize = 0x14D;
const CART_GLOBAL_CHECKSUM1: usize = 0x14E;
const CART_GLOBAL_CHECKSUM2: usize = 0x14F;

#[allow(clippy::struct_excessive_bools)]
#[derive(Debug)]
pub struct Metadata {
    pub title: String,
    pub mbc_number: u8,
    pub has_ram: bool,
    pub has_battery: bool,
    pub rom_bank_count: usize,
    pub ram_bank_count: usize,
    pub passed_header_check: bool,
    pub passed_global_check: bool,
}

impl Metadata {
    pub fn new(rom: &[u8]) -> Self {
        let title = rom[CART_TITLE_START..=CART_TITLE_END]
            .iter()
            .map(|byte| char::from(*byte))
            .filter(char::is_ascii)
            .collect();

        let cartridge_type = rom[CART_CARTRIDGE_TYPE];

        let mbc_number = match cartridge_type {
            0x00 | 0x08 | 0x09 => 0,
            0x01..=0x03 => 1,
            0x0F..=0x13 => 3,
            0x19..=0x1E => 5,
            val => panic!("Memory bank controller for {val:#X} not implemented"),
        };

        let has_ram = matches!(
            cartridge_type,
            0x02 | 0x03
                | 0x08
                | 0x09
                | 0x0C
                | 0x0D
                | 0x10
                | 0x12
                | 0x13
                | 0x1A
                | 0x1B
                | 0x1D
                | 0x1E
                | 0x22
                | 0xFF
        );

        let has_battery = matches!(
            cartridge_type,
            0x03 | 0x06 | 0x09 | 0x0D | 0x0F | 0x10 | 0x13 | 0x1B | 0x1E | 0x22 | 0xFF
        );

        let rom_bank_count = match rom[CART_ROM_SIZE] {
            n @ 0x00..=0x08 => 1 << (n + 1),
            val => panic!("Invalid value {val:#X} for ROM size in cartridge header."),
        };

        let ram_bank_count = match rom[CART_RAM_SIZE] {
            0x00 => 0,
            0x02 => 1,
            0x03 => 4,
            0x04 => 16,
            0x05 => 8,
            val => panic!("Invalid value {val:#02X} for RAM size in cartridge header."),
        };

        let passed_header_check = rom[CART_HEADER_CHECKSUM] == calculate_header_checksum(rom);

        let passed_global_check =
            u16::from_be_bytes([rom[CART_GLOBAL_CHECKSUM1], rom[CART_GLOBAL_CHECKSUM2]])
                == calculate_global_checksum(rom);

        Self {
            title,
            mbc_number,
            has_ram,
            has_battery,
            rom_bank_count,
            ram_bank_count,
            passed_header_check,
            passed_global_check,
        }
    }
}

fn calculate_header_checksum(rom: &[u8]) -> u8 {
    let mut checksum: u8 = 0;
    for byte in &rom[CART_TITLE_START..CART_TITLE_END] {
        checksum = checksum.wrapping_sub(*byte).wrapping_sub(1);
    }
    checksum
}

fn calculate_global_checksum(rom: &[u8]) -> u16 {
    let mut checksum: u16 = 0;
    for (address, byte) in rom.iter().enumerate() {
        if address != CART_GLOBAL_CHECKSUM1 && address != CART_GLOBAL_CHECKSUM2 {
            checksum = checksum.wrapping_add(*byte as u16);
        }
    }
    checksum
}
