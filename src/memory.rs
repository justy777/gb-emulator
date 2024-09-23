use crate::cartridge::Cartridge;
use crate::util::{Bit, Size};
use bitflags::bitflags;

const MEM_JOYPAD: u16 = 0xFF00;
const MEM_IRQ_FLAG: u16 = 0xFF0F;
const MEM_IRQ_ENABLE: u16 = 0xFFFF;

const VIDEO_RAM_SIZE: Size = Size::from_kilobytes(8);
const WORK_RAM_SIZE: Size = Size::from_kilobytes(8);
const OBJECT_ATTRIBUTE_MEMORY_SIZE: usize = 0xFE9F - 0xFE00 + 1;
const HIGH_RAM_SIZE: usize = 0xFFFE - 0xFF80 + 1;

#[derive(Debug, Clone)]
struct IORegisters {
    joypad: u8,
    interrupt_flag: InterruptFlags,
    // TODO: implement all IO Registers
}

impl IORegisters {
    const fn new() -> Self {
        Self {
            joypad: u8::MAX,
            interrupt_flag: InterruptFlags::empty(),
        }
    }

    fn read_byte(&self, address: u16) -> u8 {
        match address {
            MEM_JOYPAD => self.joypad,
            MEM_IRQ_FLAG => self.interrupt_flag.bits(),
            _ => panic!("Register is not mapped {address:X}"),
        }
    }

    fn write_byte(&mut self, address: u16, value: u8) {
        match address {
            MEM_JOYPAD => self.joypad = value,
            MEM_IRQ_FLAG => self.interrupt_flag = InterruptFlags::from_bits_truncate(value),
            _ => panic!("Register is not mapped {address:X}"),
        }
    }
}

bitflags! {
    #[repr(transparent)]
    #[derive(Debug, Clone, Copy)]
    pub(crate) struct InterruptFlags: u8 {
        const VBLANK = Bit::new(0).as_u8();
        const LCD = Bit::new(1).as_u8();
        const TIMER = Bit::new(2).as_u8();
        const SERIAL = Bit::new(3).as_u8();
        const JOYPAD = Bit::new(4).as_u8();
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
            0xFF00..=0xFF7F => {
                self.io_registers.read_byte(address)
            }
            0xFF80..=0xFFFE => {
                let offset = (address - 0xFF80) as usize;
                self.high_ram[offset]
            }
            MEM_IRQ_ENABLE => self.interrupt_enable.bits(),
            0xE000..=0xFDFF | 0xFEA0..=0xFEFF => panic!("Use of this area is prohibited {address:#X}"),
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
            MEM_IRQ_ENABLE => self.interrupt_enable = InterruptFlags::from_bits_truncate(value),
            0xE000..=0xFDFF | 0xFEA0..=0xFEFF => panic!("Use of this area is prohibited {address:#X}"),
        }
    }

    pub(crate) fn read_joypad(&self) -> u8 {
        self.read_byte(MEM_JOYPAD)
    }

    pub(crate) fn read_interrupt_flag(&self) -> InterruptFlags {
        let byte = self.read_byte(MEM_IRQ_FLAG);
        InterruptFlags::from_bits_truncate(byte)
    }

    pub(crate) fn write_interrupt_flag(&mut self, value: InterruptFlags) {
        self.write_byte(MEM_IRQ_FLAG, value.bits());
    }

    pub(crate) const fn read_interrupt_enable(&self) -> InterruptFlags {
        self.interrupt_enable
    }
}
