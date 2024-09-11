use bitflags::bitflags;

struct Registers {
    a: u8,
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    f: Flags,
    h: u8,
    l: u8,
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
        }
    }

    const fn get_af(&self) -> u16 {
        (self.a as u16) << 8 | self.f.bits() as u16
    }

    fn set_af(&mut self, value: u16) {
        self.a = (value >> 8) as u8;
        self.f = Flags::from_bits_truncate(value as u8);
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
            R16::BC => (self.b as u16) << 8 | self.c as u16,
            R16::DE => (self.d as u16) << 8 | self.e as u16,
            R16::HL => (self.h as u16) << 8 | self.l as u16,
        }
    }

    fn write16(&mut self, register: R16, value: u16) {
        match register {
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
    ADDHL(R16),
    ADC(R8),
    SUB(R8),
    SBC(R8),
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
    BC,
    DE,
    HL,
}

struct Cpu {
    registers: Registers,
}

impl Cpu {
    fn execute(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::ADD(target) => {
                let value = self.registers.read(target);
                let new_value = self.add(value);
                self.registers.a = new_value;
            }
            Instruction::ADC(target) => {
                let value = self.registers.read(target);
                let new_value = self.adc(value);
                self.registers.a =  new_value;
            }
            Instruction::ADDHL(target) => {
                let value = self.registers.read16(target);
                let new_value = self.add_hl(value);
                self.registers.write16(R16::HL, new_value);
            }
            Instruction::SUB(target) => {
                let value = self.registers.read(target);
                let new_value = self.sub(value);
                self.registers.a = new_value;
            }
            Instruction::SBC(target) => {
                let value = self.registers.read(target);
                let new_value = self.sbc(value);
                self.registers.a = new_value;
            }
            _ => { /* TODO: support more instructions */ }
        }
    }

    fn add(&mut self, value: u8) -> u8 {
        let (new_value, did_overflow) = self.registers.a.overflowing_add(value);
        self.registers.f.set(Flags::ZERO, new_value == 0);
        self.registers.f.set(Flags::SUBTRACT, false);
        self.registers.f.set(Flags::CARRY, did_overflow);
        // Half carry is set if adding the lower nibbles of the value and register A
        // together result in a value bigger than 0xF. If the result is larger than 0xF
        // than the addition caused a carry from the lower nibble to the upper nibble.
        let half_carry = (self.registers.a & 0xF) + (value & 0xF) > 0xF;
        self.registers.f.set(Flags::HALF_CARRY, half_carry);
        new_value
    }

    fn adc(&mut self, value: u8) -> u8 {
        let cf = self.registers.f.contains(Flags::CARRY) as u8;
        let new_value = self.registers.a.wrapping_add(value).wrapping_add(cf);
        self.registers.f.set(Flags::ZERO, new_value == 0);
        self.registers.f.set(Flags::SUBTRACT, false);
        let carry = self.registers.a as u16 + value as u16 + cf as u16 > 0xFF;
        self.registers.f.set(Flags::CARRY, carry);
        let half_carry = (self.registers.a & 0xF) + (value & 0xF) + cf > 0xF;
        self.registers.f.set(Flags::HALF_CARRY, half_carry);
        new_value
    }

    fn add_hl(&mut self, value: u16) -> u16 {
        let (new_value, did_overflow) = self.registers.read16(R16::HL).overflowing_add(value);
        self.registers.f.set(Flags::ZERO, new_value == 0);
        self.registers.f.set(Flags::SUBTRACT, false);
        self.registers.f.set(Flags::CARRY, did_overflow);
        // 16-bit operation, bitmask 12
        let half_carry = (self.registers.read16(R16::HL) & 0xFFF) + (value & 0xFFF) > 0xFFF;
        self.registers.f.set(Flags::HALF_CARRY, half_carry);
        new_value
    }

    fn sub(&mut self, value: u8) -> u8 {
        let (new_value, did_overflow) = self.registers.a.overflowing_sub(value);
        self.registers.f.set(Flags::ZERO, new_value == 0);
        self.registers.f.set(Flags::SUBTRACT, true);
        self.registers.f.set(Flags::CARRY, did_overflow);
        let half_carry = (self.registers.a & 0xF).wrapping_sub(value & 0xF) & (0xF - 1) != 0;
        self.registers.f.set(Flags::HALF_CARRY, half_carry);
        new_value
    }

    fn sbc(&mut self, value: u8) -> u8 {
        let cf = self.registers.f.contains(Flags::CARRY) as u8;
        let new_value = self.registers.a.wrapping_sub(value).wrapping_sub(cf);
        self.registers.f.set(Flags::ZERO, new_value == 0);
        self.registers.f.set(Flags::SUBTRACT, true);
        let carry = (self.registers.a as u16) < (value as u16) + (cf as u16);
        self.registers.f.set(Flags::CARRY, carry);
        let half_carry = (self.registers.a & 0xF).wrapping_sub(value & 0xF).wrapping_sub(cf) & (0xF - 1) != 0;
        self.registers.f.set(Flags::HALF_CARRY, half_carry);
        new_value
    }
}
