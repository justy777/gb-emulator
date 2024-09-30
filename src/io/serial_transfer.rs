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
        // TODO: Implement serial transfer
    }
}

#[derive(Debug, Clone)]
pub struct SerialTransfer {
    data: u8,
    control: SerialTransferControl,
}

impl SerialTransfer {
    pub const fn new() -> Self {
        Self {
            data: 0,
            control: SerialTransferControl::empty(),
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
                self.control = SerialTransferControl::from_bits_truncate(value);
            }
            _ => unreachable!(),
        }
    }
}
