use crate::io::display::Display;
use crate::io::interrupts::InterruptFlags;
use crate::io::joypad::Joypad;
use crate::io::serial_transfer::SerialTransfer;
use crate::io::timer::Timer;

mod display;
pub mod interrupts;
pub mod joypad;
mod serial_transfer;
mod timer;

const MEM_JOYPAD: u16 = 0xFF00;
const MEM_SERIAL_TRANSFER_START: u16 = 0xFF01;
const MEM_SERIAL_TRANSFER_END: u16 = 0xFF02;
const MEM_TIMER_START: u16 = 0xFF04;
const MEM_TIMER_END: u16 = 0xFF07;
const MEM_INTERRUPT_FLAG: u16 = 0xFF0F;
const MEM_DISPLAY_START: u16 = 0xFF40;
const MEM_DISPLAY_END: u16 = 0xFF4B;
const MEM_DISABLE_BOOT_ROM: u16 = 0xFF50;

#[derive(Debug, Clone)]
pub struct IORegisters {
    pub(crate) joypad: Joypad,
    serial_transfer: SerialTransfer,
    timer: Timer,
    pub(crate) interrupt_flag: InterruptFlags,
    display: Display,
    // TODO: implement all IO Registers
}

impl IORegisters {
    pub(crate) const fn new() -> Self {
        Self {
            joypad: Joypad::new(),
            serial_transfer: SerialTransfer::new(),
            timer: Timer::new(),
            interrupt_flag: InterruptFlags::empty(),
            display: Display::new(),
        }
    }

    pub(crate) fn read_byte(&self, address: u16) -> u8 {
        match address {
            MEM_JOYPAD => self.joypad.bits(),
            MEM_SERIAL_TRANSFER_START..=MEM_SERIAL_TRANSFER_END => {
                self.serial_transfer.read_byte(address)
            }
            MEM_TIMER_START..=MEM_TIMER_END => self.timer.read_byte(address),
            MEM_INTERRUPT_FLAG => self.interrupt_flag.bits(),
            MEM_DISPLAY_START..=MEM_DISPLAY_END => self.display.read_byte(address),
            MEM_DISABLE_BOOT_ROM => 1,
            _ => panic!("I/O register is not mapped {address:#X}"),
        }
    }

    pub(crate) fn write_byte(&mut self, address: u16, value: u8) {
        match address {
            MEM_JOYPAD => self.joypad = Joypad::from_bits_truncate(value),
            MEM_SERIAL_TRANSFER_START..=MEM_SERIAL_TRANSFER_END => {
                self.serial_transfer.write_byte(address, value);
            }
            MEM_TIMER_START..=MEM_TIMER_END => self.timer.write_byte(address, value),
            MEM_INTERRUPT_FLAG => self.interrupt_flag = InterruptFlags::from_bits_truncate(value),
            MEM_DISPLAY_START..=MEM_DISPLAY_END => self.display.write_byte(address, value),
            _ => panic!("I/O register is not mapped {address:#X}"),
        }
    }
}
