use crate::cartridge::Cartridge;
use crate::display::Display;
use crate::interrupts::InterruptFlags;
use crate::joypad::Joypad;
use crate::serial_port::SerialPort;
use crate::timer::Timer;

const VIDEO_RAM_SIZE: usize = 8 * 1024;
const WORK_RAM_SIZE: usize = 8 * 1024;
const SPRITE_RAM_SIZE: usize = 0xFE9F - 0xFE00 + 1;
const AUDIO_SIZE: usize = 0xFF3F - 0xFF10 + 1;
const HIGH_RAM_SIZE: usize = 0xFFFE - 0xFF80 + 1;
const MEM_INTERRUPT_ENABLE: u16 = 0xFFFF;

pub struct AddressBus {
    // ROM and External RAM
    cartridge: Cartridge,
    // VRAM
    video_ram: [u8; VIDEO_RAM_SIZE],
    // WRAM
    work_ram: [u8; WORK_RAM_SIZE],
    // OAM
    sprite_ram: [u8; SPRITE_RAM_SIZE],
    // P1/JOYP
    joypad: Joypad,
    serial_port: SerialPort,
    timer: Timer,
    // IF
    interrupt_flag: InterruptFlags,
    audio: [u8; AUDIO_SIZE],
    display: Display,
    // HRAM
    high_ram: [u8; HIGH_RAM_SIZE],
    // IE
    interrupt_enable: InterruptFlags,
}

impl AddressBus {
    #[must_use]
    pub const fn new(cartridge: Cartridge) -> Self {
        Self {
            cartridge,
            video_ram: [0; VIDEO_RAM_SIZE],
            work_ram: [0; WORK_RAM_SIZE],
            sprite_ram: [0; SPRITE_RAM_SIZE],
            joypad: Joypad::new(),
            serial_port: SerialPort::new(),
            timer: Timer::new(),
            interrupt_flag: InterruptFlags::empty(),
            audio: [0; AUDIO_SIZE],
            display: Display::new(),
            high_ram: [0; HIGH_RAM_SIZE],
            interrupt_enable: InterruptFlags::empty(),
        }
    }

    pub(crate) fn read_byte(&self, address: u16) -> u8 {
        match address {
            0x0000..=0x3FFF => self.cartridge.read_rom_bank0(address),
            0x4000..=0x7FFF => {
                let offset = address - 0x4000;
                self.cartridge.read_rom_bank1(offset)
            }
            0x8000..=0x9FFF => {
                let offset = (address - 0x8000) as usize;
                self.video_ram[offset]
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
                let offset = (address - 0xFE00) as usize;
                self.sprite_ram[offset]
            }
            0xFF00..=0xFF7F => self.read_io(address),
            0xFF80..=0xFFFE => {
                let offset = (address - 0xFF80) as usize;
                self.high_ram[offset]
            }
            MEM_INTERRUPT_ENABLE => self.interrupt_enable.bits(),
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
            0xFF10..=0xFF3F => {
                let offset = (address - 0xFF10) as usize;
                self.audio[offset]
            }
            0xFF40..=0xFF4B => self.display.read_byte(address),
            0xFF50 => 1,
            _ => panic!("Address {address:#X} is not mapped in I/O registers."),
        }
    }

    pub(crate) fn write_byte(&mut self, address: u16, value: u8) {
        match address {
            0x0000..=0x7FFF => self.cartridge.write_rom(address, value),
            0x8000..=0x9FFF => {
                let offset = (address - 0x8000) as usize;
                self.video_ram[offset] = value;
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
                let offset = (address - 0xFE00) as usize;
                self.sprite_ram[offset] = value;
            }
            0xFF00..=0xFF7F => {
                self.write_io(address, value);
            }
            0xFF80..=0xFFFE => {
                let offset = (address - 0xFF80) as usize;
                self.high_ram[offset] = value;
            }
            MEM_INTERRUPT_ENABLE => {
                self.interrupt_enable = InterruptFlags::from_bits_truncate(value);
            }
            0xE000..=0xFDFF | 0xFEA0..=0xFEFF => {
                panic!("Use of this area is prohibited {address:#X}")
            }
        }
    }

    fn write_io(&mut self, address: u16, value: u8) {
        match address {
            0xFF00 => self.joypad = Joypad::from_bits_truncate(value),
            0xFF01..=0xFF02 => self.serial_port.write_byte(address, value),
            0xFF04..=0xFF07 => self.timer.write_byte(address, value),
            0xFF0F => self.interrupt_flag = InterruptFlags::from_bits_truncate(value),
            0xFF10..=0xFF3F => {
                let offset = (address - 0xFF10) as usize;
                self.audio[offset] = value;
            }
            0xFF40..=0xFF4B => self.display.write_byte(address, value),
            _ => panic!("Address {address:#X} is not mapped in I/O registers."),
        }
    }

    pub fn step(&mut self, cycles: usize) {
        self.timer.tick(cycles, &mut self.interrupt_flag);
        self.serial_port.step();
    }

    pub(crate) const fn get_joypad(&self) -> Joypad {
        self.joypad
    }

    pub(crate) const fn get_interrupt_flag(&self) -> InterruptFlags {
        self.interrupt_flag
    }

    pub(crate) fn set_interrupt_flag(&mut self, value: InterruptFlags) {
        self.interrupt_flag = value;
    }

    pub(crate) const fn get_interrupt_enable(&self) -> InterruptFlags {
        self.interrupt_enable
    }
}
