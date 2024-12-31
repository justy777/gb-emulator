use gb_emulator::cartridge::Cartridge;
use gb_emulator::hardware::GameboyHardware;
use gb_emulator::util::Data;
use std::{env, fs, io};

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let rom = fs::read(&args[1])?;
    let cartridge = Cartridge::new(rom);
    let metadata = cartridge.metadata();

    println!("Title: {}", metadata.get_title());
    println!("CGB Flag: {:#04X}", metadata.get_cgb_flag());
    println!("Cartridge Type: {:#04X}", metadata.get_cartridge_type());
    println!("Has RAM: {}", metadata.has_ram());
    println!("Has battery: {}", metadata.has_battery());
    println!("Supports RTC: {}", metadata.supports_rtc());
    println!("Supports rumble: {}", metadata.supports_rumble());
    println!("ROM banks: {}", metadata.get_rom_banks());
    println!("ROM size: {}", Data::from_bytes(metadata.get_rom_size()));
    println!("RAM banks: {}", metadata.get_ram_banks());
    println!("RAM size: {}", Data::from_bytes(metadata.get_ram_size()));
    println!("Destination code: {:#04X}", metadata.get_destination_code());
    println!("Version number: {}", metadata.get_version_number());

    metadata
        .verify_header_checksum()
        .unwrap_or_else(|err| eprintln!("Warning: {err}"));

    metadata
        .verify_global_checksum()
        .unwrap_or_else(|err| eprintln!("Warning: {err}"));

    let mut gameboy = GameboyHardware::new(cartridge);
    loop {
        gameboy.step();
    }
}
