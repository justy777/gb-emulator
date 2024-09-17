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

/// 8-bit registers (r8)
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

/// 16-bit registers (r16)
#[derive(Debug, Clone, Copy)]
enum R16 {
    AF,
    BC,
    DE,
    HL,
    SP,
    PC,
}

#[derive(Debug, Clone, Copy)]
enum Addr {
    BC,
    DE,
    HL,
    HLi,
    HLd,
    N16,
}

#[derive(Debug, Clone, Copy)]
enum HighAddr {
    C,
    N8,
}

/// Unit struct to represent next byte (n8)
#[derive(Debug, Clone, Copy)]
struct N8;

/// Unit struct to represent next word (n16)
#[derive(Debug, Clone, Copy)]
struct N16;

trait ReadByte<S> {
    fn read_byte(&mut self, src: S) -> u8;
}

impl ReadByte<R8> for Cpu {
    fn read_byte(&mut self, src: R8) -> u8 {
        self.registers.read(src)
    }
}

impl ReadByte<Addr> for Cpu {
    fn read_byte(&mut self, src: Addr) -> u8 {
        match src {
            Addr::BC => {
                let address = self.registers.read16(R16::BC);
                self.bus.read_byte(address)
            }
            Addr::DE => {
                let address = self.registers.read16(R16::DE);
                self.bus.read_byte(address)
            }
            Addr::HL => {
                let address = self.registers.read16(R16::HL);
                self.bus.read_byte(address)
            }
            Addr::HLi => {
                let address = self.registers.read16(R16::HL);
                let new_address = address.wrapping_add(1);
                self.registers.write16(R16::HL, new_address);
                self.bus.read_byte(address)
            }
            Addr::HLd => {
                let address = self.registers.read16(R16::HL);
                let new_address = address.wrapping_sub(1);
                self.registers.write16(R16::HL, new_address);
                self.bus.read_byte(address)
            }
            Addr::N16 => {
                let address = self.read_next_word();
                self.bus.read_byte(address)
            }
        }
    }
}

impl ReadByte<HighAddr> for Cpu {
    fn read_byte(&mut self, src: HighAddr) -> u8 {
        match src {
            HighAddr::C => {
                let address = self.registers.read(R8::C) as u16;
                self.bus.read_byte(0xFF00 + address)
            }
            HighAddr::N8 => {
                let address = self.read_next_byte() as u16;
                self.bus.read_byte(0xFF00 + address)
            }
        }
    }
}

impl ReadByte<N8> for Cpu {
    fn read_byte(&mut self, _: N8) -> u8 {
        self.read_next_byte()
    }
}

trait WriteByte<T> {
    fn write_byte(&mut self, target: T, value: u8);
}

impl WriteByte<R8> for Cpu {
    fn write_byte(&mut self, target: R8, value: u8) {
        self.registers.write(target, value);
    }
}

impl WriteByte<Addr> for Cpu {
    fn write_byte(&mut self, target: Addr, value: u8) {
        match target {
            Addr::BC => {
                let address = self.registers.read16(R16::BC);
                self.bus.write_byte(address, value);
            }
            Addr::DE => {
                let address = self.registers.read16(R16::DE);
                self.bus.write_byte(address, value);
            }
            Addr::HL => {
                let address = self.registers.read16(R16::HL);
                self.bus.write_byte(address, value);
            }
            Addr::HLi => {
                let address = self.registers.read16(R16::HL);
                let new_address = address.wrapping_add(1);
                self.registers.write16(R16::HL, new_address);
                self.bus.write_byte(address, value);
            }
            Addr::HLd => {
                let address = self.registers.read16(R16::HL);
                let new_address = address.wrapping_sub(1);
                self.registers.write16(R16::HL, new_address);
                self.bus.write_byte(address, value);
            }
            Addr::N16 => {
                let address = self.read_next_word();
                self.bus.write_byte(address, value);
            }
        }
    }
}

impl WriteByte<HighAddr> for Cpu {
    fn write_byte(&mut self, target: HighAddr, value: u8) {
        match target {
            HighAddr::C => {
                let address = self.registers.read(R8::C) as u16;
                self.bus.write_byte(0xFF00 + address, value);
            }
            HighAddr::N8 => {
                let address = self.read_next_byte() as u16;
                self.bus.write_byte(0xFF00 + address, value);
            }
        }
    }
}

trait ReadWord<S> {
    fn read_word(&mut self, src: S) -> u16;
}

impl ReadWord<R16> for Cpu {
    fn read_word(&mut self, src: R16) -> u16 {
        self.registers.read16(src)
    }
}

impl ReadWord<N16> for Cpu {
    fn read_word(&mut self, _: N16) -> u16 {
        self.read_next_word()
    }
}

trait WriteWord<T> {
    fn write_word(&mut self, target: T, value: u16);
}

impl WriteWord<R16> for Cpu {
    fn write_word(&mut self, target: R16, value: u16) {
        self.registers.write16(target, value);
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
        // Game Boy is little endian, so read the second byte as the most significant byte
        // and the first as the least significant
        let lsb = self.bus.read_byte(self.registers.pc);
        self.registers.pc = self.registers.pc.wrapping_add(1);
        let msb = self.bus.read_byte(self.registers.pc);
        self.registers.pc = self.registers.pc.wrapping_add(1);
        u16::from_le_bytes([lsb, msb])
    }

