use gb_core::cartridge::Cartridge;
use gb_core::hardware::GameboyHardware;
use gb_core::{RegisterU8, RegisterU16};
use std::fs;
use std::time::{Duration, Instant};
use yare::parameterized;

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
  instr_timing = {"tests/roms/blargg/instr_timing.gb", "tests/roms/blargg/instr_timing.txt", 1},
)]
fn test_rom_serial(input: &str, output: &str, secs: u64) -> Result<(), Box<dyn std::error::Error>> {
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

#[parameterized(
  dmg_sound_01 = {"tests/roms/blargg/dmg_sound/01-registers.gb"},
  dmg_sound_02 = {"tests/roms/blargg/dmg_sound/02-len ctr.gb"},
  dmg_sound_03 = {"tests/roms/blargg/dmg_sound/03-trigger.gb"},
  dmg_sound_04 = {"tests/roms/blargg/dmg_sound/04-sweep.gb"},
  dmg_sound_05 = {"tests/roms/blargg/dmg_sound/05-sweep details.gb"},
  dmg_sound_06 = {"tests/roms/blargg/dmg_sound/06-overflow on trigger.gb"},
  dmg_sound_07 = {"tests/roms/blargg/dmg_sound/07-len sweep period sync.gb"},
  dmg_sound_08 = {"tests/roms/blargg/dmg_sound/08-len ctr during power.gb"},
  dmg_sound_09 = {"tests/roms/blargg/dmg_sound/09-wave read while on.gb"},
  dmg_sound_10 = {"tests/roms/blargg/dmg_sound/10-wave trigger while on.gb"},
  dmg_sound_11 = {"tests/roms/blargg/dmg_sound/11-regs after power.gb"},
  dmg_sound_12 = {"tests/roms/blargg/dmg_sound/12-wave write while on.gb"},
  mem_timing_01 = {"tests/roms/blargg/mem_timing-2/01-read_timing.gb"},
  mem_timing_02 = {"tests/roms/blargg/mem_timing-2/02-write_timing.gb"},
  mem_timing_03 = {"tests/roms/blargg/mem_timing-2/03-modify_timing.gb"},
  oam_bug_1 = {"tests/roms/blargg/oam_bug/1-lcd_sync.gb"},
  oam_bug_2 = {"tests/roms/blargg/oam_bug/2-causes.gb"},
  oam_bug_3 = {"tests/roms/blargg/oam_bug/3-non_causes.gb"},
  oam_bug_4 = {"tests/roms/blargg/oam_bug/4-scanline_timing.gb"},
  oam_bug_5 = {"tests/roms/blargg/oam_bug/5-timing_bug.gb"},
  oam_bug_6 = {"tests/roms/blargg/oam_bug/6-timing_no_bug.gb"},
  oam_bug_7 = {"tests/roms/blargg/oam_bug/7-timing_effect.gb"},
  oam_bug_8 = {"tests/roms/blargg/oam_bug/8-instr_effect.gb"},
)]
fn test_rom_memory(input: &str) -> Result<(), Box<dyn std::error::Error>> {
    let rom = fs::read(input)?;
    let cartridge = Cartridge::new(rom)?;
    let mut gameboy = GameboyHardware::new(cartridge);

    let mut state = State::new();
    while !state.is_done() {
        gameboy.step();

        let result = gameboy.memory(0xA000);
        state = state.update(result);
    }

    let result = gameboy.memory(0xA000);

    let mut i = 0;
    loop {
        let print = gameboy.memory(0xA004 + i);
        if print == 0 {
            break;
        }
        print!("{}", char::from(print));
        i += 1;
    }

    assert_eq!(result, 0);

    Ok(())
}

#[derive(Debug, Copy, Clone)]
enum State {
    Start,
    Run,
    Done,
}

impl State {
    fn new() -> Self {
        Self::Start
    }

    fn is_done(&self) -> bool {
        matches!(self, State::Done)
    }

    fn update(self, val: u8) -> Self {
        match (self, val) {
            (State::Start, 0x80) => State::Run,
            (State::Run, n) if n != 0x80 => State::Done,
            (state, _) => state,
        }
    }
}

