use crate::apu::Apu;
use crate::cartridge::Cartridge;
use crate::cpu::Cpu;
use crate::interrupts::InterruptFlags;
use crate::joypad::Joypad;
use crate::ppu::Ppu;
use crate::serial_port::SerialPort;
use crate::timer::Timer;

const WORK_RAM_SIZE: usize = 8 * 1024;
const WAVE_PATTERN_RAM_SIZE: usize = 0xFF3F - 0xFF30 + 1;
const HIGH_RAM_SIZE: usize = 0xFFFE - 0xFF80 + 1;

#[allow(clippy::module_name_repetitions)]
pub struct GameboyHardware {
    cpu: Cpu,
    // ROM and External RAM
    cartridge: Cartridge,
    // Picture Processing Unit
    ppu: Ppu,
    // WRAM
    work_ram: [u8; WORK_RAM_SIZE],
    // P1/JOYP
    joypad: Joypad,
    // Link Cable
    serial_port: SerialPort,
    timer: Timer,
    // IF
    interrupt_flag: InterruptFlags,
    // Audio Processing Unit
    apu: Apu,
    wave_pattern_ram: [u8; WAVE_PATTERN_RAM_SIZE],
    // HRAM
    high_ram: [u8; HIGH_RAM_SIZE],
    // IE
    interrupt_enable: InterruptFlags,
}

impl GameboyHardware {
    #[must_use]
    pub const fn new(cartridge: Cartridge) -> Self {
        Self {
            cpu: Cpu::new(),
            cartridge,
            ppu: Ppu::new(),
            work_ram: [0; WORK_RAM_SIZE],
            joypad: Joypad::new(),
            serial_port: SerialPort::new(),
            timer: Timer::new(),
            interrupt_flag: InterruptFlags::from_bits(InterruptFlags::VBLANK),
            apu: Apu::new(),
            wave_pattern_ram: [0xFF; WAVE_PATTERN_RAM_SIZE],
            high_ram: [0; HIGH_RAM_SIZE],
            interrupt_enable: InterruptFlags::empty(),
        }
    }

    pub fn step(&mut self) {
        let mut memory = AddressBus {
            cartridge: &mut self.cartridge,
            ppu: &mut self.ppu,
            work_ram: &mut self.work_ram,
            joypad: &mut self.joypad,
            serial_port: &mut self.serial_port,
            timer: &mut self.timer,
            interrupt_flag: &mut self.interrupt_flag,
            apu: &mut self.apu,
            wave_pattern_ram: &mut self.wave_pattern_ram,
            high_ram: &mut self.high_ram,
            interrupt_enable: &mut self.interrupt_enable,
        };

        let cycles = self.cpu.step(&mut memory);
        self.timer.tick(cycles / 4, &mut self.interrupt_flag);
        self.serial_port.step();
    }
}

pub struct AddressBus<'a> {
    // ROM and External RAM
    cartridge: &'a mut Cartridge,
    // Picture Processing Unit
    ppu: &'a mut Ppu,
    // WRAM
    work_ram: &'a mut [u8],
    // P1/JOYP
    joypad: &'a mut Joypad,
    // Link Cable
    serial_port: &'a mut SerialPort,
    timer: &'a mut Timer,
    // IF
    interrupt_flag: &'a mut InterruptFlags,
    // Audio Proccessing Unit
    apu: &'a mut Apu,
    wave_pattern_ram: &'a mut [u8],
    // HRAM
    high_ram: &'a mut [u8],
    // IE
    interrupt_enable: &'a mut InterruptFlags,
}