    fn step(&mut self) {
        let instruction_byte = self.read_next_byte();
        self.execute(instruction_byte);
    }

    fn execute(&mut self, byte: u8) {
        match byte {
            // ---- 8-bit Arithmetic
            // ADD
            0x87 => self.add(R8::A),
            0x80 => self.add(R8::B),
            0x81 => self.add(R8::C),
            0x82 => self.add(R8::D),
            0x83 => self.add(R8::E),
            0x84 => self.add(R8::H),
            0x85 => self.add(R8::L),
            0x86 => self.add(Addr::HL),
            0xC6 => self.add(N8),
            // ADC
            0x8F => self.add_with_carry(R8::A),
            0x88 => self.add_with_carry(R8::B),
            0x89 => self.add_with_carry(R8::C),
            0x8A => self.add_with_carry(R8::D),
            0x8B => self.add_with_carry(R8::E),
            0x8C => self.add_with_carry(R8::H),
            0x8D => self.add_with_carry(R8::L),
            0x8E => self.add_with_carry(Addr::HL),
            0xCE => self.add_with_carry(N8),
            // SUB
            0x97 => self.subtract(R8::A),
            0x90 => self.subtract(R8::B),
            0x91 => self.subtract(R8::C),
            0x92 => self.subtract(R8::D),
            0x93 => self.subtract(R8::E),
            0x94 => self.subtract(R8::H),
            0x95 => self.subtract(R8::L),
            0x96 => self.subtract(Addr::HL),
            0xD6 => self.subtract(N8),
            // SBC
            0x9F => self.subtract_with_carry(R8::A),
            0x98 => self.subtract_with_carry(R8::B),
            0x99 => self.subtract_with_carry(R8::C),
            0x9A => self.subtract_with_carry(R8::D),
            0x9B => self.subtract_with_carry(R8::E),
            0x9C => self.subtract_with_carry(R8::H),
            0x9D => self.subtract_with_carry(R8::L),
            0x9E => self.subtract_with_carry(Addr::HL),
            0xDE => self.subtract_with_carry(N8),
            // AND
            0xA7 => self.and(R8::A),
            0xA0 => self.and(R8::B),
            0xA1 => self.and(R8::C),
            0xA2 => self.and(R8::D),
            0xA3 => self.and(R8::E),
            0xA4 => self.and(R8::H),
            0xA5 => self.and(R8::L),
            0xA6 => self.and(Addr::HL),
            0xE6 => self.and(N8),
            // XOR
            0xAF => self.xor(R8::A),
            0xA8 => self.xor(R8::B),
            0xA9 => self.xor(R8::C),
            0xAA => self.xor(R8::D),
            0xAB => self.xor(R8::E),
            0xAC => self.xor(R8::H),
            0xAD => self.xor(R8::L),
            0xAE => self.xor(Addr::HL),
            0xEE => self.xor(N8),
            // OR
            0xB7 => self.or(R8::A),
            0xB0 => self.or(R8::B),
            0xB1 => self.or(R8::C),
            0xB2 => self.or(R8::D),
            0xB3 => self.or(R8::E),
            0xB4 => self.or(R8::H),
            0xB5 => self.or(R8::L),
            0xB6 => self.or(Addr::HL),
            0xF6 => self.or(N8),
            // CP
            0xBF => self.compare(R8::A),
            0xB8 => self.compare(R8::B),
            0xB9 => self.compare(R8::C),
            0xBA => self.compare(R8::D),
            0xBB => self.compare(R8::E),
            0xBC => self.compare(R8::H),
            0xBD => self.compare(R8::L),
            0xBE => self.compare(Addr::HL),
            0xFE => self.compare(N8),
            // INC
            0x3C => self.increment(R8::A),
            0x04 => self.increment(R8::B),
            0x0C => self.increment(R8::C),
            0x14 => self.increment(R8::D),
            0x1C => self.increment(R8::E),
            0x24 => self.increment(R8::H),
            0x2C => self.increment(R8::L),
            0x34 => self.increment(Addr::HL),
            // DEC
            0x3D => self.decrement(R8::A),
            0x05 => self.decrement(R8::B),
            0x0D => self.decrement(R8::C),
            0x15 => self.decrement(R8::D),
            0x1D => self.decrement(R8::E),
            0x25 => self.decrement(R8::H),
            0x2D => self.decrement(R8::L),
            0x35 => self.decrement(Addr::HL),
            // DAA
            0x27 => self.decimal_adjust_accumulator(),
            // SCF
            0x37 => self.set_carry_flag(),
            // CPL
            0x2F => self.complement_accumulator(),
            // CCF
            0x3F => self.complement_carry_flag(),
            // ---- 16-bit Arithmetic
            // ADD
            0x09 => self.add16_hl(R16::BC),
            0x19 => self.add16_hl(R16::DE),
            0x29 => self.add16_hl(R16::HL),
            0x39 => self.add16_hl(R16::SP),
            0xE8 => self.add16_sp(),
            // INC
            0x03 => self.increment16(R16::BC),
            0x13 => self.increment16(R16::DE),
            0x23 => self.increment16(R16::HL),
            0x33 => self.increment16(R16::SP),
            // DEC
            0x0B => self.decrement16(R16::BC),
            0x1B => self.decrement16(R16::DE),
            0x2B => self.decrement16(R16::HL),
            0x3B => self.decrement16(R16::SP),
            // ---- Bit Shift
            // RLCA
            0x07 => self.rotate_left_circular_accumulator(),
            // RRCA
            0x0F => self.rotate_right_circular_accumulator(),
            // RLA
            0x17 => self.rotate_left_accumulator(),
            // RRA
            0x1F => self.rotate_right_accumulator(),
            // ---- 8-bit Load
            // LD
            0x47 => self.load(R8::B, R8::A),
            0x40 => self.load(R8::B, R8::B),
            0x41 => self.load(R8::B, R8::C),
            0x42 => self.load(R8::B, R8::D),
            0x43 => self.load(R8::B, R8::E),
            0x44 => self.load(R8::B, R8::H),
            0x45 => self.load(R8::B, R8::L),
            0x46 => self.load(R8::B, Addr::HL),
            0x06 => self.load(R8::B, N8),
            0x4F => self.load(R8::C, R8::A),
            0x48 => self.load(R8::C, R8::B),
            0x49 => self.load(R8::C, R8::C),
            0x4A => self.load(R8::C, R8::D),
            0x4B => self.load(R8::C, R8::E),
            0x4C => self.load(R8::C, R8::H),
            0x4D => self.load(R8::C, R8::L),
            0x4E => self.load(R8::C, Addr::HL),
            0x0E => self.load(R8::C, N8),
            0x57 => self.load(R8::D, R8::A),
            0x50 => self.load(R8::D, R8::B),
            0x51 => self.load(R8::D, R8::C),
            0x52 => self.load(R8::D, R8::D),
            0x53 => self.load(R8::D, R8::E),
            0x54 => self.load(R8::D, R8::H),
            0x55 => self.load(R8::D, R8::L),
            0x56 => self.load(R8::D, Addr::HL),
            0x16 => self.load(R8::D, N8),
            0x5F => self.load(R8::E, R8::A),
            0x58 => self.load(R8::E, R8::B),
            0x59 => self.load(R8::E, R8::C),
            0x5A => self.load(R8::E, R8::D),
            0x5B => self.load(R8::E, R8::E),
            0x5C => self.load(R8::E, R8::H),
            0x5D => self.load(R8::E, R8::L),
            0x5E => self.load(R8::E, Addr::HL),
            0x1E => self.load(R8::E, N8),
            0x67 => self.load(R8::H, R8::A),
            0x60 => self.load(R8::H, R8::B),
            0x61 => self.load(R8::H, R8::C),
            0x62 => self.load(R8::H, R8::D),
            0x63 => self.load(R8::H, R8::E),
            0x64 => self.load(R8::H, R8::H),
            0x65 => self.load(R8::H, R8::L),
            0x66 => self.load(R8::H, Addr::HL),
            0x26 => self.load(R8::H, N8),
            0x6F => self.load(R8::L, R8::A),
            0x68 => self.load(R8::L, R8::B),
            0x69 => self.load(R8::L, R8::C),
            0x6A => self.load(R8::L, R8::D),
            0x6B => self.load(R8::L, R8::E),
            0x6C => self.load(R8::L, R8::H),
            0x6D => self.load(R8::L, R8::L),
            0x6E => self.load(R8::L, Addr::HL),
            0x2E => self.load(R8::L, N8),
            0x77 => self.load(Addr::HL, R8::A),
            0x70 => self.load(Addr::HL, R8::B),
            0x71 => self.load(Addr::HL, R8::C),
            0x72 => self.load(Addr::HL, R8::D),
            0x73 => self.load(Addr::HL, R8::E),
            0x74 => self.load(Addr::HL, R8::H),
            0x75 => self.load(Addr::HL, R8::L),
            0x36 => self.load(Addr::HL, N8),
            0x7F => self.load(R8::A, R8::A),
            0x78 => self.load(R8::A, R8::B),
            0x79 => self.load(R8::A, R8::C),
            0x7A => self.load(R8::A, R8::D),
            0x7B => self.load(R8::A, R8::E),
            0x7C => self.load(R8::A, R8::H),
            0x7D => self.load(R8::A, R8::L),
            0x7E => self.load(R8::A, Addr::HL),
            0x3E => self.load(R8::A, N8),
            0x02 => self.load(Addr::BC, R8::A),
            0x12 => self.load(Addr::DE, R8::A),
            0x22 => self.load(Addr::HLi, R8::A),
            0x32 => self.load(Addr::HLd, R8::A),
            0x0A => self.load(R8::A, Addr::BC),
            0x1A => self.load(R8::A, Addr::DE),
            0x2A => self.load(R8::A, Addr::HLi),
            0x3A => self.load(R8::A, Addr::HLd),
            0xEA => self.load(Addr::N16, R8::A),
            0xFA => self.load(R8::A, Addr::N16),
            // LDH
            0xE0 => self.load(HighAddr::N8, R8::A),
            0xF0 => self.load(R8::A, HighAddr::N8),
            0xE2 => self.load(HighAddr::C, R8::A),
            0xF2 => self.load(R8::A, HighAddr::C),
            // ---- 16-bit Load
            // LD
            0x01 => self.load16(R16::BC, N16),
            0x11 => self.load16(R16::DE, N16),
            0x21 => self.load16(R16::HL, N16),
            0x31 => self.load16(R16::SP, N16),
            0xF9 => self.load16(R16::SP, R16::HL),
            0x08 => self.load16_a16_sp(),
            0xF8 => self.load16_hl_sp(),
            // PUSH
            0xC5 => self.push(R16::BC),
            0xD5 => self.push(R16::DE),
            0xE5 => self.push(R16::HL),
            0xF5 => self.push(R16::AF),
            // POP
            0xC1 => self.pop(R16::BC),
            0xD1 => self.pop(R16::DE),
            0xE1 => self.pop(R16::HL),
            0xF1 => self.pop(R16::AF),
            // ---- Jumps
            // JP
            0xE9 => self.jump_to_hl(),
            0xC3 => self.jump(JumpCondition::Always),
            0xC2 => self.jump(JumpCondition::NotZero),
            0xCA => self.jump(JumpCondition::Zero),
            0xD2 => self.jump(JumpCondition::NotCarry),
            0xDA => self.jump(JumpCondition::Carry),
            // JR
            0x18 => self.jump_relative(JumpCondition::Always),
            0x20 => self.jump_relative(JumpCondition::NotZero),
            0x28 => self.jump_relative(JumpCondition::Zero),
            0x30 => self.jump_relative(JumpCondition::NotCarry),
            0x38 => self.jump_relative(JumpCondition::Carry),
            // CALL
            0xCD => self.call(JumpCondition::Always),
            0xC4 => self.call(JumpCondition::NotZero),
            0xCC => self.call(JumpCondition::Zero),
            0xD4 => self.call(JumpCondition::NotCarry),
            0xDC => self.call(JumpCondition::Carry),
            // RET
            0xC9 => self.return_(JumpCondition::Always),
            0xC0 => self.return_(JumpCondition::NotZero),
            0xC8 => self.return_(JumpCondition::Zero),
            0xD0 => self.return_(JumpCondition::NotCarry),
            0xD8 => self.return_(JumpCondition::Carry),
            // RETI
            0xD9 => self.return_from_interrupt_handler(),
            // RST
            0xC7 => self.restart(0x00),
            0xCF => self.restart(0x08),
            0xD7 => self.restart(0x10),
            0xDF => self.restart(0x18),
            0xE7 => self.restart(0x20),
            0xEF => self.restart(0x28),
            0xF7 => self.restart(0x30),
            0xFF => self.restart(0x38),
            // ---- Control
            //NOP
            0x00 => self.no_operation(),
            // STOP
            0x10 => self.stop(),
            // HALT
            0x76 => self.halt(),
            // PREFIX
            0xCB => {
                let next_byte = self.read_next_byte();
                self.execute_prefixed(next_byte);
            }
            // DI
            0xF3 => self.disable_interrupt(),
            // EI
            0xFB => self.enable_interrupt(),
            // ---- Undefined
            n @ (0xD3 | 0xDB | 0xDD | 0xE3 | 0xE4 | 0xEB | 0xEC | 0xED | 0xF4 | 0xFC | 0xFD) => {
                Self::undefined(n);
            }
        }
    }

