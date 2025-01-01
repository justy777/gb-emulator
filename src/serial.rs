use crate::interrupt::{Interrupt, InterruptFlags};

const MEM_SB: u16 = 0xFF01;
const MEM_SC: u16 = 0xFF02;

#[derive(Debug, Clone, Copy)]
struct SerialTransferControl(u8);

impl SerialTransferControl {
    const TRANSFER_ENABLE: u8 = 0b1000_0000;
    const CLOCK_SELECT: u8 = 0b0000_0001;
    const UNUSED: u8 = 0b0111_1110;

    const fn empty() -> Self {
        Self::from_bits(0)
    }

    const fn from_bits(bits: u8) -> Self {
        Self(bits | Self::UNUSED)
    }

    const fn bits(self) -> u8 {
        self.0
    }

    fn set_transfer_enable(&mut self, enable: bool) {
        if enable {
            self.0 |= Self::TRANSFER_ENABLE;
        } else {
            self.0 &= !Self::TRANSFER_ENABLE;
        }
    }

    const fn is_transfer_enabled(self) -> bool {
        self.0 & Self::TRANSFER_ENABLE == Self::TRANSFER_ENABLE
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
    // SC
    control: SerialTransferControl,
    state: TransferState,
}

impl SerialPort {
    pub const fn new() -> Self {
        Self {
            data: 0,
            control: SerialTransferControl::empty(),
            state: TransferState::Idle,
        }
    }

    pub fn step(&mut self, interrupt_flags: &mut InterruptFlags) {
        if self.control.is_transfer_enabled() {
            self.state = match self.state {
                TransferState::Ongoing(8) => {
                    //let c = char::from(self.data);
                    //print!("{c}");
                    println!("{}", self.data);
                    self.control.set_transfer_enable(false);
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
            MEM_SC => self.control.bits(),
            _ => unreachable!(),
        }
    }

    pub fn write_byte(&mut self, addr: u16, value: u8) {
        match addr {
            MEM_SB => self.data = value,
            MEM_SC => {
                self.control = SerialTransferControl::from_bits(value);
                if self.control.is_transfer_enabled() {
                    self.state = TransferState::Ongoing(0);
                } else {
                    self.state = TransferState::Idle;
                }
            }
            _ => unreachable!(),
        }
    }
}