impl AddressBus<'_> {
    pub(crate) fn read_byte(&self, address: u16) -> u8 {
        match address {
            0x0000..=0x3FFF => self.cartridge.read_rom_bank0(address),
            0x4000..=0x7FFF => {
                let offset = address - 0x4000;
                self.cartridge.read_rom_bank1(offset)
            }
            0x8000..=0x9FFF => {
                let offset = address - 0x8000;
                self.ppu.read_vram(offset)
            }
            0xA000..=0xBFFF => {
                let offset = address - 0xA000;
                self.cartridge.read_ram(offset)
            }
            0xC000..=0xDFFF => {
                let offset = (address - 0xC000) as usize;
                self.work_ram[offset]
            }
            0xFE00..=0xFE9F => {
                let offset = address - 0xFE00;
                self.ppu.read_sprite(offset)
            }
            0xFF00..=0xFF7F => self.read_io(address),
            0xFF80..=0xFFFE => {
                let offset = (address - 0xFF80) as usize;
                self.high_ram[offset]
            }
            0xFFFF => self.interrupt_enable.bits(),
            0xE000..=0xFDFF | 0xFEA0..=0xFEFF => {
                panic!("Use of this area is prohibited {address:#X}")
            }
        }
    }

    fn read_io(&self, address: u16) -> u8 {
        match address {
            0xFF00 => self.joypad.bits(),
            0xFF01..=0xFF02 => self.serial_port.read_byte(address),
            0xFF04..=0xFF07 => self.timer.read_byte(address),
            0xFF0F => self.interrupt_flag.bits(),
            0xFF10..=0xFF26 => self.apu.read_audio(address),
            0xFF30..=0xFF3F => {
                let offset = (address - 0xFF30) as usize;
                self.wave_pattern_ram[offset]
            }
            0xFF40..=0xFF4B => self.ppu.read_display(address),
            _ => {
                println!("Warning: Address {address:#X} is not mapped to an I/O register.");
                0xFF
            }
        }
    }

    pub(crate) fn write_byte(&mut self, address: u16, value: u8) {
        match address {
            0x0000..=0x7FFF => self.cartridge.write_rom(address, value),
            0x8000..=0x9FFF => {
                let offset = address - 0x8000;
                self.ppu.write_vram(offset, value);
            }
            0xA000..=0xBFFF => {
                let offset = address - 0xA000;
                self.cartridge.write_ram(offset, value);
            }
            0xC000..=0xDFFF => {
                let offset = (address - 0xC000) as usize;
                self.work_ram[offset] = value;
            }
            0xFE00..=0xFE9F => {
                let offset = address - 0xFE00;
                self.ppu.write_sprite(offset, value);
            }
            0xFF00..=0xFF7F => {
                self.write_io(address, value);
            }
            0xFF80..=0xFFFE => {
                let offset = (address - 0xFF80) as usize;
                self.high_ram[offset] = value;
            }
            0xFFFF => {
                *self.interrupt_enable = InterruptFlags::from_bits(value);
            }
            0xE000..=0xFDFF | 0xFEA0..=0xFEFF => {
                panic!("Use of this area is prohibited {address:#X}")
            }
        }
    }

    fn write_io(&mut self, address: u16, value: u8) {
        match address {
            0xFF00 => *self.joypad = Joypad::from_bits(value),
            0xFF01..=0xFF02 => self.serial_port.write_byte(address, value),
            0xFF04..=0xFF07 => self.timer.write_byte(address, value),
            0xFF0F => *self.interrupt_flag = InterruptFlags::from_bits(value),
            0xFF10..=0xFF26 => self.apu.write_audio(address, value),
            0xFF30..=0xFF3F => {
                let offset = (address - 0xFF30) as usize;
                self.wave_pattern_ram[offset] = value;
            }
            0xFF40..=0xFF4B => self.ppu.write_display(address, value),
            _ => println!("Warning: Address {address:#X} is not mapped to an I/O register."),
        }
    }

    pub(crate) const fn get_joypad(&self) -> Joypad {
        *self.joypad
    }

    pub(crate) const fn interrupt_flag(&mut self) -> &mut InterruptFlags {
        self.interrupt_flag
    }

    pub(crate) fn get_interrupts_pending(&self) -> InterruptFlags {
        *self.interrupt_enable & *self.interrupt_flag
    }
}
