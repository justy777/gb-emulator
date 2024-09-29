use crate::error::TryFromUintError;

enum BackgroundPalette {
    White,
    LightGray,
    DarkGray,
    Black,
}

impl From<BackgroundPalette> for u8 {
    fn from(palette: BackgroundPalette) -> Self {
        match palette {
            BackgroundPalette::White => 0b00,
            BackgroundPalette::LightGray => 0b01,
            BackgroundPalette::DarkGray => 0b10,
            BackgroundPalette::Black => 0b11,
        }
    }
}

impl TryFrom<u8> for BackgroundPalette {
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
