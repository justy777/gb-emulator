use crate::error::TryFromUintError;
use crate::interrupts::{Interrupt, InterruptFlags};

const VIDEO_RAM_SIZE: usize = 8 * 1024;
const SPRITE_RAM_SIZE: usize = 0xFE9F - 0xFE00 + 1;

const MEM_DISPLAY_CONTROL: u16 = 0xFF40;
const MEM_DISPLAY_STATUS: u16 = 0xFF41;
const MEM_SCROLL_Y: u16 = 0xFF42;
const MEM_SCROLL_X: u16 = 0xFF43;
const MEM_LY: u16 = 0xFF44;
const MEM_LYC: u16 = 0xFF45;
const MEM_TRANSFER_AND_START_ADDRESS: u16 = 0xFF46;
const MEM_BACKGROUND_PALETTE_DATA: u16 = 0xFF47;
const MEM_OBJECT_PALETTE_0_DATA: u16 = 0xFF48;
const MEM_OBJECT_PALETTE_1_DATA: u16 = 0xFF49;
const MEM_WINDOW_Y: u16 = 0xFF4A;
const MEM_WINDOW_X: u16 = 0xFF4B;

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
    background_palette_data: u8,
    // OBP0
    object_palette_0_data: u8,
    // OBP1
    object_palette_1_data: u8,
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
            background_palette_data: 0xFC,
            object_palette_0_data: 0xFF,
            object_palette_1_data: 0xFF,
            window_y: 0,
            window_x: 0,
            frame_cycles: 0,
        }
    }

    pub fn step(&mut self, interrupt_flags: &mut InterruptFlags) {
        self.frame_cycles = (self.frame_cycles + 4) % CYCLES_PER_FRAME;
        self.ly = (self.frame_cycles / CYCLES_PER_LINE) as u8;
        if self.frame_cycles == (144 * CYCLES_PER_LINE) {
            interrupt_flags.set(Interrupt::VBlank, true);
        }
    }

    pub const fn read_vram(&self, addr: u16) -> u8 {
        self.video_ram[addr as usize]
    }

    pub fn write_vram(&mut self, addr: u16, data: u8) {
        self.video_ram[addr as usize] = data;
    }

    pub const fn read_sprite(&self, addr: u16) -> u8 {
        self.sprite_ram[addr as usize]
    }

    pub fn write_sprite(&mut self, addr: u16, data: u8) {
        self.sprite_ram[addr as usize] = data;
    }

    pub(crate) const fn get_sprite_transfer_addr(&self) -> u16 {
        self.sprite_transfer_addr
    }

    pub(crate) fn set_sprite_transfer_addr(&mut self, addr: u16) {
        self.sprite_transfer_addr = addr;
    }

    pub const fn read_display(&self, addr: u16) -> u8 {
        match addr {
            MEM_DISPLAY_CONTROL => self.control.bits(),
            MEM_DISPLAY_STATUS => self.status.bits(),
            MEM_SCROLL_Y => self.scroll_y,
            MEM_SCROLL_X => self.scroll_x,
            MEM_LY => self.ly,
            MEM_LYC => self.lyc,
            MEM_TRANSFER_AND_START_ADDRESS => (self.sprite_transfer_addr >> 8) as u8,
            MEM_BACKGROUND_PALETTE_DATA => self.background_palette_data,
            MEM_OBJECT_PALETTE_0_DATA => self.object_palette_0_data,
            MEM_OBJECT_PALETTE_1_DATA => self.object_palette_1_data,
            MEM_WINDOW_Y => self.window_y,
            MEM_WINDOW_X => self.window_x,
            _ => unreachable!(),
        }
    }

    pub fn write_display(&mut self, addr: u16, value: u8) {
        match addr {
            MEM_DISPLAY_CONTROL => self.control = DisplayControl::from_bits(value),
            MEM_DISPLAY_STATUS => self.status = DisplayStatus::from_bits(value),
            MEM_SCROLL_Y => self.scroll_y = value,
            MEM_SCROLL_X => self.scroll_x = value,
            MEM_LY => self.ly = value,
            MEM_LYC => self.lyc = value,
            MEM_TRANSFER_AND_START_ADDRESS => self.sprite_transfer_addr = (value as u16) << 8,
            MEM_BACKGROUND_PALETTE_DATA => self.background_palette_data = value,
            MEM_OBJECT_PALETTE_0_DATA => self.object_palette_0_data = value,
            MEM_OBJECT_PALETTE_1_DATA => self.object_palette_1_data = value,
            MEM_WINDOW_Y => self.window_y = value,
            MEM_WINDOW_X => self.window_x = value,
            _ => unreachable!(),
        }
    }
}