#[parameterized(
  bits_mem_oam = {"tests/roms/mooneye/acceptance/bits/mem_oam.gb"},
  bits_reg_f = {"tests/roms/mooneye/acceptance/bits/reg_f.gb"},
  bits_unused_hwio = {"tests/roms/mooneye/acceptance/bits/unused_hwio-GS.gb"},
  boot_div = {"tests/roms/mooneye/acceptance/boot/boot_div-dmgABCmgb.gb"},
  boot_hwio = {"tests/roms/mooneye/acceptance/boot/boot_hwio-dmgABCmgb.gb"},
  boot_regs = {"tests/roms/mooneye/acceptance/boot/boot_regs-dmgABC.gb"},
  halt_ime0_ei = {"tests/roms/mooneye/acceptance/halt/halt_ime0_ei.gb"},
  halt_ime0_nointr_timing = {"tests/roms/mooneye/acceptance/halt/halt_ime0_nointr_timing.gb"},
  halt_ime1_timing = {"tests/roms/mooneye/acceptance/halt/halt_ime1_timing.gb"},
  halt_ime1_timing2 = {"tests/roms/mooneye/acceptance/halt/halt_ime1_timing2-GS.gb"},
  instr_add_sp_e_timing = {"tests/roms/mooneye/acceptance/instr/add_sp_e_timing.gb"},
  instr_call_cc_timing = {"tests/roms/mooneye/acceptance/instr/call_cc_timing.gb"},
  instr_call_cc_timing2 = {"tests/roms/mooneye/acceptance/instr/call_cc_timing2.gb"},
  instr_call_timing = {"tests/roms/mooneye/acceptance/instr/call_timing.gb"},
  instr_call_timing2 = {"tests/roms/mooneye/acceptance/instr/call_timing2.gb"},
  instr_daa = {"tests/roms/mooneye/acceptance/instr/daa.gb"},
  instr_jp_cc_timing = {"tests/roms/mooneye/acceptance/instr/jp_cc_timing.gb"},
  instr_jp_timing = {"tests/roms/mooneye/acceptance/instr/jp_timing.gb"},
  instr_ld_hl_sp_e_timing = {"tests/roms/mooneye/acceptance/instr/ld_hl_sp_e_timing.gb"},
  instr_pop_timing = {"tests/roms/mooneye/acceptance/instr/pop_timing.gb"},
  instr_push_timing = {"tests/roms/mooneye/acceptance/instr/push_timing.gb"},
  instr_ret_cc_timing = {"tests/roms/mooneye/acceptance/instr/ret_cc_timing.gb"},
  instr_ret_timing = {"tests/roms/mooneye/acceptance/instr/ret_timing.gb"},
  instr_reti_intr_timing = {"tests/roms/mooneye/acceptance/instr/reti_intr_timing.gb"},
  instr_reti_timing = {"tests/roms/mooneye/acceptance/instr/reti_timing.gb"},
  instr_rst_timing = {"tests/roms/mooneye/acceptance/instr/rst_timing.gb"},
  intr_di_timing = {"tests/roms/mooneye/acceptance/intr/di_timing-GS.gb"},
  intr_ei_sequence = {"tests/roms/mooneye/acceptance/intr/ei_sequence.gb"},
  intr_ei_timing = {"tests/roms/mooneye/acceptance/intr/ei_timing.gb"},
  intr_ie_push = {"tests/roms/mooneye/acceptance/intr/ie_push.gb"},
  intr_if_ie_registers = {"tests/roms/mooneye/acceptance/intr/if_ie_registers.gb"},
  intr_timing = {"tests/roms/mooneye/acceptance/intr/intr_timing.gb"},
  intr_rapid_di_ei = {"tests/roms/mooneye/acceptance/intr/rapid_di_ei.gb"},
  mbc1_bits_bank1 = {"tests/roms/mooneye/emulator-only/mbc1/bits_bank1.gb"},
  mbc1_bits_bank2 = {"tests/roms/mooneye/emulator-only/mbc1/bits_bank2.gb"},
  mbc1_bits_mode = {"tests/roms/mooneye/emulator-only/mbc1/bits_mode.gb"},
  mbc1_bits_ramg = {"tests/roms/mooneye/emulator-only/mbc1/bits_ramg.gb"},
  mbc1_multicart_rom_8mb = {"tests/roms/mooneye/emulator-only/mbc1/multicart_rom_8Mb.gb"},
  mbc1_ram_64kb = {"tests/roms/mooneye/emulator-only/mbc1/ram_64kb.gb"},
  mbc1_ram_256kb = {"tests/roms/mooneye/emulator-only/mbc1/ram_256kb.gb"},
  mbc1_rom_1mb = {"tests/roms/mooneye/emulator-only/mbc1/rom_1Mb.gb"},
  mbc1_rom_2mb = {"tests/roms/mooneye/emulator-only/mbc1/rom_2Mb.gb"},
  mbc1_rom_4mb = {"tests/roms/mooneye/emulator-only/mbc1/rom_4Mb.gb"},
  mbc1_rom_8mb = {"tests/roms/mooneye/emulator-only/mbc1/rom_8Mb.gb"},
  mbc1_rom_16mb = {"tests/roms/mooneye/emulator-only/mbc1/rom_16Mb.gb"},
  mbc1_rom_512kb = {"tests/roms/mooneye/emulator-only/mbc1/rom_512kb.gb"},
  mbc2_bits_ramg = {"tests/roms/mooneye/emulator-only/mbc2/bits_ramg.gb"},
  mbc2_bits_romb = {"tests/roms/mooneye/emulator-only/mbc2/bits_romb.gb"},
  mbc2_bits_unused = {"tests/roms/mooneye/emulator-only/mbc2/bits_unused.gb"},
  mbc2_ram = {"tests/roms/mooneye/emulator-only/mbc2/ram.gb"},
  mbc2_rom_1mb = {"tests/roms/mooneye/emulator-only/mbc2/rom_1Mb.gb"},
  mbc2_rom_2mb = {"tests/roms/mooneye/emulator-only/mbc2/rom_2Mb.gb"},
  mbc2_rom_512kb = {"tests/roms/mooneye/emulator-only/mbc2/rom_512kb.gb"},
  mbc5_rom_1mb = {"tests/roms/mooneye/emulator-only/mbc5/rom_1Mb.gb"},
  mbc5_rom_2mb = {"tests/roms/mooneye/emulator-only/mbc5/rom_2Mb.gb"},
  mbc5_rom_4mb = {"tests/roms/mooneye/emulator-only/mbc5/rom_4Mb.gb"},
  mbc5_rom_8mb = {"tests/roms/mooneye/emulator-only/mbc5/rom_8Mb.gb"},
  mbc5_rom_16mb = {"tests/roms/mooneye/emulator-only/mbc5/rom_16Mb.gb"},
  mbc5_rom_32mb = {"tests/roms/mooneye/emulator-only/mbc5/rom_32Mb.gb"},
  mbc5_rom_64mb = {"tests/roms/mooneye/emulator-only/mbc5/rom_64Mb.gb"},
  mbc5_rom_512kb = {"tests/roms/mooneye/emulator-only/mbc5/rom_512kb.gb"},
  oam_dma_basic = {"tests/roms/mooneye/acceptance/oam_dma/basic.gb"},
  oam_dma_restart = {"tests/roms/mooneye/acceptance/oam_dma/oam_dma_restart.gb"},
  oam_dma_start = {"tests/roms/mooneye/acceptance/oam_dma/oam_dma_start.gb"},
  oam_dma_timing = {"tests/roms/mooneye/acceptance/oam_dma/oam_dma_timing.gb"},
  oam_dma_reg_read = {"tests/roms/mooneye/acceptance/oam_dma/reg_read.gb"},
  oam_dma_sources = {"tests/roms/mooneye/acceptance/oam_dma/sources-GS.gb"},
