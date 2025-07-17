#[allow(clippy::too_many_lines)]
mod execute;
mod instruction;

use crate::hardware::AddressBus;
use crate::interrupt::Interrupt;

enum Flag {
    Zero = 0b1000_0000,
    Subtract = 0b0100_0000,
    HalfCarry = 0b0010_0000,
    Carry = 0b0001_0000,
}

#[derive(Debug, Clone, Copy)]
pub enum JumpCondition {
    NotZero,
    Zero,
    NotCarry,
    Carry,
    Always,
}

#[derive(Debug, Clone, Copy)]
struct FlagsRegister(u8);

impl FlagsRegister {
    const UNUSED: u8 = 0b0000_1111;

    const fn new() -> Self {
        Self::from_bits(Flag::Zero as u8 | Flag::HalfCarry as u8 | Flag::Carry as u8)
    }

    const fn from_bits(bits: u8) -> Self {
        Self(bits & !Self::UNUSED)
    }

    const fn bits(self) -> u8 {
        self.0
    }

    const fn set(&mut self, flag: Flag, enable: bool) {
        let bits = flag as u8;
        if enable {
            self.0 |= bits;
        } else {
            self.0 &= !bits;
        }
    }

    const fn contains(self, flag: Flag) -> bool {
        let bits = flag as u8;
        (self.0 & bits) == bits
    }

    const fn test(self, condition: JumpCondition) -> bool {
        match condition {
            JumpCondition::NotZero => !self.contains(Flag::Zero),
            JumpCondition::Zero => self.contains(Flag::Zero),
            JumpCondition::NotCarry => !self.contains(Flag::Carry),
            JumpCondition::Carry => self.contains(Flag::Carry),
            JumpCondition::Always => true,
        }
    }
}

pub trait ReadByte<S> {
    fn read_byte(&mut self, bus: &mut AddressBus, src: S) -> u8;
}

pub trait WriteByte<D> {
    fn write_byte(&mut self, bus: &mut AddressBus, dest: D, value: u8);
}

pub trait ReadWord<S> {
    fn read_word(&mut self, bus: &mut AddressBus, src: S) -> u16;
}

pub trait WriteWord<D> {
    fn write_word(&mut self, bus: &mut AddressBus, dest: D, value: u16);
}

/// 8-bit registers (r8)
#[derive(Debug, Clone, Copy)]
pub enum RegisterU8 {
    A,
    F,
    B,
    C,
    D,
    E,
    H,
    L,
}

impl ReadByte<RegisterU8> for Cpu {
    fn read_byte(&mut self, _: &mut AddressBus, src: RegisterU8) -> u8 {
        self.register_u8(src)
    }
}

impl WriteByte<RegisterU8> for Cpu {
    fn write_byte(&mut self, _: &mut AddressBus, dest: RegisterU8, value: u8) {
        self.set_register_u8(dest, value);
    }
}

/// 16-bit registers (r16)
#[derive(Debug, Clone, Copy)]
pub enum RegisterU16 {
    AF,
    BC,
    DE,
    HL,
    SP,
    PC,
}

impl ReadWord<RegisterU16> for Cpu {
    fn read_word(&mut self, _: &mut AddressBus, src: RegisterU16) -> u16 {
        self.register_u16(src)
    }
}

impl WriteWord<RegisterU16> for Cpu {
    fn write_word(&mut self, _: &mut AddressBus, dest: RegisterU16, value: u16) {
        self.set_register_u16(dest, value);
    }
}

/// Unit struct to represent Immediate memory access.
/// next byte or word (n8 or n16)
#[derive(Debug, Clone, Copy)]
pub struct Immediate;

impl ReadByte<Immediate> for Cpu {
    fn read_byte(&mut self, bus: &mut AddressBus, _: Immediate) -> u8 {
        self.read_next_byte(bus)
    }
}

impl ReadWord<Immediate> for Cpu {
    fn read_word(&mut self, bus: &mut AddressBus, _: Immediate) -> u16 {
        let low = self.read_next_byte(bus);
        let high = self.read_next_byte(bus);
        u16::from_le_bytes([low, high])
    }
}

/// New type to represent Direct memory access.
/// Use register contents as address ([])
#[derive(Debug, Clone, Copy)]
pub struct Direct<T>(T);

impl<T> ReadByte<Direct<T>> for Cpu
where
    Self: ReadWord<T>,
{
    fn read_byte(&mut self, bus: &mut AddressBus, src: Direct<T>) -> u8 {
        let addr = self.read_word(bus, src.0);
        let byte = bus.read_byte(addr);
        bus.tick();
        byte
    }
}

impl<T> WriteByte<Direct<T>> for Cpu
where
    Self: ReadWord<T>,
{
    fn write_byte(&mut self, bus: &mut AddressBus, dest: Direct<T>, value: u8) {
        let addr = self.read_word(bus, dest.0);
        bus.write_byte(addr, value);
        bus.tick();
    }
}

impl<T> WriteWord<Direct<T>> for Cpu
where
    Self: ReadWord<T>,
{
    fn write_word(&mut self, bus: &mut AddressBus, dest: Direct<T>, value: u16) {
        let addr = self.read_word(bus, dest.0);
        let [low, high] = value.to_le_bytes();
        bus.write_byte(addr, low);
        bus.tick();
        bus.write_byte(addr.wrapping_add(1), high);
        bus.tick();
    }
}

/// New type to represent register increment.
/// Increment value in register after read (+)
#[derive(Debug, Clone, Copy)]
pub struct Increment<T>(T);

