const MEM_NR10: u16 = 0xFF10;
const MEM_NR11: u16 = 0xFF11;
const MEM_NR12: u16 = 0xFF12;
const MEM_NR13: u16 = 0xFF13;
const MEM_NR14: u16 = 0xFF14;
const MEM_NR21: u16 = 0xFF16;
const MEM_NR22: u16 = 0xFF17;
const MEM_NR23: u16 = 0xFF18;
const MEM_NR24: u16 = 0xFF19;
const MEM_NR30: u16 = 0xFF1A;
const MEM_NR31: u16 = 0xFF1B;
const MEM_NR32: u16 = 0xFF1C;
const MEM_NR33: u16 = 0xFF1D;
const MEM_NR34: u16 = 0xFF1E;
const MEM_NR41: u16 = 0xFF20;
const MEM_NR42: u16 = 0xFF21;
const MEM_NR43: u16 = 0xFF22;
const MEM_NR44: u16 = 0xFF23;
const MEM_NR50: u16 = 0xFF24;
const MEM_NR51: u16 = 0xFF25;
const MEM_NR52: u16 = 0xFF26;

pub struct Apu {
    // NR10
    nr10: u8,
    // NR11
    nr11: u8,
    // NR12
    nr12: u8,
    // NR13
    nr13: u8,
    // NR14
    nr14: u8,
    // NR21
    nr21: u8,
    // NR22
    nr22: u8,
    // NR23
    nr23: u8,
    // NR24
    nr24: u8,
    // NR30
    nr30: u8,
    // NR31
    nr31: u8,
    // NR32
    nr32: u8,
    // NR33
    nr33: u8,
    // NR34
    nr34: u8,
    // NR41
    nr41: u8,
    // NR42
    nr42: u8,
    // NR43
    nr43: u8,
    // NR44
    nr44: u8,
    // NR50
    nr50: u8,
    // NR51
    nr51: u8,
    // NR52
    nr52: u8,
}

impl Apu {
    pub const fn new() -> Self {
        Self {
            nr10: 0x80,
            nr11: 0xBF,
            nr12: 0xF3,
            nr13: 0xFF,
            nr14: 0xBF,
            nr21: 0x3F,
            nr22: 0,
            nr23: 0xFF,
            nr24: 0xBF,
            nr30: 0x7F,
            nr31: 0xFF,
            nr32: 0x9F,
            nr33: 0xFF,
            nr34: 0xBF,
            nr41: 0xFF,
            nr42: 0,
            nr43: 0,
            nr44: 0xBF,
            nr50: 0x77,
            nr51: 0xF3,
            nr52: 0xF1,
        }
    }

    pub fn read_audio(&self, address: u16) -> u8 {
        match address {
            MEM_NR10 => self.nr10,
            MEM_NR11 => self.nr11,
            MEM_NR12 => self.nr12,
            MEM_NR13 => self.nr13,
            MEM_NR14 => self.nr14,
            MEM_NR21 => self.nr21,
            MEM_NR22 => self.nr22,
            MEM_NR23 => self.nr23,
            MEM_NR24 => self.nr24,
            MEM_NR30 => self.nr30,
            MEM_NR31 => self.nr31,
            MEM_NR32 => self.nr32,
            MEM_NR33 => self.nr33,
            MEM_NR34 => self.nr34,
            MEM_NR41 => self.nr41,
            MEM_NR42 => self.nr42,
            MEM_NR43 => self.nr43,
            MEM_NR44 => self.nr44,
            MEM_NR50 => self.nr50,
            MEM_NR51 => self.nr51,
            MEM_NR52 => self.nr52,
            _ => {
                println!("Warning: Address {address:#X} is not mapped to an I/O register.");
                0xFF
            }
        }
    }

    pub fn write_audio(&mut self, address: u16, value: u8) {
        match address {
            MEM_NR10 => self.nr10 = value,
            MEM_NR11 => self.nr11 = value,
            MEM_NR12 => self.nr12 = value,
            MEM_NR13 => self.nr13 = value,
            MEM_NR14 => self.nr14 = value,
            MEM_NR21 => self.nr21 = value,
            MEM_NR22 => self.nr22 = value,
            MEM_NR23 => self.nr23 = value,
            MEM_NR24 => self.nr24 = value,
            MEM_NR30 => self.nr30 = value,
            MEM_NR31 => self.nr31 = value,
            MEM_NR32 => self.nr32 = value,
            MEM_NR33 => self.nr33 = value,
            MEM_NR34 => self.nr34 = value,
            MEM_NR41 => self.nr41 = value,
            MEM_NR42 => self.nr42 = value,
            MEM_NR43 => self.nr43 = value,
            MEM_NR44 => self.nr44 = value,
            MEM_NR50 => self.nr50 = value,
            MEM_NR51 => self.nr51 = value,
            MEM_NR52 => self.nr52 = value,
            _ => panic!("Address {address:#X} is not mapped to an I/O register."),
        }
    }
}
