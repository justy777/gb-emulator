#[allow(clippy::too_many_lines)]
mod execute;
mod instructions;

use crate::hardware::AddressBus;
use crate::interrupts::InterruptFlags;

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
            a: 0x01,
            b: 0x00,
            c: 0x13,
            d: 0x00,
            e: 0xD8,
            f: RegisterFlags::new(),
            h: 0x01,
            l: 0x4D,
            sp: 0xFFFE,
            pc: 0x100,
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
                self.f = RegisterFlags::from_bits(low);
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

#[derive(Debug, Clone, Copy)]
struct RegisterFlags(u8);

impl RegisterFlags {
    const ZERO: u8 = 0b1000_0000;
    const SUBTRACT: u8 = 0b0100_0000;
    const HALF_CARRY: u8 = 0b0010_0000;
    const CARRY: u8 = 0b0001_0000;
    const UNUSED: u8 = 0b0000_1111;

    const fn new() -> Self {
        Self::from_bits(Self::ZERO | Self::HALF_CARRY | Self::CARRY)
    }

    const fn from_bits(bits: u8) -> Self {
        Self(bits & !Self::UNUSED)
    }

    const fn bits(self) -> u8 {
        self.0
    }

    fn set(&mut self, bits: u8, enable: bool) {
        if enable {
            self.0 |= bits;
        } else {
            self.0 &= !bits;
        }
        self.0 &= !Self::UNUSED;
    }

    const fn contains(self, bits: u8) -> bool {
        (self.0 & bits) == bits
    }

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
pub enum R8 {
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
pub enum R16 {
    AF,
    BC,
    DE,
    HL,
    SP,
    PC,
}

#[derive(Debug, Clone, Copy)]
pub enum Addr {
    BC,
    DE,
    HL,
    HLi,
    HLd,
    N16,
}

#[derive(Debug, Clone, Copy)]
pub enum HighAddr {
    C,
    N8,
}

/// Unit struct to represent next byte (n8)
#[derive(Debug, Clone, Copy)]
pub struct N8;

/// Unit struct to represent next word (n16)
#[derive(Debug, Clone, Copy)]
pub struct N16;

pub trait ReadByte<S> {
    fn read_byte(&mut self, bus: &AddressBus, src: S) -> u8;
}

impl ReadByte<R8> for Cpu {
    fn read_byte(&mut self, _: &AddressBus, src: R8) -> u8 {
        self.registers.read_byte(src)
    }
}

impl ReadByte<Addr> for Cpu {
    fn read_byte(&mut self, bus: &AddressBus, src: Addr) -> u8 {
        match src {
            Addr::BC => {
                let addr = self.registers.read_word(R16::BC);
                bus.read_byte(addr)
            }
            Addr::DE => {
                let addr = self.registers.read_word(R16::DE);
                bus.read_byte(addr)
            }
            Addr::HL => {
                let addr = self.registers.read_word(R16::HL);
                bus.read_byte(addr)
            }
            Addr::HLi => {
                let addr = self.registers.read_word(R16::HL);
                let new_addr = addr.wrapping_add(1);
                self.registers.write_word(R16::HL, new_addr);
                bus.read_byte(addr)
            }
            Addr::HLd => {
                let addr = self.registers.read_word(R16::HL);
                let new_addr = addr.wrapping_sub(1);
                self.registers.write_word(R16::HL, new_addr);
                bus.read_byte(addr)
            }
            Addr::N16 => {
                let addr = self.read_next_word(bus);
                bus.read_byte(addr)
            }
        }
    }
}

impl ReadByte<HighAddr> for Cpu {
    fn read_byte(&mut self, bus: &AddressBus, src: HighAddr) -> u8 {
        match src {
            HighAddr::C => {
                let addr = self.registers.read_byte(R8::C) as u16;
                bus.read_byte(0xFF00 + addr)
            }
            HighAddr::N8 => {
                let addr = self.read_next_byte(bus) as u16;
                bus.read_byte(0xFF00 + addr)
            }
        }
    }
}

impl ReadByte<N8> for Cpu {
    fn read_byte(&mut self, bus: &AddressBus, _: N8) -> u8 {
        self.read_next_byte(bus)
    }
}

pub trait WriteByte<D> {
    fn write_byte(&mut self, bus: &mut AddressBus, dst: D, value: u8);
}

impl WriteByte<R8> for Cpu {
    fn write_byte(&mut self, _: &mut AddressBus, dst: R8, value: u8) {
        self.registers.write_byte(dst, value);
    }
}

impl WriteByte<Addr> for Cpu {
    fn write_byte(&mut self, bus: &mut AddressBus, dst: Addr, value: u8) {
        match dst {
            Addr::BC => {
                let addr = self.registers.read_word(R16::BC);
                bus.write_byte(addr, value);
            }
            Addr::DE => {
                let addr = self.registers.read_word(R16::DE);
                bus.write_byte(addr, value);
            }
            Addr::HL => {
                let addr = self.registers.read_word(R16::HL);
                bus.write_byte(addr, value);
            }
            Addr::HLi => {
                let addr = self.registers.read_word(R16::HL);
                let new_addr = addr.wrapping_add(1);
                self.registers.write_word(R16::HL, new_addr);
                bus.write_byte(addr, value);
            }
            Addr::HLd => {
                let addr = self.registers.read_word(R16::HL);
                let new_addr = addr.wrapping_sub(1);
                self.registers.write_word(R16::HL, new_addr);
                bus.write_byte(addr, value);
            }
            Addr::N16 => {
                let addr = self.read_next_word(bus);
                bus.write_byte(addr, value);
            }
        }
    }
}

impl WriteByte<HighAddr> for Cpu {
    fn write_byte(&mut self, bus: &mut AddressBus, dst: HighAddr, value: u8) {
        match dst {
            HighAddr::C => {
                let addr = self.registers.read_byte(R8::C) as u16;
                bus.write_byte(0xFF00 + addr, value);
            }
            HighAddr::N8 => {
                let addr = self.read_next_byte(bus) as u16;
                bus.write_byte(0xFF00 + addr, value);
            }
        }
    }
}

pub trait ReadWord<S> {
    fn read_word(&mut self, bus: &AddressBus, src: S) -> u16;
}

impl ReadWord<R16> for Cpu {
    fn read_word(&mut self, _: &AddressBus, src: R16) -> u16 {
        self.registers.read_word(src)
    }
}

impl ReadWord<N16> for Cpu {
    fn read_word(&mut self, bus: &AddressBus, _: N16) -> u16 {
        self.read_next_word(bus)
    }
}

pub trait WriteWord<D> {
    fn write_word(&mut self, dst: D, value: u16);
}

impl WriteWord<R16> for Cpu {
    fn write_word(&mut self, dst: R16, value: u16) {
        self.registers.write_word(dst, value);
    }
}

#[derive(Debug, Clone, Copy)]
pub enum JumpCondition {
    NotZero,
    Zero,
    NotCarry,
    Carry,
    Always,
}

#[derive(Clone)]
pub struct Cpu {
    registers: Registers,
    halted: bool,
    // IME: Interrupt Master Enable
    ime: bool,
    // Used to delay setting IME after calling EI
    ime_delay_counter: Option<u8>,
}

impl Cpu {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            registers: Registers::new(),
            halted: false,
            ime: false,
            ime_delay_counter: None,
        }
    }