    fn execute_prefixed(&mut self, byte: u8) {
        match byte {
            // ---- Bit Shift
            // RLC
            0x00 => self.rotate_left_circular(R8::B),
            0x01 => self.rotate_left_circular(R8::C),
            0x02 => self.rotate_left_circular(R8::D),
            0x03 => self.rotate_left_circular(R8::E),
            0x04 => self.rotate_left_circular(R8::H),
            0x05 => self.rotate_left_circular(R8::L),
            0x06 => self.rotate_left_circular(Addr::HL),
            0x07 => self.rotate_left_circular(R8::A),
            // RRC
            0x08 => self.rotate_right_circular(R8::B),
            0x09 => self.rotate_right_circular(R8::C),
            0x0A => self.rotate_right_circular(R8::D),
            0x0B => self.rotate_right_circular(R8::E),
            0x0C => self.rotate_right_circular(R8::H),
            0x0D => self.rotate_right_circular(R8::L),
            0x0E => self.rotate_right_circular(Addr::HL),
            0x0F => self.rotate_right_circular(R8::A),
            // RL
            0x10 => self.rotate_left(R8::B),
            0x11 => self.rotate_left(R8::C),
            0x12 => self.rotate_left(R8::D),
            0x13 => self.rotate_left(R8::E),
            0x14 => self.rotate_left(R8::H),
            0x15 => self.rotate_left(R8::L),
            0x16 => self.rotate_left(Addr::HL),
            0x17 => self.rotate_left(R8::A),
            // RR
            0x18 => self.rotate_right(R8::B),
            0x19 => self.rotate_right(R8::C),
            0x1A => self.rotate_right(R8::D),
            0x1B => self.rotate_right(R8::E),
            0x1C => self.rotate_right(R8::H),
            0x1D => self.rotate_right(R8::L),
            0x1E => self.rotate_right(Addr::HL),
            0x1F => self.rotate_right(R8::A),
            // SLA
            0x20 => self.shift_left_arithmetic(R8::B),
            0x21 => self.shift_left_arithmetic(R8::C),
            0x22 => self.shift_left_arithmetic(R8::D),
            0x23 => self.shift_left_arithmetic(R8::E),
            0x24 => self.shift_left_arithmetic(R8::H),
            0x25 => self.shift_left_arithmetic(R8::L),
            0x26 => self.shift_left_arithmetic(Addr::HL),
            0x27 => self.shift_left_arithmetic(R8::A),
            // SRA
            0x28 => self.shift_right_arithmetic(R8::B),
            0x29 => self.shift_right_arithmetic(R8::C),
            0x2A => self.shift_right_arithmetic(R8::D),
            0x2B => self.shift_right_arithmetic(R8::E),
            0x2C => self.shift_right_arithmetic(R8::H),
            0x2D => self.shift_right_arithmetic(R8::L),
            0x2E => self.shift_right_arithmetic(Addr::HL),
            0x2F => self.shift_right_arithmetic(R8::A),
            // SWAP
            0x30 => self.swap(R8::B),
            0x31 => self.swap(R8::C),
            0x32 => self.swap(R8::D),
            0x33 => self.swap(R8::E),
            0x34 => self.swap(R8::H),
            0x35 => self.swap(R8::L),
            0x36 => self.swap(Addr::HL),
            0x37 => self.swap(R8::A),
            // SRL
            0x38 => self.shift_right_logical(R8::B),
            0x39 => self.shift_right_logical(R8::C),
            0x3A => self.shift_right_logical(R8::D),
            0x3B => self.shift_right_logical(R8::E),
            0x3C => self.shift_right_logical(R8::H),
            0x3D => self.shift_right_logical(R8::L),
            0x3E => self.shift_right_logical(Addr::HL),
            0x3F => self.shift_right_logical(R8::A),
            // ---- Bit Operations
            // BIT
            0x40 => self.bit_test(0, R8::B),
            0x41 => self.bit_test(0, R8::C),
            0x42 => self.bit_test(0, R8::D),
            0x43 => self.bit_test(0, R8::E),
            0x44 => self.bit_test(0, R8::H),
            0x45 => self.bit_test(0, R8::L),
            0x46 => self.bit_test(0, Addr::HL),
            0x47 => self.bit_test(0, R8::A),
            0x48 => self.bit_test(1, R8::B),
            0x49 => self.bit_test(1, R8::C),
            0x4A => self.bit_test(1, R8::D),
            0x4B => self.bit_test(1, R8::E),
            0x4C => self.bit_test(1, R8::H),
            0x4D => self.bit_test(1, R8::L),
            0x4E => self.bit_test(1, Addr::HL),
            0x4F => self.bit_test(1, R8::A),
            0x50 => self.bit_test(2, R8::B),
            0x51 => self.bit_test(2, R8::C),
            0x52 => self.bit_test(2, R8::D),
            0x53 => self.bit_test(2, R8::E),
            0x54 => self.bit_test(2, R8::H),
            0x55 => self.bit_test(2, R8::L),
            0x56 => self.bit_test(2, Addr::HL),
            0x57 => self.bit_test(2, R8::A),
            0x58 => self.bit_test(3, R8::B),
            0x59 => self.bit_test(3, R8::C),
            0x5A => self.bit_test(3, R8::D),
            0x5B => self.bit_test(3, R8::E),
            0x5C => self.bit_test(3, R8::H),
            0x5D => self.bit_test(3, R8::L),
            0x5E => self.bit_test(3, Addr::HL),
            0x5F => self.bit_test(3, R8::A),
            0x60 => self.bit_test(4, R8::B),
            0x61 => self.bit_test(4, R8::C),
            0x62 => self.bit_test(4, R8::D),
            0x63 => self.bit_test(4, R8::E),
            0x64 => self.bit_test(4, R8::H),
            0x65 => self.bit_test(4, R8::L),
            0x66 => self.bit_test(4, Addr::HL),
            0x67 => self.bit_test(4, R8::A),
            0x68 => self.bit_test(5, R8::B),
            0x69 => self.bit_test(5, R8::C),
            0x6A => self.bit_test(5, R8::D),
            0x6B => self.bit_test(5, R8::E),
            0x6C => self.bit_test(5, R8::H),
            0x6D => self.bit_test(5, R8::L),
            0x6E => self.bit_test(5, Addr::HL),
            0x6F => self.bit_test(5, R8::A),
            0x70 => self.bit_test(6, R8::B),
            0x71 => self.bit_test(6, R8::C),
            0x72 => self.bit_test(6, R8::D),
            0x73 => self.bit_test(6, R8::E),
            0x74 => self.bit_test(6, R8::H),
            0x75 => self.bit_test(6, R8::L),
            0x76 => self.bit_test(6, Addr::HL),
            0x77 => self.bit_test(6, R8::A),
            0x78 => self.bit_test(7, R8::B),
            0x79 => self.bit_test(7, R8::C),
            0x7A => self.bit_test(7, R8::D),
            0x7B => self.bit_test(7, R8::E),
            0x7C => self.bit_test(7, R8::H),
            0x7D => self.bit_test(7, R8::L),
            0x7E => self.bit_test(7, Addr::HL),
            0x7F => self.bit_test(7, R8::A),
            // RES
            0x80 => self.bit_reset(0, R8::B),
            0x81 => self.bit_reset(0, R8::C),
            0x82 => self.bit_reset(0, R8::D),
            0x83 => self.bit_reset(0, R8::E),
            0x84 => self.bit_reset(0, R8::H),
            0x85 => self.bit_reset(0, R8::L),
            0x86 => self.bit_reset(0, Addr::HL),
            0x87 => self.bit_reset(0, R8::A),
            0x88 => self.bit_reset(1, R8::B),
            0x89 => self.bit_reset(1, R8::C),
            0x8A => self.bit_reset(1, R8::D),
            0x8B => self.bit_reset(1, R8::E),
            0x8C => self.bit_reset(1, R8::H),
            0x8D => self.bit_reset(1, R8::L),
            0x8E => self.bit_reset(1, Addr::HL),
            0x8F => self.bit_reset(1, R8::A),
            0x90 => self.bit_reset(2, R8::B),
            0x91 => self.bit_reset(2, R8::C),
            0x92 => self.bit_reset(2, R8::D),
            0x93 => self.bit_reset(2, R8::E),
            0x94 => self.bit_reset(2, R8::H),
            0x95 => self.bit_reset(2, R8::L),
            0x96 => self.bit_reset(2, Addr::HL),
            0x97 => self.bit_reset(2, R8::A),
            0x98 => self.bit_reset(3, R8::B),
            0x99 => self.bit_reset(3, R8::C),
            0x9A => self.bit_reset(3, R8::D),
            0x9B => self.bit_reset(3, R8::E),
            0x9C => self.bit_reset(3, R8::H),
            0x9D => self.bit_reset(3, R8::L),
            0x9E => self.bit_reset(3, Addr::HL),
            0x9F => self.bit_reset(3, R8::A),
            0xA0 => self.bit_reset(4, R8::B),
            0xA1 => self.bit_reset(4, R8::C),
            0xA2 => self.bit_reset(4, R8::D),
            0xA3 => self.bit_reset(4, R8::E),
            0xA4 => self.bit_reset(4, R8::H),
            0xA5 => self.bit_reset(4, R8::L),
            0xA6 => self.bit_reset(4, Addr::HL),
            0xA7 => self.bit_reset(4, R8::A),
            0xA8 => self.bit_reset(5, R8::B),
            0xA9 => self.bit_reset(5, R8::C),
            0xAA => self.bit_reset(5, R8::D),
            0xAB => self.bit_reset(5, R8::E),
            0xAC => self.bit_reset(5, R8::H),
            0xAD => self.bit_reset(5, R8::L),
            0xAE => self.bit_reset(5, Addr::HL),
            0xAF => self.bit_reset(5, R8::A),
            0xB0 => self.bit_reset(6, R8::B),
            0xB1 => self.bit_reset(6, R8::C),
            0xB2 => self.bit_reset(6, R8::D),
            0xB3 => self.bit_reset(6, R8::E),
            0xB4 => self.bit_reset(6, R8::H),
            0xB5 => self.bit_reset(6, R8::L),
            0xB6 => self.bit_reset(6, Addr::HL),
            0xB7 => self.bit_reset(6, R8::A),
            0xB8 => self.bit_reset(7, R8::B),
            0xB9 => self.bit_reset(7, R8::C),
            0xBA => self.bit_reset(7, R8::D),
            0xBB => self.bit_reset(7, R8::E),
            0xBC => self.bit_reset(7, R8::H),
            0xBD => self.bit_reset(7, R8::L),
            0xBE => self.bit_reset(7, Addr::HL),
            0xBF => self.bit_reset(7, R8::A),
            // SET
            0xC0 => self.bit_set(0, R8::B),
            0xC1 => self.bit_set(0, R8::C),
            0xC2 => self.bit_set(0, R8::D),
            0xC3 => self.bit_set(0, R8::E),
            0xC4 => self.bit_set(0, R8::H),
            0xC5 => self.bit_set(0, R8::L),
            0xC6 => self.bit_set(0, Addr::HL),
            0xC7 => self.bit_set(0, R8::A),
            0xC8 => self.bit_set(1, R8::B),
            0xC9 => self.bit_set(1, R8::C),
            0xCA => self.bit_set(1, R8::D),
            0xCB => self.bit_set(1, R8::E),
            0xCC => self.bit_set(1, R8::H),
            0xCD => self.bit_set(1, R8::L),
            0xCE => self.bit_set(1, Addr::HL),
            0xCF => self.bit_set(1, R8::A),
            0xD0 => self.bit_set(2, R8::B),
            0xD1 => self.bit_set(2, R8::C),
            0xD2 => self.bit_set(2, R8::D),
            0xD3 => self.bit_set(2, R8::E),
            0xD4 => self.bit_set(2, R8::H),
            0xD5 => self.bit_set(2, R8::L),
            0xD6 => self.bit_set(2, Addr::HL),
            0xD7 => self.bit_set(2, R8::A),
            0xD8 => self.bit_set(3, R8::B),
            0xD9 => self.bit_set(3, R8::C),
            0xDA => self.bit_set(3, R8::D),
            0xDB => self.bit_set(3, R8::E),
            0xDC => self.bit_set(3, R8::H),
            0xDD => self.bit_set(3, R8::L),
            0xDE => self.bit_set(3, Addr::HL),
            0xDF => self.bit_set(3, R8::A),
            0xE0 => self.bit_set(4, R8::B),
            0xE1 => self.bit_set(4, R8::C),
            0xE2 => self.bit_set(4, R8::D),
            0xE3 => self.bit_set(4, R8::E),
            0xE4 => self.bit_set(4, R8::H),
            0xE5 => self.bit_set(4, R8::L),
            0xE6 => self.bit_set(4, Addr::HL),
            0xE7 => self.bit_set(4, R8::A),
            0xE8 => self.bit_set(5, R8::B),
            0xE9 => self.bit_set(5, R8::C),
            0xEA => self.bit_set(5, R8::D),
            0xEB => self.bit_set(5, R8::E),
            0xEC => self.bit_set(5, R8::H),
            0xED => self.bit_set(5, R8::L),
            0xEE => self.bit_set(5, Addr::HL),
            0xEF => self.bit_set(5, R8::A),
            0xF0 => self.bit_set(6, R8::B),
            0xF1 => self.bit_set(6, R8::C),
            0xF2 => self.bit_set(6, R8::D),
            0xF3 => self.bit_set(6, R8::E),
            0xF4 => self.bit_set(6, R8::H),
            0xF5 => self.bit_set(6, R8::L),
            0xF6 => self.bit_set(6, Addr::HL),
            0xF7 => self.bit_set(6, R8::A),
            0xF8 => self.bit_set(7, R8::B),
            0xF9 => self.bit_set(7, R8::C),
            0xFA => self.bit_set(7, R8::D),
            0xFB => self.bit_set(7, R8::E),
            0xFC => self.bit_set(7, R8::H),
            0xFD => self.bit_set(7, R8::L),
            0xFE => self.bit_set(7, Addr::HL),
            0xFF => self.bit_set(7, R8::A),
        }
    }

