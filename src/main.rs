use gb_emulator::cartridge::Cartridge;
use gb_emulator::cpu::Cpu;
use gb_emulator::memory::AddressBus;
use std::{env, fs, io};

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let data = fs::read(&args[0])?;
    let cartridge = Cartridge::new(data);
    let mut memory = AddressBus::new(cartridge);
    let mut cpu = Cpu::new();
    loop {
        cpu.step(&mut memory);
    }
}