//ppu_hblank_ly_scx_timing = {"tests/roms/mooneye/acceptance/ppu/hblank_ly_scx_timing-GS.gb"},
//ppu_intr_1_2_timing = {"tests/roms/mooneye/acceptance/ppu/intr_1_2_timing-GS.gb"},
//ppu_intr_2_0_timing = {"tests/roms/mooneye/acceptance/ppu/intr_2_0_timing.gb"},
//ppu_intr_2_mode0_timing = {"tests/roms/mooneye/acceptance/ppu/intr_2_mode0_timing.gb"},
//ppu_intr_2_mode0_timing_sprites = {"tests/roms/mooneye/acceptance/ppu/intr_2_mode0_timing_sprites.gb"},
//ppu_intr_2_mode3_timing = {"tests/roms/mooneye/acceptance/ppu/intr_2_mode3_timing.gb"},
//ppu_intr_2_oam_ok_timing = {"tests/roms/mooneye/acceptance/ppu/intr_2_oam_ok_timing.gb"},
  ppu_lcdon_timing = {"tests/roms/mooneye/acceptance/ppu/lcdon_timing-GS.gb"},
  ppu_lcdon_write_timing = {"tests/roms/mooneye/acceptance/ppu/lcdon_write_timing-GS.gb"},
  ppu_stat_irq_blocking = {"tests/roms/mooneye/acceptance/ppu/stat_irq_blocking.gb"},
  ppu_stat_lyc_onoff = {"tests/roms/mooneye/acceptance/ppu/stat_lyc_onoff.gb"},
