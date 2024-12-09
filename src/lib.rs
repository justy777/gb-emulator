#![allow(
    clippy::cast_lossless,
    clippy::option_if_let_else,
    clippy::verbose_bit_mask
)]

mod apu;
pub mod cartridge;
mod cpu;
mod error;
pub mod hardware;
mod interrupts;
mod joypad;
mod ppu;
mod serial_port;
mod timer;
mod util;
