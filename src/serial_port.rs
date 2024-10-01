use crate::memory::io::serial_transfer::SerialTransferControl;
use crate::memory::AddressBus;

// TODO: Implement serial transfer between emulators
pub struct SerialPort {}

impl SerialPort {
    pub fn step(memory: &mut AddressBus) {
        let mut control = memory.get_serial_transfer_control();
        if control.contains(SerialTransferControl::TRANSFER_REQUESTED) {
            let data = memory.get_serial_transfer_data();
            let c = char::from(data);
            print!("{c}");
            control.set(SerialTransferControl::TRANSFER_ENABLE, false);
            memory.set_serial_transfer_control(control);
        }
    }
}
