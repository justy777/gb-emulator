//! The core library of gb-emulator.
//!
//! This library holds the emulated hardware for a Game Boy (DMG) that can be reused
//! and re-exported in different implementations.

pub mod cartridge;
pub mod hardware;

mod apu;
mod cpu;
mod interrupt;
mod joypad;
mod ppu;
mod serial;
mod timer;

pub use cpu::{RegisterU8, RegisterU16};
