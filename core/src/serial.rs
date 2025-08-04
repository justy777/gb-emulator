use crate::interrupt::{Interrupt, InterruptFlags};

const MEM_SB: u16 = 0xFF01;
const MEM_SC: u16 = 0xFF02;

const SC_UNUSED: u8 = 0x7E;
const TRANSFER_ENABLED_MASK: u8 = 0x80;
const CLOCK_SELECT_MASK: u8 = 0x01;

#[derive(Debug, Clone, Copy)]
enum ClockSelect {
    External = 0,
    Internal = 1,
}

impl From<u8> for ClockSelect {
    fn from(value: u8) -> Self {
        match value & 0b1 {
            0 => Self::External,
            1 => Self::Internal,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum TransferState {
    Idle,
    Ongoing(u8),
}

#[derive(Debug, Clone)]
pub struct SerialPort {
    // SB
    data: u8,
    transfer_enabled: bool,
    clock_select: ClockSelect,
    state: TransferState,
}

impl SerialPort {
    pub const fn new() -> Self {
        Self {
            data: 0,
            transfer_enabled: false,
            clock_select: ClockSelect::External,
            state: TransferState::Idle,
        }
    }

    pub const fn step(&mut self, interrupt_flags: &mut InterruptFlags) {
        if self.transfer_enabled {
            self.state = match self.state {
                TransferState::Ongoing(8) => {
                    self.transfer_enabled = false;
                    interrupt_flags.set(Interrupt::Serial, true);

                    TransferState::Idle
                }
                TransferState::Ongoing(n) => TransferState::Ongoing(n + 1),
                TransferState::Idle => TransferState::Idle,
            }
        }
    }

    pub const fn read_byte(&self, addr: u16) -> u8 {
        match addr {
            MEM_SB => self.data,
            MEM_SC => self.sc_bits(),
            _ => unreachable!(),
        }
    }

    pub fn write_byte(&mut self, addr: u16, value: u8) {
        match addr {
            MEM_SB => self.data = value,
            MEM_SC => self.set_sc(value),
            _ => unreachable!(),
        }
    }

    const fn sc_bits(&self) -> u8 {
        let mut bits = SC_UNUSED;
        if self.transfer_enabled {
            bits |= TRANSFER_ENABLED_MASK;
        }
        bits |= self.clock_select as u8;
        bits
    }

    fn set_sc(&mut self, value: u8) {
        self.transfer_enabled = value & TRANSFER_ENABLED_MASK != 0;
        self.clock_select = ClockSelect::from(value & CLOCK_SELECT_MASK);
        if self.transfer_enabled {
            self.state = TransferState::Ongoing(0);
        } else {
            self.state = TransferState::Idle;
        }
    }
}
