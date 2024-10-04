#![allow(
    clippy::cast_lossless,
    clippy::option_if_let_else,
    clippy::verbose_bit_mask
)]
pub mod cartridge;
pub mod cpu;
mod error;
pub mod memory;
pub mod serial_port;
mod util;
