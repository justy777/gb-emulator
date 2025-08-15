//! gb-emulator is an original Game Boy (DMG) emulator written in Rust.

use crate::debug::Debugger;
use crate::util::DataUnit;
use gb_core::cartridge::Cartridge;
use gb_core::hardware::GameboyHardware;
use std::{env, fs};

mod debug;
mod util;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    let rom = fs::read(&args[1])?;
    let cartridge = Cartridge::new(rom)?;
    let metadata = cartridge.metadata();

    println!("Title: {}", metadata.title());
    println!("CGB Flag: {:#04X}", metadata.cgb_flag());
    println!("Cartridge Type: {:#04X}", metadata.cartridge_type());
    println!("Has RAM: {}", metadata.has_ram());
    println!("Has battery: {}", metadata.has_battery());
    println!("Supports RTC: {}", metadata.supports_rtc());
    println!("Supports rumble: {}", metadata.supports_rumble());
    println!("ROM banks: {}", metadata.rom_banks());
    println!("ROM size: {}", DataUnit::from_bytes(metadata.rom_size()));
    println!("RAM banks: {}", metadata.ram_banks());
    println!("RAM size: {}", DataUnit::from_bytes(metadata.ram_size()));
    println!("Destination code: {:#04X}", metadata.destination_code());
    println!("Version number: {}", metadata.version_number());
    println!();

    metadata
        .verify_header_checksum()
        .unwrap_or_else(|err| eprintln!("Warning: {err}"));

    metadata
        .verify_global_checksum()
        .unwrap_or_else(|err| eprintln!("Warning: {err}"));

    let mut debugger = Debugger::new();
    let mut gameboy = GameboyHardware::new(cartridge);

    let exit = debugger.debug(&mut gameboy)?;
    if exit {
        return Ok(());
    }

    loop {
        if debugger.check_points(&mut gameboy) {
            debugger.update_watchpoints(&mut gameboy);
            let exit = debugger.debug(&mut gameboy)?;
            if exit {
                return Ok(());
            }
        }

        gameboy.step();
    }
}
