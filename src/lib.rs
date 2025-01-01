#![allow(
    clippy::cast_lossless,
    clippy::module_name_repetitions,
    clippy::option_if_let_else,
    clippy::verbose_bit_mask,
)]

mod apu;
pub mod cartridge;
mod cpu;
mod error;
pub mod hardware;
mod interrupt;
mod joypad;
mod ppu;
mod serial;
mod timer;
pub mod util;
