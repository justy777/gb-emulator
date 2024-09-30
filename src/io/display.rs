use crate::bits;
use crate::error::TryFromUintError;
use crate::util::bit;
use bitflags::bitflags;

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

bitflags! {
    #[repr(transparent)]
    #[derive(Debug, Clone, Copy)]
    struct DisplayControl: u8 {
        const DISPLAY_AND_PPU_ENABLE = bit(7);
        const WINDOW_TILE_MAP_AREA = bit(6);
        const WINDOW_ENABLE = bit(5);
        const BACKGROUND_AND_WINDOW_TILE_DATA_AREA = bit(4);
        const BACKGROUND_TILE_MAP_AREA = bit(3);
        const OBJECT_SIZE = bit(2);
        const OBJECT_ENABLE = bit(1);
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
pub struct Display {
    control: DisplayControl,
    status: DisplayStatus,
    scroll_y: u8,
    scroll_x: u8,
    ly: u8,
    lyc: u8,
    transfer_and_start_address: u8,
    background_palette_data: u8,
    object_palette_0_data: u8,
    object_palette_1_data: u8,
    window_y: u8,
    window_x: u8,
}

impl Display {
    pub const fn new() -> Self {
        Self {
            control: DisplayControl::empty(),
            status: DisplayStatus::empty(),
            scroll_y: 0,
            scroll_x: 0,
            ly: 0,
            lyc: 0,
            transfer_and_start_address: 0,
            background_palette_data: 0,
            object_palette_0_data: 0,
            object_palette_1_data: 0,
            window_y: 0,
            window_x: 0,
        }
    }

    pub const fn read_byte(&self, address: u16) -> u8 {
        match address {
            MEM_DISPLAY_CONTROL => self.control.bits(),
            MEM_DISPLAY_STATUS => self.status.bits(),
            MEM_SCROLL_Y => self.scroll_y,
            MEM_SCROLL_X => self.scroll_x,
            MEM_LY => self.ly,
            MEM_LYC => self.lyc,
            MEM_TRANSFER_AND_START_ADDRESS => self.transfer_and_start_address,
            MEM_BACKGROUND_PALETTE_DATA => self.background_palette_data,
            MEM_OBJECT_PALETTE_0_DATA => self.object_palette_0_data,
            MEM_OBJECT_PALETTE_1_DATA => self.object_palette_1_data,
            MEM_WINDOW_Y => self.window_y,
            MEM_WINDOW_X => self.window_x,
            _ => unreachable!(),
        }
    }

    pub fn write_byte(&mut self, address: u16, value: u8) {
        match address {
            MEM_DISPLAY_CONTROL => self.control = DisplayControl::from_bits_truncate(value),
            MEM_DISPLAY_STATUS => self.status = DisplayStatus::from_bits_truncate(value),
            MEM_SCROLL_Y => self.scroll_y = value,
            MEM_SCROLL_X => self.scroll_x = value,
            MEM_LY => self.ly = value,
            MEM_LYC => self.lyc = value,
            MEM_TRANSFER_AND_START_ADDRESS => self.transfer_and_start_address = value,
            MEM_BACKGROUND_PALETTE_DATA => self.background_palette_data = value,
            MEM_OBJECT_PALETTE_0_DATA => self.object_palette_0_data = value,
            MEM_OBJECT_PALETTE_1_DATA => self.object_palette_1_data = value,
            MEM_WINDOW_Y => self.window_y = value,
            MEM_WINDOW_X => self.window_x = value,
            _ => unreachable!(),
        }
    }
}
