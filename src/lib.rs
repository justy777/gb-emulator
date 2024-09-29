#[allow(clippy::cast_lossless)]
pub mod cartridge;
#[allow(clippy::cast_lossless)]
pub mod cpu;
mod error;
pub(crate) mod io;
pub mod memory;
mod ppu;
mod util;
