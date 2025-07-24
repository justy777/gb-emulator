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
  cpu_instrs_04 = {"tests/roms/blargg/cpu_instrs/04-op r,imm.gb", "tests/roms/blargg/cpu_instrs/04-op r,imm.txt", 3},
  cpu_instrs_05 = {"tests/roms/blargg/cpu_instrs/05-op rp.gb", "tests/roms/blargg/cpu_instrs/05-op rp.txt", 4},
  cpu_instrs_06 = {"tests/roms/blargg/cpu_instrs/06-ld r,r.gb", "tests/roms/blargg/cpu_instrs/06-ld r,r.txt", 1},
  cpu_instrs_07 = {"tests/roms/blargg/cpu_instrs/07-jr,jp,call,ret,rst.gb", "tests/roms/blargg/cpu_instrs/07-jr,jp,call,ret,rst.txt", 1},
  cpu_instrs_08 = {"tests/roms/blargg/cpu_instrs/08-misc instrs.gb", "tests/roms/blargg/cpu_instrs/08-misc instrs.txt", 1},
  cpu_instrs_09 = {"tests/roms/blargg/cpu_instrs/09-op r,r.gb", "tests/roms/blargg/cpu_instrs/09-op r,r.txt", 9},
  cpu_instrs_10 = {"tests/roms/blargg/cpu_instrs/10-bit ops.gb", "tests/roms/blargg/cpu_instrs/10-bit ops.txt", 14},
  cpu_instrs_11 = {"tests/roms/blargg/cpu_instrs/11-op a,(hl).gb", "tests/roms/blargg/cpu_instrs/11-op a,(hl).txt", 15},
  dmg_sound_01 = {"tests/roms/blargg/dmg_sound/01-registers.gb", "tests/roms/blargg/dmg_sound/01-registers.txt", 10},
  dmg_sound_02 = {"tests/roms/blargg/dmg_sound/02-len ctr.gb", "tests/roms/blargg/dmg_sound/02-len ctr.txt", 10},
  dmg_sound_03 = {"tests/roms/blargg/dmg_sound/03-trigger.gb", "tests/roms/blargg/dmg_sound/03-trigger.txt", 10},
  dmg_sound_04 = {"tests/roms/blargg/dmg_sound/04-sweep.gb", "tests/roms/blargg/dmg_sound/04-sweep.txt", 10},
  dmg_sound_05 = {"tests/roms/blargg/dmg_sound/05-sweep details.gb", "tests/roms/blargg/dmg_sound/05-sweep details.txt", 10},
  dmg_sound_06 = {"tests/roms/blargg/dmg_sound/06-overflow on trigger.gb", "tests/roms/blargg/dmg_sound/06-overflow on trigger.txt", 10},
  dmg_sound_07 = {"tests/roms/blargg/dmg_sound/07-len sweep period sync.gb", "tests/roms/blargg/dmg_sound/07-len sweep period sync.txt", 10},
  dmg_sound_08 = {"tests/roms/blargg/dmg_sound/08-len ctr during power.gb", "tests/roms/blargg/dmg_sound/08-len ctr during power.txt", 10},
  dmg_sound_09 = {"tests/roms/blargg/dmg_sound/09-wave read while on.gb", "tests/roms/blargg/dmg_sound/09-wave read while on.txt", 10},
  dmg_sound_10 = {"tests/roms/blargg/dmg_sound/10-wave trigger while on.gb", "tests/roms/blargg/dmg_sound/10-wave trigger while on.txt", 10},
  dmg_sound_11 = {"tests/roms/blargg/dmg_sound/11-regs after power.gb", "tests/roms/blargg/dmg_sound/11-regs after power.txt", 10},
  dmg_sound_12 = {"tests/roms/blargg/dmg_sound/12-wave write while on.gb", "tests/roms/blargg/dmg_sound/12-wave write while on.txt", 10},
  instr_timing = {"tests/roms/blargg/instr_timing.gb", "tests/roms/blargg/instr_timing.txt", 1},
  mem_timing_01 = {"tests/roms/blargg/mem_timing/01-read_timing.gb", "tests/roms/blargg/mem_timing/01-read_timing.txt", 1},
  mem_timing_02 = {"tests/roms/blargg/mem_timing/02-write_timing.gb", "tests/roms/blargg/mem_timing/02-write_timing.txt", 1},
  mem_timing_03 = {"tests/roms/blargg/mem_timing/03-modify_timing.gb", "tests/roms/blargg/mem_timing/03-modify_timing.txt", 1},
  oam_bug_1 = {"tests/roms/blargg/oam_bug/1-lcd_sync.gb", "tests/roms/blargg/oam_bug/1-lcd_sync.txt", 10},
  oam_bug_2 = {"tests/roms/blargg/oam_bug/2-causes.gb", "tests/roms/blargg/oam_bug/2-causes.txt", 10},
  oam_bug_3 = {"tests/roms/blargg/oam_bug/3-non_causes.gb", "tests/roms/blargg/oam_bug/3-non_causes.txt", 10},
  oam_bug_4 = {"tests/roms/blargg/oam_bug/4-scanline_timing.gb", "tests/roms/blargg/oam_bug/4-scanline_timing.txt", 10},
  oam_bug_5 = {"tests/roms/blargg/oam_bug/5-timing_bug.gb", "tests/roms/blargg/oam_bug/5-timing_bug.txt", 10},
  oam_bug_6 = {"tests/roms/blargg/oam_bug/6-timing_no_bug.gb", "tests/roms/blargg/oam_bug/6-timing_no_bug.txt", 10},
  oam_bug_7 = {"tests/roms/blargg/oam_bug/7-timing_effect.gb", "tests/roms/blargg/oam_bug/7-timing_effect.txt", 10},
  oam_bug_8 = {"tests/roms/blargg/oam_bug/8-instr_effect.gb", "tests/roms/blargg/oam_bug/8-instr_effect.txt", 10},
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