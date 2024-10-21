use crate::bits;
use crate::error::TryFromUintError;
use crate::interrupts::InterruptFlags;
use crate::util::bit;
use bitflags::bitflags;
use std::cmp::{min, PartialEq};

const VIDEO_RAM_SIZE: usize = 8 * 1024;
const SPRITE_RAM_SIZE: usize = 0xFE9F - 0xFE00 + 1;

const MEM_DISPLAY_CONTROL: u16 = 0xFF40;
const MEM_DISPLAY_STATUS: u16 = 0xFF41;
const MEM_SCROLL_Y: u16 = 0xFF42;
const MEM_SCROLL_X: u16 = 0xFF43;
const MEM_SCANLINE_Y: u16 = 0xFF44;
const MEM_SCANLINE_Y_COMPARE: u16 = 0xFF45;
const MEM_SPRITE_TRANSFER_SOURCE_ADDRESS: u16 = 0xFF46;
const MEM_BACKGROUND_PALETTE_DATA: u16 = 0xFF47;
const MEM_SPRITE_PALETTE_0_DATA: u16 = 0xFF48;
const MEM_SPRITE_PALETTE_1_DATA: u16 = 0xFF49;
const MEM_WINDOW_Y: u16 = 0xFF4A;
const MEM_WINDOW_X: u16 = 0xFF4B;

bitflags! {
    #[repr(transparent)]
    #[derive(Debug, Clone, Copy)]
    struct DisplayControl: u8 {
        const DISPLAY_AND_PPU_ENABLE = bit(7);
        const WINDOW_TILE_MAP_AREA = bit(6);
        const WINDOW_ENABLE = bit(5);
        const BACKGROUND_AND_WINDOW_TILE_DATA_AREA = bit(4);
        const BACKGROUND_TILE_MAP_AREA = bit(3);
        const SPRITE_SIZE = bit(2);
        const SPRITE_ENABLE = bit(1);
        const BACKGROUND_AND_WINDOW_ENABLE = bit(0);
    }
}

bitflags! {
    #[repr(transparent)]
    #[derive(Debug, Clone, Copy)]
    struct DisplayStatus: u8 {
        const LYC = bit(6);
        const MODE_2 = bit(5);
        const MODE_1 = bit(4);
        const MODE_0 = bit(3);
        const LYC_EQ_LY = bit(2);
        const PPU_MODE = bits![0, 1];
    }
}

impl DisplayStatus {
    fn get_ppu_mode(&self) -> PpuMode {
        let ppu_mode = self.intersection(DisplayStatus::PPU_MODE);
        PpuMode::try_from(ppu_mode.bits()).unwrap()
    }

    fn set_ppu_mode(&mut self, ppu_mode: PpuMode) {
        let ppu_mode_bits = ppu_mode as u8;
        let retain = self.bits() & !DisplayStatus::PPU_MODE.bits();
        *self = DisplayStatus::from_bits_truncate(retain | ppu_mode_bits);
    }
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum PpuMode {
    HBlank = 0b00,
    VBlank = 0b01,
    SpriteScan = 0b10,
    Draw = 0b11,
}

impl From<PpuMode> for u8 {
    fn from(mode: PpuMode) -> Self {
        mode as Self
    }
}

impl TryFrom<u8> for PpuMode {
    type Error = TryFromUintError;

