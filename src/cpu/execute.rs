use crate::cpu::{Addr, Cpu, HighAddr, JumpCondition, N16, N8, R16, R8};
use crate::hardware::AddressBus;

impl Cpu {
    pub(crate) fn execute(&mut self, memory: &mut AddressBus, byte: u8) -> usize {
        match byte {
            // ---- 8-bit Arithmetic
            // ADD
            0x87 => {
                self.add(memory, R8::A);
                4
            }
            0x80 => {
                self.add(memory, R8::B);
                4
            }
            0x81 => {
                self.add(memory, R8::C);
                4
            }
            0x82 => {
                self.add(memory, R8::D);
                4
            }
            0x83 => {
                self.add(memory, R8::E);
                4
            }
            0x84 => {
                self.add(memory, R8::H);
                4
            }
            0x85 => {
                self.add(memory, R8::L);
                4
            }
            0x86 => {
                self.add(memory, Addr::HL);
                8
            }
            0xC6 => {
                self.add(memory, N8);
                8
            }
            // ADC
            0x8F => {
                self.add_with_carry(memory, R8::A);
                4
            }
            0x88 => {
                self.add_with_carry(memory, R8::B);
                4
            }
            0x89 => {
                self.add_with_carry(memory, R8::C);
                4
            }
            0x8A => {
                self.add_with_carry(memory, R8::D);
                4
            }
            0x8B => {
                self.add_with_carry(memory, R8::E);
                4
            }
            0x8C => {
                self.add_with_carry(memory, R8::H);
                4
            }
            0x8D => {
                self.add_with_carry(memory, R8::L);
                4
            }
            0x8E => {
                self.add_with_carry(memory, Addr::HL);
                8
            }
            0xCE => {
                self.add_with_carry(memory, N8);
                8
            }
            // SUB
            0x97 => {
                self.subtract(memory, R8::A);
                4
            }
            0x90 => {
                self.subtract(memory, R8::B);
                4
            }
            0x91 => {
                self.subtract(memory, R8::C);
                4
            }
            0x92 => {
                self.subtract(memory, R8::D);
                4
            }
            0x93 => {
                self.subtract(memory, R8::E);
                4
            }
            0x94 => {
                self.subtract(memory, R8::H);
                4
            }
            0x95 => {
                self.subtract(memory, R8::L);
                4
            }
            0x96 => {
                self.subtract(memory, Addr::HL);
                8
            }
            0xD6 => {
                self.subtract(memory, N8);
                8
            }
            // SBC
            0x9F => {
                self.subtract_with_carry(memory, R8::A);
                4
            }
            0x98 => {
                self.subtract_with_carry(memory, R8::B);
                4
            }
            0x99 => {
                self.subtract_with_carry(memory, R8::C);
                4
            }
            0x9A => {
                self.subtract_with_carry(memory, R8::D);
                4
            }
            0x9B => {
                self.subtract_with_carry(memory, R8::E);
                4
            }
            0x9C => {
                self.subtract_with_carry(memory, R8::H);
                4
            }
            0x9D => {
                self.subtract_with_carry(memory, R8::L);
                4
            }
            0x9E => {
                self.subtract_with_carry(memory, Addr::HL);
                8
            }
            0xDE => {
                self.subtract_with_carry(memory, N8);
                8
            }
            // AND
            0xA7 => {
                self.and(memory, R8::A);
                4
            }
            0xA0 => {
                self.and(memory, R8::B);
                4
            }
            0xA1 => {
                self.and(memory, R8::C);
                4
            }
            0xA2 => {
                self.and(memory, R8::D);
                4
            }
            0xA3 => {
                self.and(memory, R8::E);
                4
            }
            0xA4 => {
                self.and(memory, R8::H);
                4
            }
            0xA5 => {
                self.and(memory, R8::L);
                4
            }
            0xA6 => {
                self.and(memory, Addr::HL);
                8
            }
            0xE6 => {
                self.and(memory, N8);
                8
            }
            // XOR
            0xAF => {
                self.xor(memory, R8::A);
                4
            }
            0xA8 => {
                self.xor(memory, R8::B);
                4
            }
            0xA9 => {
                self.xor(memory, R8::C);
                4
            }
            0xAA => {
                self.xor(memory, R8::D);
                4
            }
            0xAB => {
                self.xor(memory, R8::E);
                4
            }
            0xAC => {
                self.xor(memory, R8::H);
                4
            }
            0xAD => {
                self.xor(memory, R8::L);
                4
            }
            0xAE => {
                self.xor(memory, Addr::HL);
                8
            }
            0xEE => {
                self.xor(memory, N8);
                8
            }
            // OR
            0xB7 => {
                self.or(memory, R8::A);
                4
            }
            0xB0 => {
                self.or(memory, R8::B);
                4
            }
            0xB1 => {
                self.or(memory, R8::C);
                4
            }
            0xB2 => {
                self.or(memory, R8::D);
                4
            }
            0xB3 => {
                self.or(memory, R8::E);
                4
            }
            0xB4 => {
                self.or(memory, R8::H);
                4
            }
            0xB5 => {
                self.or(memory, R8::L);
                4
            }
            0xB6 => {
                self.or(memory, Addr::HL);
                8
            }
            0xF6 => {
                self.or(memory, N8);
                8
            }
            // CP
            0xBF => {
                self.compare(memory, R8::A);
                4
            }
            0xB8 => {
                self.compare(memory, R8::B);
                4
            }
            0xB9 => {
                self.compare(memory, R8::C);
                4
            }
            0xBA => {
                self.compare(memory, R8::D);
                4
            }
            0xBB => {
                self.compare(memory, R8::E);
                4
            }
            0xBC => {
                self.compare(memory, R8::H);
                4
            }
            0xBD => {
                self.compare(memory, R8::L);
                4
            }
            0xBE => {
                self.compare(memory, Addr::HL);
                8
            }
            0xFE => {
                self.compare(memory, N8);
                8
            }
            // INC
            0x3C => {
                self.increment(memory, R8::A);
                4
            }
            0x04 => {
                self.increment(memory, R8::B);
                4
            }
            0x0C => {
                self.increment(memory, R8::C);
                4
            }
            0x14 => {
                self.increment(memory, R8::D);
                4
            }
            0x1C => {
                self.increment(memory, R8::E);
                4
            }
            0x24 => {
                self.increment(memory, R8::H);
                4
            }
            0x2C => {
                self.increment(memory, R8::L);
                4
            }
            0x34 => {
                self.increment(memory, Addr::HL);
                12
            }
            // DEC
            0x3D => {
                self.decrement(memory, R8::A);
                4
            }
            0x05 => {
                self.decrement(memory, R8::B);
                4
            }
            0x0D => {
                self.decrement(memory, R8::C);
                4
            }
            0x15 => {
                self.decrement(memory, R8::D);
                4
            }
            0x1D => {
                self.decrement(memory, R8::E);
                4
            }
            0x25 => {
                self.decrement(memory, R8::H);
                4
            }
            0x2D => {
                self.decrement(memory, R8::L);
                4
            }
            0x35 => {
                self.decrement(memory, Addr::HL);
                12
            }
            // DAA
            0x27 => {
                self.decimal_adjust_accumulator();
                4
            }
            // SCF
            0x37 => {
                self.set_carry_flag();
                4
            }
            // CPL
            0x2F => {
                self.complement_accumulator();
                4
            }
            // CCF
            0x3F => {
                self.complement_carry_flag();
                4
            }
            // ---- 16-bit Arithmetic
            // ADD
            0x09 => {
                self.add16_hl(R16::BC);
                8
            }
            0x19 => {
                self.add16_hl(R16::DE);
                8
            }
            0x29 => {
                self.add16_hl(R16::HL);
                8
            }
            0x39 => {
                self.add16_hl(R16::SP);
                8
            }
            0xE8 => {
                self.add16_sp(memory);
                16
            }
            // INC
            0x03 => {
                self.increment16(R16::BC);
                8
            }
            0x13 => {
                self.increment16(R16::DE);
                8
            }
            0x23 => {
                self.increment16(R16::HL);
                8
            }
            0x33 => {
                self.increment16(R16::SP);
                8
            }
            // DEC
            0x0B => {
                self.decrement16(R16::BC);
                8
            }
            0x1B => {
                self.decrement16(R16::DE);
                8
            }
            0x2B => {
                self.decrement16(R16::HL);
                8
            }
            0x3B => {
                self.decrement16(R16::SP);
                8
            }
            // ---- Bit Shift
            // RLCA
            0x07 => {
                self.rotate_left_circular_accumulator();
                4
            }
            // RRCA
            0x0F => {
                self.rotate_right_circular_accumulator();
                4
            }
            // RLA
            0x17 => {
                self.rotate_left_accumulator();
                4
            }
            // RRA
            0x1F => {
                self.rotate_right_accumulator();
                4
            }
            // ---- 8-bit Load
            // LD
            0x47 => {
                self.load(memory, R8::B, R8::A);
                4
            }
            0x40 => {
                self.load(memory, R8::B, R8::B);
                4
            }
            0x41 => {
                self.load(memory, R8::B, R8::C);
                4
            }
            0x42 => {
                self.load(memory, R8::B, R8::D);
                4
            }
            0x43 => {
                self.load(memory, R8::B, R8::E);
                4
            }
            0x44 => {
                self.load(memory, R8::B, R8::H);
                4
            }
            0x45 => {
                self.load(memory, R8::B, R8::L);
                4
            }
            0x46 => {
                self.load(memory, R8::B, Addr::HL);
                8
            }
            0x06 => {
                self.load(memory, R8::B, N8);
                8
            }
            0x4F => {
                self.load(memory, R8::C, R8::A);
                4
            }
            0x48 => {
                self.load(memory, R8::C, R8::B);
                4
            }
            0x49 => {
                self.load(memory, R8::C, R8::C);
                4
            }
            0x4A => {
                self.load(memory, R8::C, R8::D);
                4
            }
            0x4B => {
                self.load(memory, R8::C, R8::E);
                4
            }
            0x4C => {
                self.load(memory, R8::C, R8::H);
                4
            }
            0x4D => {
                self.load(memory, R8::C, R8::L);
                4
            }
            0x4E => {
                self.load(memory, R8::C, Addr::HL);
                8
            }
            0x0E => {
                self.load(memory, R8::C, N8);
                8
            }
            0x57 => {
                self.load(memory, R8::D, R8::A);
                4
            }
            0x50 => {
                self.load(memory, R8::D, R8::B);
                4
            }
            0x51 => {
                self.load(memory, R8::D, R8::C);
                4
            }
            0x52 => {
                self.load(memory, R8::D, R8::D);
                4
            }
            0x53 => {
                self.load(memory, R8::D, R8::E);
                4
            }
            0x54 => {
                self.load(memory, R8::D, R8::H);
                4
            }
            0x55 => {
                self.load(memory, R8::D, R8::L);
                4
            }
            0x56 => {
                self.load(memory, R8::D, Addr::HL);
                8
            }
            0x16 => {
                self.load(memory, R8::D, N8);
                8
            }
            0x5F => {
                self.load(memory, R8::E, R8::A);
                4
            }
            0x58 => {
                self.load(memory, R8::E, R8::B);
                4
            }
            0x59 => {
                self.load(memory, R8::E, R8::C);
                4
            }
            0x5A => {
                self.load(memory, R8::E, R8::D);
                4
            }
            0x5B => {
                self.load(memory, R8::E, R8::E);
                4
            }
            0x5C => {
                self.load(memory, R8::E, R8::H);
                4
            }
            0x5D => {
                self.load(memory, R8::E, R8::L);
                4
            }
            0x5E => {
                self.load(memory, R8::E, Addr::HL);
                8
            }
            0x1E => {
                self.load(memory, R8::E, N8);
                8
            }
            0x67 => {
                self.load(memory, R8::H, R8::A);
                4
            }
            0x60 => {
                self.load(memory, R8::H, R8::B);
                4
            }
            0x61 => {
                self.load(memory, R8::H, R8::C);
                4
            }
            0x62 => {
                self.load(memory, R8::H, R8::D);
                4
            }
            0x63 => {
                self.load(memory, R8::H, R8::E);
                4
            }
            0x64 => {
                self.load(memory, R8::H, R8::H);
                4
            }
            0x65 => {
                self.load(memory, R8::H, R8::L);
                4
            }
            0x66 => {
                self.load(memory, R8::H, Addr::HL);
                8
            }
            0x26 => {
                self.load(memory, R8::H, N8);
                8
            }
            0x6F => {
                self.load(memory, R8::L, R8::A);
                4
            }
            0x68 => {
                self.load(memory, R8::L, R8::B);
                4
            }
            0x69 => {
                self.load(memory, R8::L, R8::C);
                4
            }
            0x6A => {
                self.load(memory, R8::L, R8::D);
                4
            }
            0x6B => {
                self.load(memory, R8::L, R8::E);
                4
            }
            0x6C => {
                self.load(memory, R8::L, R8::H);
                4
            }
            0x6D => {
                self.load(memory, R8::L, R8::L);
                4
            }
            0x6E => {
                self.load(memory, R8::L, Addr::HL);
                8
            }
            0x2E => {
                self.load(memory, R8::L, N8);
                8
            }
            0x77 => {
                self.load(memory, Addr::HL, R8::A);
                8
            }
            0x70 => {
                self.load(memory, Addr::HL, R8::B);
                8
            }
            0x71 => {
                self.load(memory, Addr::HL, R8::C);
                8
            }
            0x72 => {
                self.load(memory, Addr::HL, R8::D);
                8
            }
            0x73 => {
                self.load(memory, Addr::HL, R8::E);
                8
            }
            0x74 => {
                self.load(memory, Addr::HL, R8::H);
                8
            }
            0x75 => {
                self.load(memory, Addr::HL, R8::L);
                8
            }
            0x36 => {
                self.load(memory, Addr::HL, N8);
                12
            }
            0x7F => {
                self.load(memory, R8::A, R8::A);
                4
            }
            0x78 => {
                self.load(memory, R8::A, R8::B);
                4
            }
            0x79 => {
                self.load(memory, R8::A, R8::C);
                4
            }
            0x7A => {
                self.load(memory, R8::A, R8::D);
                4
            }
            0x7B => {
                self.load(memory, R8::A, R8::E);
                4
            }
            0x7C => {
                self.load(memory, R8::A, R8::H);
                4
            }
            0x7D => {
                self.load(memory, R8::A, R8::L);
                4
            }
            0x7E => {
                self.load(memory, R8::A, Addr::HL);
                8
            }
            0x3E => {
                self.load(memory, R8::A, N8);
                8
            }
            0x02 => {
                self.load(memory, Addr::BC, R8::A);
                8
            }
            0x12 => {
                self.load(memory, Addr::DE, R8::A);
                8
            }
            0x22 => {
                self.load(memory, Addr::HLi, R8::A);
                8
            }
            0x32 => {
                self.load(memory, Addr::HLd, R8::A);
                8
            }
            0x0A => {
                self.load(memory, R8::A, Addr::BC);
                8
            }
            0x1A => {
                self.load(memory, R8::A, Addr::DE);
                8
            }
            0x2A => {
                self.load(memory, R8::A, Addr::HLi);
                8
            }
            0x3A => {
                self.load(memory, R8::A, Addr::HLd);
                8
            }
            0xEA => {
                self.load(memory, Addr::N16, R8::A);
                16
            }
            0xFA => {
                self.load(memory, R8::A, Addr::N16);
                16
            }
            // LDH
            0xE0 => {
                self.load(memory, HighAddr::N8, R8::A);
                12
            }
            0xF0 => {
                self.load(memory, R8::A, HighAddr::N8);
                12
            }
            0xE2 => {
                self.load(memory, HighAddr::C, R8::A);
                8
            }
            0xF2 => {
                self.load(memory, R8::A, HighAddr::C);
                8
            }
            // ---- 16-bit Load
            // LD
            0x01 => {
                self.load16(memory, R16::BC, N16);
                12
            }
            0x11 => {
                self.load16(memory, R16::DE, N16);
                12
            }
            0x21 => {
                self.load16(memory, R16::HL, N16);
                12
            }
            0x31 => {
                self.load16(memory, R16::SP, N16);
                12
            }
            0xF9 => {
                self.load16(memory, R16::SP, R16::HL);
                8
            }
            0x08 => {
                self.load16_a16_sp(memory);
                20
            }
            0xF8 => {
                self.load16_hl_sp(memory);
                12
            }
            // PUSH
            0xC5 => {
                self.push(memory, R16::BC);
                16
            }
            0xD5 => {
                self.push(memory, R16::DE);
                16
            }
            0xE5 => {
                self.push(memory, R16::HL);
                16
            }
            0xF5 => {
                self.push(memory, R16::AF);
                16
            }
            // POP
            0xC1 => {
                self.pop(memory, R16::BC);
                12
            }
            0xD1 => {
                self.pop(memory, R16::DE);
                12
            }
            0xE1 => {
                self.pop(memory, R16::HL);
                12
            }
            0xF1 => {
                self.pop(memory, R16::AF);
                12
            }
            // ---- Jumps
            // JP
            0xE9 => {
                self.jump_to_hl();
                4
            }
            0xC3 => self.jump(memory, JumpCondition::Always),
            0xC2 => self.jump(memory, JumpCondition::NotZero),
            0xCA => self.jump(memory, JumpCondition::Zero),
            0xD2 => self.jump(memory, JumpCondition::NotCarry),
            0xDA => self.jump(memory, JumpCondition::Carry),
            // JR
            0x18 => self.jump_relative(memory, JumpCondition::Always),
            0x20 => self.jump_relative(memory, JumpCondition::NotZero),
            0x28 => self.jump_relative(memory, JumpCondition::Zero),
            0x30 => self.jump_relative(memory, JumpCondition::NotCarry),
            0x38 => self.jump_relative(memory, JumpCondition::Carry),
            // CALL
            0xCD => self.call(memory, JumpCondition::Always),
            0xC4 => self.call(memory, JumpCondition::NotZero),
            0xCC => self.call(memory, JumpCondition::Zero),
            0xD4 => self.call(memory, JumpCondition::NotCarry),
            0xDC => self.call(memory, JumpCondition::Carry),
            // RET
            0xC9 => {
                self.return_(memory, JumpCondition::Always);
                16
            }
            0xC0 => self.return_(memory, JumpCondition::NotZero),
            0xC8 => self.return_(memory, JumpCondition::Zero),
            0xD0 => self.return_(memory, JumpCondition::NotCarry),
            0xD8 => self.return_(memory, JumpCondition::Carry),
            // RETI
            0xD9 => {
                self.return_from_interrupt_handler(memory);
                16
            }
            // RST
            0xC7 => {
                self.restart(memory, 0x00);
                16
            }
            0xCF => {
                self.restart(memory, 0x08);
                16
            }
            0xD7 => {
                self.restart(memory, 0x10);
                16
            }
            0xDF => {
                self.restart(memory, 0x18);
                16
            }
            0xE7 => {
                self.restart(memory, 0x20);
                16
            }
            0xEF => {
                self.restart(memory, 0x28);
                16
            }
            0xF7 => {
                self.restart(memory, 0x30);
                16
            }
            0xFF => {
                self.restart(memory, 0x38);
                16
            }
            // ---- Control
            //NOP
            0x00 => {
                Self::no_operation();
                4
            }
            // STOP
            0x10 => {
                self.stop(memory);
                4
            }
            // HALT
            0x76 => {
                self.halt();
                4
            }
            // PREFIX
            0xCB => {
                let next_byte = self.read_next_byte(memory);
                self.execute_prefixed(memory, next_byte)
            }
            // DI
            0xF3 => {
                self.disable_interrupt();
                4
            }
            // EI
            0xFB => {
                self.enable_interrupt();
                4
            }
            // ---- Undefined
            n @ (0xD3 | 0xDB | 0xDD | 0xE3 | 0xE4 | 0xEB | 0xEC | 0xED | 0xF4 | 0xFC | 0xFD) => {
                Self::undefined(n);
                unreachable!()
            }
        }
    }