    pub fn step(&mut self, bus: &mut AddressBus) -> usize {
        // Checks for next instruction after EI is called
        self.ime_delay_counter = self.ime_delay_counter.map(|n| n - 1);
        if self.ime_delay_counter.is_some_and(|n| n == 0) {
            self.ime = true;
            self.ime_delay_counter = None;
        }

        // Checks for pending interrupts
        let interrupt_pending = bus.get_interrupts_pending();

        for flag in InterruptFlags::flags() {
            if interrupt_pending.contains(flag.bits()) {
                self.halted = false;
                if self.ime {
                    // Calls interrupt handler
                    self.ime = false;
                    bus.interrupt_flag().set(flag.bits(), false);
                    self.push(bus, R16::PC);
                    self.registers.pc = flag.handler_addr();
                }
                break;
            }
        }

        if self.halted {
            return 4;
        }

        let opcode = self.read_next_byte(bus);
        self.execute(bus, opcode)
    }

    fn read_next_byte(&mut self, bus: &AddressBus) -> u8 {
        let byte = bus.read_byte(self.registers.pc);
        self.registers.pc = self.registers.pc.wrapping_add(1);
        byte
    }

    #[allow(clippy::cast_possible_wrap)]
    fn read_next_byte_signed(&mut self, bus: &AddressBus) -> i8 {
        self.read_next_byte(bus) as i8
    }

    fn read_next_word(&mut self, bus: &AddressBus) -> u16 {
        // Game Boy is little endian, so read the second byte as the most significant byte
        // and the first as the least significant
        let low = bus.read_byte(self.registers.pc);
        self.registers.pc = self.registers.pc.wrapping_add(1);
        let high = bus.read_byte(self.registers.pc);
        self.registers.pc = self.registers.pc.wrapping_add(1);
        u16::from_le_bytes([low, high])
    }
}

impl Default for Cpu {
    fn default() -> Self {
        Self::new()
    }
}
