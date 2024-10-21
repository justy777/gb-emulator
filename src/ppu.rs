use crate::bits;
use crate::error::TryFromUintError;
use crate::interrupts::InterruptFlags;
use crate::util::bit;
use bitflags::bitflags;
use std::cmp::{min, PartialEq};
use std::collections::VecDeque;

const VIDEO_RAM_SIZE: usize = 8 * 1024;
const SPRITE_RAM_SIZE: usize = 0xFE9F - 0xFE00 + 1;

const MEM_LCDC: u16 = 0xFF40;
const MEM_STAT: u16 = 0xFF41;
const MEM_SCY: u16 = 0xFF42;
const MEM_SCX: u16 = 0xFF43;
const MEM_LY: u16 = 0xFF44;
const MEM_LYC: u16 = 0xFF45;
const MEM_DMA: u16 = 0xFF46;
const MEM_BGP: u16 = 0xFF47;
const MEM_OBP0: u16 = 0xFF48;
const MEM_OPB1: u16 = 0xFF49;
const MEM_WY: u16 = 0xFF4A;
const MEM_WX: u16 = 0xFF4B;

bitflags! {
    #[repr(transparent)]
    #[derive(Debug, Clone, Copy)]
    struct DisplayControl: u8 {
        const DISPLAY_ENABLE = bit(7);
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
        const COINCIDENCE = bit(2);
        const PPU_MODE = bits![0, 1];
    }
}

impl DisplayStatus {
    fn get_ppu_mode(self) -> PpuMode {
        let ppu_mode = self.intersection(Self::PPU_MODE);
        PpuMode::try_from(ppu_mode.bits()).unwrap()
    }

    fn set_ppu_mode(&mut self, ppu_mode: PpuMode) {
        let ppu_mode_bits = ppu_mode as u8;
        let retain = self.bits() & !Self::PPU_MODE.bits();
        *self = Self::from_bits_truncate(retain | ppu_mode_bits);
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
    display_control: DisplayControl,
    // STAT
    display_status: DisplayStatus,
    // SCY
    background_y: u8,
    // SCX
    background_x: u8,
    // LY
    scanline: u8,
    // LYC
    scanline_compare: u8,
    // DMA
    // TODO: Implement direct transfer to sprite ram
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
    current_frame_dot: usize,
    sprite_buffer: Vec<Sprite>,
    background_queue: VecDeque<u8>,
    sprite_queue: VecDeque<u8>,
    // Used to check for rising edge
    interrupt_signal: bool,
}

impl Ppu {
    pub const fn new() -> Self {
        Self {
            video_ram: [0; VIDEO_RAM_SIZE],
            sprite_ram: [0; SPRITE_RAM_SIZE],
            display_control: DisplayControl::empty(),
            display_status: DisplayStatus::empty(),
            background_y: 0,
            background_x: 0,
            scanline: 0,
            scanline_compare: 0,
            sprite_transfer_source_address: 0,
            background_palette_data: 0,
            sprite_palette_0_data: 0,
            sprite_palette_1_data: 0,
            window_y: 0,
            window_x: 0,
            current_frame_dot: 0,
            sprite_buffer: Vec::new(),
            background_queue: VecDeque::new(),
            sprite_queue: VecDeque::new(),
            interrupt_signal: false,
        }
    }

    // TODO: When should the mode change?
    // TODO: when should stat interrupt be checked?
    // 4 dots per single speed M-cycle
    // 2 dots per double speed M-cycle
    pub fn step(&mut self, dots: usize, interrupt_flag: &mut InterruptFlags) {
        if !self.display_control.contains(DisplayControl::DISPLAY_ENABLE) {
            return;
        }

        let mut ppu_mode = self.display_status.get_ppu_mode();
        let mut dot_count = dots;

        while dot_count > 0 {
            if ppu_mode == PpuMode::HBlank {
                let dot_cost = min(dot_count, 4);
                self.current_frame_dot += dot_cost;
                dot_count -= dot_cost;

                // End of scanline
                if self.current_frame_dot % 456 == 0 {
                    if self.current_frame_dot / 456 == 144 {
                        ppu_mode = PpuMode::VBlank;
                        self.display_status.set_ppu_mode(ppu_mode);
                        interrupt_flag.set(InterruptFlags::VBLANK, true);
                    } else {
                        ppu_mode = PpuMode::SpriteScan;
                        self.display_status.set_ppu_mode(ppu_mode);
                    }
                    self.scanline += 1;
                    let coincidence = self.scanline == self.scanline_compare;
                    self.display_status
                        .set(DisplayStatus::COINCIDENCE, coincidence);
                }
            } else if ppu_mode == PpuMode::VBlank {
                self.current_frame_dot += 4;
                dot_count -= 4;

                if self.current_frame_dot == 70224 {
                    ppu_mode = PpuMode::SpriteScan;
                    self.display_status.set_ppu_mode(ppu_mode);
                    self.current_frame_dot = 0;
                    self.scanline = 0;
                    let coincidence = self.scanline == self.scanline_compare;
                    self.display_status
                        .set(DisplayStatus::COINCIDENCE, coincidence);
                }
            } else if ppu_mode == PpuMode::SpriteScan {
                let index = (self.current_frame_dot % 456) * 2;

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

                // Checks if this is a tall sprite
                let sprite_height = if self.display_control.contains(DisplayControl::SPRITE_SIZE) {
                    16
                } else {
                    8
                };

                if self.sprite_buffer.len() < 10
                    && sprite.x > 0
                    && self.scanline + 16 >= sprite.y
                    && self.scanline + 16 <= sprite.y + sprite_height
                {
                    self.sprite_buffer.push(sprite);
                }

                self.current_frame_dot += 2;
                dot_count -= 2;

                if self.current_frame_dot % 456 == 80 {
                    ppu_mode = PpuMode::Draw;
                    self.display_status.set_ppu_mode(ppu_mode);
                }
            } else if ppu_mode == PpuMode::Draw {
                // TODO: implement DRAW Mode
            }
        }
    }