    fn undefined(byte: u8) {
        panic!("Undefined instruction found: {byte:#02X}");
    }

    /// NOP
    /// 1 4
    /// - - - -
    ///
    /// Nothing happens.
    fn no_operation(&self) {}

    /// STOP n8
    /// 2 4
    /// - - - -
    ///
    /// Stop CPU & LCD display until button pressed.
    fn stop(&self) {
        // TODO: implement stop method
    }

    /// HALT
    /// 1 4
    /// - - - -
    ///
    /// Halt CPU until an interrupt occurs.
    fn halt(&self) {
        // TODO: implement halt method
    }

    /// LD r8, r8
    /// 1 4
    /// - - - -
    ///
    /// Load src (right) and copy into target (left).
    fn load<T, S>(&mut self, target: T, src: S)
    where
        Self: ReadByte<S> + WriteByte<T>,
    {
        let value = self.read_byte(src);
        self.write_byte(target, value);
    }

    /// LD r16, n16
    /// 3 12
    /// - - - -
    ///
    /// Load src (right) and copy into target (left).
    fn load16<T, S>(&mut self, target: T, src: S)
    where
        Self: ReadWord<S> + WriteWord<T>,
    {
        let value = self.read_word(src);
        self.write_word(target, value);
    }

    /// LD \[a16\], SP
    /// 3 20
    /// - - - -
    ///
    /// Load SP at address a16.
    fn load16_a16_sp(&mut self) {
        let value = self.registers.sp;
        let addr = self.read_next_word();
        self.bus.write_byte(addr, value as u8);
        self.bus
            .write_byte(addr.wrapping_add(1), (value >> 8) as u8);
    }