    fn execute_prefixed(&mut self, memory: &mut AddressBus, byte: u8) -> usize {
        match byte {
            // ---- Bit Shift
            // RLC
            0x00 => {
                self.rotate_left_circular(memory, R8::B);
                8
            }
            0x01 => {
                self.rotate_left_circular(memory, R8::C);
                8
            }
            0x02 => {
                self.rotate_left_circular(memory, R8::D);
                8
            }
            0x03 => {
                self.rotate_left_circular(memory, R8::E);
                8
            }
            0x04 => {
                self.rotate_left_circular(memory, R8::H);
                8
            }
            0x05 => {
                self.rotate_left_circular(memory, R8::L);
                8
            }
            0x06 => {
                self.rotate_left_circular(memory, Addr::HL);
                16
            }
            0x07 => {
                self.rotate_left_circular(memory, R8::A);
                8
            }
            // RRC
            0x08 => {
                self.rotate_right_circular(memory, R8::B);
                8
            }
            0x09 => {
                self.rotate_right_circular(memory, R8::C);
                8
            }
            0x0A => {
                self.rotate_right_circular(memory, R8::D);
                8
            }
            0x0B => {
                self.rotate_right_circular(memory, R8::E);
                8
            }
            0x0C => {
                self.rotate_right_circular(memory, R8::H);
                8
            }
            0x0D => {
                self.rotate_right_circular(memory, R8::L);
                8
            }
            0x0E => {
                self.rotate_right_circular(memory, Addr::HL);
                16
            }
            0x0F => {
                self.rotate_right_circular(memory, R8::A);
                8
            }
            // RL
            0x10 => {
                self.rotate_left(memory, R8::B);
                8
            }
            0x11 => {
                self.rotate_left(memory, R8::C);
                8
            }
            0x12 => {
                self.rotate_left(memory, R8::D);
                8
            }
            0x13 => {
                self.rotate_left(memory, R8::E);
                8
            }
            0x14 => {
                self.rotate_left(memory, R8::H);
                8
            }
            0x15 => {
                self.rotate_left(memory, R8::L);
                8
            }
            0x16 => {
                self.rotate_left(memory, Addr::HL);
                16
            }
            0x17 => {
                self.rotate_left(memory, R8::A);
                8
            }
            // RR
            0x18 => {
                self.rotate_right(memory, R8::B);
                8
            }
            0x19 => {
                self.rotate_right(memory, R8::C);
                8
            }
            0x1A => {
                self.rotate_right(memory, R8::D);
                8
            }
            0x1B => {
                self.rotate_right(memory, R8::E);
                8
            }
            0x1C => {
                self.rotate_right(memory, R8::H);
                8
            }
            0x1D => {
                self.rotate_right(memory, R8::L);
                8
            }
            0x1E => {
                self.rotate_right(memory, Addr::HL);
                16
            }
            0x1F => {
                self.rotate_right(memory, R8::A);
                8
            }
            // SLA
            0x20 => {
                self.shift_left_arithmetic(memory, R8::B);
                8
            }
            0x21 => {
                self.shift_left_arithmetic(memory, R8::C);
                8
            }
            0x22 => {
                self.shift_left_arithmetic(memory, R8::D);
                8
            }
            0x23 => {
                self.shift_left_arithmetic(memory, R8::E);
                8
            }
            0x24 => {
                self.shift_left_arithmetic(memory, R8::H);
                8
            }
            0x25 => {
                self.shift_left_arithmetic(memory, R8::L);
                8
            }
            0x26 => {
                self.shift_left_arithmetic(memory, Addr::HL);
                16
            }
            0x27 => {
                self.shift_left_arithmetic(memory, R8::A);
                8
            }
            // SRA
            0x28 => {
                self.shift_right_arithmetic(memory, R8::B);
                8
            }
            0x29 => {
                self.shift_right_arithmetic(memory, R8::C);
                8
            }
            0x2A => {
                self.shift_right_arithmetic(memory, R8::D);
                8
            }
            0x2B => {
                self.shift_right_arithmetic(memory, R8::E);
                8
            }
            0x2C => {
                self.shift_right_arithmetic(memory, R8::H);
                8
            }
            0x2D => {
                self.shift_right_arithmetic(memory, R8::L);
                8
            }
            0x2E => {
                self.shift_right_arithmetic(memory, Addr::HL);
                16
            }
            0x2F => {
                self.shift_right_arithmetic(memory, R8::A);
                8
            }
            // SWAP
            0x30 => {
                self.swap(memory, R8::B);
                8
            }
            0x31 => {
                self.swap(memory, R8::C);
                8
            }
            0x32 => {
                self.swap(memory, R8::D);
                8
            }
            0x33 => {
                self.swap(memory, R8::E);
                8
            }
            0x34 => {
                self.swap(memory, R8::H);
                8
            }
            0x35 => {
                self.swap(memory, R8::L);
                8
            }
            0x36 => {
                self.swap(memory, Addr::HL);
                16
            }
            0x37 => {
                self.swap(memory, R8::A);
                8
            }
            // SRL
            0x38 => {
                self.shift_right_logical(memory, R8::B);
                8
            }
            0x39 => {
                self.shift_right_logical(memory, R8::C);
                8
            }
            0x3A => {
                self.shift_right_logical(memory, R8::D);
                8
            }
            0x3B => {
                self.shift_right_logical(memory, R8::E);
                8
            }
            0x3C => {
                self.shift_right_logical(memory, R8::H);
                8
            }
            0x3D => {
                self.shift_right_logical(memory, R8::L);
                8
            }
            0x3E => {
                self.shift_right_logical(memory, Addr::HL);
                16
            }
            0x3F => {
                self.shift_right_logical(memory, R8::A);
                8
            }
            // ---- Bit Operations
            // BIT
            0x40 => {
                self.bit_test(memory, 0, R8::B);
                8
            }
            0x41 => {
                self.bit_test(memory, 0, R8::C);
                8
            }
            0x42 => {
                self.bit_test(memory, 0, R8::D);
                8
            }
            0x43 => {
                self.bit_test(memory, 0, R8::E);
                8
            }
            0x44 => {
                self.bit_test(memory, 0, R8::H);
                8
            }
            0x45 => {
                self.bit_test(memory, 0, R8::L);
                8
            }
            0x46 => {
                self.bit_test(memory, 0, Addr::HL);
                12
            }
            0x47 => {
                self.bit_test(memory, 0, R8::A);
                8
            }
            0x48 => {
                self.bit_test(memory, 1, R8::B);
                8
            }
            0x49 => {
                self.bit_test(memory, 1, R8::C);
                8
            }
            0x4A => {
                self.bit_test(memory, 1, R8::D);
                8
            }
            0x4B => {
                self.bit_test(memory, 1, R8::E);
                8
            }
            0x4C => {
                self.bit_test(memory, 1, R8::H);
                8
            }
            0x4D => {
                self.bit_test(memory, 1, R8::L);
                8
            }
            0x4E => {
                self.bit_test(memory, 1, Addr::HL);
                12
            }
            0x4F => {
                self.bit_test(memory, 1, R8::A);
                8
            }
            0x50 => {
                self.bit_test(memory, 2, R8::B);
                8
            }
            0x51 => {
                self.bit_test(memory, 2, R8::C);
                8
            }
            0x52 => {
                self.bit_test(memory, 2, R8::D);
                8
            }
            0x53 => {
                self.bit_test(memory, 2, R8::E);
                8
            }
            0x54 => {
                self.bit_test(memory, 2, R8::H);
                8
            }
            0x55 => {
                self.bit_test(memory, 2, R8::L);
                8
            }
            0x56 => {
                self.bit_test(memory, 2, Addr::HL);
                12
            }
            0x57 => {
                self.bit_test(memory, 2, R8::A);
                8
            }
            0x58 => {
                self.bit_test(memory, 3, R8::B);
                8
            }
            0x59 => {
                self.bit_test(memory, 3, R8::C);
                8
            }
            0x5A => {
                self.bit_test(memory, 3, R8::D);
                8
            }
            0x5B => {
                self.bit_test(memory, 3, R8::E);
                8
            }
            0x5C => {
                self.bit_test(memory, 3, R8::H);
                8
            }
            0x5D => {
                self.bit_test(memory, 3, R8::L);
                8
            }
            0x5E => {
                self.bit_test(memory, 3, Addr::HL);
                12
            }
            0x5F => {
                self.bit_test(memory, 3, R8::A);
                8
            }
            0x60 => {
                self.bit_test(memory, 4, R8::B);
                8
            }
            0x61 => {
                self.bit_test(memory, 4, R8::C);
                8
            }
            0x62 => {
                self.bit_test(memory, 4, R8::D);
                8
            }
            0x63 => {
                self.bit_test(memory, 4, R8::E);
                8
            }
            0x64 => {
                self.bit_test(memory, 4, R8::H);
                8
            }
            0x65 => {
                self.bit_test(memory, 4, R8::L);
                8
            }
            0x66 => {
                self.bit_test(memory, 4, Addr::HL);
                12
            }
            0x67 => {
                self.bit_test(memory, 4, R8::A);
                8
            }
            0x68 => {
                self.bit_test(memory, 5, R8::B);
                8
            }
            0x69 => {
                self.bit_test(memory, 5, R8::C);
                8
            }
            0x6A => {
                self.bit_test(memory, 5, R8::D);
                8
            }
            0x6B => {
                self.bit_test(memory, 5, R8::E);
                8
            }
            0x6C => {
                self.bit_test(memory, 5, R8::H);
                8
            }
            0x6D => {
                self.bit_test(memory, 5, R8::L);
                8
            }
            0x6E => {
                self.bit_test(memory, 5, Addr::HL);
                12
            }
            0x6F => {
                self.bit_test(memory, 5, R8::A);
                8
            }
            0x70 => {
                self.bit_test(memory, 6, R8::B);
                8
            }
            0x71 => {
                self.bit_test(memory, 6, R8::C);
                8
            }
            0x72 => {
                self.bit_test(memory, 6, R8::D);
                8
            }
            0x73 => {
                self.bit_test(memory, 6, R8::E);
                8
            }
            0x74 => {
                self.bit_test(memory, 6, R8::H);
                8
            }
            0x75 => {
                self.bit_test(memory, 6, R8::L);
                8
            }
            0x76 => {
                self.bit_test(memory, 6, Addr::HL);
                12
            }
            0x77 => {
                self.bit_test(memory, 6, R8::A);
                8
            }
            0x78 => {
                self.bit_test(memory, 7, R8::B);
                8
            }
            0x79 => {
                self.bit_test(memory, 7, R8::C);
                8
            }
            0x7A => {
                self.bit_test(memory, 7, R8::D);
                8
            }
            0x7B => {
                self.bit_test(memory, 7, R8::E);
                8
            }
            0x7C => {
                self.bit_test(memory, 7, R8::H);
                8
            }
            0x7D => {
                self.bit_test(memory, 7, R8::L);
                8
            }
            0x7E => {
                self.bit_test(memory, 7, Addr::HL);
                12
            }
            0x7F => {
                self.bit_test(memory, 7, R8::A);
                8
            }
            // RES
            0x80 => {
                self.bit_reset(memory, 0, R8::B);
                8
            }
            0x81 => {
                self.bit_reset(memory, 0, R8::C);
                8
            }
            0x82 => {
                self.bit_reset(memory, 0, R8::D);
                8
            }
            0x83 => {
                self.bit_reset(memory, 0, R8::E);
                8
            }
            0x84 => {
                self.bit_reset(memory, 0, R8::H);
                8
            }
            0x85 => {
                self.bit_reset(memory, 0, R8::L);
                8
            }
            0x86 => {
                self.bit_reset(memory, 0, Addr::HL);
                16
            }
            0x87 => {
                self.bit_reset(memory, 0, R8::A);
                8
            }
            0x88 => {
                self.bit_reset(memory, 1, R8::B);
                8
            }
            0x89 => {
                self.bit_reset(memory, 1, R8::C);
                8
            }
            0x8A => {
                self.bit_reset(memory, 1, R8::D);
                8
            }
            0x8B => {
                self.bit_reset(memory, 1, R8::E);
                8
            }
            0x8C => {
                self.bit_reset(memory, 1, R8::H);
                8
            }
            0x8D => {
                self.bit_reset(memory, 1, R8::L);
                8
            }
            0x8E => {
                self.bit_reset(memory, 1, Addr::HL);
                16
            }
            0x8F => {
                self.bit_reset(memory, 1, R8::A);
                8
            }
            0x90 => {
                self.bit_reset(memory, 2, R8::B);
                8
            }
            0x91 => {
                self.bit_reset(memory, 2, R8::C);
                8
            }
            0x92 => {
                self.bit_reset(memory, 2, R8::D);
                8
            }
            0x93 => {
                self.bit_reset(memory, 2, R8::E);
                8
            }
            0x94 => {
                self.bit_reset(memory, 2, R8::H);
                8
            }
            0x95 => {
                self.bit_reset(memory, 2, R8::L);
                8
            }
            0x96 => {
                self.bit_reset(memory, 2, Addr::HL);
                16
            }
            0x97 => {
                self.bit_reset(memory, 2, R8::A);
                8
            }
            0x98 => {
                self.bit_reset(memory, 3, R8::B);
                8
            }
            0x99 => {
                self.bit_reset(memory, 3, R8::C);
                8
            }
            0x9A => {
                self.bit_reset(memory, 3, R8::D);
                8
            }
            0x9B => {
                self.bit_reset(memory, 3, R8::E);
                8
            }
            0x9C => {
                self.bit_reset(memory, 3, R8::H);
                8
            }
            0x9D => {
                self.bit_reset(memory, 3, R8::L);
                8
            }
            0x9E => {
                self.bit_reset(memory, 3, Addr::HL);
                16
            }
            0x9F => {
                self.bit_reset(memory, 3, R8::A);
                8
            }
            0xA0 => {
                self.bit_reset(memory, 4, R8::B);
                8
            }
            0xA1 => {
                self.bit_reset(memory, 4, R8::C);
                8
            }
            0xA2 => {
                self.bit_reset(memory, 4, R8::D);
                8
            }
            0xA3 => {
                self.bit_reset(memory, 4, R8::E);
                8
            }
            0xA4 => {
                self.bit_reset(memory, 4, R8::H);
                8
            }
            0xA5 => {
                self.bit_reset(memory, 4, R8::L);
                8
            }
            0xA6 => {
                self.bit_reset(memory, 4, Addr::HL);
                16
            }
            0xA7 => {
                self.bit_reset(memory, 4, R8::A);
                8
            }
            0xA8 => {
                self.bit_reset(memory, 5, R8::B);
                8
            }
            0xA9 => {
                self.bit_reset(memory, 5, R8::C);
                8
            }
            0xAA => {
                self.bit_reset(memory, 5, R8::D);
                8
            }
            0xAB => {
                self.bit_reset(memory, 5, R8::E);
                8
            }
            0xAC => {
                self.bit_reset(memory, 5, R8::H);
                8
            }
            0xAD => {
                self.bit_reset(memory, 5, R8::L);
                8
            }
            0xAE => {
                self.bit_reset(memory, 5, Addr::HL);
                16
            }
            0xAF => {
                self.bit_reset(memory, 5, R8::A);
                8
            }
            0xB0 => {
                self.bit_reset(memory, 6, R8::B);
                8
            }
            0xB1 => {
                self.bit_reset(memory, 6, R8::C);
                8
            }
            0xB2 => {
                self.bit_reset(memory, 6, R8::D);
                8
            }
            0xB3 => {
                self.bit_reset(memory, 6, R8::E);
                8
            }
            0xB4 => {
                self.bit_reset(memory, 6, R8::H);
                8
            }
            0xB5 => {
                self.bit_reset(memory, 6, R8::L);
                8
            }
            0xB6 => {
                self.bit_reset(memory, 6, Addr::HL);
                16
            }
            0xB7 => {
                self.bit_reset(memory, 6, R8::A);
                8
            }
            0xB8 => {
                self.bit_reset(memory, 7, R8::B);
                8
            }
            0xB9 => {
                self.bit_reset(memory, 7, R8::C);
                8
            }
            0xBA => {
                self.bit_reset(memory, 7, R8::D);
                8
            }
            0xBB => {
                self.bit_reset(memory, 7, R8::E);
                8
            }
            0xBC => {
                self.bit_reset(memory, 7, R8::H);
                8
            }
            0xBD => {
                self.bit_reset(memory, 7, R8::L);
                8
            }
            0xBE => {
                self.bit_reset(memory, 7, Addr::HL);
                16
            }
            0xBF => {
                self.bit_reset(memory, 7, R8::A);
                8
            }
            // SET
            0xC0 => {
                self.bit_set(memory, 0, R8::B);
                8
            }
            0xC1 => {
                self.bit_set(memory, 0, R8::C);
                8
            }
            0xC2 => {
                self.bit_set(memory, 0, R8::D);
                8
            }
            0xC3 => {
                self.bit_set(memory, 0, R8::E);
                8
            }
            0xC4 => {
                self.bit_set(memory, 0, R8::H);
                8
            }
            0xC5 => {
                self.bit_set(memory, 0, R8::L);
                8
            }
            0xC6 => {
                self.bit_set(memory, 0, Addr::HL);
                16
            }
            0xC7 => {
                self.bit_set(memory, 0, R8::A);
                8
            }
            0xC8 => {
                self.bit_set(memory, 1, R8::B);
                8
            }
            0xC9 => {
                self.bit_set(memory, 1, R8::C);
                8
            }
            0xCA => {
                self.bit_set(memory, 1, R8::D);
                8
            }
            0xCB => {
                self.bit_set(memory, 1, R8::E);
                8
            }
            0xCC => {
                self.bit_set(memory, 1, R8::H);
                8
            }
            0xCD => {
                self.bit_set(memory, 1, R8::L);
                8
            }
            0xCE => {
                self.bit_set(memory, 1, Addr::HL);
                16
            }
            0xCF => {
                self.bit_set(memory, 1, R8::A);
                8
            }
            0xD0 => {
                self.bit_set(memory, 2, R8::B);
                8
            }
            0xD1 => {
                self.bit_set(memory, 2, R8::C);
                8
            }
            0xD2 => {
                self.bit_set(memory, 2, R8::D);
                8
            }
            0xD3 => {
                self.bit_set(memory, 2, R8::E);
                8
            }
            0xD4 => {
                self.bit_set(memory, 2, R8::H);
                8
            }
            0xD5 => {
                self.bit_set(memory, 2, R8::L);
                8
            }
            0xD6 => {
                self.bit_set(memory, 2, Addr::HL);
                16
            }
            0xD7 => {
                self.bit_set(memory, 2, R8::A);
                8
            }
            0xD8 => {
                self.bit_set(memory, 3, R8::B);
                8
            }
            0xD9 => {
                self.bit_set(memory, 3, R8::C);
                8
            }
            0xDA => {
                self.bit_set(memory, 3, R8::D);
                8
            }
            0xDB => {
                self.bit_set(memory, 3, R8::E);
                8
            }
            0xDC => {
                self.bit_set(memory, 3, R8::H);
                8
            }
            0xDD => {
                self.bit_set(memory, 3, R8::L);
                8
            }
            0xDE => {
                self.bit_set(memory, 3, Addr::HL);
                16
            }
            0xDF => {
                self.bit_set(memory, 3, R8::A);
                8
            }
            0xE0 => {
                self.bit_set(memory, 4, R8::B);
                8
            }
            0xE1 => {
                self.bit_set(memory, 4, R8::C);
                8
            }
            0xE2 => {
                self.bit_set(memory, 4, R8::D);
                8
            }
            0xE3 => {
                self.bit_set(memory, 4, R8::E);
                8
            }
            0xE4 => {
                self.bit_set(memory, 4, R8::H);
                8
            }
            0xE5 => {
                self.bit_set(memory, 4, R8::L);
                8
            }
            0xE6 => {
                self.bit_set(memory, 4, Addr::HL);
                16
            }
            0xE7 => {
                self.bit_set(memory, 4, R8::A);
                8
            }
            0xE8 => {
                self.bit_set(memory, 5, R8::B);
                8
            }
            0xE9 => {
                self.bit_set(memory, 5, R8::C);
                8
            }
            0xEA => {
                self.bit_set(memory, 5, R8::D);
                8
            }
            0xEB => {
                self.bit_set(memory, 5, R8::E);
                8
            }
            0xEC => {
                self.bit_set(memory, 5, R8::H);
                8
            }
            0xED => {
                self.bit_set(memory, 5, R8::L);
                8
            }
            0xEE => {
                self.bit_set(memory, 5, Addr::HL);
                16
            }
            0xEF => {
                self.bit_set(memory, 5, R8::A);
                8
            }
            0xF0 => {
                self.bit_set(memory, 6, R8::B);
                8
            }
            0xF1 => {
                self.bit_set(memory, 6, R8::C);
                8
            }
            0xF2 => {
                self.bit_set(memory, 6, R8::D);
                8
            }
            0xF3 => {
                self.bit_set(memory, 6, R8::E);
                8
            }
            0xF4 => {
                self.bit_set(memory, 6, R8::H);
                8
            }
            0xF5 => {
                self.bit_set(memory, 6, R8::L);
                8
            }
            0xF6 => {
                self.bit_set(memory, 6, Addr::HL);
                16
            }
            0xF7 => {
                self.bit_set(memory, 6, R8::A);
                8
            }
            0xF8 => {
                self.bit_set(memory, 7, R8::B);
                8
            }
            0xF9 => {
                self.bit_set(memory, 7, R8::C);
                8
            }
            0xFA => {
                self.bit_set(memory, 7, R8::D);
                8
            }
            0xFB => {
                self.bit_set(memory, 7, R8::E);
                8
            }
            0xFC => {
                self.bit_set(memory, 7, R8::H);
                8
            }
            0xFD => {
                self.bit_set(memory, 7, R8::L);
                8
            }
            0xFE => {
                self.bit_set(memory, 7, Addr::HL);
                16
            }
            0xFF => {
                self.bit_set(memory, 7, R8::A);
                8
            }
        }
    }
}