impl<T> ReadWord<Increment<T>> for Cpu
where
    Self: ReadWord<T> + WriteWord<T>,
    T: Copy,
{
    fn read_word(&mut self, bus: &mut AddressBus, src: Increment<T>) -> u16 {
        let word = self.read_word(bus, src.0);
        let new_word = word.wrapping_add(1);
        self.write_word(bus, src.0, new_word);
        word
    }
}

/// New type to represent register decrement.
/// Decrement value in register after read (-)
#[derive(Debug, Clone, Copy)]
pub struct Decrement<T>(T);

impl<T> ReadWord<Decrement<T>> for Cpu
where
    Self: ReadWord<T> + WriteWord<T>,
    T: Copy,
{
    fn read_word(&mut self, bus: &mut AddressBus, src: Decrement<T>) -> u16 {
        let word = self.read_word(bus, src.0);
        let new_word = word.wrapping_sub(1);
        self.write_word(bus, src.0, new_word);
        word
    }
}

/// New type to represent Indexed memory access.
/// Indexed from High RAM (0xFF00)
#[derive(Debug, Clone, Copy)]
pub struct HighIndexed<T>(T);

impl<T> ReadWord<HighIndexed<T>> for Cpu
where
    Self: ReadByte<T>,
{
    fn read_word(&mut self, bus: &mut AddressBus, src: HighIndexed<T>) -> u16 {
        let byte = self.read_byte(bus, src.0) as u16;
        0xFF00 | byte
    }
}

#[derive(Debug, Clone)]
pub struct Cpu {
    /// Accumulator
    a: u8,
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    /// Flags Register
    f: FlagsRegister,
    h: u8,
    l: u8,
    /// Stack Pointer
    sp: u16,
    /// Program Counter
    pc: u16,
    halted: bool,
    // IME: Interrupt Master Enable
    interrupt_enabled: bool,
    // Used to delay setting IME after calling EI
    interrupt_delay: Option<u8>,
}

impl Cpu {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            a: 0x01,
            b: 0x00,
            c: 0x13,
            d: 0x00,
            e: 0xD8,
            f: FlagsRegister::new(),
            h: 0x01,
            l: 0x4D,
            sp: 0xFFFE,
            pc: 0x0100,
            halted: false,
            interrupt_enabled: false,
            interrupt_delay: None,
        }
    }

    pub fn step(&mut self, bus: &mut AddressBus) {
        // Checks for next instruction after EI is called
        self.interrupt_delay = match self.interrupt_delay {
            Some(0) => {
                self.interrupt_enabled = true;
                None
            }
            Some(n) => Some(n - 1),
            None => None,
        };

        for interrupt in Interrupt::iter() {
            if bus.is_interrupt_pending(*interrupt) {
                // Disables HALT when interrupt is pending
                self.halted = false;
                if self.interrupt_enabled {
                    self.interrupt_enabled = false;
                    bus.interrupt_flags_mut().set(*interrupt, false);
                    // Calls interrupt handler
                    self.push(bus, RegisterU16::PC);
                    self.pc = interrupt.handler_addr();
                    bus.tick();
                    bus.tick();
                }
                break;
            }
        }

        if self.halted {
            bus.tick();
            return;
        }

        let opcode = self.read_next_byte(bus);
        self.execute(bus, opcode);
    }

    fn read_next_byte(&mut self, bus: &mut AddressBus) -> u8 {
        let byte = bus.read_byte(self.pc);
        self.pc = self.pc.wrapping_add(1);
        bus.tick();
        byte
    }

    #[allow(clippy::cast_possible_wrap)]
    fn read_next_byte_signed(&mut self, bus: &mut AddressBus) -> i8 {
        self.read_next_byte(bus) as i8
    }

    pub(crate) const fn register_u8(&self, reg: RegisterU8) -> u8 {
        match reg {
            RegisterU8::A => self.a,
            RegisterU8::F => self.f.bits(),
            RegisterU8::B => self.b,
            RegisterU8::C => self.c,
            RegisterU8::D => self.d,
            RegisterU8::E => self.e,
            RegisterU8::H => self.h,
            RegisterU8::L => self.l,
        }
    }

    const fn set_register_u8(&mut self, reg: RegisterU8, value: u8) {
        match reg {
            RegisterU8::A => self.a = value,
            RegisterU8::F => self.f = FlagsRegister::from_bits(value),
            RegisterU8::B => self.b = value,
            RegisterU8::C => self.c = value,
            RegisterU8::D => self.d = value,
            RegisterU8::E => self.e = value,
            RegisterU8::H => self.h = value,
            RegisterU8::L => self.l = value,
        }
    }

    pub(crate) const fn register_u16(&self, reg: RegisterU16) -> u16 {
        match reg {
            RegisterU16::AF => u16::from_le_bytes([self.f.bits(), self.a]),
            RegisterU16::BC => u16::from_le_bytes([self.c, self.b]),
            RegisterU16::DE => u16::from_le_bytes([self.e, self.d]),
            RegisterU16::HL => u16::from_le_bytes([self.l, self.h]),
            RegisterU16::SP => self.sp,
            RegisterU16::PC => self.pc,
        }
    }

    const fn set_register_u16(&mut self, reg: RegisterU16, value: u16) {
        let [low, high] = value.to_le_bytes();
        match reg {
            RegisterU16::AF => {
                self.a = high;
                self.f = FlagsRegister::from_bits(low);
            }
            RegisterU16::BC => {
                self.b = high;
                self.c = low;
            }
            RegisterU16::DE => {
                self.d = high;
                self.e = low;
            }
            RegisterU16::HL => {
                self.h = high;
                self.l = low;
            }
            RegisterU16::SP => {
                self.sp = value;
            }
            RegisterU16::PC => {
                self.pc = value;
            }
        }
    }
}

impl Default for Cpu {
    fn default() -> Self {
        Self::new()
    }
}