//ppu_vblank_stat_intr = {"tests/roms/mooneye/acceptance/ppu/vblank_stat_intr-GS.gb"},
  serial_boot_sclk_align = {"tests/roms/mooneye/acceptance/serial/boot_sclk_align-dmgABCmgb.gb"},
  timer_div_timing = {"tests/roms/mooneye/acceptance/timer/div_timing.gb"},
  timer_div_write = {"tests/roms/mooneye/acceptance/timer/div_write.gb"},
  timer_rapid_toggle = {"tests/roms/mooneye/acceptance/timer/rapid_toggle.gb"},
  timer_tim00 = {"tests/roms/mooneye/acceptance/timer/tim00.gb"},
  timer_tim00_div_trigger = {"tests/roms/mooneye/acceptance/timer/tim00_div_trigger.gb"},
  timer_tim01 = {"tests/roms/mooneye/acceptance/timer/tim01.gb"},
  timer_tim01_div_trigger = {"tests/roms/mooneye/acceptance/timer/tim01_div_trigger.gb"},
  timer_tim10 = {"tests/roms/mooneye/acceptance/timer/tim10.gb"},
  timer_tim10_div_trigger = {"tests/roms/mooneye/acceptance/timer/tim10_div_trigger.gb"},
  timer_tim11 = {"tests/roms/mooneye/acceptance/timer/tim11.gb"},
  timer_tim11_div_trigger = {"tests/roms/mooneye/acceptance/timer/tim11_div_trigger.gb"},
  timer_tima_reload = {"tests/roms/mooneye/acceptance/timer/tima_reload.gb"},
  timer_tima_write_reloding = {"tests/roms/mooneye/acceptance/timer/tima_write_reloading.gb"},
  timer_tma_write_reloading = {"tests/roms/mooneye/acceptance/timer/tma_write_reloading.gb"},
)]
fn test_rom_register(input: &str) -> Result<(), Box<dyn std::error::Error>> {
    let rom = fs::read(input)?;
    let cartridge = Cartridge::new(rom)?;
    let mut gameboy = GameboyHardware::new(cartridge);

    loop {
        gameboy.step();
        let pc = gameboy.register_u16(RegisterU16::PC);
        let result = gameboy.memory(pc);
        // Find LD B,B
        if result == 0x40 {
            break;
        }
    }

    assert_eq!(gameboy.register_u8(RegisterU8::B), 3);
    assert_eq!(gameboy.register_u8(RegisterU8::C), 5);
    assert_eq!(gameboy.register_u8(RegisterU8::D), 8);
    assert_eq!(gameboy.register_u8(RegisterU8::E), 13);
    assert_eq!(gameboy.register_u8(RegisterU8::H), 21);
    assert_eq!(gameboy.register_u8(RegisterU8::L), 34);

    Ok(())
}
