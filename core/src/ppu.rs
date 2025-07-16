use crate::error::TryFromUintError;
use crate::interrupt::{Interrupt, InterruptFlags};

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
const MEM_OBP1: u16 = 0xFF49;
const MEM_WY: u16 = 0xFF4A;
const MEM_WX: u16 = 0xFF4B;

const CYCLES_PER_LINE: usize = 456;
const CYCLES_PER_FRAME: usize = 70224;

#[derive(Debug, Clone, Copy)]
struct DisplayControl(u8);

impl DisplayControl {
    const DISPLAY_AND_PPU_ENABLE: u8 = 0b1000_0000;
    const WINDOW_TILE_MAP_AREA: u8 = 0b0100_0000;
    const WINDOW_ENABLE: u8 = 0b0010_0000;
    const BACKGROUND_AND_WINDOW_TILE_DATA_AREA: u8 = 0b0001_0000;
    const BACKGROUND_TILE_MAP_AREA: u8 = 0b0000_1000;
    const SPRITE_SIZE: u8 = 0b0000_0100;
    const SPRITE_ENABLE: u8 = 0b0000_0010;
    const BACKGROUND_AND_WINDOW_ENABLE: u8 = 0b0000_0001;

    const fn new() -> Self {
        Self::from_bits(
            Self::DISPLAY_AND_PPU_ENABLE
                | Self::BACKGROUND_AND_WINDOW_TILE_DATA_AREA
                | Self::BACKGROUND_AND_WINDOW_ENABLE,
        )
    }

    const fn from_bits(bits: u8) -> Self {
        Self(bits)
    }

    const fn bits(self) -> u8 {
        self.0
    }
}

#[derive(Debug, Clone, Copy)]
struct DisplayStatus(u8);

impl DisplayStatus {
    const LYC: u8 = 0b0100_0000;
    const MODE_2: u8 = 0b0010_0000;
    const MODE_1: u8 = 0b0001_0000;
    const MODE_0: u8 = 0b0000_1000;
    const LYC_EQ_LY: u8 = 0b0000_0100;
    const PPU_MODE: u8 = 0b0000_0011;
    const UNUSED: u8 = 0b1000_0000;

    const fn empty() -> Self {
        Self::from_bits(0)
    }

    const fn from_bits(bits: u8) -> Self {
        Self(bits | Self::UNUSED)
    }

    const fn bits(self) -> u8 {
        self.0
    }
}

enum MonochromePalette {
    White,
    LightGray,
    DarkGray,
    Black,
}

impl From<MonochromePalette> for u8 {
    fn from(palette: MonochromePalette) -> Self {
        match palette {
            MonochromePalette::White => 0b00,
            MonochromePalette::LightGray => 0b01,
            MonochromePalette::DarkGray => 0b10,
            MonochromePalette::Black => 0b11,
        }
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
    ly: u8,
    // LYC
    lyc: u8,
    // DMA
    sprite_transfer_addr: u16,
    // BGP
    background_palette: u8,
    // OBP0
    sprite_palette_0: u8,
    // OBP1
    sprite_palette_1: u8,
    // WY
    window_y: u8,
    // WX
    window_x: u8,
    frame_cycles: usize,
}

impl Ppu {
    pub const fn new() -> Self {
        Self {
            video_ram: [0; VIDEO_RAM_SIZE],
            sprite_ram: [0; SPRITE_RAM_SIZE],
            control: DisplayControl::new(),
            status: DisplayStatus::empty(),
            scroll_y: 0,
            scroll_x: 0,
            ly: 0,
            lyc: 0,
            sprite_transfer_addr: 0xFF,
            background_palette: 0xFC,
            sprite_palette_0: 0xFF,
            sprite_palette_1: 0xFF,
            window_y: 0,
            window_x: 0,
            frame_cycles: 0,
        }
    }

    pub const fn step(&mut self, interrupt_flags: &mut InterruptFlags) {
        self.frame_cycles = (self.frame_cycles + 4) % CYCLES_PER_FRAME;
        self.ly = (self.frame_cycles / CYCLES_PER_LINE) as u8;
        if self.frame_cycles == (144 * CYCLES_PER_LINE) {
            interrupt_flags.set(Interrupt::VBlank, true);
        }
    }

    pub const fn read_vram(&self, addr: u16) -> u8 {
        self.video_ram[addr as usize]
    }

    pub const fn write_vram(&mut self, addr: u16, data: u8) {
        self.video_ram[addr as usize] = data;
    }

    pub const fn read_sprite(&self, addr: u16) -> u8 {
        self.sprite_ram[addr as usize]
    }

    pub const fn write_sprite(&mut self, addr: u16, data: u8) {
        self.sprite_ram[addr as usize] = data;
    }

    pub(crate) const fn sprite_transfer_addr(&self) -> u16 {
        self.sprite_transfer_addr
    }

    pub(crate) const fn set_sprite_transfer_addr(&mut self, addr: u16) {
        self.sprite_transfer_addr = addr;
    }

    pub const fn read_display(&self, addr: u16) -> u8 {
        match addr {
            MEM_LCDC => self.control.bits(),
            MEM_STAT => self.status.bits(),
            MEM_SCY => self.scroll_y,
            MEM_SCX => self.scroll_x,
            MEM_LY => self.ly,
            MEM_LYC => self.lyc,
            MEM_DMA => (self.sprite_transfer_addr >> 8) as u8,
            MEM_BGP => self.background_palette,
            MEM_OBP0 => self.sprite_palette_0,
            MEM_OBP1 => self.sprite_palette_1,
            MEM_WY => self.window_y,
            MEM_WX => self.window_x,
            _ => unreachable!(),
        }
    }

    pub const fn write_display(&mut self, addr: u16, value: u8) {
        match addr {
            MEM_LCDC => self.control = DisplayControl::from_bits(value),
            MEM_STAT => self.status = DisplayStatus::from_bits(value),
            MEM_SCY => self.scroll_y = value,
            MEM_SCX => self.scroll_x = value,
            MEM_LY => self.ly = value,
            MEM_LYC => self.lyc = value,
            MEM_DMA => self.sprite_transfer_addr = (value as u16) << 8,
            MEM_BGP => self.background_palette = value,
            MEM_OBP0 => self.sprite_palette_0 = value,
            MEM_OBP1 => self.sprite_palette_1 = value,
            MEM_WY => self.window_y = value,
            MEM_WX => self.window_x = value,
            _ => unreachable!(),
        }
    }
}
