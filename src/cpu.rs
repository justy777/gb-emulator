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
    RL(R8),
    RLA,
    RLC(R8),
    RLCA,
    RR(R8),
    RRA,
    RRC(R8),
    RRCA,
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
    /// Toggle the carry flag.
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
}
