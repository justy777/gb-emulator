use bitflags::bitflags;

#[derive(Debug, Clone, Copy)]
pub struct Registers {
    /// Accumulator
    a: u8,
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    /// Flags Register
    f: Flags,
    h: u8,
    l: u8,
    /// Stack Pointer
    sp: u16,
    /// Program Counter
    pc: u16,
}

impl Registers {
    const fn new() -> Self {
        Self {
            a: 0,
            b: 0,
            c: 0,
            d: 0,
            e: 0,
            f: Flags::empty(),
            h: 0,
            l: 0,
            sp: 0,
            pc: 0,
        }
    }

    const fn read(&self, register: R8) -> u8 {
        match register {
            R8::A => self.a,
            R8::B => self.b,
            R8::C => self.c,
            R8::D => self.d,
            R8::E => self.e,
            R8::H => self.h,
            R8::L => self.l,
        }
    }

    fn write(&mut self, register: R8, value: u8) {
        match register {
            R8::A => self.a = value,
            R8::B => self.b = value,
            R8::C => self.c = value,
            R8::D => self.d = value,
            R8::E => self.e = value,
            R8::H => self.h = value,
            R8::L => self.l = value,
        }
    }

    const fn read16(&self, register: R16) -> u16 {
        match register {
            R16::AF => (self.a as u16) << 8 | self.f.bits() as u16,
            R16::BC => (self.b as u16) << 8 | self.c as u16,
            R16::DE => (self.d as u16) << 8 | self.e as u16,
            R16::HL => (self.h as u16) << 8 | self.l as u16,
            R16::SP => self.sp,
        }
    }

    fn write16(&mut self, register: R16, value: u16) {
        match register {
            R16::AF => {
                self.a = (value >> 8) as u8;
                self.f = Flags::from_bits_truncate(value as u8);
            }
            R16::BC => {
                self.b = (value >> 8) as u8;
                self.c = (value & 0xFF) as u8;
            }
            R16::DE => {
                self.d = (value >> 8) as u8;
                self.e = (value & 0xFF) as u8;
            }
            R16::HL => {
                self.h = (value >> 8) as u8;
                self.l = (value & 0xFF) as u8;
            }
            R16::SP => {
                self.sp = value;
            }
        }
    }
}

bitflags! {
    #[repr(transparent)]
    #[derive(Debug, Clone, Copy)]
    struct Flags: u8 {
        const ZERO = 0b1000_0000;
        const SUBTRACT = 0b0100_0000;
        const HALF_CARRY = 0b0010_0000;
        const CARRY = 0b0001_0000;
    }
}

