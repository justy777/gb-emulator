#![allow(
    clippy::cast_lossless,
    clippy::option_if_let_else,
    clippy::verbose_bit_mask
)]
pub mod cartridge;
mod cpu;
pub mod hardware;
mod error;
mod interrupts;
mod joypad;
mod ppu;
mod serial_port;
mod timer;
mod util;
