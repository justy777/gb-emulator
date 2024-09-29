#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

#[derive(Debug, Clone, Copy, Default)]
pub struct Joypad(u8);

impl Joypad {
    pub const fn new() -> Self {
        Self(0x3F)
    }

    pub const fn from_bits_truncate(byte: u8) -> Self {
        Self(byte & 0x3F)
    }

    pub const fn bits(self) -> u8 {
        self.0
    }

    pub const fn is_any_pressed(self) -> bool {
        self.0 & 0xF != 0xF
    }

    pub const fn is_pressed(self, button: Button) -> bool {
        match button {
            Button::A => self.0 & 0x21 == 0x00,
            Button::B => self.0 & 0x22 == 0x00,
            Button::Select => self.0 & 0x24 == 0x00,
            Button::Start => self.0 & 0x28 == 0x00,
            Button::Right => self.0 & 0x11 == 0x00,
            Button::Left => self.0 & 0x12 == 0x00,
            Button::Up => self.0 & 0x14 == 0x00,
            Button::Down => self.0 & 0x18 == 0x00,
        }
    }

    pub fn press_button(&mut self, button: Button) {
        // Sets the select if different set of buttons are pressed
        match button {
            Button::A | Button::B | Button::Select | Button::Start => {
                if self.0 & 0x20 != 0x00 {
                    self.0 = 0x1F;
                }
            }
            Button::Right | Button::Left | Button::Up | Button::Down => {
                if self.0 & 0x10 != 0x00 {
                    self.0 = 0x2F;
                }
            }
        }
        // Sets the individual button
        match button {
            Button::A | Button::Right => self.0 &= 0x3E,
            Button::B | Button::Left => self.0 &= 0x3D,
            Button::Select | Button::Up => self.0 &= 0x3B,
            Button::Start | Button::Down => self.0 &= 0x37,
        }
    }

    pub fn release_button(&mut self, button: Button) {
        // Release the individual button
        match button {
            Button::A | Button::Right => self.0 |= 0x01,
            Button::B | Button::Left => self.0 |= 0x02,
            Button::Select | Button::Up => self.0 |= 0x04,
            Button::Start | Button::Down => self.0 |= 0x08,
        }
        // Release select if no buttons are pressed
        if self.0 & 0xF == 0xF {
            self.0 = 0x3F;
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::io::joypad::{Button, Joypad};

    #[test]
    fn press_buttons() {
        let mut joypad = Joypad::new();

        joypad.press_button(Button::A);
        assert!(joypad.is_pressed(Button::A));
        joypad.press_button(Button::B);
        assert!(joypad.is_pressed(Button::A));
        assert!(joypad.is_pressed(Button::B));
        joypad.press_button(Button::Select);
        assert!(joypad.is_pressed(Button::A));
        assert!(joypad.is_pressed(Button::B));
        assert!(joypad.is_pressed(Button::Select));
        joypad.press_button(Button::Start);
        assert!(joypad.is_pressed(Button::A));
        assert!(joypad.is_pressed(Button::B));
        assert!(joypad.is_pressed(Button::Select));
        assert!(joypad.is_pressed(Button::Start));

        joypad.press_button(Button::Right);
        assert!(!joypad.is_pressed(Button::A));
        assert!(!joypad.is_pressed(Button::B));
        assert!(!joypad.is_pressed(Button::Select));
        assert!(!joypad.is_pressed(Button::Start));
        assert!(joypad.is_pressed(Button::Right));
        joypad.press_button(Button::Left);
        assert!(joypad.is_pressed(Button::Right));
        assert!(joypad.is_pressed(Button::Left));
        joypad.press_button(Button::Up);
        assert!(joypad.is_pressed(Button::Right));
        assert!(joypad.is_pressed(Button::Left));
        assert!(joypad.is_pressed(Button::Up));
        joypad.press_button(Button::Down);
        assert!(joypad.is_pressed(Button::Right));
        assert!(joypad.is_pressed(Button::Left));
        assert!(joypad.is_pressed(Button::Up));
        assert!(joypad.is_pressed(Button::Down));
    }

    #[test]
    fn release_buttons() {
        let mut joypad = Joypad::new();

        joypad.press_button(Button::A);
        assert!(joypad.is_pressed(Button::A));
        joypad.release_button(Button::A);
        assert!(!joypad.is_pressed(Button::A));

        assert_eq!(joypad.bits(), 0x3F);

        joypad.press_button(Button::Up);
        assert!(joypad.is_pressed(Button::Up));
        joypad.release_button(Button::Up);
        assert!(!joypad.is_pressed(Button::Up));

        assert_eq!(joypad.bits(), 0x3F);

        joypad.press_button(Button::A);
        joypad.press_button(Button::B);
        joypad.release_button(Button::A);
        assert!(!joypad.is_pressed(Button::A));
        assert!(joypad.is_pressed(Button::B));

        joypad.press_button(Button::Up);
        joypad.press_button(Button::Right);
        joypad.release_button(Button::Up);
        assert!(!joypad.is_pressed(Button::Up));
        assert!(joypad.is_pressed(Button::Right));
    }
}
