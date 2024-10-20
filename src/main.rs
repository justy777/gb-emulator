use gb_emulator::cartridge::Cartridge;
use gb_emulator::cpu::Cpu;
use gb_emulator::memory::AddressBus;
use std::{env, fs, io};

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let rom = fs::read(&args[1])?;
    let cartridge = Cartridge::new(rom);

    println!("Title: {}", cartridge.get_title());
    println!("ROM Size: {}", cartridge.get_rom_size());
    println!("RAM Size: {}", cartridge.get_ram_size());

    if !cartridge.passed_header_check() {
        println!(
            "Warning: Header checksum on cartridge failed verification. Run at your own Risk."
        );
    }

    if !cartridge.passed_global_check() {
        println!(
            "Warning: Global checksum on cartridge failed verification. Run at your own Risk."
        );
    }

    let mut memory = AddressBus::new(cartridge);
    let mut cpu = Cpu::new();
    loop {
        let cycles = cpu.step(&mut memory);
        memory.step(cycles);
    }
}
