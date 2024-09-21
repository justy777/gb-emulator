use gb_emulator::cpu::Cpu;
use gb_emulator::memory::AddressBus;

fn main() {
    let mut memory = AddressBus::new();
    let mut cpu = Cpu::new();
    loop {
        cpu.step(&mut memory);
    }
}
