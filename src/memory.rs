use crate::cartridge::Cartridge;
use crate::memory::io::serial_transfer::SerialTransferControl;
use crate::util::DataSize;
use io::interrupts::InterruptFlags;
use io::joypad::Joypad;
use io::IORegisters;

pub(crate) mod io;

const VIDEO_RAM_SIZE: DataSize = DataSize::from_kilobytes(8);
const WORK_RAM_SIZE: DataSize = DataSize::from_kilobytes(8);
const OBJECT_ATTRIBUTE_MEMORY_SIZE: usize = 0xFE9F - 0xFE00 + 1;
const HIGH_RAM_SIZE: usize = 0xFFFE - 0xFF80 + 1;
const MEM_INTERRUPT_ENABLE: u16 = 0xFFFF;

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

    pub fn tick(&mut self, cycles: usize) {
        self.io_registers
            .timer
            .tick(cycles, &mut self.io_registers.interrupt_flag);
    }

    pub(crate) const fn get_joypad(&self) -> Joypad {
        self.io_registers.joypad
    }

    pub(crate) fn set_joypad(&mut self, joypad: Joypad) {
        self.io_registers.joypad = joypad;
    }

    pub(crate) const fn get_serial_transfer_data(&self) -> u8 {
        self.io_registers.serial_transfer.data
    }

    pub(crate) fn set_serial_transfer_data(&mut self, value: u8) {
        self.io_registers.serial_transfer.data = value;
    }

    pub(crate) const fn get_serial_transfer_control(&self) -> SerialTransferControl {
        self.io_registers.serial_transfer.control
    }

    pub(crate) fn set_serial_transfer_control(&mut self, value: SerialTransferControl) {
        self.io_registers.serial_transfer.control = value;
    }

    pub(crate) const fn get_interrupt_flag(&self) -> InterruptFlags {
        self.io_registers.interrupt_flag
    }

    pub(crate) fn set_interrupt_flag(&mut self, value: InterruptFlags) {
        self.io_registers.interrupt_flag = value;
    }

    pub(crate) const fn get_interrupt_enable(&self) -> InterruptFlags {
        self.interrupt_enable
    }

    pub(crate) fn set_interrupt_enable(&mut self, value: InterruptFlags) {
        self.interrupt_enable = value;
    }
}
