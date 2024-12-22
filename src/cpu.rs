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
    f: FlagsRegister,
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
            f: FlagsRegister::new(),
            h: 0x01,
            l: 0x4D,
            sp: 0xFFFE,
            pc: 0x100,
        }
    }

    const fn read_byte(&self, register: Register8) -> u8 {
        match register {
            Register8::A => self.a,
            Register8::B => self.b,
            Register8::C => self.c,
            Register8::D => self.d,
            Register8::E => self.e,
            Register8::H => self.h,
            Register8::L => self.l,
        }
    }

    fn write_byte(&mut self, register: Register8, value: u8) {
        match register {
            Register8::A => self.a = value,
            Register8::B => self.b = value,
            Register8::C => self.c = value,
            Register8::D => self.d = value,
            Register8::E => self.e = value,
            Register8::H => self.h = value,
            Register8::L => self.l = value,
        }
    }

    const fn read_word(&self, register: Register16) -> u16 {
        match register {
            Register16::AF => u16::from_le_bytes([self.f.bits(), self.a]),
            Register16::BC => u16::from_le_bytes([self.c, self.b]),
            Register16::DE => u16::from_le_bytes([self.e, self.d]),
            Register16::HL => u16::from_le_bytes([self.l, self.h]),
            Register16::SP => self.sp,
            Register16::PC => self.pc,
        }
    }

    fn write_word(&mut self, register: Register16, value: u16) {
        match register {
            Register16::AF => {
                let [low, high] = value.to_le_bytes();
                self.a = high;
                self.f = FlagsRegister::from_bits(low);
            }
            Register16::BC => {
                let [low, high] = value.to_le_bytes();
                self.b = high;
                self.c = low;
            }
            Register16::DE => {
                let [low, high] = value.to_le_bytes();
                self.d = high;
                self.e = low;
            }
            Register16::HL => {
                let [low, high] = value.to_le_bytes();
                self.h = high;
                self.l = low;
            }
            Register16::SP => {
                self.sp = value;
            }
            Register16::PC => {
                self.pc = value;
            }
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct FlagsRegister(u8);

impl FlagsRegister {
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

pub trait AccessReadByte<S> {
    fn read_byte(&mut self, bus: &mut AddressBus, src: S) -> u8;
}

pub trait AccessWriteByte<D> {
    fn write_byte(&mut self, bus: &mut AddressBus, dest: D, value: u8);
}

pub trait AccessReadWord<S> {
    fn read_word(&mut self, bus: &mut AddressBus, src: S) -> u16;
}

pub trait AccessWriteWord<D> {
    fn write_word(&mut self, bus: &mut AddressBus, dest: D, value: u16);
}

/// 8-bit registers (r8)
#[derive(Debug, Clone, Copy)]
pub enum Register8 {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
}

impl AccessReadByte<Register8> for Cpu {
    fn read_byte(&mut self, _: &mut AddressBus, src: Register8) -> u8 {
        self.registers.read_byte(src)
    }
}

impl AccessWriteByte<Register8> for Cpu {
    fn write_byte(&mut self, _: &mut AddressBus, dest: Register8, value: u8) {
        self.registers.write_byte(dest, value);
    }
}

/// 16-bit registers (r16)
#[derive(Debug, Clone, Copy)]
pub enum Register16 {
    AF,
    BC,
    DE,
    HL,
    SP,
    PC,
}

impl AccessReadWord<Register16> for Cpu {
    fn read_word(&mut self, _: &mut AddressBus, src: Register16) -> u16 {
        self.registers.read_word(src)
    }
}

impl AccessWriteWord<Register16> for Cpu {
    fn write_word(&mut self, _: &mut AddressBus, dest: Register16, value: u16) {
        self.registers.write_word(dest, value);
    }
}

/// Unit struct to represent Immediate memory access.
/// next byte or word (n8 or n16)
#[derive(Debug, Clone, Copy)]
pub struct Immediate;

impl AccessReadByte<Immediate> for Cpu {
    fn read_byte(&mut self, bus: &mut AddressBus, _: Immediate) -> u8 {
        let byte = self.read_next_byte(bus);
        bus.tick();
        byte
    }
}

impl AccessReadWord<Immediate> for Cpu {
    fn read_word(&mut self, bus: &mut AddressBus, _: Immediate) -> u16 {
        let low = self.read_next_byte(bus);
        bus.tick();
        let high = self.read_next_byte(bus);
        bus.tick();
        u16::from_le_bytes([low, high])
    }
}

/// New type to represent Direct memory access.
/// Use register contents as address ([])
#[derive(Debug, Clone, Copy)]
pub struct Direct<T>(T);

impl<T> AccessReadByte<Direct<T>> for Cpu
where
    Self: AccessReadWord<T>,
{
    fn read_byte(&mut self, bus: &mut AddressBus, src: Direct<T>) -> u8 {
        let addr = self.read_word(bus, src.0);
        let byte = bus.read_byte(addr);
        bus.tick();
        byte
    }
}

impl<T> AccessWriteByte<Direct<T>> for Cpu
where
    Self: AccessReadWord<T>,
{
    fn write_byte(&mut self, bus: &mut AddressBus, dest: Direct<T>, value: u8) {
        let addr = self.read_word(bus, dest.0);
        bus.write_byte(addr, value);
        bus.tick();
    }
}

impl<T> AccessWriteWord<Direct<T>> for Cpu
where
    Self: AccessReadWord<T>,
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

impl<T> AccessReadWord<Increment<T>> for Cpu
where
    Self: AccessReadWord<T> + AccessWriteWord<T>,
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

impl<T> AccessReadWord<Decrement<T>> for Cpu
where
    Self: AccessReadWord<T> + AccessWriteWord<T>,
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
/// offset from High RAM (0xFF00)
#[derive(Debug, Clone, Copy)]
pub struct HighIndexed<T>(T);

impl<T> AccessReadWord<HighIndexed<T>> for Cpu
where
    Self: AccessReadByte<T>,
{
    fn read_word(&mut self, bus: &mut AddressBus, src: HighIndexed<T>) -> u16 {
        let byte = self.read_byte(bus, src.0) as u16;
        0xFF00 | byte
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

    pub fn step(&mut self, bus: &mut AddressBus) {
        // Checks for next instruction after EI is called
        self.ime_delay_counter = self.ime_delay_counter.map(|n| n - 1);
        if self.ime_delay_counter.is_some_and(|n| n == 0) {
            self.ime = true;
            self.ime_delay_counter = None;
        }

        if self.ime {
            // Checks for pending interrupts
            let interrupt_pending = bus.get_interrupts_pending();

            for flag in InterruptFlags::flags() {
                if interrupt_pending.contains(flag.bits()) {
                    self.halted = false;
                    // Calls interrupt handler
                    self.ime = false;
                    bus.interrupt_flag().set(flag.bits(), false);
                    self.push(bus, Register16::PC);
                    self.registers.pc = flag.handler_addr();
                    break;
                }
            }
        }

        if self.halted {
            bus.tick();
            return;
        }

        let opcode = self.read_next_byte(bus);
        self.execute(bus, opcode);
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
}

impl Default for Cpu {
    fn default() -> Self {
        Self::new()
    }
}