    const fn is_inside_window(&self, x: u8, y: u8) -> bool {
        let top = self.window_y;
        let left = self.window_x;
        // Wraps at 256
        let bottom = self.window_y.wrapping_add(143);
        // Wraps at 256
        let right = self.window_x.wrapping_add(159);

        let contains_x = if left > right {
            (left <= x) != (x <= right)
        } else {
            (left <= x) && (x <= right)
        };

        let contains_y = if top > bottom {
            (top <= y) != (y <= bottom)
        } else {
            (top <= y) && (y <= bottom)
        };

        contains_x && contains_y
    }

    /// Checks that display is enabled or in `VBlank` mode.
    /// If this invariant is broken the display may be damaged.
    fn check_display_disable_invariant(&self) -> bool {
        self.display_control
            .contains(DisplayControl::DISPLAY_ENABLE)
            || self.display_status.get_ppu_mode() == PpuMode::VBlank
    }

    fn check_stat_interrupt_signal(&mut self, interrupt_flag: &mut InterruptFlags) {
        let mut new_signal = false;

        if self.display_status.contains(DisplayStatus::LYC) {
            new_signal |= self.display_status.contains(DisplayStatus::COINCIDENCE);
        }
        if self.display_status.contains(DisplayStatus::MODE_2) {
            new_signal |= self.display_status.get_ppu_mode() == PpuMode::SpriteScan;
        }
        if self.display_status.contains(DisplayStatus::MODE_1) {
            new_signal |= self.display_status.get_ppu_mode() == PpuMode::VBlank;
        }
        if self.display_status.contains(DisplayStatus::MODE_0) {
            new_signal |= self.display_status.get_ppu_mode() == PpuMode::HBlank;
        }

        if !self.interrupt_signal && new_signal {
            interrupt_flag.set(InterruptFlags::STAT, true);
        }
        self.interrupt_signal = new_signal;
    }

    pub fn read_vram(&self, addr: u16) -> u8 {
        let ppu_mode = self.display_status.get_ppu_mode();
        // VRAM is inaccessible during draw (Mode 3)
        if ppu_mode == PpuMode::Draw {
            return 0xFF;
        }

        self.video_ram[addr as usize]
    }

    pub fn write_vram(&mut self, addr: u16, data: u8) {
        let ppu_mode = self.display_status.get_ppu_mode();
        // VRAM is inaccessible during draw (Mode 3)
        if ppu_mode == PpuMode::Draw {
            return;
        }

        self.video_ram[addr as usize] = data;
    }

    pub fn read_sprite(&self, addr: u16) -> u8 {
        let ppu_mode = self.display_status.get_ppu_mode();
        // VRAM is inaccessible during sprite scan (Mode 2) or draw (Mode 3)
        if ppu_mode == PpuMode::SpriteScan || ppu_mode == PpuMode::Draw {
            return 0xFF;
        }

        self.sprite_ram[addr as usize]
    }

    pub fn write_sprite(&mut self, addr: u16, data: u8) {
        let ppu_mode = self.display_status.get_ppu_mode();
        // VRAM is inaccessible during sprite scan (Mode 2) or draw (Mode 3)
        if ppu_mode == PpuMode::SpriteScan || ppu_mode == PpuMode::Draw {
            return;
        }

        self.sprite_ram[addr as usize] = data;
    }

    pub const fn read_display(&self, address: u16) -> u8 {
        match address {
            MEM_LCDC => self.display_control.bits(),
            MEM_STAT => self.display_status.bits(),
            MEM_SCY => self.background_y,
            MEM_SCX => self.background_x,
            MEM_LY => self.scanline,
            MEM_LYC => self.scanline_compare,
            MEM_DMA => self.sprite_transfer_source_address,
            MEM_BGP => self.background_palette_data,
            MEM_OBP0 => self.sprite_palette_0_data,
            MEM_OPB1 => self.sprite_palette_1_data,
            MEM_WY => self.window_y,
            MEM_WX => self.window_x,
            _ => unreachable!(),
        }
    }

    pub fn write_display(&mut self, address: u16, value: u8) {
        match address {
            MEM_LCDC => {
                self.display_control = DisplayControl::from_bits_truncate(value);
                assert!(
                    self.check_display_disable_invariant(),
                    "Aborting to avoid damage to the screen. Display disabled while not in VBlank."
                );
            }
            MEM_STAT => self.display_status = DisplayStatus::from_bits_truncate(value),
            MEM_SCY => self.background_y = value,
            MEM_SCX => self.background_x = value,
            // LY is read-only
            MEM_LYC => {
                self.scanline_compare = value;
                let coincidence = self.scanline == self.scanline_compare;
                self.display_status
                    .set(DisplayStatus::COINCIDENCE, coincidence);
            }
            MEM_DMA => self.sprite_transfer_source_address = value,
            MEM_BGP => self.background_palette_data = value,
            MEM_OBP0 => self.sprite_palette_0_data = value,
            MEM_OPB1 => self.sprite_palette_1_data = value,
            MEM_WY => self.window_y = value,
            MEM_WX => self.window_x = value,
            _ => unreachable!(),
        }
    }
}
