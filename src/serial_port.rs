const MEM_SERIAL_TRANSFER_DATA: u16 = 0xFF01;
const MEM_SERIAL_TRANSFER_CONTROL: u16 = 0xFF02;

#[derive(Debug, Clone, Copy)]
pub struct SerialTransferControl(u8);

impl SerialTransferControl {
    const TRANSFER_ENABLE: u8 = 0b1000_0000;
    const CLOCK_SELECT: u8 = 0b0000_0001;
    const UNUSED: u8 = 0b0111_1110;
    const TRANSFER_REQUESTED: u8 = Self::TRANSFER_ENABLE | Self::CLOCK_SELECT;

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

    const fn is_transfer_requested(self) -> bool {
        self.0 & Self::TRANSFER_REQUESTED == Self::TRANSFER_REQUESTED
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
            control: SerialTransferControl::empty(),
        }
    }

    pub fn step(&mut self) {
        if self.control.is_transfer_requested() {
            //let c = char::from(self.data);
            //print!("{c}");
            println!("{}", self.data);
            self.control.set_transfer_enable(false);
        }
    }

    pub const fn read_byte(&self, addr: u16) -> u8 {
        match addr {
            MEM_SERIAL_TRANSFER_DATA => self.data,
            MEM_SERIAL_TRANSFER_CONTROL => self.control.bits(),
            _ => unreachable!(),
        }
    }

    pub fn write_byte(&mut self, addr: u16, value: u8) {
        match addr {
            MEM_SERIAL_TRANSFER_DATA => {
                self.data = value;
            }
            MEM_SERIAL_TRANSFER_CONTROL => {
                self.control = SerialTransferControl::from_bits(value);
            }
            _ => unreachable!(),
        }
    }
}
