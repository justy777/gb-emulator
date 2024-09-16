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
            R16::PC => self.pc,
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
            R16::PC => {
                self.pc = value;
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
    const fn test(self, condition: JumpCondition) -> bool {
        match condition {
            JumpCondition::NotZero => !self.contains(Self::ZERO),
            JumpCondition::Zero => self.contains(Self::ZERO),
            JumpCondition::NotCarry => !self.contains(Self::CARRY),
            JumpCondition::Carry => self.contains(Self::CARRY),
            JumpCondition::Always => true,
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Instruction {
    Add(R8),
    AddWithCarry(R8),
    Subtract(R8),
    SubtractWithCarry(R8),
    And(R8),
    Xor(R8),
    Or(R8),
    Compare(R8),
    Increment(R8),
    Decrement(R8),
    Add16(R16),
    Increment16(R16),
    Decrement16(R16),
    BitTest(u8, R8),
    BitReset(u8, R8),
    BitSet(u8, R8),
    Swap(R8),
    RotateLeft(R8),
    RotateLeftAccumulator,
    RotateLeftCircular(R8),
    RotateLeftCircularAccumulator,
    RotateRight(R8),
    RotateRightAccumulator,
    RotateRightCircular(R8),
    RotateRightCircularAccumulator,
    ShiftLeftArithmetic(R8),
    ShiftRightArithmetic(R8),
    ShiftRightLogical(R8),
    Call(JumpCondition),
    JumpToHL,
    Jump(JumpCondition),
    JumpRelative(JumpCondition),
    Return(JumpCondition),
    ReturnFromInterruptHandler,
    Pop(R16),
    Push(R16),
    SetCarryFlag,
    Complement,
    ComplimentCarryFlag,
    DisableInterrupt,
    EnableInterrupt,
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
            0x07 => Some(Self::RotateLeftCircular(R8::A)),
            0x00 => Some(Self::RotateLeftCircular(R8::B)),
            0x01 => Some(Self::RotateLeftCircular(R8::C)),
            0x02 => Some(Self::RotateLeftCircular(R8::D)),
            0x03 => Some(Self::RotateLeftCircular(R8::E)),
            0x04 => Some(Self::RotateLeftCircular(R8::H)),
            0x05 => Some(Self::RotateLeftCircular(R8::L)),

            0x0F => Some(Self::RotateRightCircular(R8::A)),
            0x08 => Some(Self::RotateRightCircular(R8::B)),
            0x09 => Some(Self::RotateRightCircular(R8::C)),
            0x0A => Some(Self::RotateRightCircular(R8::D)),
            0x0B => Some(Self::RotateRightCircular(R8::E)),
            0x0C => Some(Self::RotateRightCircular(R8::H)),
            0x0D => Some(Self::RotateRightCircular(R8::L)),

            0x17 => Some(Self::RotateLeft(R8::A)),
            0x10 => Some(Self::RotateLeft(R8::B)),
            0x11 => Some(Self::RotateLeft(R8::C)),
            0x12 => Some(Self::RotateLeft(R8::D)),
            0x13 => Some(Self::RotateLeft(R8::E)),
            0x14 => Some(Self::RotateLeft(R8::H)),
            0x15 => Some(Self::RotateLeft(R8::L)),

            0x1F => Some(Self::RotateRight(R8::A)),
            0x18 => Some(Self::RotateRight(R8::B)),
            0x19 => Some(Self::RotateRight(R8::C)),
            0x1A => Some(Self::RotateRight(R8::D)),
            0x1B => Some(Self::RotateRight(R8::E)),
            0x1C => Some(Self::RotateRight(R8::H)),
            0x1D => Some(Self::RotateRight(R8::L)),

            0x27 => Some(Self::ShiftLeftArithmetic(R8::A)),
            0x20 => Some(Self::ShiftLeftArithmetic(R8::B)),
            0x21 => Some(Self::ShiftLeftArithmetic(R8::C)),
            0x22 => Some(Self::ShiftLeftArithmetic(R8::D)),
            0x23 => Some(Self::ShiftLeftArithmetic(R8::E)),
            0x24 => Some(Self::ShiftLeftArithmetic(R8::H)),
            0x25 => Some(Self::ShiftLeftArithmetic(R8::L)),

            0x2F => Some(Self::ShiftRightArithmetic(R8::A)),
            0x28 => Some(Self::ShiftRightArithmetic(R8::B)),
            0x29 => Some(Self::ShiftRightArithmetic(R8::C)),
            0x2A => Some(Self::ShiftRightArithmetic(R8::D)),
            0x2B => Some(Self::ShiftRightArithmetic(R8::E)),
            0x2C => Some(Self::ShiftRightArithmetic(R8::H)),
            0x2D => Some(Self::ShiftRightArithmetic(R8::L)),

            0x37 => Some(Self::Swap(R8::A)),
            0x30 => Some(Self::Swap(R8::B)),
            0x31 => Some(Self::Swap(R8::C)),
            0x32 => Some(Self::Swap(R8::D)),
            0x33 => Some(Self::Swap(R8::E)),
            0x34 => Some(Self::Swap(R8::H)),
            0x35 => Some(Self::Swap(R8::L)),

            0x3F => Some(Self::ShiftRightLogical(R8::A)),
            0x38 => Some(Self::ShiftRightLogical(R8::B)),
            0x39 => Some(Self::ShiftRightLogical(R8::C)),
            0x3A => Some(Self::ShiftRightLogical(R8::D)),
            0x3B => Some(Self::ShiftRightLogical(R8::E)),
            0x3C => Some(Self::ShiftRightLogical(R8::H)),
            0x3D => Some(Self::ShiftRightLogical(R8::L)),

            0x47 => Some(Self::BitTest(0, R8::A)),
            0x40 => Some(Self::BitTest(0, R8::B)),
            0x41 => Some(Self::BitTest(0, R8::C)),
            0x42 => Some(Self::BitTest(0, R8::D)),
            0x43 => Some(Self::BitTest(0, R8::E)),
            0x44 => Some(Self::BitTest(0, R8::H)),
            0x45 => Some(Self::BitTest(0, R8::L)),

            0x4F => Some(Self::BitTest(1, R8::A)),
            0x48 => Some(Self::BitTest(1, R8::B)),
            0x49 => Some(Self::BitTest(1, R8::C)),
            0x4A => Some(Self::BitTest(1, R8::D)),
            0x4B => Some(Self::BitTest(1, R8::E)),
            0x4C => Some(Self::BitTest(1, R8::H)),
            0x4D => Some(Self::BitTest(1, R8::L)),

            0x57 => Some(Self::BitTest(2, R8::A)),
            0x50 => Some(Self::BitTest(2, R8::B)),
            0x51 => Some(Self::BitTest(2, R8::C)),
            0x52 => Some(Self::BitTest(2, R8::D)),
            0x53 => Some(Self::BitTest(2, R8::E)),
            0x54 => Some(Self::BitTest(2, R8::H)),
            0x55 => Some(Self::BitTest(2, R8::L)),

            0x5F => Some(Self::BitTest(3, R8::A)),
            0x58 => Some(Self::BitTest(3, R8::B)),
            0x59 => Some(Self::BitTest(3, R8::C)),
            0x5A => Some(Self::BitTest(3, R8::D)),
            0x5B => Some(Self::BitTest(3, R8::E)),
            0x5C => Some(Self::BitTest(3, R8::H)),
            0x5D => Some(Self::BitTest(3, R8::L)),

            0x67 => Some(Self::BitTest(4, R8::A)),
            0x60 => Some(Self::BitTest(4, R8::B)),
            0x61 => Some(Self::BitTest(4, R8::C)),
            0x62 => Some(Self::BitTest(4, R8::D)),
            0x63 => Some(Self::BitTest(4, R8::E)),
            0x64 => Some(Self::BitTest(4, R8::H)),
            0x65 => Some(Self::BitTest(4, R8::L)),

            0x6F => Some(Self::BitTest(5, R8::A)),
            0x68 => Some(Self::BitTest(5, R8::B)),
            0x69 => Some(Self::BitTest(5, R8::C)),
            0x6A => Some(Self::BitTest(5, R8::D)),
            0x6B => Some(Self::BitTest(5, R8::E)),
            0x6C => Some(Self::BitTest(5, R8::H)),
            0x6D => Some(Self::BitTest(5, R8::L)),

            0x77 => Some(Self::BitTest(6, R8::A)),
            0x70 => Some(Self::BitTest(6, R8::B)),
            0x71 => Some(Self::BitTest(6, R8::C)),
            0x72 => Some(Self::BitTest(6, R8::D)),
            0x73 => Some(Self::BitTest(6, R8::E)),
            0x74 => Some(Self::BitTest(6, R8::H)),
            0x75 => Some(Self::BitTest(6, R8::L)),

            0x7F => Some(Self::BitTest(7, R8::A)),
            0x78 => Some(Self::BitTest(7, R8::B)),
            0x79 => Some(Self::BitTest(7, R8::C)),
            0x7A => Some(Self::BitTest(7, R8::D)),
            0x7B => Some(Self::BitTest(7, R8::E)),
            0x7C => Some(Self::BitTest(7, R8::H)),
            0x7D => Some(Self::BitTest(7, R8::L)),

            0x87 => Some(Self::BitReset(0, R8::A)),
            0x80 => Some(Self::BitReset(0, R8::B)),
            0x81 => Some(Self::BitReset(0, R8::C)),
            0x82 => Some(Self::BitReset(0, R8::D)),
            0x83 => Some(Self::BitReset(0, R8::E)),
            0x84 => Some(Self::BitReset(0, R8::H)),
            0x85 => Some(Self::BitReset(0, R8::L)),

            0x8F => Some(Self::BitReset(1, R8::A)),
            0x88 => Some(Self::BitReset(1, R8::B)),
            0x89 => Some(Self::BitReset(1, R8::C)),
            0x8A => Some(Self::BitReset(1, R8::D)),
            0x8B => Some(Self::BitReset(1, R8::E)),
            0x8C => Some(Self::BitReset(1, R8::H)),
            0x8D => Some(Self::BitReset(1, R8::L)),

            0x97 => Some(Self::BitReset(2, R8::A)),
            0x90 => Some(Self::BitReset(2, R8::B)),
            0x91 => Some(Self::BitReset(2, R8::C)),
            0x92 => Some(Self::BitReset(2, R8::D)),
            0x93 => Some(Self::BitReset(2, R8::E)),
            0x94 => Some(Self::BitReset(2, R8::H)),
            0x95 => Some(Self::BitReset(2, R8::L)),

            0x9F => Some(Self::BitReset(3, R8::A)),
            0x98 => Some(Self::BitReset(3, R8::B)),
            0x99 => Some(Self::BitReset(3, R8::C)),
            0x9A => Some(Self::BitReset(3, R8::D)),
            0x9B => Some(Self::BitReset(3, R8::E)),
            0x9C => Some(Self::BitReset(3, R8::H)),
            0x9D => Some(Self::BitReset(3, R8::L)),

            0xA7 => Some(Self::BitReset(4, R8::A)),
            0xA0 => Some(Self::BitReset(4, R8::B)),
            0xA1 => Some(Self::BitReset(4, R8::C)),
            0xA2 => Some(Self::BitReset(4, R8::D)),
            0xA3 => Some(Self::BitReset(4, R8::E)),
            0xA4 => Some(Self::BitReset(4, R8::H)),
            0xA5 => Some(Self::BitReset(4, R8::L)),

            0xAF => Some(Self::BitReset(5, R8::A)),
            0xA8 => Some(Self::BitReset(5, R8::B)),
            0xA9 => Some(Self::BitReset(5, R8::C)),
            0xAA => Some(Self::BitReset(5, R8::D)),
            0xAB => Some(Self::BitReset(5, R8::E)),
            0xAC => Some(Self::BitReset(5, R8::H)),
            0xAD => Some(Self::BitReset(5, R8::L)),

            0xB7 => Some(Self::BitReset(6, R8::A)),
            0xB0 => Some(Self::BitReset(6, R8::B)),
            0xB1 => Some(Self::BitReset(6, R8::C)),
            0xB2 => Some(Self::BitReset(6, R8::D)),
            0xB3 => Some(Self::BitReset(6, R8::E)),
            0xB4 => Some(Self::BitReset(6, R8::H)),
            0xB5 => Some(Self::BitReset(6, R8::L)),

            0xBF => Some(Self::BitReset(7, R8::A)),
            0xB8 => Some(Self::BitReset(7, R8::B)),
            0xB9 => Some(Self::BitReset(7, R8::C)),
            0xBA => Some(Self::BitReset(7, R8::D)),
            0xBB => Some(Self::BitReset(7, R8::E)),
            0xBC => Some(Self::BitReset(7, R8::H)),
            0xBD => Some(Self::BitReset(7, R8::L)),

            0xC7 => Some(Self::BitSet(0, R8::A)),
            0xC0 => Some(Self::BitSet(0, R8::B)),
            0xC1 => Some(Self::BitSet(0, R8::C)),
            0xC2 => Some(Self::BitSet(0, R8::D)),
            0xC3 => Some(Self::BitSet(0, R8::E)),
            0xC4 => Some(Self::BitSet(0, R8::H)),
            0xC5 => Some(Self::BitSet(0, R8::L)),

            0xCF => Some(Self::BitSet(1, R8::A)),
            0xC8 => Some(Self::BitSet(1, R8::B)),
            0xC9 => Some(Self::BitSet(1, R8::C)),
            0xCA => Some(Self::BitSet(1, R8::D)),
            0xCB => Some(Self::BitSet(1, R8::E)),
            0xCC => Some(Self::BitSet(1, R8::H)),
            0xCD => Some(Self::BitSet(1, R8::L)),

            0xD7 => Some(Self::BitSet(2, R8::A)),
            0xD0 => Some(Self::BitSet(2, R8::B)),
            0xD1 => Some(Self::BitSet(2, R8::C)),
            0xD2 => Some(Self::BitSet(2, R8::D)),
            0xD3 => Some(Self::BitSet(2, R8::E)),
            0xD4 => Some(Self::BitSet(2, R8::H)),
            0xD5 => Some(Self::BitSet(2, R8::L)),

            0xDF => Some(Self::BitSet(3, R8::A)),
            0xD8 => Some(Self::BitSet(3, R8::B)),
            0xD9 => Some(Self::BitSet(3, R8::C)),
            0xDA => Some(Self::BitSet(3, R8::D)),
            0xDB => Some(Self::BitSet(3, R8::E)),
            0xDC => Some(Self::BitSet(3, R8::H)),
            0xDD => Some(Self::BitSet(3, R8::L)),

            0xE7 => Some(Self::BitSet(4, R8::A)),
            0xE0 => Some(Self::BitSet(4, R8::B)),
            0xE1 => Some(Self::BitSet(4, R8::C)),
            0xE2 => Some(Self::BitSet(4, R8::D)),
            0xE3 => Some(Self::BitSet(4, R8::E)),
            0xE4 => Some(Self::BitSet(4, R8::H)),
            0xE5 => Some(Self::BitSet(4, R8::L)),

            0xEF => Some(Self::BitSet(5, R8::A)),
            0xE8 => Some(Self::BitSet(5, R8::B)),
            0xE9 => Some(Self::BitSet(5, R8::C)),
            0xEA => Some(Self::BitSet(5, R8::D)),
            0xEB => Some(Self::BitSet(5, R8::E)),
            0xEC => Some(Self::BitSet(5, R8::H)),
            0xED => Some(Self::BitSet(5, R8::L)),

            0xF7 => Some(Self::BitSet(6, R8::A)),
            0xF0 => Some(Self::BitSet(6, R8::B)),
            0xF1 => Some(Self::BitSet(6, R8::C)),
            0xF2 => Some(Self::BitSet(6, R8::D)),
            0xF3 => Some(Self::BitSet(6, R8::E)),
            0xF4 => Some(Self::BitSet(6, R8::H)),
            0xF5 => Some(Self::BitSet(6, R8::L)),

            0xFF => Some(Self::BitSet(7, R8::A)),
            0xF8 => Some(Self::BitSet(7, R8::B)),
            0xF9 => Some(Self::BitSet(7, R8::C)),
            0xFA => Some(Self::BitSet(7, R8::D)),
            0xFB => Some(Self::BitSet(7, R8::E)),
            0xFC => Some(Self::BitSet(7, R8::H)),
            0xFD => Some(Self::BitSet(7, R8::L)),

            // TODO: add mapping for the rest of instructions
            _ => None,
        }
    }

    const fn from_byte_not_prefixed(byte: u8) -> Option<Self> {
        match byte {
            // 8-bit arithmetic
            0x87 => Some(Self::Add(R8::A)),
            0x80 => Some(Self::Add(R8::B)),
            0x81 => Some(Self::Add(R8::C)),
            0x82 => Some(Self::Add(R8::D)),
            0x83 => Some(Self::Add(R8::E)),
            0x84 => Some(Self::Add(R8::H)),
            0x85 => Some(Self::Add(R8::L)),

            0x8F => Some(Self::AddWithCarry(R8::A)),
            0x88 => Some(Self::AddWithCarry(R8::B)),
            0x89 => Some(Self::AddWithCarry(R8::C)),
            0x8A => Some(Self::AddWithCarry(R8::D)),
            0x8B => Some(Self::AddWithCarry(R8::E)),
            0x8C => Some(Self::AddWithCarry(R8::H)),
            0x8D => Some(Self::AddWithCarry(R8::L)),

            0x97 => Some(Self::Subtract(R8::A)),
            0x90 => Some(Self::Subtract(R8::B)),
            0x91 => Some(Self::Subtract(R8::C)),
            0x92 => Some(Self::Subtract(R8::D)),
            0x93 => Some(Self::Subtract(R8::E)),
            0x94 => Some(Self::Subtract(R8::H)),
            0x95 => Some(Self::Subtract(R8::L)),

            0x9F => Some(Self::SubtractWithCarry(R8::A)),
            0x98 => Some(Self::SubtractWithCarry(R8::B)),
            0x99 => Some(Self::SubtractWithCarry(R8::C)),
            0x9A => Some(Self::SubtractWithCarry(R8::D)),
            0x9B => Some(Self::SubtractWithCarry(R8::E)),
            0x9C => Some(Self::SubtractWithCarry(R8::H)),
            0x9D => Some(Self::SubtractWithCarry(R8::L)),

            0xA7 => Some(Self::And(R8::A)),
            0xA0 => Some(Self::And(R8::B)),
            0xA1 => Some(Self::And(R8::C)),
            0xA2 => Some(Self::And(R8::D)),
            0xA3 => Some(Self::And(R8::E)),
            0xA4 => Some(Self::And(R8::H)),
            0xA5 => Some(Self::And(R8::L)),

            0xAF => Some(Self::Xor(R8::A)),
            0xA8 => Some(Self::Xor(R8::B)),
            0xA9 => Some(Self::Xor(R8::C)),
            0xAA => Some(Self::Xor(R8::D)),
            0xAB => Some(Self::Xor(R8::E)),
            0xAC => Some(Self::Xor(R8::H)),
            0xAD => Some(Self::Xor(R8::L)),

            0xB7 => Some(Self::Or(R8::A)),
            0xB0 => Some(Self::Or(R8::B)),
            0xB1 => Some(Self::Or(R8::C)),
            0xB2 => Some(Self::Or(R8::D)),
            0xB3 => Some(Self::Or(R8::E)),
            0xB4 => Some(Self::Or(R8::H)),
            0xB5 => Some(Self::Or(R8::L)),

            0xBF => Some(Self::Compare(R8::A)),
            0xB8 => Some(Self::Compare(R8::B)),
            0xB9 => Some(Self::Compare(R8::C)),
            0xBA => Some(Self::Compare(R8::D)),
            0xBB => Some(Self::Compare(R8::E)),
            0xBC => Some(Self::Compare(R8::H)),
            0xBD => Some(Self::Compare(R8::L)),

            0x3C => Some(Self::Increment(R8::A)),
            0x04 => Some(Self::Increment(R8::B)),
            0x0C => Some(Self::Increment(R8::C)),
            0x14 => Some(Self::Increment(R8::D)),
            0x1C => Some(Self::Increment(R8::E)),
            0x24 => Some(Self::Increment(R8::H)),
            0x2C => Some(Self::Increment(R8::L)),

            0x3D => Some(Self::Decrement(R8::A)),
            0x05 => Some(Self::Decrement(R8::B)),
            0x0D => Some(Self::Decrement(R8::C)),
            0x15 => Some(Self::Decrement(R8::D)),
            0x1D => Some(Self::Decrement(R8::E)),
            0x25 => Some(Self::Decrement(R8::H)),
            0x2D => Some(Self::Decrement(R8::L)),

            // 16-bit arithmetic
            0x09 => Some(Self::Add16(R16::BC)),
            0x19 => Some(Self::Add16(R16::DE)),
            0x29 => Some(Self::Add16(R16::HL)),
            0x39 => Some(Self::Add16(R16::SP)),

            0x03 => Some(Self::Increment16(R16::BC)),
            0x13 => Some(Self::Increment16(R16::DE)),
            0x23 => Some(Self::Increment16(R16::HL)),
            0x33 => Some(Self::Increment16(R16::SP)),

            0x0B => Some(Self::Decrement16(R16::BC)),
            0x1B => Some(Self::Decrement16(R16::DE)),
            0x2B => Some(Self::Decrement16(R16::HL)),
            0x3B => Some(Self::Decrement16(R16::SP)),

            // Bit shift
            0x07 => Some(Self::RotateLeftCircularAccumulator),
            0x17 => Some(Self::RotateLeftAccumulator),
            0x0F => Some(Self::RotateRightCircularAccumulator),
            0x1F => Some(Self::RotateRightAccumulator),

            // Jumps
            0xCD => Some(Self::Call(JumpCondition::Always)),
            0xC4 => Some(Self::Call(JumpCondition::NotZero)),
            0xCC => Some(Self::Call(JumpCondition::Zero)),
            0xD4 => Some(Self::Call(JumpCondition::NotCarry)),
            0xDC => Some(Self::Call(JumpCondition::Carry)),

            0xE9 => Some(Self::JumpToHL),

            0xC3 => Some(Self::Jump(JumpCondition::Always)),
            0xC2 => Some(Self::Jump(JumpCondition::NotZero)),
            0xCA => Some(Self::Jump(JumpCondition::Zero)),
            0xD2 => Some(Self::Jump(JumpCondition::NotCarry)),
            0xDA => Some(Self::Jump(JumpCondition::Carry)),

            0x18 => Some(Self::JumpRelative(JumpCondition::Always)),
            0x20 => Some(Self::JumpRelative(JumpCondition::NotZero)),
            0x28 => Some(Self::JumpRelative(JumpCondition::Zero)),
            0x30 => Some(Self::JumpRelative(JumpCondition::NotCarry)),
            0x38 => Some(Self::JumpRelative(JumpCondition::Carry)),

            0xC9 => Some(Self::Return(JumpCondition::Always)),
            0xC0 => Some(Self::Return(JumpCondition::NotZero)),
            0xC8 => Some(Self::Return(JumpCondition::Zero)),
            0xD0 => Some(Self::Return(JumpCondition::NotCarry)),
            0xD8 => Some(Self::Return(JumpCondition::Carry)),

            0xD9 => Some(Self::ReturnFromInterruptHandler),

            // Stack
            0xC1 => Some(Self::Pop(R16::BC)),
            0xD1 => Some(Self::Pop(R16::DE)),
            0xE1 => Some(Self::Pop(R16::HL)),
            0xF1 => Some(Self::Pop(R16::AF)),

            0xC5 => Some(Self::Push(R16::BC)),
            0xD5 => Some(Self::Push(R16::DE)),
            0xE5 => Some(Self::Push(R16::HL)),
            0xF5 => Some(Self::Push(R16::AF)),

            // Misc
            0x37 => Some(Self::SetCarryFlag),
            0x2F => Some(Self::Complement),
            0x3F => Some(Self::ComplimentCarryFlag),
            0xF3 => Some(Self::DisableInterrupt),
            0xFB => Some(Self::EnableInterrupt),

            // Undefined
            0xD3 | 0xDB | 0xDD | 0xE3 | 0xE4 | 0xEB | 0xEC | 0xED | 0xF4 | 0xFC | 0xFD => None,

            // TODO: add mapping for the rest of instructions
            _ => None,
        }
    }
}

/// 8-bit registers
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

/// 16-bit registers
#[derive(Debug, Clone, Copy)]
enum R16 {
    AF,
    BC,
    DE,
    HL,
    SP,
    PC,
}

enum Addr {
    HL,
}

trait ReadByte<T> {
    fn read_byte(&self, src: T) -> u8;
}

impl ReadByte<R8> for Cpu {
    fn read_byte(&self, src: R8) -> u8 {
        self.registers.read(src)
    }
}

impl ReadByte<Addr> for Cpu {
    fn read_byte(&self, src: Addr) -> u8 {
        match src {
            Addr::HL => {
                let address = self.registers.read16(R16::HL);
                self.bus.read_byte(address)
            }
        }
    }
}

trait WriteByte<T> {
    fn write_byte(&mut self, src: T, value: u8);
}

impl WriteByte<R8> for Cpu {
    fn write_byte(&mut self, src: R8, value: u8) {
        self.registers.write(src, value);
    }
}

impl WriteByte<Addr> for Cpu {
    fn write_byte(&mut self, src: Addr, value: u8) {
        match src {
            Addr::HL => {
                let address = self.registers.read16(R16::HL);
                self.bus.write_byte(address, value);
            }
        }
    }
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
    ime: bool,
}

impl Cpu {
    const fn new() -> Self {
        Self {
            registers: Registers::new(),
            bus: MemoryBus::new(),
            ime: false,
        }
    }

    fn read_next_byte(&mut self) -> u8 {
        let byte = self.bus.read_byte(self.registers.pc);
        self.registers.pc = self.registers.pc.wrapping_add(1);
        byte
    }

    fn read_next_word(&mut self) -> u16 {
        // Gameboy is little endian, so read the second byte as the most significant byte
        // and the first as the least significant
        let lsb = self.bus.read_byte(self.registers.pc);
        self.registers.pc = self.registers.pc.wrapping_add(1);
        let msb = self.bus.read_byte(self.registers.pc);
        self.registers.pc = self.registers.pc.wrapping_add(1);
        u16::from_le_bytes([lsb, msb])
    }

    fn step(&mut self) {
        let mut instruction_byte = self.read_next_byte();

        let prefixed = instruction_byte == 0xCB;
        if prefixed {
            instruction_byte = self.read_next_byte();
        }

        if let Some(instruction) = Instruction::from_byte(instruction_byte, prefixed) {
            self.execute(instruction);
        } else {
            let description = format!("0x{}{instruction_byte:x}", if prefixed { "CB" } else { "" });
            panic!("Unknown instruction found for: {description}");
        };
    }

    fn execute(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::Add(target) => {
                self.add(target);
            }
            Instruction::AddWithCarry(target) => {
                self.add_with_carry(target);
            }
            Instruction::Subtract(target) => {
                self.subtract(target);
            }
            Instruction::SubtractWithCarry(target) => {
                self.subtract_with_carry(target);
            }
            Instruction::And(target) => {
                self.and(target);
            }
            Instruction::Xor(target) => {
                self.xor(target);
            }
            Instruction::Or(target) => {
                self.or(target);
            }
            Instruction::Compare(target) => {
                self.compare(target);
            }
            Instruction::Increment(target) => {
                self.increment(target);
            }
            Instruction::Decrement(target) => {
                self.decrement(target);
            }
            Instruction::Add16(target) => {
                self.add16(target);
            }
            Instruction::Increment16(target) => {
                self.increment16(target);
            }
            Instruction::Decrement16(target) => {
                self.decrement16(target);
            }
            Instruction::BitTest(bit, target) => {
                self.bit_test(bit, target);
            }
            Instruction::BitReset(bit, target) => {
                self.bit_reset(bit, target);
            }
            Instruction::BitSet(bit, target) => {
                self.bit_set(bit, target);
            }
            Instruction::Swap(target) => {
                self.swap(target);
            }
            Instruction::RotateLeft(target) => {
                self.rotate_left(target);
            }
            Instruction::RotateLeftCircular(target) => {
                self.rotate_left_circular(target);
            }
            Instruction::RotateLeftCircularAccumulator => {
                self.rotate_left_circular_accumulator();
            }
            Instruction::RotateLeftAccumulator => {
                self.rotate_left_accumulator();
            }
            Instruction::RotateRight(target) => {
                self.rotate_right(target);
            }
            Instruction::RotateRightCircular(target) => {
                self.rotate_right_circular(target);
            }
            Instruction::RotateRightCircularAccumulator => {
                self.rotate_right_circular_accumulator();
            }
            Instruction::RotateRightAccumulator => {
                self.rotate_right_accumulator();
            }
            Instruction::ShiftLeftArithmetic(target) => {
                self.shift_left_arithmetic(target);
            }
            Instruction::ShiftRightArithmetic(target) => {
                self.shift_right_arithmetic(target);
            }
            Instruction::ShiftRightLogical(target) => {
                self.shift_right_logical(target);
            }
            Instruction::Call(condition) => self.call(condition),
            Instruction::JumpToHL => self.jump_to_hl(),
            Instruction::Jump(condition) => self.jump(condition),
            Instruction::JumpRelative(condition) => self.jump_relative(condition),
            Instruction::Return(condition) => self.returns(condition),
            Instruction::ReturnFromInterruptHandler => self.return_from_interrupt_handler(),
            Instruction::Pop(target) => {
                self.pop(target);
            }
            Instruction::Push(target) => {
                self.push(target);
            }
            Instruction::SetCarryFlag => {
                self.set_carry_flag();
            }
            Instruction::Complement => {
                self.complement();
            }
            Instruction::ComplimentCarryFlag => {
                self.complement_carry_flag();
            }
            Instruction::DisableInterrupt => {
                self.disable_interrupts();
            }
            Instruction::EnableInterrupt => {
                self.enable_interrupts();
            }
        }
    }

    /// ADD A, r8
    /// 1 4
    /// Z 0 H C
    ///
    /// Add the value in r8 to register A.
    fn add<T>(&mut self, operand: T)
    where
        Self: ReadByte<T>,
    {
        let value = self.read_byte(operand);
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
        self.registers.a = new_value;
    }

    /// ADC A, r8
    /// 1 4
    /// Z 0 H C
    ///
    /// Add the value in r8 plus the carry flag to register A.
    fn add_with_carry<T>(&mut self, operand: T)
    where
        Self: ReadByte<T>,
    {
        let value = self.read_byte(operand);
        let a = self.registers.a;
        let cf = self.registers.f.contains(Flags::CARRY) as u8;
        let new_value = a.wrapping_add(value).wrapping_add(cf);
        self.registers.f.set(Flags::ZERO, new_value == 0);
        self.registers.f.set(Flags::SUBTRACT, false);
        let carry = a as u16 + value as u16 + cf as u16 > 0xFF;
        self.registers.f.set(Flags::CARRY, carry);
        let half_carry = (a & 0xF) + (value & 0xF) + cf > 0xF;
        self.registers.f.set(Flags::HALF_CARRY, half_carry);
        self.registers.a = new_value;
    }

    /// SUB A, r8
    /// 1 4
    /// Z 1 H C
    ///
    /// Subtract the value in r8 from register A.
    fn subtract<T>(&mut self, subtrahend: T)
    where
        Self: ReadByte<T>,
    {
        let value = self.read_byte(subtrahend);
        let a = self.registers.a;
        let (new_value, did_overflow) = a.overflowing_sub(value);
        self.registers.f.set(Flags::ZERO, new_value == 0);
        self.registers.f.set(Flags::SUBTRACT, true);
        self.registers.f.set(Flags::CARRY, did_overflow);
        let half_carry = (a & 0xF) < (value & 0xF);
        self.registers.f.set(Flags::HALF_CARRY, half_carry);
        self.registers.a = new_value;
    }

    /// SBC A, r8
    /// 1 4
    /// Z 1 H C
    ///
    /// Subtract the value in r8 and the carry flag from register A.
    fn subtract_with_carry<T>(&mut self, subtrahend: T)
    where
        Self: ReadByte<T>,
    {
        let value = self.read_byte(subtrahend);
        let a = self.registers.a;
        let cf = self.registers.f.contains(Flags::CARRY) as u8;
        let new_value = a.wrapping_sub(value).wrapping_sub(cf);
        self.registers.f.set(Flags::ZERO, new_value == 0);
        self.registers.f.set(Flags::SUBTRACT, true);
        let carry = (a as u16) < (value as u16) + (cf as u16);
        self.registers.f.set(Flags::CARRY, carry);
        let half_carry = (a & 0xF) < (value & 0xF) + cf;
        self.registers.f.set(Flags::HALF_CARRY, half_carry);
        self.registers.a = new_value;
    }

    /// AND A, r8
    /// 1 4
    /// Z 0 1 0
    ///
    /// Bitwise AND between the value in r8 and register A.
    fn and<T>(&mut self, operand: T)
    where
        Self: ReadByte<T>,
    {
        let value = self.read_byte(operand);
        let new_value = self.registers.a & value;
        self.registers.f.set(Flags::ZERO, new_value == 0);
        self.registers.f.set(Flags::SUBTRACT, false);
        self.registers.f.set(Flags::HALF_CARRY, true);
        self.registers.f.set(Flags::CARRY, false);
        self.registers.a = new_value;
    }

    /// XOR A, r8
    /// 1 4
    /// Z 0 0 0
    ///
    /// Bitwise XOR between the value in r8 and register A.
    fn xor<T>(&mut self, operand: T)
    where
        Self: ReadByte<T>,
    {
        let value = self.read_byte(operand);
        let new_value = self.registers.a ^ value;
        self.registers.f.set(Flags::ZERO, new_value == 0);
        self.registers.f.set(Flags::SUBTRACT, false);
        self.registers.f.set(Flags::HALF_CARRY, false);
        self.registers.f.set(Flags::CARRY, false);
        self.registers.a = new_value;
    }

    /// OR A, r8
    /// 1 4
    /// Z 0 0 0
    ///
    /// Bitwise OR between the value in r8 and register A.
    fn or<T>(&mut self, operand: T)
    where
        Self: ReadByte<T>,
    {
        let value = self.read_byte(operand);
        let new_value = self.registers.a | value;
        self.registers.f.set(Flags::ZERO, new_value == 0);
        self.registers.f.set(Flags::SUBTRACT, true);
        self.registers.f.set(Flags::HALF_CARRY, false);
        self.registers.f.set(Flags::CARRY, false);
        self.registers.a = new_value;
    }

    /// CP A, r8
    /// 1 4
    /// Z 1 H C
    ///
    /// Subtract the value in r8 from register A and set flags accordingly, but don't store the result.
    fn compare<T>(&mut self, subtrahend: T)
    where
        Self: ReadByte<T>,
    {
        let value = self.read_byte(subtrahend);
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
    fn increment<T>(&mut self, src: T)
    where
        T: Copy,
        Self: ReadByte<T> + WriteByte<T>,
    {
        let value = self.read_byte(src);
        let new_value = value.wrapping_add(1);
        self.registers.f.set(Flags::ZERO, new_value == 0);
        self.registers.f.set(Flags::SUBTRACT, false);
        let half_carry = (value & 0xF) == 0xF;
        self.registers.f.set(Flags::HALF_CARRY, half_carry);
        // CARRY is left untouched
        self.write_byte(src, new_value);
    }

    /// DEC r8
    /// 1 4
    /// Z 1 H -
    ///
    /// Decrement value in register r8 by 1.
    fn decrement<T>(&mut self, src: T)
    where
        T: Copy,
        Self: ReadByte<T> + WriteByte<T>,
    {
        let value = self.read_byte(src);
        let new_value = value.wrapping_sub(1);
        self.registers.f.set(Flags::ZERO, new_value == 0);
        self.registers.f.set(Flags::SUBTRACT, true);
        let half_carry = (value & 0xF) == 0;
        self.registers.f.set(Flags::HALF_CARRY, half_carry);
        // CARRY is left untouched
        self.write_byte(src, new_value);
    }

    /// ADD HL, r16
    /// 1 8
    /// - 0 H C
    ///
    /// Add the value in r16 to register HL.
    fn add16(&mut self, operand: R16) {
        let value = self.registers.read16(operand);
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
        self.registers.write16(R16::HL, new_value);
    }

    /// INC r16
    /// 1 8
    /// - - - -
    ///
    /// Increment value in register r16 by 1.
    fn increment16(&mut self, src: R16) {
        let value = self.registers.read16(src);
        let new_value = value.wrapping_add(1);
        self.registers.write16(src, new_value);
    }

    /// DEC r16
    /// 1 8
    /// - - - -
    ///
    /// Decrement value in register r16 by 1.
    fn decrement16(&mut self, src: R16) {
        let value = self.registers.read16(src);
        let new_value = value.wrapping_sub(1);
        self.registers.write16(src, new_value);
    }

    /// RLCA
    /// 1 4
    /// 0 0 0 C
    ///
    /// Rotate register A left.
    fn rotate_left_circular_accumulator(&mut self) {
        let value = self.registers.a;
        let new_value = self.registers.a.rotate_left(1);
        self.registers.f.set(Flags::ZERO, false);
        self.registers.f.set(Flags::SUBTRACT, false);
        self.registers.f.set(Flags::HALF_CARRY, false);
        let carry = value & 0x80 != 0;
        self.registers.f.set(Flags::CARRY, carry);
        self.registers.a = new_value;
    }

    /// RLA
    /// 1 4
    /// 0 0 0 C
    ///
    /// Rotate register A left, through the carry flag.
    fn rotate_left_accumulator(&mut self) {
        let value = self.registers.a;
        let cf = self.registers.f.contains(Flags::CARRY) as u8;
        let new_value = (value << 1) | cf;
        self.registers.f.set(Flags::ZERO, false);
        self.registers.f.set(Flags::SUBTRACT, false);
        self.registers.f.set(Flags::HALF_CARRY, false);
        let carry = value & 0x80 != 0;
        self.registers.f.set(Flags::CARRY, carry);
        self.registers.a = new_value;
    }

    /// RRCA
    /// 1 4
    /// 0 0 0 C
    ///
    /// Rotate register A right.
    fn rotate_right_circular_accumulator(&mut self) {
        let value = self.registers.a;
        let new_value = self.registers.a.rotate_right(1);
        self.registers.f.set(Flags::ZERO, false);
        self.registers.f.set(Flags::SUBTRACT, false);
        self.registers.f.set(Flags::HALF_CARRY, false);
        let carry = value & 0x01 != 0;
        self.registers.f.set(Flags::CARRY, carry);
        self.registers.a = new_value;
    }

    /// RRA
    /// 1 4
    /// 0 0 0 C
    ///
    /// Rotate register A right, through the carry flag.
    fn rotate_right_accumulator(&mut self) {
        let value = self.registers.a;
        let cf = self.registers.f.contains(Flags::CARRY) as u8;
        let new_value = (value >> 1) | (cf << 7);
        self.registers.f.set(Flags::ZERO, false);
        self.registers.f.set(Flags::SUBTRACT, false);
        self.registers.f.set(Flags::HALF_CARRY, false);
        let carry = value & 0x01 != 0;
        self.registers.f.set(Flags::CARRY, carry);
        self.registers.a = new_value;
    }

    /// SCF
    /// 1 4
    /// - 0 0 1
    ///
    /// Set the carry flag.
    fn set_carry_flag(&mut self) {
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
    fn complement(&mut self) {
        let value = self.registers.a;
        // ZERO left untouched
        self.registers.f.set(Flags::SUBTRACT, true);
        self.registers.f.set(Flags::HALF_CARRY, true);
        // CARRY left untouched
        self.registers.a = !value;
    }

    /// CCF
    /// 1 4
    /// - 0 0 C
    ///
    /// Complement the carry flag.
    fn complement_carry_flag(&mut self) {
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
    fn rotate_left_circular<T>(&mut self, src: T)
    where
        T: Copy,
        Self: ReadByte<T> + WriteByte<T>,
    {
        let value = self.read_byte(src);
        let new_value = value.rotate_left(1);
        self.registers.f.set(Flags::ZERO, new_value == 0);
        self.registers.f.set(Flags::SUBTRACT, false);
        self.registers.f.set(Flags::HALF_CARRY, false);
        let carry = value & 0x80 != 0;
        self.registers.f.set(Flags::CARRY, carry);
        self.write_byte(src, new_value);
    }

    /// RRC r8
    /// 2 8
    /// Z 0 0 C
    ///
    /// Rotate register r8 right.
    fn rotate_right_circular<T>(&mut self, src: T)
    where
        T: Copy,
        Self: ReadByte<T> + WriteByte<T>,
    {
        let value = self.read_byte(src);
        let new_value = value.rotate_right(1);
        self.registers.f.set(Flags::ZERO, new_value == 0);
        self.registers.f.set(Flags::SUBTRACT, false);
        self.registers.f.set(Flags::HALF_CARRY, false);
        let carry = value & 0x01 != 0;
        self.registers.f.set(Flags::CARRY, carry);
        self.write_byte(src, new_value);
    }

    /// RL r8
    /// 2 8
    /// Z 0 0 C
    ///
    /// Rotate bits in register r8 left, through the carry flag.
    fn rotate_left<T>(&mut self, src: T)
    where
        T: Copy,
        Self: ReadByte<T> + WriteByte<T>,
    {
        let value = self.read_byte(src);
        let cf = self.registers.f.contains(Flags::CARRY) as u8;
        let new_value = (value << 1) | cf;
        self.registers.f.set(Flags::ZERO, new_value == 0);
        self.registers.f.set(Flags::SUBTRACT, false);
        self.registers.f.set(Flags::HALF_CARRY, false);
        let carry = value & 0x80 != 0;
        self.registers.f.set(Flags::CARRY, carry);
        self.write_byte(src, new_value);
    }

    /// RR r8
    /// 2 8
    /// Z 0 0 C
    ///
    /// Rotate register r8 right, through the carry flag.
    fn rotate_right<T>(&mut self, src: T)
    where
        T: Copy,
        Self: ReadByte<T> + WriteByte<T>,
    {
        let value = self.read_byte(src);
        let cf = self.registers.f.contains(Flags::CARRY) as u8;
        let new_value = (value >> 1) | (cf << 7);
        self.registers.f.set(Flags::ZERO, new_value == 0);
        self.registers.f.set(Flags::SUBTRACT, false);
        self.registers.f.set(Flags::HALF_CARRY, false);
        let carry = value & 0x01 != 0;
        self.registers.f.set(Flags::CARRY, carry);
        self.write_byte(src, new_value);
    }

    /// SLA r8
    /// 2 8
    /// Z 0 0 C
    ///
    /// Shift Left Arithmetically register r8.
    fn shift_left_arithmetic<T>(&mut self, src: T)
    where
        T: Copy,
        Self: ReadByte<T> + WriteByte<T>,
    {
        let value = self.read_byte(src);
        let new_value = value << 1;
        self.registers.f.set(Flags::ZERO, new_value == 0);
        self.registers.f.set(Flags::SUBTRACT, false);
        self.registers.f.set(Flags::HALF_CARRY, false);
        let carry = value & 0x80 != 0;
        self.registers.f.set(Flags::CARRY, carry);
        self.write_byte(src, new_value);
    }

    /// SRA r8
    /// 2 8
    /// Z 0 0 C
    ///
    /// Shift Right Arithmetically register r8 (bit 7 of r8 is unchanged).
    fn shift_right_arithmetic<T>(&mut self, src: T)
    where
        T: Copy,
        Self: ReadByte<T> + WriteByte<T>,
    {
        let value = self.read_byte(src);
        let new_value = (value >> 1) | (value & 0x80);
        self.registers.f.set(Flags::ZERO, new_value == 0);
        self.registers.f.set(Flags::SUBTRACT, false);
        self.registers.f.set(Flags::HALF_CARRY, false);
        let carry = value & 0x01 != 0;
        self.registers.f.set(Flags::CARRY, carry);
        self.write_byte(src, new_value);
    }

    /// SWAP r8
    /// 2 8
    /// Z 0 0 0
    ///
    /// Swap the upper 4 bits in register r8 and the lower 4 ones.
    fn swap<T>(&mut self, src: T)
    where
        T: Copy,
        Self: ReadByte<T> + WriteByte<T>,
    {
        let value = self.read_byte(src);
        // Rotating by 4 swaps the upper bits with the lower bits
        let new_value = value.rotate_left(4);
        self.registers.f.set(Flags::ZERO, new_value == 0);
        self.registers.f.set(Flags::SUBTRACT, false);
        self.registers.f.set(Flags::HALF_CARRY, false);
        self.registers.f.set(Flags::CARRY, false);
        self.write_byte(src, new_value);
    }

    /// SRL r8
    /// 2 8
    /// Z 0 0 C
    ///
    /// Shift Right Logically register r8.
    fn shift_right_logical<T>(&mut self, src: T)
    where
        T: Copy,
        Self: ReadByte<T> + WriteByte<T>,
    {
        let value = self.read_byte(src);
        let new_value = value >> 1;
        self.registers.f.set(Flags::ZERO, new_value == 0);
        self.registers.f.set(Flags::SUBTRACT, false);
        self.registers.f.set(Flags::HALF_CARRY, false);
        let carry = value & 0x01 != 0;
        self.registers.f.set(Flags::CARRY, carry);
        self.write_byte(src, new_value);
    }

    /// BIT u3, r8
    /// 2 8
    /// Z 0 1 -
    ///
    /// Test bit u3 in register r8, set the zero flag if bit not set.
    fn bit_test<T>(&mut self, bit: u8, src: T)
    where
        Self: ReadByte<T>,
    {
        let value = self.read_byte(src);
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
    fn bit_reset<T>(&mut self, bit: u8, src: T)
    where
        T: Copy,
        Self: ReadByte<T> + WriteByte<T>,
    {
        let value = self.read_byte(src);
        let new_value = value & !(1 << bit);
        // Flags left untouched
        self.write_byte(src, new_value);
    }

    /// SET u3, r8
    /// 2 8
    /// - - - -
    ///
    /// Set bit u3 in register r8 to 1. Bit 0 is the rightmost one, bit 7 the leftmost one.
    fn bit_set<T>(&mut self, bit: u8, src: T)
    where
        T: Copy,
        Self: ReadByte<T> + WriteByte<T>,
    {
        let value = self.read_byte(src);
        let new_value = value | (1 << bit);
        // Flags left untouched
        self.write_byte(src, new_value);
    }

    /// JP HL
    /// 1 4
    /// - - - -
    ///
    /// Jump to address in HL; effectively, load PC with value in register HL.
    fn jump_to_hl(&mut self) {
        self.registers.pc = self.registers.read16(R16::HL);
    }

    /// JP cc, n16
    /// 3 16/12
    /// - - - -
    ///
    /// Jump to address n16 if condition cc is met.
    fn jump(&mut self, condition: JumpCondition) {
        let should_jump = self.registers.f.test(condition);
        let address = self.read_next_word();
        if should_jump {
            self.registers.pc = address;
        }
    }

    /// JR cc, e8
    /// 2 12/8
    /// - - - -
    ///
    /// Relative Jump to current address plus e8 offset if condition cc is met.
    fn jump_relative(&mut self, condition: JumpCondition) {
        let should_jump = self.registers.f.test(condition);
        let offset = self.read_next_byte() as i16;
        if should_jump {
            self.registers.pc = self.registers.pc.wrapping_add_signed(offset);
        }
    }

    /// PUSH r16
    /// 1 16
    /// - - - -
    ///
    /// Push register r16 into the stack.
    fn push(&mut self, register: R16) {
        let value = self.registers.read16(register);
        self.registers.sp = self.registers.sp.wrapping_sub(1);
        self.bus
            .write_byte(self.registers.sp, ((value & 0xFF00) >> 8) as u8);

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
    fn pop(&mut self, register: R16) {
        let lsb = self.bus.read_byte(self.registers.sp) as u16;
        self.registers.sp = self.registers.sp.wrapping_add(1);

        let msb = self.bus.read_byte(self.registers.sp) as u16;
        self.registers.sp = self.registers.sp.wrapping_add(1);

        let value = (msb << 8) | lsb;
        self.registers.write16(register, value);
    }

    /// CALL cc, n16
    /// 3 24/12
    /// - - - -
    ///
    /// Call address n16 if condition cc is met.
    fn call(&mut self, condition: JumpCondition) {
        let should_jump = self.registers.f.test(condition);
        let address = self.read_next_word();
        if should_jump {
            self.push(R16::PC);
            self.registers.pc = address;
        }
    }

    /// RET cc
    /// 1 20/8
    /// - - - -
    ///
    /// Return from subroutine if condition cc is met.
    fn returns(&mut self, condition: JumpCondition) {
        let should_jump = self.registers.f.test(condition);
        if should_jump {
            self.pop(R16::PC);
        }
    }

    /// RETI
    /// 1 16
    /// - - - -
    ///
    /// Return from subroutine and enable interrupts.
    /// This is basically equivalent to executing EI then RET, meaning that IME is set right after this instruction.
    fn return_from_interrupt_handler(&mut self) {
        self.returns(JumpCondition::Always);
        self.ime = true;
    }

    /// DI
    /// 1 4
    /// - - - -
    ///
    /// Disable Interrupts by clearing the IME flag.
    fn disable_interrupts(&mut self) {
        self.ime = false;
    }

    /// EI
    /// 1 4
    /// - - - -
    ///
    /// Enable Interrupts by setting the IME flag.
    /// The flag is only set after the instruction following EI.
    fn enable_interrupts(&mut self) {
        self.step();
        self.ime = true;
    }
}