    fn try_from(byte: u8) -> Result<Self, Self::Error> {
        match byte {
            0b00 => Ok(Self::HBlank),
            0b01 => Ok(Self::VBlank),
            0b10 => Ok(Self::SpriteScan),
            0b11 => Ok(Self::Draw),
            _ => Err(TryFromUintError(())),
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Sprite {
    y: u8,
    x: u8,
    tile_index: u8,
    flags: SpriteFlags,
}

bitflags! {
    #[repr(transparent)]
    #[derive(Debug, Clone, Copy)]
    struct SpriteFlags: u8 {
        const PRIORITY = bit(7);
        const Y_FLIP = bit(6);
        const X_FLIP = bit(5);
        const PALETTE = bit(4);
    }
}

#[repr(u8)]
enum MonochromePalette {
    White = 0b00,
    LightGray = 0b01,
    DarkGray = 0b10,
    Black = 0b11,
}

impl From<MonochromePalette> for u8 {
    fn from(palette: MonochromePalette) -> Self {
        palette as Self
    }
}

impl TryFrom<u8> for MonochromePalette {
    type Error = TryFromUintError;

    fn try_from(byte: u8) -> Result<Self, Self::Error> {
        match byte {
            0b00 => Ok(Self::White),
            0b01 => Ok(Self::LightGray),
            0b10 => Ok(Self::DarkGray),
            0b11 => Ok(Self::Black),
            _ => Err(TryFromUintError(())),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Ppu {
    // VRAM
    video_ram: [u8; VIDEO_RAM_SIZE],
    // OAM
    sprite_ram: [u8; SPRITE_RAM_SIZE],
    // LCDC
    control: DisplayControl,
    // STAT
    status: DisplayStatus,
    // SCY
    scroll_y: u8,
    // SCX
    scroll_x: u8,
    // LY
    scanline_y: u8,
    // LYC
    scanline_y_compare: u8,
    // DMA
    sprite_transfer_source_address: u8,
    // BGP
    background_palette_data: u8,
    // OBP0
    sprite_palette_0_data: u8,
    // OBP1
    sprite_palette_1_data: u8,
    // WY
    window_y: u8,
    // WX
    window_x: u8,
    frame_cycles: usize,
    sprite_buffer: Vec<Sprite>,
}

impl Ppu {
    pub const fn new() -> Self {
        Self {
            video_ram: [0; VIDEO_RAM_SIZE],
            sprite_ram: [0; SPRITE_RAM_SIZE],
            control: DisplayControl::empty(),
            status: DisplayStatus::empty(),
            scroll_y: 0,
            scroll_x: 0,
            scanline_y: 0,
            scanline_y_compare: 0,
            sprite_transfer_source_address: 0,
            background_palette_data: 0,
            sprite_palette_0_data: 0,
            sprite_palette_1_data: 0,
            window_y: 0,
            window_x: 0,
            frame_cycles: 0,
            sprite_buffer: Vec::new(),
        }
    }

    pub fn step(&mut self, cycles: usize, _interrupt_flag: &mut InterruptFlags) {
        let mut ppu_mode = self.status.get_ppu_mode();
        let mut cycle_count = cycles;

        while cycle_count > 0 {
            if ppu_mode == PpuMode::HBlank {
                let dots = min(cycle_count, 4);
                self.frame_cycles += dots;
                cycle_count -= dots;

                let scanline = self.frame_cycles / 456;
                let dot = self.frame_cycles % 456;

                // End of scanline
                if dot == 0 {
                    if scanline == 144 {
                        ppu_mode = PpuMode::VBlank;
                    } else {
                        ppu_mode = PpuMode::SpriteScan;
                    }
                    self.scanline_y += 1;
                    let value = self.scanline_y == self.scanline_y_compare;
                    self.status.set(DisplayStatus::LYC_EQ_LY, value);
                }
            } else if ppu_mode == PpuMode::VBlank {
                self.frame_cycles += 4;
                cycle_count -= 4;

                if self.frame_cycles == 70224 {
                    ppu_mode = PpuMode::SpriteScan;
                    self.frame_cycles = 0;
                    self.scanline_y = 0;
                    let value = self.scanline_y == self.scanline_y_compare;
                    self.status.set(DisplayStatus::LYC_EQ_LY, value);
                }
            } else if ppu_mode == PpuMode::SpriteScan {
                let index = (self.frame_cycles % 456) * 2;

                let y = self.sprite_ram[index];
                let x = self.sprite_ram[index + 1];
                let tile_index = self.sprite_ram[index + 2];
                let flags = self.sprite_ram[index + 3];
                let sprite = Sprite {
                    y,
                    x,
                    tile_index,
                    flags: SpriteFlags::from_bits_truncate(flags),
                };

                // TODO: support tall sprites
                if self.sprite_buffer.len() < 10
                    && sprite.x > 0
                    && self.scanline_y + 16 >= sprite.y
                    && self.scanline_y + 16 <= sprite.y + 8
                {
                    self.sprite_buffer.push(sprite);
                }

                self.frame_cycles += 2;
                cycle_count -= 2;

                let dot = self.frame_cycles % 456;
                if dot == 80 {
                    ppu_mode = PpuMode::Draw;
                }
            }

            self.status.set_ppu_mode(ppu_mode);
        }
    }

    pub fn read_vram(&self, addr: u16) -> u8 {
        let ppu_mode = self.status.get_ppu_mode();
        // VRAM is inaccessible during draw (Mode 3)
        if ppu_mode == PpuMode::Draw {
            return 0xFF;
        }

        self.video_ram[addr as usize]
    }

    pub fn write_vram(&mut self, addr: u16, data: u8) {
        let ppu_mode = self.status.get_ppu_mode();
        // VRAM is inaccessible during draw (Mode 3)
        if ppu_mode == PpuMode::Draw {
            return;
        }

        self.video_ram[addr as usize] = data;
    }

    pub fn read_sprite(&self, addr: u16) -> u8 {
        let ppu_mode = self.status.get_ppu_mode();
        // VRAM is inaccessible during sprite scan (Mode 2) or draw (Mode 3)
        if ppu_mode == PpuMode::SpriteScan || ppu_mode == PpuMode::Draw {
            return 0xFF;
        }

        self.sprite_ram[addr as usize]
    }

    pub fn write_sprite(&mut self, addr: u16, data: u8) {
        let ppu_mode = self.status.get_ppu_mode();
        // VRAM is inaccessible during sprite scan (Mode 2) or draw (Mode 3)
        if ppu_mode == PpuMode::SpriteScan || ppu_mode == PpuMode::Draw {
            return;
        }

        self.sprite_ram[addr as usize] = data;
    }

    pub const fn read_display(&self, address: u16) -> u8 {
        match address {
            MEM_DISPLAY_CONTROL => self.control.bits(),
            MEM_DISPLAY_STATUS => self.status.bits(),
            MEM_SCROLL_Y => self.scroll_y,
            MEM_SCROLL_X => self.scroll_x,
            MEM_SCANLINE_Y => self.scanline_y,
            MEM_SCANLINE_Y_COMPARE => self.scanline_y_compare,
            MEM_SPRITE_TRANSFER_SOURCE_ADDRESS => self.sprite_transfer_source_address,
            MEM_BACKGROUND_PALETTE_DATA => self.background_palette_data,
            MEM_SPRITE_PALETTE_0_DATA => self.sprite_palette_0_data,
            MEM_SPRITE_PALETTE_1_DATA => self.sprite_palette_1_data,
            MEM_WINDOW_Y => self.window_y,
            MEM_WINDOW_X => self.window_x,
            _ => unreachable!(),
        }
    }

    pub fn write_display(&mut self, address: u16, value: u8) {
        match address {
            MEM_DISPLAY_CONTROL => self.control = DisplayControl::from_bits_truncate(value),
            MEM_DISPLAY_STATUS => self.status = DisplayStatus::from_bits_truncate(value),
            MEM_SCROLL_Y => self.scroll_y = value,
            MEM_SCROLL_X => self.scroll_x = value,
            // LY is read-only
            MEM_SCANLINE_Y_COMPARE => {
                self.scanline_y_compare = value;
                let value = self.scanline_y == self.scanline_y;
                self.status.set(DisplayStatus::LYC_EQ_LY, value);
            },
            MEM_SPRITE_TRANSFER_SOURCE_ADDRESS => self.sprite_transfer_source_address = value,
            MEM_BACKGROUND_PALETTE_DATA => self.background_palette_data = value,
            MEM_SPRITE_PALETTE_0_DATA => self.sprite_palette_0_data = value,
            MEM_SPRITE_PALETTE_1_DATA => self.sprite_palette_1_data = value,
            MEM_WINDOW_Y => self.window_y = value,
            MEM_WINDOW_X => self.window_x = value,
            _ => unreachable!(),
        }
    }
}
