#![allow(
    clippy::cast_lossless,
    clippy::option_if_let_else,
    clippy::verbose_bit_mask
)]
pub mod cartridge;
pub mod cpu;
mod error;
mod interrupts;
mod joypad;
pub mod memory;
mod ppu;
mod serial_port;
mod timer;
mod util;
