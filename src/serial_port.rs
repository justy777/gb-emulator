use crate::util::bit;
use bitflags::bitflags;

const MEM_SERIAL_TRANSFER_DATA: u16 = 0xFF01;
const MEM_SERIAL_TRANSFER_CONTROL: u16 = 0xFF02;

bitflags! {
    #[repr(transparent)]
    #[derive(Debug, Clone, Copy)]
    pub struct SerialTransferControl: u8 {
        const TRANSFER_ENABLE = bit(7);
        const CLOCK_SELECT = bit(0);

        const TRANSFER_REQUESTED = Self::TRANSFER_ENABLE.bits() | Self::CLOCK_SELECT.bits();
    }
}

impl SerialTransferControl {
    pub const fn unknown() -> Self {
        // 0x7E
        Self::from_bits_retain(0b0111_1110)
    }
}

#[derive(Debug, Clone)]
pub struct SerialPort {
    // SB
    pub(crate) data: u8,
    // SC
    pub(crate) control: SerialTransferControl,
}

impl SerialPort {
    pub const fn new() -> Self {
        Self {
            data: 0,
            control: SerialTransferControl::unknown(),
        }
    }

    pub fn step(&mut self) {
        if self
            .control
            .contains(SerialTransferControl::TRANSFER_REQUESTED)
        {
            //let c = char::from(self.data);
            //print!("{c}");
            println!("{}", self.data);
            self.control
                .set(SerialTransferControl::TRANSFER_ENABLE, false);
        }
    }

    pub const fn read_byte(&self, address: u16) -> u8 {
        match address {
            MEM_SERIAL_TRANSFER_DATA => self.data,
            MEM_SERIAL_TRANSFER_CONTROL => self.control.bits(),
            _ => unreachable!(),
        }
    }

    pub fn write_byte(&mut self, address: u16, value: u8) {
        match address {
            MEM_SERIAL_TRANSFER_DATA => {
                self.data = value;
            }
            MEM_SERIAL_TRANSFER_CONTROL => {
                self.control = SerialTransferControl::from_bits_retain(value);
            }
            _ => unreachable!(),
        }
    }
}
