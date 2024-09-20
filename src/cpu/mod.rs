mod instructions;
#[allow(clippy::too_many_lines)]
mod optables;

use crate::memory::AddressBus;
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
    f: RegisterFlags,
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
            f: RegisterFlags::empty(),
            h: 0,
            l: 0,
            sp: 0,
            pc: 0,
        }
    }

    const fn read_byte(&self, register: R8) -> u8 {
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

    fn write_byte(&mut self, register: R8, value: u8) {
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

    const fn read_word(&self, register: R16) -> u16 {
        match register {
            R16::AF => u16::from_le_bytes([self.f.bits(), self.a]),
            R16::BC => u16::from_le_bytes([self.c, self.b]),
            R16::DE => u16::from_le_bytes([self.e, self.d]),
            R16::HL => u16::from_le_bytes([self.l, self.h]),
            R16::SP => self.sp,
            R16::PC => self.pc,
        }
    }

    fn write_word(&mut self, register: R16, value: u16) {
        match register {
            R16::AF => {
                let [low, high] = value.to_le_bytes();
                self.a = high;
                self.f = RegisterFlags::from_bits_truncate(low);
            }
            R16::BC => {
                let [low, high] = value.to_le_bytes();
                self.b = high;
                self.c = low;
            }
            R16::DE => {
                let [low, high] = value.to_le_bytes();
                self.d = high;
                self.e = low;
            }
            R16::HL => {
                let [low, high] = value.to_le_bytes();
                self.h = high;
                self.l = low;
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
    struct RegisterFlags: u8 {
        const ZERO = 0b1000_0000;
        const SUBTRACT = 0b0100_0000;
        const HALF_CARRY = 0b0010_0000;
        const CARRY = 0b0001_0000;
    }
}

impl RegisterFlags {
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
pub(crate) enum R8 {
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
pub(crate) enum R16 {
    AF,
    BC,
    DE,
    HL,
    SP,
    PC,
}

#[derive(Debug, Clone, Copy)]
pub(crate) enum Addr {
    BC,
    DE,
    HL,
    HLi,
    HLd,
    N16,
}

#[derive(Debug, Clone, Copy)]
pub(crate) enum HighAddr {
    C,
    N8,
}

/// Unit struct to represent next byte (n8)
#[derive(Debug, Clone, Copy)]
pub(crate) struct N8;

/// Unit struct to represent next word (n16)
#[derive(Debug, Clone, Copy)]
pub(crate) struct N16;

pub(crate) trait ReadByte<S> {
    fn read_byte(&mut self, memory: &AddressBus, src: S) -> u8;
}

impl ReadByte<R8> for Cpu {
    fn read_byte(&mut self, _: &AddressBus, src: R8) -> u8 {
        self.registers.read_byte(src)
    }
}

impl ReadByte<Addr> for Cpu {
    fn read_byte(&mut self, memory: &AddressBus, src: Addr) -> u8 {
        match src {
            Addr::BC => {
                let address = self.registers.read_word(R16::BC);
                memory.read_byte(address)
            }
            Addr::DE => {
                let address = self.registers.read_word(R16::DE);
                memory.read_byte(address)
            }
            Addr::HL => {
                let address = self.registers.read_word(R16::HL);
                memory.read_byte(address)
            }
            Addr::HLi => {
                let address = self.registers.read_word(R16::HL);
                let new_address = address.wrapping_add(1);
                self.registers.write_word(R16::HL, new_address);
                memory.read_byte(address)
            }
            Addr::HLd => {
                let address = self.registers.read_word(R16::HL);
                let new_address = address.wrapping_sub(1);
                self.registers.write_word(R16::HL, new_address);
                memory.read_byte(address)
            }
            Addr::N16 => {
                let address = self.read_next_word(memory);
                memory.read_byte(address)
            }
        }
    }
}

impl ReadByte<HighAddr> for Cpu {
    fn read_byte(&mut self, memory: &AddressBus, src: HighAddr) -> u8 {
        match src {
            HighAddr::C => {
                let address = self.registers.read_byte(R8::C) as u16;
                memory.read_byte(0xFF00 + address)
            }
            HighAddr::N8 => {
                let address = self.read_next_byte(memory) as u16;
                memory.read_byte(0xFF00 + address)
            }
        }
    }
}

impl ReadByte<N8> for Cpu {
    fn read_byte(&mut self, memory: &AddressBus, _: N8) -> u8 {
        self.read_next_byte(memory)
    }
}

pub(crate) trait WriteByte<D> {
    fn write_byte(&mut self, memory: &mut AddressBus, dst: D, value: u8);
}

impl WriteByte<R8> for Cpu {
    fn write_byte(&mut self, _: &mut AddressBus, dst: R8, value: u8) {
        self.registers.write_byte(dst, value);
    }
}

impl WriteByte<Addr> for Cpu {
    fn write_byte(&mut self, memory: &mut AddressBus, dst: Addr, value: u8) {
        match dst {
            Addr::BC => {
                let address = self.registers.read_word(R16::BC);
                memory.write_byte(address, value);
            }
            Addr::DE => {
                let address = self.registers.read_word(R16::DE);
                memory.write_byte(address, value);
            }
            Addr::HL => {
                let address = self.registers.read_word(R16::HL);
                memory.write_byte(address, value);
            }
            Addr::HLi => {
                let address = self.registers.read_word(R16::HL);
                let new_address = address.wrapping_add(1);
                self.registers.write_word(R16::HL, new_address);
                memory.write_byte(address, value);
            }
            Addr::HLd => {
                let address = self.registers.read_word(R16::HL);
                let new_address = address.wrapping_sub(1);
                self.registers.write_word(R16::HL, new_address);
                memory.write_byte(address, value);
            }
            Addr::N16 => {
                let address = self.read_next_word(memory);
                memory.write_byte(address, value);
            }
        }
    }
}

impl WriteByte<HighAddr> for Cpu {
    fn write_byte(&mut self, memory: &mut AddressBus, dst: HighAddr, value: u8) {
        match dst {
            HighAddr::C => {
                let address = self.registers.read_byte(R8::C) as u16;
                memory.write_byte(0xFF00 + address, value);
            }
            HighAddr::N8 => {
                let address = self.read_next_byte(memory) as u16;
                memory.write_byte(0xFF00 + address, value);
            }
        }
    }
}

pub(crate) trait ReadWord<S> {
    fn read_word(&mut self, memory: &AddressBus, src: S) -> u16;
}

impl ReadWord<R16> for Cpu {
    fn read_word(&mut self, _: &AddressBus, src: R16) -> u16 {
        self.registers.read_word(src)
    }
}

impl ReadWord<N16> for Cpu {
    fn read_word(&mut self, memory: &AddressBus, _: N16) -> u16 {
        self.read_next_word(memory)
    }
}

pub(crate) trait WriteWord<D> {
    fn write_word(&mut self, dst: D, value: u16);
}

impl WriteWord<R16> for Cpu {
    fn write_word(&mut self, dst: R16, value: u16) {
        self.registers.write_word(dst, value);
    }
}

#[derive(Debug, Clone, Copy)]
pub(crate) enum JumpCondition {
    NotZero,
    Zero,
    NotCarry,
    Carry,
    Always,
}

#[derive(Clone)]
pub struct Cpu {
    registers: Registers,
    /// IME: Interrupt Master Enable
    ime: bool,
    // Used to delay setting IME after calling EI
    enable_irq: Option<i8>,
}

impl Cpu {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            registers: Registers::new(),
            ime: false,
            enable_irq: None,
        }
    }

    fn step(&mut self, memory: &mut AddressBus) {
        let instruction_byte = self.read_next_byte(memory);
        self.execute(memory, instruction_byte);

        // Checks for next command after EI is called
        self.enable_irq = self.enable_irq.map(|n| n - 1);
        if self.enable_irq.is_some_and(|n| n == 0) {
            self.ime = true;
            self.enable_irq = None;
        }
    }

    fn read_next_byte(&mut self, memory: &AddressBus) -> u8 {
        let byte = memory.read_byte(self.registers.pc);
        self.registers.pc = self.registers.pc.wrapping_add(1);
        byte
    }

    #[allow(clippy::cast_possible_wrap)]
    fn read_next_byte_signed(&mut self, memory: &AddressBus) -> i8 {
        self.read_next_byte(memory) as i8
    }

    fn read_next_word(&mut self, memory: &AddressBus) -> u16 {
        // Game Boy is little endian, so read the second byte as the most significant byte
        // and the first as the least significant
        let low = memory.read_byte(self.registers.pc);
        self.registers.pc = self.registers.pc.wrapping_add(1);
        let high = memory.read_byte(self.registers.pc);
        self.registers.pc = self.registers.pc.wrapping_add(1);
        u16::from_le_bytes([low, high])
    }
}

impl Default for Cpu {
    fn default() -> Self {
        Self::new()
    }
}
