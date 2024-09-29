use crate::cartridge::Cartridge;
use crate::joypad::Joypad;
use crate::timer::{Timer, TimerControl};
use crate::util::{bit, DataSize};
use bitflags::bitflags;

const MEM_JOYPAD: u16 = 0xFF00;
const MEM_SERIAL_TRANSFER_DATA: u16 = 0xFF01;
const MEM_SERIAL_TRANSFER_CONTROL: u16 = 0xFF02;
const MEM_DIVIDER_REGISTER: u16 = 0xFF04;
const MEM_TIMER_COUNTER: u16 = 0xFF05;
const MEM_TIMER_MODULO: u16 = 0xFF06;
const MEM_TIMER_CONTROL: u16 = 0xFF07;
const MEM_INTERRUPT_FLAG: u16 = 0xFF0F;
const MEM_INTERRUPT_ENABLE: u16 = 0xFFFF;

const VIDEO_RAM_SIZE: DataSize = DataSize::from_kilobytes(8);
const WORK_RAM_SIZE: DataSize = DataSize::from_kilobytes(8);
const OBJECT_ATTRIBUTE_MEMORY_SIZE: usize = 0xFE9F - 0xFE00 + 1;
const HIGH_RAM_SIZE: usize = 0xFFFE - 0xFF80 + 1;

bitflags! {
    #[repr(transparent)]
    #[derive(Debug, Clone, Copy)]
    struct SerialTransferControl: u8 {
        const TRANSFER_ENABLE = bit(7);
        const CLOCK_SELECT = bit(0);
        // TODO: Implement serial transfer
    }
}

#[derive(Debug, Clone)]
struct IORegisters {
    joypad: Joypad,
    serial_transfer_data: u8,
    serial_transfer_control: SerialTransferControl,
    timer: Timer,
    interrupt_flag: InterruptFlags,
    // TODO: implement all IO Registers
}

impl IORegisters {
    const fn new() -> Self {
        Self {
            joypad: Joypad::new(),
            serial_transfer_data: 0,
            serial_transfer_control: SerialTransferControl::empty(),
            timer: Timer::new(),
            interrupt_flag: InterruptFlags::empty(),
        }
    }

    fn read_byte(&self, address: u16) -> u8 {
        match address {
            MEM_JOYPAD => self.joypad.bits(),
            MEM_SERIAL_TRANSFER_DATA => self.serial_transfer_data,
            MEM_SERIAL_TRANSFER_CONTROL => self.serial_transfer_control.bits(),
            MEM_DIVIDER_REGISTER => self.timer.divider,
            MEM_TIMER_COUNTER => self.timer.counter,
            MEM_TIMER_MODULO => self.timer.modulo,
            MEM_TIMER_CONTROL => self.timer.control.bits(),
            MEM_INTERRUPT_FLAG => self.interrupt_flag.bits(),
            _ => panic!("IO register is not mapped {address:#X}"),
        }
    }

    fn write_byte(&mut self, address: u16, value: u8) {
        match address {
            MEM_JOYPAD => self.joypad = Joypad::from_bits_truncate(value),
            MEM_SERIAL_TRANSFER_DATA => self.serial_transfer_data = value,
            MEM_SERIAL_TRANSFER_CONTROL => {
                self.serial_transfer_control = SerialTransferControl::from_bits_truncate(value);
            }
            MEM_DIVIDER_REGISTER => self.timer.divider = value,
            MEM_TIMER_COUNTER => self.timer.counter = value,
            MEM_TIMER_MODULO => self.timer.modulo = value,
            MEM_TIMER_CONTROL => self.timer.control = TimerControl::from_bits_truncate(value),
            MEM_INTERRUPT_FLAG => self.interrupt_flag = InterruptFlags::from_bits_truncate(value),
            _ => panic!("IO register is not mapped {address:#X}"),
        }
    }
}

bitflags! {
    #[repr(transparent)]
    #[derive(Debug, Clone, Copy)]
    pub(crate) struct InterruptFlags: u8 {
        const VBLANK = bit(0);
        const LCD = bit(1);
        const TIMER = bit(2);
        const SERIAL = bit(3);
        const JOYPAD = bit(4);
    }
}

#[derive(Clone)]
pub struct AddressBus {
    // ROM and External RAM
    cartridge: Cartridge,
    // VRAM
    video_ram: [u8; VIDEO_RAM_SIZE.as_bytes()],
    // WRAM
    work_ram: [u8; WORK_RAM_SIZE.as_bytes()],
    // OAM
    object_attribute_memory: [u8; OBJECT_ATTRIBUTE_MEMORY_SIZE],
    io_registers: IORegisters,
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
            video_ram: [0; VIDEO_RAM_SIZE.as_bytes()],
            work_ram: [0; WORK_RAM_SIZE.as_bytes()],
            object_attribute_memory: [0; OBJECT_ATTRIBUTE_MEMORY_SIZE],
            io_registers: IORegisters::new(),
            high_ram: [0; HIGH_RAM_SIZE],
            interrupt_enable: InterruptFlags::empty(),
        }
    }

    pub(crate) fn read_byte(&self, address: u16) -> u8 {
        match address {
            0x0000..=0x7FFF => self.cartridge.read_rom(address),
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
                self.object_attribute_memory[offset]
            }
            0xFF00..=0xFF7F => self.io_registers.read_byte(address),
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

    pub(crate) fn write_byte(&mut self, address: u16, value: u8) {
        match address {
            0x0000..=0x7FFF => panic!("Writing to ROM is prohibited {address:#X}"),
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
                self.object_attribute_memory[offset] = value;
            }
            0xFF00..=0xFF7F => {
                self.io_registers.write_byte(address, value);
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

    pub(crate) const fn read_joypad(&self) -> Joypad {
        self.io_registers.joypad
    }

    pub(crate) fn write_joypad(&mut self, joypad: Joypad) {
        self.io_registers.joypad = joypad;
    }

    pub(crate) const fn read_interrupt_flag(&self) -> InterruptFlags {
        self.io_registers.interrupt_flag
    }

    pub(crate) fn write_interrupt_flag(&mut self, value: InterruptFlags) {
        self.io_registers.interrupt_flag = value;
    }

    pub(crate) const fn read_interrupt_enable(&self) -> InterruptFlags {
        self.interrupt_enable
    }

    pub(crate) fn write_interrupt_enable(&mut self, value: InterruptFlags) {
        self.interrupt_enable = value;
    }
}