impl Flags {
    fn test(&self, condition: JumpCondition) -> bool{
        match condition {
            JumpCondition::NotZero => !self.contains(Flags::ZERO),
            JumpCondition::Zero => self.contains(Flags::ZERO),
            JumpCondition::NotCarry => !self.contains(Flags::CARRY),
            JumpCondition::Carry => self.contains(Flags::CARRY),
            JumpCondition::Always => true,
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Instruction {
    ADD(R8),
    ADC(R8),
    SUB(R8),
    SBC(R8),
    AND(R8),
    XOR(R8),
    OR(R8),
    CP(R8),
    INC(R8),
    DEC(R8),
    ADD16(R16),
    INC16(R16),
    DEC16(R16),
    BIT(u8, R8),
    RES(u8, R8),
    SET(u8, R8),
    SWAP(R8),
    RL(R8),
    RLA,
    RLC(R8),
    RLCA,
    RR(R8),
    RRA,
    RRC(R8),
    RRCA,
    SLA(R8),
    SRA(R8),
    SRL(R8),
    CALL(JumpCondition),
    JP_HL,
    JP(JumpCondition),
    JR(JumpCondition),
    RET(JumpCondition),
    POP(R16),
    PUSH(R16),
    SCF,
    CPL,
    CCF,
}

impl Instruction {
    const fn from_byte(byte: u8, prefixed: bool) -> Option<Self> {
        if prefixed {
            Self::from_byte_prefixed(byte)
        } else {
            Self::from_byte_not_prefixed(byte)
        }
    }

    const fn from_byte_prefixed(byte: u8) -> Option<Self> {
        match byte {
            0x07 => Some(Self::RLC(R8::A)),
            0x00 => Some(Self::RLC(R8::B)),
            0x01 => Some(Self::RLC(R8::C)),
            0x02 => Some(Self::RLC(R8::D)),
            0x03 => Some(Self::RLC(R8::E)),
            0x04 => Some(Self::RLC(R8::H)),
            0x05 => Some(Self::RLC(R8::L)),

            0x0F => Some(Self::RRC(R8::A)),
            0x08 => Some(Self::RRC(R8::B)),
            0x09 => Some(Self::RRC(R8::C)),
            0x0A => Some(Self::RRC(R8::D)),
            0x0B => Some(Self::RRC(R8::E)),
            0x0C => Some(Self::RRC(R8::H)),
            0x0D => Some(Self::RRC(R8::L)),

            0x17 => Some(Self::RL(R8::A)),
            0x10 => Some(Self::RL(R8::B)),
            0x11 => Some(Self::RL(R8::C)),
            0x12 => Some(Self::RL(R8::D)),
            0x13 => Some(Self::RL(R8::E)),
            0x14 => Some(Self::RL(R8::H)),
            0x15 => Some(Self::RL(R8::L)),

            0x1F => Some(Self::RR(R8::A)),
            0x18 => Some(Self::RR(R8::B)),
            0x19 => Some(Self::RR(R8::C)),
            0x1A => Some(Self::RR(R8::D)),
            0x1B => Some(Self::RR(R8::E)),
            0x1C => Some(Self::RR(R8::H)),
            0x1D => Some(Self::RR(R8::L)),

            0x27 => Some(Self::SLA(R8::A)),
            0x20 => Some(Self::SLA(R8::B)),
            0x21 => Some(Self::SLA(R8::C)),
            0x22 => Some(Self::SLA(R8::D)),
            0x23 => Some(Self::SLA(R8::E)),
            0x24 => Some(Self::SLA(R8::H)),
            0x25 => Some(Self::SLA(R8::L)),

            0x2F => Some(Self::SRA(R8::A)),
            0x28 => Some(Self::SRA(R8::B)),
            0x29 => Some(Self::SRA(R8::C)),
            0x2A => Some(Self::SRA(R8::D)),
            0x2B => Some(Self::SRA(R8::E)),
            0x2C => Some(Self::SRA(R8::H)),
            0x2D => Some(Self::SRA(R8::L)),

            0x37 => Some(Self::SWAP(R8::A)),
            0x30 => Some(Self::SWAP(R8::B)),
            0x31 => Some(Self::SWAP(R8::C)),
            0x32 => Some(Self::SWAP(R8::D)),
            0x33 => Some(Self::SWAP(R8::E)),
            0x34 => Some(Self::SWAP(R8::H)),
            0x35 => Some(Self::SWAP(R8::L)),

            0x3F => Some(Self::SRL(R8::A)),
            0x38 => Some(Self::SRL(R8::B)),
            0x39 => Some(Self::SRL(R8::C)),
            0x3A => Some(Self::SRL(R8::D)),
            0x3B => Some(Self::SRL(R8::E)),
            0x3C => Some(Self::SRL(R8::H)),
            0x3D => Some(Self::SRL(R8::L)),

            0x47 => Some(Self::BIT(0, R8::A)),
            0x40 => Some(Self::BIT(0, R8::B)),
            0x41 => Some(Self::BIT(0, R8::C)),
            0x42 => Some(Self::BIT(0, R8::D)),
            0x43 => Some(Self::BIT(0, R8::E)),
            0x44 => Some(Self::BIT(0, R8::H)),
            0x45 => Some(Self::BIT(0, R8::L)),

            0x4F => Some(Self::BIT(1, R8::A)),
            0x48 => Some(Self::BIT(1, R8::B)),
            0x49 => Some(Self::BIT(1, R8::C)),
            0x4A => Some(Self::BIT(1, R8::D)),
            0x4B => Some(Self::BIT(1, R8::E)),
            0x4C => Some(Self::BIT(1, R8::H)),
            0x4D => Some(Self::BIT(1, R8::L)),

            0x57 => Some(Self::BIT(2, R8::A)),
            0x50 => Some(Self::BIT(2, R8::B)),
            0x51 => Some(Self::BIT(2, R8::C)),
            0x52 => Some(Self::BIT(2, R8::D)),
            0x53 => Some(Self::BIT(2, R8::E)),
            0x54 => Some(Self::BIT(2, R8::H)),
            0x55 => Some(Self::BIT(2, R8::L)),

            0x5F => Some(Self::BIT(3, R8::A)),
            0x58 => Some(Self::BIT(3, R8::B)),
            0x59 => Some(Self::BIT(3, R8::C)),
            0x5A => Some(Self::BIT(3, R8::D)),
            0x5B => Some(Self::BIT(3, R8::E)),
            0x5C => Some(Self::BIT(3, R8::H)),
            0x5D => Some(Self::BIT(3, R8::L)),

            0x67 => Some(Self::BIT(4, R8::A)),
            0x60 => Some(Self::BIT(4, R8::B)),
            0x61 => Some(Self::BIT(4, R8::C)),
            0x62 => Some(Self::BIT(4, R8::D)),
            0x63 => Some(Self::BIT(4, R8::E)),
            0x64 => Some(Self::BIT(4, R8::H)),
            0x65 => Some(Self::BIT(4, R8::L)),

            0x6F => Some(Self::BIT(5, R8::A)),
            0x68 => Some(Self::BIT(5, R8::B)),
            0x69 => Some(Self::BIT(5, R8::C)),
            0x6A => Some(Self::BIT(5, R8::D)),
            0x6B => Some(Self::BIT(5, R8::E)),
            0x6C => Some(Self::BIT(5, R8::H)),
            0x6D => Some(Self::BIT(5, R8::L)),

            0x77 => Some(Self::BIT(6, R8::A)),
            0x70 => Some(Self::BIT(6, R8::B)),
            0x71 => Some(Self::BIT(6, R8::C)),
            0x72 => Some(Self::BIT(6, R8::D)),
            0x73 => Some(Self::BIT(6, R8::E)),
            0x74 => Some(Self::BIT(6, R8::H)),
            0x75 => Some(Self::BIT(6, R8::L)),

            0x7F => Some(Self::BIT(7, R8::A)),
            0x78 => Some(Self::BIT(7, R8::B)),
            0x79 => Some(Self::BIT(7, R8::C)),
            0x7A => Some(Self::BIT(7, R8::D)),
            0x7B => Some(Self::BIT(7, R8::E)),
            0x7C => Some(Self::BIT(7, R8::H)),
            0x7D => Some(Self::BIT(7, R8::L)),

            0x87 => Some(Self::RES(0, R8::A)),
            0x80 => Some(Self::RES(0, R8::B)),
            0x81 => Some(Self::RES(0, R8::C)),
            0x82 => Some(Self::RES(0, R8::D)),
            0x83 => Some(Self::RES(0, R8::E)),
            0x84 => Some(Self::RES(0, R8::H)),
            0x85 => Some(Self::RES(0, R8::L)),

            0x8F => Some(Self::RES(1, R8::A)),
            0x88 => Some(Self::RES(1, R8::B)),
            0x89 => Some(Self::RES(1, R8::C)),
            0x8A => Some(Self::RES(1, R8::D)),
            0x8B => Some(Self::RES(1, R8::E)),
            0x8C => Some(Self::RES(1, R8::H)),
            0x8D => Some(Self::RES(1, R8::L)),

            0x97 => Some(Self::RES(2, R8::A)),
            0x90 => Some(Self::RES(2, R8::B)),
            0x91 => Some(Self::RES(2, R8::C)),
            0x92 => Some(Self::RES(2, R8::D)),
            0x93 => Some(Self::RES(2, R8::E)),
            0x94 => Some(Self::RES(2, R8::H)),
            0x95 => Some(Self::RES(2, R8::L)),

            0x9F => Some(Self::RES(3, R8::A)),
            0x98 => Some(Self::RES(3, R8::B)),
            0x99 => Some(Self::RES(3, R8::C)),
            0x9A => Some(Self::RES(3, R8::D)),
            0x9B => Some(Self::RES(3, R8::E)),
            0x9C => Some(Self::RES(3, R8::H)),
            0x9D => Some(Self::RES(3, R8::L)),

            0xA7 => Some(Self::RES(4, R8::A)),
            0xA0 => Some(Self::RES(4, R8::B)),
            0xA1 => Some(Self::RES(4, R8::C)),
            0xA2 => Some(Self::RES(4, R8::D)),
            0xA3 => Some(Self::RES(4, R8::E)),
            0xA4 => Some(Self::RES(4, R8::H)),
            0xA5 => Some(Self::RES(4, R8::L)),

            0xAF => Some(Self::RES(5, R8::A)),
            0xA8 => Some(Self::RES(5, R8::B)),
            0xA9 => Some(Self::RES(5, R8::C)),
            0xAA => Some(Self::RES(5, R8::D)),
            0xAB => Some(Self::RES(5, R8::E)),
            0xAC => Some(Self::RES(5, R8::H)),
            0xAD => Some(Self::RES(5, R8::L)),

            0xB7 => Some(Self::RES(6, R8::A)),
            0xB0 => Some(Self::RES(6, R8::B)),
            0xB1 => Some(Self::RES(6, R8::C)),
            0xB2 => Some(Self::RES(6, R8::D)),
            0xB3 => Some(Self::RES(6, R8::E)),
            0xB4 => Some(Self::RES(6, R8::H)),
            0xB5 => Some(Self::RES(6, R8::L)),

            0xBF => Some(Self::RES(7, R8::A)),
            0xB8 => Some(Self::RES(7, R8::B)),
            0xB9 => Some(Self::RES(7, R8::C)),
            0xBA => Some(Self::RES(7, R8::D)),
            0xBB => Some(Self::RES(7, R8::E)),
            0xBC => Some(Self::RES(7, R8::H)),
            0xBD => Some(Self::RES(7, R8::L)),

            0xC7 => Some(Self::SET(0, R8::A)),
            0xC0 => Some(Self::SET(0, R8::B)),
            0xC1 => Some(Self::SET(0, R8::C)),
            0xC2 => Some(Self::SET(0, R8::D)),
            0xC3 => Some(Self::SET(0, R8::E)),
            0xC4 => Some(Self::SET(0, R8::H)),
            0xC5 => Some(Self::SET(0, R8::L)),

            0xCF => Some(Self::SET(1, R8::A)),
            0xC8 => Some(Self::SET(1, R8::B)),
            0xC9 => Some(Self::SET(1, R8::C)),
            0xCA => Some(Self::SET(1, R8::D)),
            0xCB => Some(Self::SET(1, R8::E)),
            0xCC => Some(Self::SET(1, R8::H)),
            0xCD => Some(Self::SET(1, R8::L)),

            0xD7 => Some(Self::SET(2, R8::A)),
            0xD0 => Some(Self::SET(2, R8::B)),
            0xD1 => Some(Self::SET(2, R8::C)),
            0xD2 => Some(Self::SET(2, R8::D)),
            0xD3 => Some(Self::SET(2, R8::E)),
            0xD4 => Some(Self::SET(2, R8::H)),
            0xD5 => Some(Self::SET(2, R8::L)),

            0xDF => Some(Self::SET(3, R8::A)),
            0xD8 => Some(Self::SET(3, R8::B)),
            0xD9 => Some(Self::SET(3, R8::C)),
            0xDA => Some(Self::SET(3, R8::D)),
            0xDB => Some(Self::SET(3, R8::E)),
            0xDC => Some(Self::SET(3, R8::H)),
            0xDD => Some(Self::SET(3, R8::L)),

            0xE7 => Some(Self::SET(4, R8::A)),
            0xE0 => Some(Self::SET(4, R8::B)),
            0xE1 => Some(Self::SET(4, R8::C)),
            0xE2 => Some(Self::SET(4, R8::D)),
            0xE3 => Some(Self::SET(4, R8::E)),
            0xE4 => Some(Self::SET(4, R8::H)),
            0xE5 => Some(Self::SET(4, R8::L)),

            0xEF => Some(Self::SET(5, R8::A)),
            0xE8 => Some(Self::SET(5, R8::B)),
            0xE9 => Some(Self::SET(5, R8::C)),
            0xEA => Some(Self::SET(5, R8::D)),
            0xEB => Some(Self::SET(5, R8::E)),
            0xEC => Some(Self::SET(5, R8::H)),
            0xED => Some(Self::SET(5, R8::L)),

            0xF7 => Some(Self::SET(6, R8::A)),
            0xF0 => Some(Self::SET(6, R8::B)),
            0xF1 => Some(Self::SET(6, R8::C)),
            0xF2 => Some(Self::SET(6, R8::D)),
            0xF3 => Some(Self::SET(6, R8::E)),
            0xF4 => Some(Self::SET(6, R8::H)),
            0xF5 => Some(Self::SET(6, R8::L)),

            0xFF => Some(Self::SET(7, R8::A)),
            0xF8 => Some(Self::SET(7, R8::B)),
            0xF9 => Some(Self::SET(7, R8::C)),
            0xFA => Some(Self::SET(7, R8::D)),
            0xFB => Some(Self::SET(7, R8::E)),
            0xFC => Some(Self::SET(7, R8::H)),
            0xFD => Some(Self::SET(7, R8::L)),

            // TODO: add mapping for the rest of instructions
            _ => None,
        }
    }

    const fn from_byte_not_prefixed(byte: u8) -> Option<Self> {
        match byte {
            // 8-bit arithmetic
            0x87 => Some(Self::ADD(R8::A)),
            0x80 => Some(Self::ADD(R8::B)),
            0x81 => Some(Self::ADD(R8::C)),
            0x82 => Some(Self::ADD(R8::D)),
            0x83 => Some(Self::ADD(R8::E)),
            0x84 => Some(Self::ADD(R8::H)),
            0x85 => Some(Self::ADD(R8::L)),

            0x8F => Some(Self::ADC(R8::A)),
            0x88 => Some(Self::ADC(R8::B)),
            0x89 => Some(Self::ADC(R8::C)),
            0x8A => Some(Self::ADC(R8::D)),
            0x8B => Some(Self::ADC(R8::E)),
            0x8C => Some(Self::ADC(R8::H)),
            0x8D => Some(Self::ADC(R8::L)),

            0x97 => Some(Self::SUB(R8::A)),
            0x90 => Some(Self::SUB(R8::B)),
            0x91 => Some(Self::SUB(R8::C)),
            0x92 => Some(Self::SUB(R8::D)),
            0x93 => Some(Self::SUB(R8::E)),
            0x94 => Some(Self::SUB(R8::H)),
            0x95 => Some(Self::SUB(R8::L)),

            0x9F => Some(Self::SBC(R8::A)),
            0x98 => Some(Self::SBC(R8::B)),
            0x99 => Some(Self::SBC(R8::C)),
            0x9A => Some(Self::SBC(R8::D)),
            0x9B => Some(Self::SBC(R8::E)),
            0x9C => Some(Self::SBC(R8::H)),
            0x9D => Some(Self::SBC(R8::L)),

            0xA7 => Some(Self::AND(R8::A)),
            0xA0 => Some(Self::AND(R8::B)),
            0xA1 => Some(Self::AND(R8::C)),
            0xA2 => Some(Self::AND(R8::D)),
            0xA3 => Some(Self::AND(R8::E)),
            0xA4 => Some(Self::AND(R8::H)),
            0xA5 => Some(Self::AND(R8::L)),

            0xAF => Some(Self::XOR(R8::A)),
            0xA8 => Some(Self::XOR(R8::B)),
            0xA9 => Some(Self::XOR(R8::C)),
            0xAA => Some(Self::XOR(R8::D)),
            0xAB => Some(Self::XOR(R8::E)),
            0xAC => Some(Self::XOR(R8::H)),
            0xAD => Some(Self::XOR(R8::L)),

            0xB7 => Some(Self::OR(R8::A)),
            0xB0 => Some(Self::OR(R8::B)),
            0xB1 => Some(Self::OR(R8::C)),
            0xB2 => Some(Self::OR(R8::D)),
            0xB3 => Some(Self::OR(R8::E)),
            0xB4 => Some(Self::OR(R8::H)),
            0xB5 => Some(Self::OR(R8::L)),

            0xBF => Some(Self::CP(R8::A)),
            0xB8 => Some(Self::CP(R8::B)),
            0xB9 => Some(Self::CP(R8::C)),
            0xBA => Some(Self::CP(R8::D)),
            0xBB => Some(Self::CP(R8::E)),
            0xBC => Some(Self::CP(R8::H)),
            0xBD => Some(Self::CP(R8::L)),

            0x3C => Some(Self::INC(R8::A)),
            0x04 => Some(Self::INC(R8::B)),
            0x0C => Some(Self::INC(R8::C)),
            0x14 => Some(Self::INC(R8::D)),
            0x1C => Some(Self::INC(R8::E)),
            0x24 => Some(Self::INC(R8::H)),
            0x2C => Some(Self::INC(R8::L)),

            0x3D => Some(Self::DEC(R8::A)),
            0x05 => Some(Self::DEC(R8::B)),
            0x0D => Some(Self::DEC(R8::C)),
            0x15 => Some(Self::DEC(R8::D)),
            0x1D => Some(Self::DEC(R8::E)),
            0x25 => Some(Self::DEC(R8::H)),
            0x2D => Some(Self::DEC(R8::L)),

            // 16-bit arithmetic
            0x09 => Some(Self::ADD16(R16::BC)),
            0x19 => Some(Self::ADD16(R16::DE)),
            0x29 => Some(Self::ADD16(R16::HL)),
            0x39 => Some(Self::ADD16(R16::SP)),

            0x03 => Some(Self::INC16(R16::BC)),
            0x13 => Some(Self::INC16(R16::DE)),
            0x23 => Some(Self::INC16(R16::HL)),
            0x33 => Some(Self::INC16(R16::SP)),

            0x0B => Some(Self::DEC16(R16::BC)),
            0x1B => Some(Self::DEC16(R16::DE)),
            0x2B => Some(Self::DEC16(R16::HL)),
            0x3B => Some(Self::DEC16(R16::SP)),

            // Bit shift
            0x07 => Some(Self::RLCA),
            0x17 => Some(Self::RLA),
            0x0F => Some(Self::RRCA),
            0x1F => Some(Self::RRA),

            // Jumps
            0xCD => Some(Self::CALL(JumpCondition::Always)),
            0xC4 => Some(Self::CALL(JumpCondition::NotZero)),
            0xCC => Some(Self::CALL(JumpCondition::Zero)),
            0xD4 => Some(Self::CALL(JumpCondition::NotCarry)),
            0xDC => Some(Self::CALL(JumpCondition::Carry)),

            0xE9 => Some(Self::JP_HL),

            0xC3 => Some(Self::JP(JumpCondition::Always)),
            0xC2 => Some(Self::JP(JumpCondition::NotZero)),
            0xCA => Some(Self::JP(JumpCondition::Zero)),
            0xD2 => Some(Self::JP(JumpCondition::NotCarry)),
            0xDA => Some(Self::JP(JumpCondition::Carry)),

            0x18 => Some(Self::JR(JumpCondition::Always)),
            0x20 => Some(Self::JR(JumpCondition::NotZero)),
            0x28 => Some(Self::JR(JumpCondition::Zero)),
            0x30 => Some(Self::JR(JumpCondition::NotCarry)),
            0x38 => Some(Self::JR(JumpCondition::Carry)),

            0xC9 => Some(Self::RET(JumpCondition::Always)),
            0xC0 => Some(Self::RET(JumpCondition::NotZero)),
            0xC8 => Some(Self::RET(JumpCondition::Zero)),
            0xD0 => Some(Self::RET(JumpCondition::NotCarry)),
            0xD8 => Some(Self::RET(JumpCondition::Carry)),

            // Stack
            0xC1 => Some(Self::POP(R16::BC)),
            0xD1 => Some(Self::POP(R16::DE)),
            0xE1 => Some(Self::POP(R16::HL)),
            0xF1 => Some(Self::POP(R16::AF)),

            0xC5 => Some(Self::PUSH(R16::BC)),
            0xD5 => Some(Self::PUSH(R16::DE)),
            0xE5 => Some(Self::PUSH(R16::HL)),
            0xF5 => Some(Self::PUSH(R16::AF)),

            // Misc
            0x37 => Some(Self::SCF),
            0x2F => Some(Self::CPL),
            0x3F => Some(Self::CCF),

            // Undefined
            0xD3 | 0xDB | 0xDD | 0xE3 | 0xE4 | 0xEB | 0xEC | 0xED | 0xF4 | 0xFC | 0xFD => None,

            // TODO: add mapping for the rest of instructions
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum R8 {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
}

#[derive(Debug, Clone, Copy)]
enum R16 {
    AF,
    BC,
    DE,
    HL,
    SP,
}

#[derive(Debug, Clone, Copy)]
enum JumpCondition {
    NotZero,
    Zero,
    NotCarry,
    Carry,
    Always,
}

#[derive(Clone)]
struct MemoryBus {
    memory: [u8; 0xFFFF],
}

impl MemoryBus {
    const fn new() -> Self {
        Self {
            memory: [0; 0xFFFF],
        }
    }

    const fn read_byte(&self, address: u16) -> u8 {
        self.memory[address as usize]
    }

    fn write_byte(&mut self, address: u16, value: u8) {
        self.memory[address as usize] = value;
    }
}

#[derive(Clone)]
pub struct Cpu {
    registers: Registers,
    bus: MemoryBus,
}

impl Cpu {
    const fn new() -> Self {
        Self {
            registers: Registers::new(),
            bus: MemoryBus::new(),
        }
    }

    fn step(&mut self) {
        let mut instruction_byte = self.bus.read_byte(self.registers.pc);

        let prefixed = instruction_byte == 0xCB;
        if prefixed {
            instruction_byte = self.bus.read_byte(self.registers.pc + 1);
        }

        let next_pc = if let Some(instruction) = Instruction::from_byte(instruction_byte, prefixed)
        {
            self.execute(instruction)
        } else {
            let description = format!("0x{}{instruction_byte:x}", if prefixed { "CB" } else { "" });
            panic!("Unknown instruction found for: {description}");
        };

        self.registers.pc = next_pc;
    }

    fn execute(&mut self, instruction: Instruction) -> u16 {
        match instruction {
            Instruction::ADD(target) => {
                let value = self.registers.read(target);
                let new_value = self.add(value);
                self.registers.a = new_value;
                self.registers.pc.wrapping_add(1)
            }
            Instruction::ADC(target) => {
                let value = self.registers.read(target);
                let new_value = self.adc(value);
                self.registers.a = new_value;
                self.registers.pc.wrapping_add(1)
            }
            Instruction::SUB(target) => {
                let value = self.registers.read(target);
                let new_value = self.sub(value);
                self.registers.a = new_value;
                self.registers.pc.wrapping_add(1)
            }
            Instruction::SBC(target) => {
                let value = self.registers.read(target);
                let new_value = self.sbc(value);
                self.registers.a = new_value;
                self.registers.pc.wrapping_add(1)
            }
            Instruction::AND(target) => {
                let value = self.registers.read(target);
                let new_value = self.and(value);
                self.registers.a = new_value;
                self.registers.pc.wrapping_add(1)
            }
            Instruction::XOR(target) => {
                let value = self.registers.read(target);
                let new_value = self.xor(value);
                self.registers.a = new_value;
                self.registers.pc.wrapping_add(1)
            }
            Instruction::OR(target) => {
                let value = self.registers.read(target);
                let new_value = self.or(value);
                self.registers.a = new_value;
                self.registers.pc.wrapping_add(1)
            }
            Instruction::CP(target) => {
                let value = self.registers.read(target);
                self.cp(value);
                self.registers.pc.wrapping_add(1)
            }
            Instruction::INC(target) => {
                let value = self.registers.read(target);
                let new_value = self.inc(value);
                self.registers.write(target, new_value);
                self.registers.pc.wrapping_add(1)
            }
            Instruction::DEC(target) => {
                let value = self.registers.read(target);
                let new_value = self.dec(value);
                self.registers.write(target, new_value);
                self.registers.pc.wrapping_add(1)
            }
            Instruction::ADD16(target) => {
                let value = self.registers.read16(target);
                let new_value = self.add16(value);
                self.registers.write16(R16::HL, new_value);
                self.registers.pc.wrapping_add(1)
            }
            Instruction::INC16(target) => {
                let value = self.registers.read16(target);
                let new_value = self.inc16(value);
                self.registers.write16(target, new_value);
                self.registers.pc.wrapping_add(1)
            }
            Instruction::DEC16(target) => {
                let value = self.registers.read16(target);
                let new_value = self.dec16(value);
                self.registers.write16(target, new_value);
                self.registers.pc.wrapping_add(1)
            }
            Instruction::BIT(bit, target) => {
                let value = self.registers.read(target);
                self.bit(bit, value);
                self.registers.pc.wrapping_add(2)
            }
            Instruction::RES(bit, target) => {
                let value = self.registers.read(target);
                let new_value = self.res(bit, value);
                self.registers.write(target, new_value);
                self.registers.pc.wrapping_add(2)
            }
            Instruction::SET(bit, target) => {
                let value = self.registers.read(target);
                let new_value = self.set(bit, value);
                self.registers.write(target, new_value);
                self.registers.pc.wrapping_add(2)
            }
            Instruction::SWAP(target) => {
                let value = self.registers.read(target);
                let new_value = self.swap(value);
                self.registers.write(target, new_value);
                self.registers.pc.wrapping_add(2)
            }
            Instruction::RL(target) => {
                let value = self.registers.read(target);
                let new_value = self.rl(value);
                self.registers.write(target, new_value);
                self.registers.pc.wrapping_add(2)
            }
            Instruction::RLC(target) => {
                let value = self.registers.read(target);
                let new_value = self.rlc(value);
                self.registers.write(target, new_value);
                self.registers.pc.wrapping_add(2)
            }
            Instruction::RLCA => {
                let new_value = self.rlca();
                self.registers.a = new_value;
                self.registers.pc.wrapping_add(1)
            }
            Instruction::RLA => {
                let new_value = self.rla();
                self.registers.a = new_value;
                self.registers.pc.wrapping_add(1)
            }
            Instruction::RR(target) => {
                let value = self.registers.read(target);
                let new_value = self.rr(value);
                self.registers.write(target, new_value);
                self.registers.pc.wrapping_add(2)
            }
            Instruction::RRC(target) => {
                let value = self.registers.read(target);
                let new_value = self.rrc(value);
                self.registers.write(target, new_value);
                self.registers.pc.wrapping_add(2)
            }
            Instruction::RRCA => {
                let new_value = self.rrca();
                self.registers.a = new_value;
                self.registers.pc.wrapping_add(1)
            }
            Instruction::RRA => {
                let new_value = self.rra();
                self.registers.a = new_value;
                self.registers.pc.wrapping_add(1)
            }
            Instruction::SLA(target) => {
                let value = self.registers.read(target);
                let new_value = self.sla(value);
                self.registers.write(target, new_value);
                self.registers.pc.wrapping_add(2)
            }
            Instruction::SRA(target) => {
                let value = self.registers.read(target);
                let new_value = self.sra(value);
                self.registers.write(target, new_value);
                self.registers.pc.wrapping_add(2)
            }
            Instruction::SRL(target) => {
                let value = self.registers.read(target);
                let new_value = self.srl(value);
                self.registers.write(target, new_value);
                self.registers.pc.wrapping_add(2)
            }
            Instruction::CALL(test) => {
                let jump_condition = self.registers.f.test(test);
                self.call(jump_condition)
            }
            Instruction::JP_HL => {
                let value = self.registers.read16(R16::HL);
                self.jump_hl(value)
            }
            Instruction::JP(test) => {
                let jump_condition = self.registers.f.test(test);
                self.jump(jump_condition)
            }
            Instruction::JR(test) => {
                let jump_condition = self.registers.f.test(test);
                self.jump_relative(jump_condition)
            }
            Instruction::RET(test) => {
                let jump_condition = self.registers.f.test(test);
                self.returns(jump_condition)
            }
            Instruction::POP(target) => {
                let value = self.pop();
                self.registers.write16(target, value);
                self.registers.pc.wrapping_add(1)
            }
            Instruction::PUSH(target) => {
                let value = self.registers.read16(target);
                self.push(value);
                self.registers.pc.wrapping_add(1)
            }
            Instruction::SCF => {
                self.scf();
                self.registers.pc.wrapping_add(1)
            }
            Instruction::CPL => {
                let new_value = self.cpl();
                self.registers.a = new_value;
                self.registers.pc.wrapping_add(1)
            }
            Instruction::CCF => {
                self.ccf();
                self.registers.pc.wrapping_add(1)
            }
        }
    }

    /// ADD A, r8
    /// 1 4
    /// Z 0 H C
    ///
    /// Add the value in r8 to register A.
    fn add(&mut self, value: u8) -> u8 {
        let a = self.registers.a;
        let (new_value, did_overflow) = a.overflowing_add(value);
        self.registers.f.set(Flags::ZERO, new_value == 0);
        self.registers.f.set(Flags::SUBTRACT, false);
        self.registers.f.set(Flags::CARRY, did_overflow);
        // Half carry is set if adding the lower bits (0-3) of the value and register A
        // together result in overflowing to bit 4. If the result is larger than 0xF
        // than the addition caused a carry from bit 3 to bit 4.
        let half_carry = (a & 0xF) + (value & 0xF) > 0xF;
        self.registers.f.set(Flags::HALF_CARRY, half_carry);
        new_value
    }

    /// ADC A, r8
    /// 1 4
    /// Z 0 H C
    ///
    /// Add the value in r8 plus the carry flag to register A.
    fn adc(&mut self, value: u8) -> u8 {
        let a = self.registers.a;
        let cf = self.registers.f.contains(Flags::CARRY) as u8;
        let new_value = a.wrapping_add(value).wrapping_add(cf);
        self.registers.f.set(Flags::ZERO, new_value == 0);
        self.registers.f.set(Flags::SUBTRACT, false);
        let carry = a as u16 + value as u16 + cf as u16 > 0xFF;
        self.registers.f.set(Flags::CARRY, carry);
        let half_carry = (a & 0xF) + (value & 0xF) + cf > 0xF;
        self.registers.f.set(Flags::HALF_CARRY, half_carry);
        new_value
    }

    /// SUB A, r8
    /// 1 4
    /// Z 1 H C
    ///
    /// Subtract the value in r8 from register A.
    fn sub(&mut self, value: u8) -> u8 {
        let a = self.registers.a;
        let (new_value, did_overflow) = a.overflowing_sub(value);
        self.registers.f.set(Flags::ZERO, new_value == 0);
        self.registers.f.set(Flags::SUBTRACT, true);
        self.registers.f.set(Flags::CARRY, did_overflow);
        let half_carry = (a & 0xF) < (value & 0xF);
        self.registers.f.set(Flags::HALF_CARRY, half_carry);
        new_value
    }

    /// SBC A, r8
    /// 1 4
    /// Z 1 H C
    ///
    /// Subtract the value in r8 and the carry flag from register A.
    fn sbc(&mut self, value: u8) -> u8 {
        let a = self.registers.a;
        let cf = self.registers.f.contains(Flags::CARRY) as u8;
        let new_value = a.wrapping_sub(value).wrapping_sub(cf);
        self.registers.f.set(Flags::ZERO, new_value == 0);
        self.registers.f.set(Flags::SUBTRACT, true);
        let carry = (a as u16) < (value as u16) + (cf as u16);
        self.registers.f.set(Flags::CARRY, carry);
        let half_carry = (a & 0xF) < (value & 0xF) + cf;
        self.registers.f.set(Flags::HALF_CARRY, half_carry);
        new_value
    }

    /// AND A, r8
    /// 1 4
    /// Z 0 1 0
    ///
    /// Bitwise AND between the value in r8 and register A.
    fn and(&mut self, value: u8) -> u8 {
        let new_value = self.registers.a & value;
        self.registers.f.set(Flags::ZERO, new_value == 0);
        self.registers.f.set(Flags::SUBTRACT, false);
        self.registers.f.set(Flags::HALF_CARRY, true);
        self.registers.f.set(Flags::CARRY, false);
        new_value
    }

    /// XOR A, r8
    /// 1 4
    /// Z 0 0 0
    ///
    /// Bitwise XOR between the value in r8 and register A.
    fn xor(&mut self, value: u8) -> u8 {
        let new_value = self.registers.a ^ value;
        self.registers.f.set(Flags::ZERO, new_value == 0);
        self.registers.f.set(Flags::SUBTRACT, false);
        self.registers.f.set(Flags::HALF_CARRY, false);
        self.registers.f.set(Flags::CARRY, false);
        new_value
    }

    /// OR A, r8
    /// 1 4
    /// Z 0 0 0
    ///
    /// Bitwise OR between the value in r8 and register A.
    fn or(&mut self, value: u8) -> u8 {
        let new_value = self.registers.a | value;
        self.registers.f.set(Flags::ZERO, new_value == 0);
        self.registers.f.set(Flags::SUBTRACT, true);
        self.registers.f.set(Flags::HALF_CARRY, false);
        self.registers.f.set(Flags::CARRY, false);
        new_value
    }

    /// CP A, r8
    /// 1 4
    /// Z 1 H C
    ///
    /// Subtract the value in r8 from register A and set flags accordingly, but don't store the result.
    fn cp(&mut self, value: u8) {
        let a = self.registers.a;
        let (new_value, did_overflow) = a.overflowing_sub(value);
        self.registers.f.set(Flags::ZERO, new_value == 0);
        self.registers.f.set(Flags::SUBTRACT, true);
        self.registers.f.set(Flags::CARRY, did_overflow);
        let half_carry = (a & 0xF) < (value & 0xF);
        self.registers.f.set(Flags::HALF_CARRY, half_carry);
    }

    /// INC r8
    /// 1 4
    /// Z 0 H -
    ///
    /// Increment value in register r8 by 1.
    fn inc(&mut self, value: u8) -> u8 {
        let new_value = value.wrapping_add(1);
        self.registers.f.set(Flags::ZERO, new_value == 0);
        self.registers.f.set(Flags::SUBTRACT, false);
        let half_carry = (value & 0xF) == 0xF;
        self.registers.f.set(Flags::HALF_CARRY, half_carry);
        // CARRY is left untouched
        new_value
    }

    /// DEC r8
    /// 1 4
    /// Z 1 H -
    ///
    /// Decrement value in register r8 by 1.
    fn dec(&mut self, value: u8) -> u8 {
        let new_value = value.wrapping_sub(1);
        self.registers.f.set(Flags::ZERO, new_value == 0);
        self.registers.f.set(Flags::SUBTRACT, true);
        let half_carry = (value & 0xF) == 0;
        self.registers.f.set(Flags::HALF_CARRY, half_carry);
        // CARRY is left untouched
        new_value
    }

    /// ADD HL, r16
    /// 1 8
    /// - 0 H C
    ///
    /// Add the value in r16 to register HL.
    fn add16(&mut self, value: u16) -> u16 {
        let hl = self.registers.read16(R16::HL);
        let (new_value, did_overflow) = hl.overflowing_add(value);
        // ZERO is left untouched
        self.registers.f.set(Flags::SUBTRACT, false);
        self.registers.f.set(Flags::CARRY, did_overflow);
        // For 16-bit operations the half-carry the register's high bit sets the flag.
        // Bit 11 overflowing to bit 12 sets the half-carry.
        // Bits are labeled from 0-15 from least to most significant.
        let half_carry = (hl & 0xFFF) + (value & 0xFFF) > 0xFFF;
        self.registers.f.set(Flags::HALF_CARRY, half_carry);
        new_value
    }

    /// INC r16
    /// 1 8
    /// - - - -
    ///
    /// Increment value in register r16 by 1.
    fn inc16(&mut self, value: u16) -> u16 {
        let new_value = value.wrapping_add(1);
        new_value
    }

    /// DEC r16
    /// 1 8
    /// - - - -
    ///
    /// Decrement value in register r16 by 1.
    fn dec16(&mut self, value: u16) -> u16 {
        let new_value = value.wrapping_sub(1);
        new_value
    }

    /// RLCA
    /// 1 4
    /// 0 0 0 C
    ///
    /// Rotate register A left.
    fn rlca(&mut self) -> u8 {
        let value = self.registers.a;
        let new_value = self.registers.a.rotate_left(1);
        self.registers.f.set(Flags::ZERO, false);
        self.registers.f.set(Flags::SUBTRACT, false);
        self.registers.f.set(Flags::HALF_CARRY, false);
        let carry = value & 0x80 != 0;
        self.registers.f.set(Flags::CARRY, carry);
        new_value
    }

    /// RLA
    /// 1 4
    /// 0 0 0 C
    ///
    /// Rotate register A left, through the carry flag.
    fn rla(&mut self) -> u8 {
        let value = self.registers.a;
        let cf = self.registers.f.contains(Flags::CARRY) as u8;
        let new_value = (value << 1) | cf;
        self.registers.f.set(Flags::ZERO, false);
        self.registers.f.set(Flags::SUBTRACT, false);
        self.registers.f.set(Flags::HALF_CARRY, false);
        let carry = value & 0x80 != 0;
        self.registers.f.set(Flags::CARRY, carry);
        new_value
    }

    /// RRCA
    /// 1 4
    /// 0 0 0 C
    ///
    /// Rotate register A right.
    fn rrca(&mut self) -> u8 {
        let value = self.registers.a;
        let new_value = self.registers.a.rotate_right(1);
        self.registers.f.set(Flags::ZERO, false);
        self.registers.f.set(Flags::SUBTRACT, false);
        self.registers.f.set(Flags::HALF_CARRY, false);
        let carry = value & 0x01 != 0;
        self.registers.f.set(Flags::CARRY, carry);
        new_value
    }

    /// RRA
    /// 1 4
    /// 0 0 0 C
    ///
    /// Rotate register A right, through the carry flag.
    fn rra(&mut self) -> u8 {
        let value = self.registers.a;
        let cf = self.registers.f.contains(Flags::CARRY) as u8;
        let new_value = (value >> 1) | (cf << 7);
        self.registers.f.set(Flags::ZERO, false);
        self.registers.f.set(Flags::SUBTRACT, false);
        self.registers.f.set(Flags::HALF_CARRY, false);
        let carry = value & 0x01 != 0;
        self.registers.f.set(Flags::CARRY, carry);
        new_value
    }

    /// SCF
    /// 1 4
    /// - 0 0 1
    ///
    /// Set the carry flag.
    fn scf(&mut self) {
        // ZERO left untouched
        self.registers.f.set(Flags::SUBTRACT, false);
        self.registers.f.set(Flags::HALF_CARRY, false);
        self.registers.f.set(Flags::CARRY, true);
    }

    /// CPL
    /// 1 4
    /// - 1 1 -
    ///
    /// Flip the bits in register A.
    fn cpl(&mut self) -> u8 {
        let value = self.registers.a;
        // ZERO left untouched
        self.registers.f.set(Flags::SUBTRACT, true);
        self.registers.f.set(Flags::HALF_CARRY, true);
        // CARRY left untouched
        !value
    }

    /// CCF
    /// 1 4
    /// - 0 0 C
    ///
    /// Complement the carry flag.
    fn ccf(&mut self) {
        let cf = self.registers.f.contains(Flags::CARRY);
        // ZERO left untouched
        self.registers.f.set(Flags::SUBTRACT, false);
        self.registers.f.set(Flags::HALF_CARRY, false);
        self.registers.f.set(Flags::CARRY, !cf);
    }

    /// RLC r8
    /// 2 8
    /// Z 0 0 C
    ///
    /// Rotate register r8 left.
    fn rlc(&mut self, value: u8) -> u8 {
        let new_value = value.rotate_left(1);
        self.registers.f.set(Flags::ZERO, new_value == 0);
        self.registers.f.set(Flags::SUBTRACT, false);
        self.registers.f.set(Flags::HALF_CARRY, false);
        let carry = value & 0x80 != 0;
        self.registers.f.set(Flags::CARRY, carry);
        new_value
    }

    /// RRC r8
    /// 2 8
    /// Z 0 0 C
    ///
    /// Rotate register r8 right.
    fn rrc(&mut self, value: u8) -> u8 {
        let new_value = value.rotate_right(1);
        self.registers.f.set(Flags::ZERO, new_value == 0);
        self.registers.f.set(Flags::SUBTRACT, false);
        self.registers.f.set(Flags::HALF_CARRY, false);
        let carry = value & 0x01 != 0;
        self.registers.f.set(Flags::CARRY, carry);
        new_value
    }

    /// RL r8
    /// 2 8
    /// Z 0 0 C
    ///
    /// Rotate bits in register r8 left, through the carry flag.
    fn rl(&mut self, value: u8) -> u8 {
        let cf = self.registers.f.contains(Flags::CARRY) as u8;
        let new_value = (value << 1) | cf;
        self.registers.f.set(Flags::ZERO, new_value == 0);
        self.registers.f.set(Flags::SUBTRACT, false);
        self.registers.f.set(Flags::HALF_CARRY, false);
        let carry = value & 0x80 != 0;
        self.registers.f.set(Flags::CARRY, carry);
        new_value
    }

    /// RR r8
    /// 2 8
    /// Z 0 0 C
    ///
    /// Rotate register r8 right, through the carry flag.
    fn rr(&mut self, value: u8) -> u8 {
        let cf = self.registers.f.contains(Flags::CARRY) as u8;
        let new_value = (value >> 1) | (cf << 7);
        self.registers.f.set(Flags::ZERO, new_value == 0);
        self.registers.f.set(Flags::SUBTRACT, false);
        self.registers.f.set(Flags::HALF_CARRY, false);
        let carry = value & 0x01 != 0;
        self.registers.f.set(Flags::CARRY, carry);
        new_value
    }

    /// SLA r8
    /// 2 8
    /// Z 0 0 C
    ///
    /// Shift Left Arithmetically register r8.
    fn sla(&mut self, value: u8) -> u8 {
        let new_value = value << 1;
        self.registers.f.set(Flags::ZERO, new_value == 0);
        self.registers.f.set(Flags::SUBTRACT, false);
        self.registers.f.set(Flags::HALF_CARRY, false);
        let carry = value & 0x80 != 0;
        self.registers.f.set(Flags::CARRY, carry);
        new_value
    }

    /// SRA r8
    /// 2 8
    /// Z 0 0 C
    ///
    /// Shift Right Arithmetically register r8 (bit 7 of r8 is unchanged).
    fn sra(&mut self, value: u8) -> u8 {
        let new_value = (value >> 1) | (value & 0x80);
        self.registers.f.set(Flags::ZERO, new_value == 0);
        self.registers.f.set(Flags::SUBTRACT, false);
        self.registers.f.set(Flags::HALF_CARRY, false);
        let carry = value & 0x01 != 0;
        self.registers.f.set(Flags::CARRY, carry);
        new_value
    }

    /// SWAP r8
    /// 2 8
    /// Z 0 0 0
    ///
    /// Swap the upper 4 bits in register r8 and the lower 4 ones.
    fn swap(&mut self, value: u8) -> u8 {
        let new_value = (value >> 4) | (value << 4);
        self.registers.f.set(Flags::ZERO, new_value == 0);
        self.registers.f.set(Flags::SUBTRACT, false);
        self.registers.f.set(Flags::HALF_CARRY, false);
        self.registers.f.set(Flags::CARRY, false);
        new_value
    }

    /// SRL r8
    /// 2 8
    /// Z 0 0 C
    ///
    /// Shift Right Logically register r8.
    fn srl(&mut self, value: u8) -> u8 {
        let new_value = value >> 1;
        self.registers.f.set(Flags::ZERO, new_value == 0);
        self.registers.f.set(Flags::SUBTRACT, false);
        self.registers.f.set(Flags::HALF_CARRY, false);
        let carry = value & 0x01 != 0;
        self.registers.f.set(Flags::CARRY, carry);
        new_value
    }

    /// BIT u3, r8
    /// 2 8
    /// Z 0 1 -
    ///
    /// Test bit u3 in register r8, set the zero flag if bit not set.
    fn bit(&mut self, bit: u8, value: u8) {
        let new_value = value & (1 << bit);
        self.registers.f.set(Flags::ZERO, new_value == 0);
        self.registers.f.set(Flags::SUBTRACT, false);
        self.registers.f.set(Flags::HALF_CARRY, true);
        // CARRY left untouched
    }

    /// RES u3, r8
    /// 2 8
    /// - - - -
    ///
    /// Set bit u3 in register r8 to 0. Bit 0 is the rightmost one, bit 7 the leftmost one.
    fn res(&mut self, bit: u8, value: u8) -> u8 {
        let new_value = value & !(1 << bit);
        // Flags left untouched
        new_value
    }

    /// SET u3, r8
    /// 2 8
    /// - - - -
    ///
    /// Set bit u3 in register r8 to 1. Bit 0 is the rightmost one, bit 7 the leftmost one.
    fn set(&mut self, bit: u8, value: u8) -> u8 {
        let new_value = value | (1 << bit);
        // Flags left untouched
        new_value
    }

    /// JP HL
    /// 1 4
    /// - - - -
    ///
    /// Jump to address in HL; effectively, load PC with value in register HL.
    fn jump_hl(&mut self, address: u16) -> u16 {
        address
    }

    /// JP cc, n16
    /// 3 16/12
    /// - - - -
    ///
    /// Jump to address n16 if condition cc is met.
    fn jump(&self, should_jump: bool) -> u16 {
        if should_jump {
            // Gameboy is little endian, so read the second byte as the most significant byte
            // and the first as the least significant
            let lsb = self.bus.read_byte(self.registers.pc + 1);
            let msb = self.bus.read_byte(self.registers.pc + 2);
            u16::from_le_bytes([lsb, msb])
        } else {
            // If it's not jumping we still need to move the program counter forward by 3 since the
            // jump instruction is 3 bytes wide (1 byte for the opcode and 2 bytes for the address)
            self.registers.pc.wrapping_add(3)
        }
    }

    /// JR cc, e8
    /// 2 12/8
    /// - - - -
    ///
    /// Relative Jump to current address plus e8 offset if condition cc is met.
    fn jump_relative(&self, should_jump: bool) -> u16 {
        if should_jump {
            let offset = self.bus.read_byte(self.registers.pc + 1) as i16;
            self.registers.pc.wrapping_add_signed(offset)
        } else {
            self.registers.pc.wrapping_add(2)
        }
    }

    /// PUSH r16
    /// 1 16
    /// - - - -
    ///
    /// Push register r16 into the stack.
    fn push(&mut self, value: u16) {
        self.registers.sp = self.registers.sp.wrapping_sub(1);
        self.bus.write_byte(self.registers.sp, ((value & 0xFF00) >> 8) as u8);

        self.registers.sp = self.registers.sp.wrapping_sub(1);
        self.bus.write_byte(self.registers.sp, (value & 0xFF) as u8);
    }

    /// POP r16
    /// 1 12
    /// - - - -
    ///
    /// Pop register r16 from the stack.
    ///
    /// NOTE: POP AF affects all flags.
    fn pop(&mut self) -> u16 {
        let lsb = self.bus.read_byte(self.registers.sp) as u16;
        self.registers.sp = self.registers.sp.wrapping_add(1);

        let msb = self.bus.read_byte(self.registers.sp) as u16;
        self.registers.sp = self.registers.sp.wrapping_add(1);

        (msb << 8) | lsb
    }

    /// CALL cc, n16
    /// 3 24/12
    /// - - - -
    ///
    /// Call address n16 if condition cc is met.
    fn call(&mut self, should_jump: bool) -> u16 {
        let next = self.registers.pc.wrapping_add(3);
        if should_jump {
            self.push(next);
            self.jump(should_jump)
        } else {
            next
        }
    }

    /// RET cc
    /// 1 20/8
    /// - - - -
    ///
    /// Return from subroutine if condition cc is met.
    fn returns(&mut self, should_jump: bool) -> u16 {
        if should_jump {
            self.pop()
        } else {
            self.registers.pc.wrapping_add(1)
        }
    }
}
