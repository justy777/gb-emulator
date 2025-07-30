use crate::interrupt::{Interrupt, InterruptFlags};
use std::fmt::Write;

const MEM_SB: u16 = 0xFF01;
const MEM_SC: u16 = 0xFF02;

#[derive(Debug, Clone, Copy)]
enum ClockSelect {
    External = 0,
    Internal = 1,
}

impl From<u8> for ClockSelect {
    fn from(value: u8) -> Self {
        match value & 0x01 {
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
    output: String,
}

impl SerialPort {
    pub const fn new() -> Self {
        Self {
            data: 0,
            transfer_enabled: false,
            clock_select: ClockSelect::External,
            state: TransferState::Idle,
            output: String::new(),
        }
    }

    pub fn step(&mut self, interrupt_flags: &mut InterruptFlags) {
        if self.transfer_enabled {
            self.state = match self.state {
                TransferState::Ongoing(8) => {
                    // Writes ASCII chars to screen
                    //let c = char::from(self.data);
                    //print!("{c}");
                    println!("{}", self.data);
                    // Accumulates output for tests
                    let _ = write!(self.output, "{} ", self.data);
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
            MEM_SC => self.control_bits(),
            _ => unreachable!(),
        }
    }

    pub fn write_byte(&mut self, addr: u16, value: u8) {
        match addr {
            MEM_SB => self.data = value,
            MEM_SC => self.set_control(value),
            _ => unreachable!(),
        }
    }

    pub fn output(&self) -> String {
        self.output.trim().into()
    }

    const fn control_bits(&self) -> u8 {
        let mut bits = 0x7E;
        if self.transfer_enabled {
            bits |= 0x80;
        }
        bits |= self.clock_select as u8;
        bits
    }

    fn set_control(&mut self, value: u8) {
        self.transfer_enabled = value & 0x80 == 0x80;
        self.clock_select = ClockSelect::from(value & 0x01);
        if self.transfer_enabled {
            self.state = TransferState::Ongoing(0);
        } else {
            self.state = TransferState::Idle;
        }
    }
}
