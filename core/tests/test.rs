use std::fs;
use std::time::{Duration, Instant};
use yare::parameterized;
use gb_core::cartridge::Cartridge;
use gb_core::hardware::GameboyHardware;

#[test]
fn do_nothing() {}

#[parameterized(
  cpu_instrs_01 = {"tests/roms/blargg/cpu_instrs/01-special.gb", "tests/roms/blargg/cpu_instrs/01-special.txt", 3},
  cpu_instrs_02 = {"tests/roms/blargg/cpu_instrs/02-interrupts.gb", "tests/roms/blargg/cpu_instrs/02-interrupts.txt", 1},
  cpu_instrs_03 = {"tests/roms/blargg/cpu_instrs/03-op sp,hl.gb", "tests/roms/blargg/cpu_instrs/03-op sp,hl.txt", 3},
  cpu_instrs_04 = {"tests/roms/blargg/cpu_instrs/04-op r,imm.gb", "tests/roms/blargg/cpu_instrs/04-op r,imm.txt", 4},
  cpu_instrs_05 = {"tests/roms/blargg/cpu_instrs/05-op rp.gb", "tests/roms/blargg/cpu_instrs/05-op rp.txt", 4},
  cpu_instrs_06 = {"tests/roms/blargg/cpu_instrs/06-ld r,r.gb", "tests/roms/blargg/cpu_instrs/06-ld r,r.txt", 1},
  cpu_instrs_07 = {"tests/roms/blargg/cpu_instrs/07-jr,jp,call,ret,rst.gb", "tests/roms/blargg/cpu_instrs/07-jr,jp,call,ret,rst.txt", 1},
  cpu_instrs_08 = {"tests/roms/blargg/cpu_instrs/08-misc instrs.gb", "tests/roms/blargg/cpu_instrs/08-misc instrs.txt", 1},
  cpu_instrs_09 = {"tests/roms/blargg/cpu_instrs/09-op r,r.gb", "tests/roms/blargg/cpu_instrs/09-op r,r.txt", 8},
  cpu_instrs_10 = {"tests/roms/blargg/cpu_instrs/10-bit ops.gb", "tests/roms/blargg/cpu_instrs/10-bit ops.txt", 12},
  cpu_instrs_11 = {"tests/roms/blargg/cpu_instrs/11-op a,(hl).gb", "tests/roms/blargg/cpu_instrs/11-op a,(hl).txt", 14},
)]
fn test_rom(input: &str, output: &str, secs: u64) -> Result<(), Box<dyn std::error::Error>> {
    let rom = fs::read(input)?;
    let cartridge = Cartridge::new(rom)?;
    let mut gameboy = GameboyHardware::new(cartridge);

    let start_time = Instant::now();
    let mut next_time = Instant::now();
    while next_time - start_time < Duration::from_secs(secs) {
        gameboy.step();
        next_time = Instant::now();
    }

    let result = gameboy.serial_output();
    for c in result.split_ascii_whitespace() {
        let i: u8 = c.parse()?;
        print!("{}", char::from(i));
    }

    let expected = fs::read_to_string(output)?;

    assert_eq!(result, expected);
    Ok(())
}