    /// LD HL, SP + e8
    /// 2 12
    /// 0 0 H C
    ///
    /// Add the signed value e8 to SP and store the result in HL.
    fn load16_hl_sp(&mut self) {
        let sp = self.registers.sp;
        let offset = self.read_next_byte() as i8;
        self.registers.f.set(Flags::ZERO, false);
        self.registers.f.set(Flags::SUBTRACT, false);
        // Half-carry from bit 3, carry from bit 7
        // Bits are labeled from 0-15 from least to most significant.
        let half_carry = (sp & 0xF).wrapping_add_signed(offset as i16 & 0xF) > 0xF;
        self.registers.f.set(Flags::HALF_CARRY, half_carry);
        let carry = (sp & 0xFF).wrapping_add_signed(offset as i16 & 0xFF) > 0xFF;
        self.registers.f.set(Flags::CARRY, carry);
        let new_value = sp.wrapping_add_signed(offset as i16);
        self.registers.write16(R16::HL, new_value);
    }

    /// ADD A, r8
    /// 1 4
    /// Z 0 H C
    ///
    /// Add the value in r8 to register A.
    fn add<S>(&mut self, src: S)
    where
        Self: ReadByte<S>,
    {
        let value = self.read_byte(src);
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
    fn add_with_carry<S>(&mut self, src: S)
    where
        Self: ReadByte<S>,
    {
        let value = self.read_byte(src);
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
    fn subtract<S>(&mut self, src: S)
    where
        Self: ReadByte<S>,
    {
        let value = self.read_byte(src);
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
    fn subtract_with_carry<S>(&mut self, src: S)
    where
        Self: ReadByte<S>,
    {
        let value = self.read_byte(src);
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
    fn and<S>(&mut self, src: S)
    where
        Self: ReadByte<S>,
    {
        let value = self.read_byte(src);
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
    fn xor<S>(&mut self, src: S)
    where
        Self: ReadByte<S>,
    {
        let value = self.read_byte(src);
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
    fn or<S>(&mut self, src: S)
    where
        Self: ReadByte<S>,
    {
        let value = self.read_byte(src);
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
    fn compare<S>(&mut self, src: S)
    where
        Self: ReadByte<S>,
    {
        let value = self.read_byte(src);
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
    fn increment<S>(&mut self, src: S)
    where
        S: Copy,
        Self: ReadByte<S> + WriteByte<S>,
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
    fn decrement<S>(&mut self, src: S)
    where
        S: Copy,
        Self: ReadByte<S> + WriteByte<S>,
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
    fn add16_hl(&mut self, src: R16) {
        let value = self.registers.read16(src);
        let hl = self.registers.read16(R16::HL);
        let (new_value, did_overflow) = hl.overflowing_add(value);
        // ZERO is left untouched
        self.registers.f.set(Flags::SUBTRACT, false);
        self.registers.f.set(Flags::CARRY, did_overflow);
        // Half-carry from bit 11, carry from bit 15
        // Bits are labeled from 0-15 from least to most significant.
        let half_carry = (hl & 0xFFF) + (value & 0xFFF) > 0xFFF;
        self.registers.f.set(Flags::HALF_CARRY, half_carry);
        self.registers.write16(R16::HL, new_value);
    }

    /// ADD SP, e8
    /// 2 16
    /// 0 0 H C
    ///
    /// Add the signed value e8 to SP.
    fn add16_sp(&mut self) {
        let offset = self.read_next_byte() as i8;
        let sp = self.registers.sp;
        self.registers.f.set(Flags::ZERO, false);
        self.registers.f.set(Flags::SUBTRACT, false);
        // Half-carry from bit 3, carry from bit 7
        // Bits are labeled from 0-15 from least to most significant.
        let half_carry = (sp & 0xF).wrapping_add_signed(offset as i16 & 0xF) > 0xF;
        self.registers.f.set(Flags::CARRY, half_carry);
        let carry = (sp & 0xFF).wrapping_add_signed(offset as i16 & 0xFF) > 0xFF;
        self.registers.f.set(Flags::CARRY, carry);
        let new_value = sp.wrapping_add_signed(offset as i16);
        self.registers.sp = new_value;
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
    fn complement_accumulator(&mut self) {
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

    /// DAA
    /// 1 4
    /// Z - 0 C
    ///
    /// Decimal Adjust register A to get a correct BCD representation after an arithmetic instruction.
    fn decimal_adjust_accumulator(&mut self) {
        let mut value = self.registers.a;

        let nf = self.registers.f.contains(Flags::SUBTRACT);
        let hf = self.registers.f.contains(Flags::HALF_CARRY);
        let mut cf = self.registers.f.contains(Flags::CARRY);

        if !nf {
            // After an addition, adjust if (half-)carry occurred or if out of bounds
            if cf || value > 0x99 {
                value = value.wrapping_add(0x60);
                cf = true;
            }
            if hf || (value & 0x0F) > 0x09 {
                value = value.wrapping_sub(0x06);
            }
        } else {
            // After a subtraction, only adjust if (half-)carry occurred
            if cf {
                value = value.wrapping_sub(0x60);
            }
            if hf {
                value = value.wrapping_sub(0x06);
            }
        }

        self.registers.f.set(Flags::ZERO, value == 0);
        // SUBTRACT left untouched
        self.registers.f.set(Flags::HALF_CARRY, false);
        self.registers.f.set(Flags::CARRY, cf);
    }

    /// RLC r8
    /// 2 8
    /// Z 0 0 C
    ///
    /// Rotate register r8 left.
    fn rotate_left_circular<S>(&mut self, src: S)
    where
        S: Copy,
        Self: ReadByte<S> + WriteByte<S>,
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
    fn rotate_right_circular<S>(&mut self, src: S)
    where
        S: Copy,
        Self: ReadByte<S> + WriteByte<S>,
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
    fn rotate_left<S>(&mut self, src: S)
    where
        S: Copy,
        Self: ReadByte<S> + WriteByte<S>,
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
    fn rotate_right<S>(&mut self, src: S)
    where
        S: Copy,
        Self: ReadByte<S> + WriteByte<S>,
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
    fn shift_left_arithmetic<S>(&mut self, src: S)
    where
        S: Copy,
        Self: ReadByte<S> + WriteByte<S>,
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
    fn shift_right_arithmetic<S>(&mut self, src: S)
    where
        S: Copy,
        Self: ReadByte<S> + WriteByte<S>,
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
    fn swap<S>(&mut self, src: S)
    where
        S: Copy,
        Self: ReadByte<S> + WriteByte<S>,
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
    fn shift_right_logical<S>(&mut self, src: S)
    where
        S: Copy,
        Self: ReadByte<S> + WriteByte<S>,
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
    fn bit_test<S>(&mut self, bit: u8, src: S)
    where
        Self: ReadByte<S>,
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
    fn bit_reset<S>(&mut self, bit: u8, src: S)
    where
        S: Copy,
        Self: ReadByte<S> + WriteByte<S>,
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
    fn bit_set<S>(&mut self, bit: u8, src: S)
    where
        S: Copy,
        Self: ReadByte<S> + WriteByte<S>,
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
        let offset = self.read_next_byte() as i8;
        if should_jump {
            self.registers.pc = self.registers.pc.wrapping_add_signed(offset as i16);
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
    fn return_(&mut self, condition: JumpCondition) {
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
        self.return_(JumpCondition::Always);
        self.ime = true;
    }

    /// RST u8
    /// 1 16
    /// - - - -
    ///
    /// Push current address onto stack, and jump to address u8.
    fn restart(&mut self, addr: u8) {
        self.push(R16::PC);
        self.registers.sp = addr as u16;
    }

    /// DI
    /// 1 4
    /// - - - -
    ///
    /// Disable Interrupts by clearing the IME flag.
    fn disable_interrupt(&mut self) {
        self.ime = false;
    }

    /// EI
    /// 1 4
    /// - - - -
    ///
    /// Enable Interrupts by setting the IME flag.
    /// The flag is only set after the instruction following EI.
    fn enable_interrupt(&mut self) {
        self.step();
        self.ime = true;
    }
}
