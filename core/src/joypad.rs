#[derive(Debug, Clone, Copy)]
pub enum Button {
    A,
    B,
    Select,
    Start,
    Right,
    Left,
    Up,
    Down,
}

#[derive(Debug, Clone, Copy)]
pub struct Joypad(u8);

impl Joypad {
    const SELECT_BUTTONS: u8 = 0b0010_0000;
    const SELECT_D_PAD: u8 = 0b0001_0000;
    const START_DOWN: u8 = 0b0000_1000;
    const SELECT_UP: u8 = 0b0000_0100;
    const B_LEFT: u8 = 0b0000_0010;
    const A_RIGHT: u8 = 0b0000_0001;
    const UNUSED: u8 = 0b1100_0000;

    pub const fn new() -> Self {
        Self::from_bits(0xCF)
    }

    pub const fn from_bits(byte: u8) -> Self {
        Self(byte | Self::UNUSED)
    }

    pub const fn bits(self) -> u8 {
        self.0
    }

    pub const fn is_pressed(self, button: Button) -> bool {
        match button {
            Button::A => self.0 & (Self::SELECT_BUTTONS | Self::A_RIGHT) == 0x00,
            Button::B => self.0 & (Self::SELECT_BUTTONS | Self::B_LEFT) == 0x00,
            Button::Select => self.0 & (Self::SELECT_BUTTONS | Self::SELECT_UP) == 0x00,
            Button::Start => self.0 & (Self::SELECT_BUTTONS | Self::START_DOWN) == 0x00,
            Button::Right => self.0 & (Self::SELECT_D_PAD | Self::A_RIGHT) == 0x00,
            Button::Left => self.0 & (Self::SELECT_D_PAD | Self::B_LEFT) == 0x00,
            Button::Up => self.0 & (Self::SELECT_D_PAD | Self::SELECT_UP) == 0x00,
            Button::Down => self.0 & (Self::SELECT_D_PAD | Self::START_DOWN) == 0x00,
        }
    }
}
