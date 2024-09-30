use gb_emulator::cartridge::Cartridge;
use gb_emulator::cpu::Cpu;
use gb_emulator::memory::AddressBus;
use std::{env, fs, io};
use gb_emulator::serial_port::SerialPort;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let rom = fs::read(&args[1])?;
    let cartridge = Cartridge::new(rom);

    println!("Title: {}", cartridge.get_title());
    println!("ROM Size: {}", cartridge.get_rom_size());
    println!("RAM Size: {}", cartridge.get_ram_size());

    let read_checksum = cartridge.get_header_checksum();
    let calculated_checksum = cartridge.calculate_header_checksum();
    if read_checksum != calculated_checksum {
        println!("Warning: Header checksum on cartridge failed verification {read_checksum:#X} != {calculated_checksum:#X}. Run at your own Risk.");
    }
    let read_checksum = cartridge.get_global_checksum();
    let calculated_checksum = cartridge.calculate_global_checksum();
    if read_checksum != calculated_checksum {
        println!("Warning: Global checksum on cartridge failed verification {read_checksum:#X} != {calculated_checksum:#X}. Run at your own Risk.");
    }

    let mut memory = AddressBus::new(cartridge);
    let mut cpu = Cpu::new();
    loop {
        cpu.step(&mut memory);
        SerialPort::step(&mut memory);
    }
}
