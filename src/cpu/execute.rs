use crate::cpu::{
    Cpu, Decrement, Direct, HighIndexed, Immediate, Increment, JumpCondition,
    Register16::{AF, BC, DE, HL, SP},
    Register8::{A, B, C, D, E, H, L},
};
use crate::hardware::AddressBus;

impl Cpu {
    pub(crate) fn execute(&mut self, bus: &mut AddressBus, opcode: u8) -> usize {
        match opcode {
            // ---- 8-bit Arithmetic
            // ADD
            0x87 => {
                self.add(bus, A);
                4
            }
            0x80 => {
                self.add(bus, B);
                4
            }
            0x81 => {
                self.add(bus, C);
                4
            }
            0x82 => {
                self.add(bus, D);
                4
            }
            0x83 => {
                self.add(bus, E);
                4
            }
            0x84 => {
                self.add(bus, H);
                4
            }
            0x85 => {
                self.add(bus, L);
                4
            }
            0x86 => {
                self.add(bus, Direct(HL));
                8
            }
            0xC6 => {
                self.add(bus, Immediate);
                8
            }
            // ADC
            0x8F => {
                self.add_with_carry(bus, A);
                4
            }
            0x88 => {
                self.add_with_carry(bus, B);
                4
            }
            0x89 => {
                self.add_with_carry(bus, C);
                4
            }
            0x8A => {
                self.add_with_carry(bus, D);
                4
            }
            0x8B => {
                self.add_with_carry(bus, E);
                4
            }
            0x8C => {
                self.add_with_carry(bus, H);
                4
            }
            0x8D => {
                self.add_with_carry(bus, L);
                4
            }
            0x8E => {
                self.add_with_carry(bus, Direct(HL));
                8
            }
            0xCE => {
                self.add_with_carry(bus, Immediate);
                8
            }
            // SUB
            0x97 => {
                self.subtract(bus, A);
                4
            }
            0x90 => {
                self.subtract(bus, B);
                4
            }
            0x91 => {
                self.subtract(bus, C);
                4
            }
            0x92 => {
                self.subtract(bus, D);
                4
            }
            0x93 => {
                self.subtract(bus, E);
                4
            }
            0x94 => {
                self.subtract(bus, H);
                4
            }
            0x95 => {
                self.subtract(bus, L);
                4
            }
            0x96 => {
                self.subtract(bus, Direct(HL));
                8
            }
            0xD6 => {
                self.subtract(bus, Immediate);
                8
            }
            // SBC
            0x9F => {
                self.subtract_with_carry(bus, A);
                4
            }
            0x98 => {
                self.subtract_with_carry(bus, B);
                4
            }
            0x99 => {
                self.subtract_with_carry(bus, C);
                4
            }
            0x9A => {
                self.subtract_with_carry(bus, D);
                4
            }
            0x9B => {
                self.subtract_with_carry(bus, E);
                4
            }
            0x9C => {
                self.subtract_with_carry(bus, H);
                4
            }
            0x9D => {
                self.subtract_with_carry(bus, L);
                4
            }
            0x9E => {
                self.subtract_with_carry(bus, Direct(HL));
                8
            }
            0xDE => {
                self.subtract_with_carry(bus, Immediate);
                8
            }
            // AND
            0xA7 => {
                self.and(bus, A);
                4
            }
            0xA0 => {
                self.and(bus, B);
                4
            }
            0xA1 => {
                self.and(bus, C);
                4
            }
            0xA2 => {
                self.and(bus, D);
                4
            }
            0xA3 => {
                self.and(bus, E);
                4
            }
            0xA4 => {
                self.and(bus, H);
                4
            }
            0xA5 => {
                self.and(bus, L);
                4
            }
            0xA6 => {
                self.and(bus, Direct(HL));
                8
            }
            0xE6 => {
                self.and(bus, Immediate);
                8
            }
            // XOR
            0xAF => {
                self.xor(bus, A);
                4
            }
            0xA8 => {
                self.xor(bus, B);
                4
            }
            0xA9 => {
                self.xor(bus, C);
                4
            }
            0xAA => {
                self.xor(bus, D);
                4
            }
            0xAB => {
                self.xor(bus, E);
                4
            }
            0xAC => {
                self.xor(bus, H);
                4
            }
            0xAD => {
                self.xor(bus, L);
                4
            }
            0xAE => {
                self.xor(bus, Direct(HL));
                8
            }
            0xEE => {
                self.xor(bus, Immediate);
                8
            }
            // OR
            0xB7 => {
                self.or(bus, A);
                4
            }
            0xB0 => {
                self.or(bus, B);
                4
            }
            0xB1 => {
                self.or(bus, C);
                4
            }
            0xB2 => {
                self.or(bus, D);
                4
            }
            0xB3 => {
                self.or(bus, E);
                4
            }
            0xB4 => {
                self.or(bus, H);
                4
            }
            0xB5 => {
                self.or(bus, L);
                4
            }
            0xB6 => {
                self.or(bus, Direct(HL));
                8
            }
            0xF6 => {
                self.or(bus, Immediate);
                8
            }
            // CP
            0xBF => {
                self.compare(bus, A);
                4
            }
            0xB8 => {
                self.compare(bus, B);
                4
            }
            0xB9 => {
                self.compare(bus, C);
                4
            }
            0xBA => {
                self.compare(bus, D);
                4
            }
            0xBB => {
                self.compare(bus, E);
                4
            }
            0xBC => {
                self.compare(bus, H);
                4
            }
            0xBD => {
                self.compare(bus, L);
                4
            }
            0xBE => {
                self.compare(bus, Direct(HL));
                8
            }
            0xFE => {
                self.compare(bus, Immediate);
                8
            }
            // INC
            0x3C => {
                self.increment(bus, A);
                4
            }
            0x04 => {
                self.increment(bus, B);
                4
            }
            0x0C => {
                self.increment(bus, C);
                4
            }
            0x14 => {
                self.increment(bus, D);
                4
            }
            0x1C => {
                self.increment(bus, E);
                4
            }
            0x24 => {
                self.increment(bus, H);
                4
            }
            0x2C => {
                self.increment(bus, L);
                4
            }
            0x34 => {
                self.increment(bus, Direct(HL));
                12
            }
            // DEC
            0x3D => {
                self.decrement(bus, A);
                4
            }
            0x05 => {
                self.decrement(bus, B);
                4
            }
            0x0D => {
                self.decrement(bus, C);
                4
            }
            0x15 => {
                self.decrement(bus, D);
                4
            }
            0x1D => {
                self.decrement(bus, E);
                4
            }
            0x25 => {
                self.decrement(bus, H);
                4
            }
            0x2D => {
                self.decrement(bus, L);
                4
            }
            0x35 => {
                self.decrement(bus, Direct(HL));
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
                self.add16_hl(BC);
                8
            }
            0x19 => {
                self.add16_hl(DE);
                8
            }
            0x29 => {
                self.add16_hl(HL);
                8
            }
            0x39 => {
                self.add16_hl(SP);
                8
            }
            0xE8 => {
                self.add16_sp(bus);
                16
            }
            // INC
            0x03 => {
                self.increment16(BC);
                8
            }
            0x13 => {
                self.increment16(DE);
                8
            }
            0x23 => {
                self.increment16(HL);
                8
            }
            0x33 => {
                self.increment16(SP);
                8
            }
            // DEC
            0x0B => {
                self.decrement16(BC);
                8
            }
            0x1B => {
                self.decrement16(DE);
                8
            }
            0x2B => {
                self.decrement16(HL);
                8
            }
            0x3B => {
                self.decrement16(SP);
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
                self.load(bus, B, A);
                4
            }
            0x40 => {
                self.load(bus, B, B);
                4
            }
            0x41 => {
                self.load(bus, B, C);
                4
            }
            0x42 => {
                self.load(bus, B, D);
                4
            }
            0x43 => {
                self.load(bus, B, E);
                4
            }
            0x44 => {
                self.load(bus, B, H);
                4
            }
            0x45 => {
                self.load(bus, B, L);
                4
            }
            0x46 => {
                self.load(bus, B, Direct(HL));
                8
            }
            0x06 => {
                self.load(bus, B, Immediate);
                8
            }
            0x4F => {
                self.load(bus, C, A);
                4
            }
            0x48 => {
                self.load(bus, C, B);
                4
            }
            0x49 => {
                self.load(bus, C, C);
                4
            }
            0x4A => {
                self.load(bus, C, D);
                4
            }
            0x4B => {
                self.load(bus, C, E);
                4
            }
            0x4C => {
                self.load(bus, C, H);
                4
            }
            0x4D => {
                self.load(bus, C, L);
                4
            }
            0x4E => {
                self.load(bus, C, Direct(HL));
                8
            }
            0x0E => {
                self.load(bus, C, Immediate);
                8
            }
            0x57 => {
                self.load(bus, D, A);
                4
            }
            0x50 => {
                self.load(bus, D, B);
                4
            }
            0x51 => {
                self.load(bus, D, C);
                4
            }
            0x52 => {
                self.load(bus, D, D);
                4
            }
            0x53 => {
                self.load(bus, D, E);
                4
            }
            0x54 => {
                self.load(bus, D, H);
                4
            }
            0x55 => {
                self.load(bus, D, L);
                4
            }
            0x56 => {
                self.load(bus, D, Direct(HL));
                8
            }
            0x16 => {
                self.load(bus, D, Immediate);
                8
            }
            0x5F => {
                self.load(bus, E, A);
                4
            }
            0x58 => {
                self.load(bus, E, B);
                4
            }
            0x59 => {
                self.load(bus, E, C);
                4
            }
            0x5A => {
                self.load(bus, E, D);
                4
            }
            0x5B => {
                self.load(bus, E, E);
                4
            }
            0x5C => {
                self.load(bus, E, H);
                4
            }
            0x5D => {
                self.load(bus, E, L);
                4
            }
            0x5E => {
                self.load(bus, E, Direct(HL));
                8
            }
            0x1E => {
                self.load(bus, E, Immediate);
                8
            }
            0x67 => {
                self.load(bus, H, A);
                4
            }
            0x60 => {
                self.load(bus, H, B);
                4
            }
            0x61 => {
                self.load(bus, H, C);
                4
            }
            0x62 => {
                self.load(bus, H, D);
                4
            }
            0x63 => {
                self.load(bus, H, E);
                4
            }
            0x64 => {
                self.load(bus, H, H);
                4
            }
            0x65 => {
                self.load(bus, H, L);
                4
            }
            0x66 => {
                self.load(bus, H, Direct(HL));
                8
            }
            0x26 => {
                self.load(bus, H, Immediate);
                8
            }
            0x6F => {
                self.load(bus, L, A);
                4
            }
            0x68 => {
                self.load(bus, L, B);
                4
            }
            0x69 => {
                self.load(bus, L, C);
                4
            }
            0x6A => {
                self.load(bus, L, D);
                4
            }
            0x6B => {
                self.load(bus, L, E);
                4
            }
            0x6C => {
                self.load(bus, L, H);
                4
            }
            0x6D => {
                self.load(bus, L, L);
                4
            }
            0x6E => {
                self.load(bus, L, Direct(HL));
                8
            }
            0x2E => {
                self.load(bus, L, Immediate);
                8
            }
            0x77 => {
                self.load(bus, Direct(HL), A);
                8
            }
            0x70 => {
                self.load(bus, Direct(HL), B);
                8
            }
            0x71 => {
                self.load(bus, Direct(HL), C);
                8
            }
            0x72 => {
                self.load(bus, Direct(HL), D);
                8
            }
            0x73 => {
                self.load(bus, Direct(HL), E);
                8
            }
            0x74 => {
                self.load(bus, Direct(HL), H);
                8
            }
            0x75 => {
                self.load(bus, Direct(HL), L);
                8
            }
            0x36 => {
                self.load(bus, Direct(HL), Immediate);
                12
            }
            0x7F => {
                self.load(bus, A, A);
                4
            }
            0x78 => {
                self.load(bus, A, B);
                4
            }
            0x79 => {
                self.load(bus, A, C);
                4
            }
            0x7A => {
                self.load(bus, A, D);
                4
            }
            0x7B => {
                self.load(bus, A, E);
                4
            }
            0x7C => {
                self.load(bus, A, H);
                4
            }
            0x7D => {
                self.load(bus, A, L);
                4
            }
            0x7E => {
                self.load(bus, A, Direct(HL));
                8
            }
            0x3E => {
                self.load(bus, A, Immediate);
                8
            }
            0x02 => {
                self.load(bus, Direct(BC), A);
                8
            }
            0x12 => {
                self.load(bus, Direct(DE), A);
                8
            }
            0x22 => {
                self.load(bus, Direct(Increment(HL)), A);
                8
            }
            0x32 => {
                self.load(bus, Direct(Decrement(HL)), A);
                8
            }
            0x0A => {
                self.load(bus, A, Direct(BC));
                8
            }
            0x1A => {
                self.load(bus, A, Direct(DE));
                8
            }
            0x2A => {
                self.load(bus, A, Direct(Increment(HL)));
                8
            }
            0x3A => {
                self.load(bus, A, Direct(Decrement(HL)));
                8
            }
            0xEA => {
                self.load(bus, Direct(Immediate), A);
                16
            }
            0xFA => {
                self.load(bus, A, Direct(Immediate));
                16
            }
            // LDH
            0xE0 => {
                self.load(bus, Direct(HighIndexed(Immediate)), A);
                12
            }
            0xF0 => {
                self.load(bus, A, Direct(HighIndexed(Immediate)));
                12
            }
            0xE2 => {
                self.load(bus, Direct(HighIndexed(C)), A);
                8
            }
            0xF2 => {
                self.load(bus, A, Direct(HighIndexed(C)));
                8
            }
            // ---- 16-bit Load
            // LD
            0x01 => {
                self.load16(bus, BC, Immediate);
                12
            }
            0x11 => {
                self.load16(bus, DE, Immediate);
                12
            }
            0x21 => {
                self.load16(bus, HL, Immediate);
                12
            }
            0x31 => {
                self.load16(bus, SP, Immediate);
                12
            }
            0xF9 => {
                self.load16(bus, SP, HL);
                8
            }
            0x08 => {
                self.load16_a16_sp(bus);
                20
            }
            0xF8 => {
                self.load16_hl_sp(bus);
                12
            }
            // PUSH
            0xC5 => {
                self.push(bus, BC);
                16
            }
            0xD5 => {
                self.push(bus, DE);
                16
            }
            0xE5 => {
                self.push(bus, HL);
                16
            }
            0xF5 => {
                self.push(bus, AF);
                16
            }
            // POP
            0xC1 => {
                self.pop(bus, BC);
                12
            }
            0xD1 => {
                self.pop(bus, DE);
                12
            }
            0xE1 => {
                self.pop(bus, HL);
                12
            }
            0xF1 => {
                self.pop(bus, AF);
                12
            }
            // ---- Jumps
            // JP
            0xE9 => {
                self.jump_to_hl();
                4
            }
            0xC3 => self.jump(bus, JumpCondition::Always),
            0xC2 => self.jump(bus, JumpCondition::NotZero),
            0xCA => self.jump(bus, JumpCondition::Zero),
            0xD2 => self.jump(bus, JumpCondition::NotCarry),
            0xDA => self.jump(bus, JumpCondition::Carry),
            // JR
            0x18 => self.jump_relative(bus, JumpCondition::Always),
            0x20 => self.jump_relative(bus, JumpCondition::NotZero),
            0x28 => self.jump_relative(bus, JumpCondition::Zero),
            0x30 => self.jump_relative(bus, JumpCondition::NotCarry),
            0x38 => self.jump_relative(bus, JumpCondition::Carry),
            // CALL
            0xCD => self.call(bus, JumpCondition::Always),
            0xC4 => self.call(bus, JumpCondition::NotZero),
            0xCC => self.call(bus, JumpCondition::Zero),
            0xD4 => self.call(bus, JumpCondition::NotCarry),
            0xDC => self.call(bus, JumpCondition::Carry),
            // RET
            0xC9 => {
                self.return_(bus, JumpCondition::Always);
                16
            }
            0xC0 => self.return_(bus, JumpCondition::NotZero),
            0xC8 => self.return_(bus, JumpCondition::Zero),
            0xD0 => self.return_(bus, JumpCondition::NotCarry),
            0xD8 => self.return_(bus, JumpCondition::Carry),
            // RETI
            0xD9 => {
                self.return_from_interrupt_handler(bus);
                16
            }
            // RST
            0xC7 => {
                self.restart(bus, 0x00);
                16
            }
            0xCF => {
                self.restart(bus, 0x08);
                16
            }
            0xD7 => {
                self.restart(bus, 0x10);
                16
            }
            0xDF => {
                self.restart(bus, 0x18);
                16
            }
            0xE7 => {
                self.restart(bus, 0x20);
                16
            }
            0xEF => {
                self.restart(bus, 0x28);
                16
            }
            0xF7 => {
                self.restart(bus, 0x30);
                16
            }
            0xFF => {
                self.restart(bus, 0x38);
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
                self.stop(bus);
                4
            }
            // HALT
            0x76 => {
                self.halt();
                4
            }
            // PREFIX
            0xCB => {
                let next_opcode = self.read_next_byte(bus);
                self.execute_prefixed(bus, next_opcode)
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
            byte @ (0xD3 | 0xDB | 0xDD | 0xE3 | 0xE4 | 0xEB | 0xEC | 0xED | 0xF4 | 0xFC | 0xFD) => {
                panic!("Error: Trying to run undefined instruction {byte:#02X}");
            }
        }
    }

    fn execute_prefixed(&mut self, bus: &mut AddressBus, opcode: u8) -> usize {
        match opcode {
            // ---- Bit Shift
            // RLC
            0x00 => {
                self.rotate_left_circular(bus, B);
                8
            }
            0x01 => {
                self.rotate_left_circular(bus, C);
                8
            }
            0x02 => {
                self.rotate_left_circular(bus, D);
                8
            }
            0x03 => {
                self.rotate_left_circular(bus, E);
                8
            }
            0x04 => {
                self.rotate_left_circular(bus, H);
                8
            }
            0x05 => {
                self.rotate_left_circular(bus, L);
                8
            }
            0x06 => {
                self.rotate_left_circular(bus, Direct(HL));
                16
            }
            0x07 => {
                self.rotate_left_circular(bus, A);
                8
            }
            // RRC
            0x08 => {
                self.rotate_right_circular(bus, B);
                8
            }
            0x09 => {
                self.rotate_right_circular(bus, C);
                8
            }
            0x0A => {
                self.rotate_right_circular(bus, D);
                8
            }
            0x0B => {
                self.rotate_right_circular(bus, E);
                8
            }
            0x0C => {
                self.rotate_right_circular(bus, H);
                8
            }
            0x0D => {
                self.rotate_right_circular(bus, L);
                8
            }
            0x0E => {
                self.rotate_right_circular(bus, Direct(HL));
                16
            }
            0x0F => {
                self.rotate_right_circular(bus, A);
                8
            }
            // RL
            0x10 => {
                self.rotate_left(bus, B);
                8
            }
            0x11 => {
                self.rotate_left(bus, C);
                8
            }
            0x12 => {
                self.rotate_left(bus, D);
                8
            }
            0x13 => {
                self.rotate_left(bus, E);
                8
            }
            0x14 => {
                self.rotate_left(bus, H);
                8
            }
            0x15 => {
                self.rotate_left(bus, L);
                8
            }
            0x16 => {
                self.rotate_left(bus, Direct(HL));
                16
            }
            0x17 => {
                self.rotate_left(bus, A);
                8
            }
            // RR
            0x18 => {
                self.rotate_right(bus, B);
                8
            }
            0x19 => {
                self.rotate_right(bus, C);
                8
            }
            0x1A => {
                self.rotate_right(bus, D);
                8
            }
            0x1B => {
                self.rotate_right(bus, E);
                8
            }
            0x1C => {
                self.rotate_right(bus, H);
                8
            }
            0x1D => {
                self.rotate_right(bus, L);
                8
            }
            0x1E => {
                self.rotate_right(bus, Direct(HL));
                16
            }
            0x1F => {
                self.rotate_right(bus, A);
                8
            }
            // SLA
            0x20 => {
                self.shift_left_arithmetic(bus, B);
                8
            }
            0x21 => {
                self.shift_left_arithmetic(bus, C);
                8
            }
            0x22 => {
                self.shift_left_arithmetic(bus, D);
                8
            }
            0x23 => {
                self.shift_left_arithmetic(bus, E);
                8
            }
            0x24 => {
                self.shift_left_arithmetic(bus, H);
                8
            }
            0x25 => {
                self.shift_left_arithmetic(bus, L);
                8
            }
            0x26 => {
                self.shift_left_arithmetic(bus, Direct(HL));
                16
            }
            0x27 => {
                self.shift_left_arithmetic(bus, A);
                8
            }
            // SRA
            0x28 => {
                self.shift_right_arithmetic(bus, B);
                8
            }
            0x29 => {
                self.shift_right_arithmetic(bus, C);
                8
            }
            0x2A => {
                self.shift_right_arithmetic(bus, D);
                8
            }
            0x2B => {
                self.shift_right_arithmetic(bus, E);
                8
            }
            0x2C => {
                self.shift_right_arithmetic(bus, H);
                8
            }
            0x2D => {
                self.shift_right_arithmetic(bus, L);
                8
            }
            0x2E => {
                self.shift_right_arithmetic(bus, Direct(HL));
                16
            }
            0x2F => {
                self.shift_right_arithmetic(bus, A);
                8
            }
            // SWAP
            0x30 => {
                self.swap(bus, B);
                8
            }
            0x31 => {
                self.swap(bus, C);
                8
            }
            0x32 => {
                self.swap(bus, D);
                8
            }
            0x33 => {
                self.swap(bus, E);
                8
            }
            0x34 => {
                self.swap(bus, H);
                8
            }
            0x35 => {
                self.swap(bus, L);
                8
            }
            0x36 => {
                self.swap(bus, Direct(HL));
                16
            }
            0x37 => {
                self.swap(bus, A);
                8
            }
            // SRL
            0x38 => {
                self.shift_right_logical(bus, B);
                8
            }
            0x39 => {
                self.shift_right_logical(bus, C);
                8
            }
            0x3A => {
                self.shift_right_logical(bus, D);
                8
            }
            0x3B => {
                self.shift_right_logical(bus, E);
                8
            }
            0x3C => {
                self.shift_right_logical(bus, H);
                8
            }
            0x3D => {
                self.shift_right_logical(bus, L);
                8
            }
            0x3E => {
                self.shift_right_logical(bus, Direct(HL));
                16
            }
            0x3F => {
                self.shift_right_logical(bus, A);
                8
            }
            // ---- Bit Operations
            // BIT
            0x40 => {
                self.bit_test(bus, 0, B);
                8
            }
            0x41 => {
                self.bit_test(bus, 0, C);
                8
            }
            0x42 => {
                self.bit_test(bus, 0, D);
                8
            }
            0x43 => {
                self.bit_test(bus, 0, E);
                8
            }
            0x44 => {
                self.bit_test(bus, 0, H);
                8
            }
            0x45 => {
                self.bit_test(bus, 0, L);
                8
            }
            0x46 => {
                self.bit_test(bus, 0, Direct(HL));
                12
            }
            0x47 => {
                self.bit_test(bus, 0, A);
                8
            }
            0x48 => {
                self.bit_test(bus, 1, B);
                8
            }
            0x49 => {
                self.bit_test(bus, 1, C);
                8
            }
            0x4A => {
                self.bit_test(bus, 1, D);
                8
            }
            0x4B => {
                self.bit_test(bus, 1, E);
                8
            }
            0x4C => {
                self.bit_test(bus, 1, H);
                8
            }
            0x4D => {
                self.bit_test(bus, 1, L);
                8
            }
            0x4E => {
                self.bit_test(bus, 1, Direct(HL));
                12
            }
            0x4F => {
                self.bit_test(bus, 1, A);
                8
            }
            0x50 => {
                self.bit_test(bus, 2, B);
                8
            }
            0x51 => {
                self.bit_test(bus, 2, C);
                8
            }
            0x52 => {
                self.bit_test(bus, 2, D);
                8
            }
            0x53 => {
                self.bit_test(bus, 2, E);
                8
            }
            0x54 => {
                self.bit_test(bus, 2, H);
                8
            }
            0x55 => {
                self.bit_test(bus, 2, L);
                8
            }
            0x56 => {
                self.bit_test(bus, 2, Direct(HL));
                12
            }
            0x57 => {
                self.bit_test(bus, 2, A);
                8
            }
            0x58 => {
                self.bit_test(bus, 3, B);
                8
            }
            0x59 => {
                self.bit_test(bus, 3, C);
                8
            }
            0x5A => {
                self.bit_test(bus, 3, D);
                8
            }
            0x5B => {
                self.bit_test(bus, 3, E);
                8
            }
            0x5C => {
                self.bit_test(bus, 3, H);
                8
            }
            0x5D => {
                self.bit_test(bus, 3, L);
                8
            }
            0x5E => {
                self.bit_test(bus, 3, Direct(HL));
                12
            }
            0x5F => {
                self.bit_test(bus, 3, A);
                8
            }
            0x60 => {
                self.bit_test(bus, 4, B);
                8
            }
            0x61 => {
                self.bit_test(bus, 4, C);
                8
            }
            0x62 => {
                self.bit_test(bus, 4, D);
                8
            }
            0x63 => {
                self.bit_test(bus, 4, E);
                8
            }
            0x64 => {
                self.bit_test(bus, 4, H);
                8
            }
            0x65 => {
                self.bit_test(bus, 4, L);
                8
            }
            0x66 => {
                self.bit_test(bus, 4, Direct(HL));
                12
            }
            0x67 => {
                self.bit_test(bus, 4, A);
                8
            }
            0x68 => {
                self.bit_test(bus, 5, B);
                8
            }
            0x69 => {
                self.bit_test(bus, 5, C);
                8
            }
            0x6A => {
                self.bit_test(bus, 5, D);
                8
            }
            0x6B => {
                self.bit_test(bus, 5, E);
                8
            }
            0x6C => {
                self.bit_test(bus, 5, H);
                8
            }
            0x6D => {
                self.bit_test(bus, 5, L);
                8
            }
            0x6E => {
                self.bit_test(bus, 5, Direct(HL));
                12
            }
            0x6F => {
                self.bit_test(bus, 5, A);
                8
            }
            0x70 => {
                self.bit_test(bus, 6, B);
                8
            }
            0x71 => {
                self.bit_test(bus, 6, C);
                8
            }
            0x72 => {
                self.bit_test(bus, 6, D);
                8
            }
            0x73 => {
                self.bit_test(bus, 6, E);
                8
            }
            0x74 => {
                self.bit_test(bus, 6, H);
                8
            }
            0x75 => {
                self.bit_test(bus, 6, L);
                8
            }
            0x76 => {
                self.bit_test(bus, 6, Direct(HL));
                12
            }
            0x77 => {
                self.bit_test(bus, 6, A);
                8
            }
            0x78 => {
                self.bit_test(bus, 7, B);
                8
            }
            0x79 => {
                self.bit_test(bus, 7, C);
                8
            }
            0x7A => {
                self.bit_test(bus, 7, D);
                8
            }
            0x7B => {
                self.bit_test(bus, 7, E);
                8
            }
            0x7C => {
                self.bit_test(bus, 7, H);
                8
            }
            0x7D => {
                self.bit_test(bus, 7, L);
                8
            }
            0x7E => {
                self.bit_test(bus, 7, Direct(HL));
                12
            }
            0x7F => {
                self.bit_test(bus, 7, A);
                8
            }
            // RES
            0x80 => {
                self.bit_reset(bus, 0, B);
                8
            }
            0x81 => {
                self.bit_reset(bus, 0, C);
                8
            }
            0x82 => {
                self.bit_reset(bus, 0, D);
                8
            }
            0x83 => {
                self.bit_reset(bus, 0, E);
                8
            }
            0x84 => {
                self.bit_reset(bus, 0, H);
                8
            }
            0x85 => {
                self.bit_reset(bus, 0, L);
                8
            }
            0x86 => {
                self.bit_reset(bus, 0, Direct(HL));
                16
            }
            0x87 => {
                self.bit_reset(bus, 0, A);
                8
            }
            0x88 => {
                self.bit_reset(bus, 1, B);
                8
            }
            0x89 => {
                self.bit_reset(bus, 1, C);
                8
            }
            0x8A => {
                self.bit_reset(bus, 1, D);
                8
            }
            0x8B => {
                self.bit_reset(bus, 1, E);
                8
            }
            0x8C => {
                self.bit_reset(bus, 1, H);
                8
            }
            0x8D => {
                self.bit_reset(bus, 1, L);
                8
            }
            0x8E => {
                self.bit_reset(bus, 1, Direct(HL));
                16
            }
            0x8F => {
                self.bit_reset(bus, 1, A);
                8
            }
            0x90 => {
                self.bit_reset(bus, 2, B);
                8
            }
            0x91 => {
                self.bit_reset(bus, 2, C);
                8
            }
            0x92 => {
                self.bit_reset(bus, 2, D);
                8
            }
            0x93 => {
                self.bit_reset(bus, 2, E);
                8
            }
            0x94 => {
                self.bit_reset(bus, 2, H);
                8
            }
            0x95 => {
                self.bit_reset(bus, 2, L);
                8
            }
            0x96 => {
                self.bit_reset(bus, 2, Direct(HL));
                16
            }
            0x97 => {
                self.bit_reset(bus, 2, A);
                8
            }
            0x98 => {
                self.bit_reset(bus, 3, B);
                8
            }
            0x99 => {
                self.bit_reset(bus, 3, C);
                8
            }
            0x9A => {
                self.bit_reset(bus, 3, D);
                8
            }
            0x9B => {
                self.bit_reset(bus, 3, E);
                8
            }
            0x9C => {
                self.bit_reset(bus, 3, H);
                8
            }
            0x9D => {
                self.bit_reset(bus, 3, L);
                8
            }
            0x9E => {
                self.bit_reset(bus, 3, Direct(HL));
                16
            }
            0x9F => {
                self.bit_reset(bus, 3, A);
                8
            }
            0xA0 => {
                self.bit_reset(bus, 4, B);
                8
            }
            0xA1 => {
                self.bit_reset(bus, 4, C);
                8
            }
            0xA2 => {
                self.bit_reset(bus, 4, D);
                8
            }
            0xA3 => {
                self.bit_reset(bus, 4, E);
                8
            }
            0xA4 => {
                self.bit_reset(bus, 4, H);
                8
            }
            0xA5 => {
                self.bit_reset(bus, 4, L);
                8
            }
            0xA6 => {
                self.bit_reset(bus, 4, Direct(HL));
                16
            }
            0xA7 => {
                self.bit_reset(bus, 4, A);
                8
            }
            0xA8 => {
                self.bit_reset(bus, 5, B);
                8
            }
            0xA9 => {
                self.bit_reset(bus, 5, C);
                8
            }
            0xAA => {
                self.bit_reset(bus, 5, D);
                8
            }
            0xAB => {
                self.bit_reset(bus, 5, E);
                8
            }
            0xAC => {
                self.bit_reset(bus, 5, H);
                8
            }
            0xAD => {
                self.bit_reset(bus, 5, L);
                8
            }
            0xAE => {
                self.bit_reset(bus, 5, Direct(HL));
                16
            }
            0xAF => {
                self.bit_reset(bus, 5, A);
                8
            }
            0xB0 => {
                self.bit_reset(bus, 6, B);
                8
            }
            0xB1 => {
                self.bit_reset(bus, 6, C);
                8
            }
            0xB2 => {
                self.bit_reset(bus, 6, D);
                8
            }
            0xB3 => {
                self.bit_reset(bus, 6, E);
                8
            }
            0xB4 => {
                self.bit_reset(bus, 6, H);
                8
            }
            0xB5 => {
                self.bit_reset(bus, 6, L);
                8
            }
            0xB6 => {
                self.bit_reset(bus, 6, Direct(HL));
                16
            }
            0xB7 => {
                self.bit_reset(bus, 6, A);
                8
            }
            0xB8 => {
                self.bit_reset(bus, 7, B);
                8
            }
            0xB9 => {
                self.bit_reset(bus, 7, C);
                8
            }
            0xBA => {
                self.bit_reset(bus, 7, D);
                8
            }
            0xBB => {
                self.bit_reset(bus, 7, E);
                8
            }
            0xBC => {
                self.bit_reset(bus, 7, H);
                8
            }
            0xBD => {
                self.bit_reset(bus, 7, L);
                8
            }
            0xBE => {
                self.bit_reset(bus, 7, Direct(HL));
                16
            }
            0xBF => {
                self.bit_reset(bus, 7, A);
                8
            }
            // SET
            0xC0 => {
                self.bit_set(bus, 0, B);
                8
            }
            0xC1 => {
                self.bit_set(bus, 0, C);
                8
            }
            0xC2 => {
                self.bit_set(bus, 0, D);
                8
            }
            0xC3 => {
                self.bit_set(bus, 0, E);
                8
            }
            0xC4 => {
                self.bit_set(bus, 0, H);
                8
            }
            0xC5 => {
                self.bit_set(bus, 0, L);
                8
            }
            0xC6 => {
                self.bit_set(bus, 0, Direct(HL));
                16
            }
            0xC7 => {
                self.bit_set(bus, 0, A);
                8
            }
            0xC8 => {
                self.bit_set(bus, 1, B);
                8
            }
            0xC9 => {
                self.bit_set(bus, 1, C);
                8
            }
            0xCA => {
                self.bit_set(bus, 1, D);
                8
            }
            0xCB => {
                self.bit_set(bus, 1, E);
                8
            }
            0xCC => {
                self.bit_set(bus, 1, H);
                8
            }
            0xCD => {
                self.bit_set(bus, 1, L);
                8
            }
            0xCE => {
                self.bit_set(bus, 1, Direct(HL));
                16
            }
            0xCF => {
                self.bit_set(bus, 1, A);
                8
            }
            0xD0 => {
                self.bit_set(bus, 2, B);
                8
            }
            0xD1 => {
                self.bit_set(bus, 2, C);
                8
            }
            0xD2 => {
                self.bit_set(bus, 2, D);
                8
            }
            0xD3 => {
                self.bit_set(bus, 2, E);
                8
            }
            0xD4 => {
                self.bit_set(bus, 2, H);
                8
            }
            0xD5 => {
                self.bit_set(bus, 2, L);
                8
            }
            0xD6 => {
                self.bit_set(bus, 2, Direct(HL));
                16
            }
            0xD7 => {
                self.bit_set(bus, 2, A);
                8
            }
            0xD8 => {
                self.bit_set(bus, 3, B);
                8
            }
            0xD9 => {
                self.bit_set(bus, 3, C);
                8
            }
            0xDA => {
                self.bit_set(bus, 3, D);
                8
            }
            0xDB => {
                self.bit_set(bus, 3, E);
                8
            }
            0xDC => {
                self.bit_set(bus, 3, H);
                8
            }
            0xDD => {
                self.bit_set(bus, 3, L);
                8
            }
            0xDE => {
                self.bit_set(bus, 3, Direct(HL));
                16
            }
            0xDF => {
                self.bit_set(bus, 3, A);
                8
            }
            0xE0 => {
                self.bit_set(bus, 4, B);
                8
            }
            0xE1 => {
                self.bit_set(bus, 4, C);
                8
            }
            0xE2 => {
                self.bit_set(bus, 4, D);
                8
            }
            0xE3 => {
                self.bit_set(bus, 4, E);
                8
            }
            0xE4 => {
                self.bit_set(bus, 4, H);
                8
            }
            0xE5 => {
                self.bit_set(bus, 4, L);
                8
            }
            0xE6 => {
                self.bit_set(bus, 4, Direct(HL));
                16
            }
            0xE7 => {
                self.bit_set(bus, 4, A);
                8
            }
            0xE8 => {
                self.bit_set(bus, 5, B);
                8
            }
            0xE9 => {
                self.bit_set(bus, 5, C);
                8
            }
            0xEA => {
                self.bit_set(bus, 5, D);
                8
            }
            0xEB => {
                self.bit_set(bus, 5, E);
                8
            }
            0xEC => {
                self.bit_set(bus, 5, H);
                8
            }
            0xED => {
                self.bit_set(bus, 5, L);
                8
            }
            0xEE => {
                self.bit_set(bus, 5, Direct(HL));
                16
            }
            0xEF => {
                self.bit_set(bus, 5, A);
                8
            }
            0xF0 => {
                self.bit_set(bus, 6, B);
                8
            }
            0xF1 => {
                self.bit_set(bus, 6, C);
                8
            }
            0xF2 => {
                self.bit_set(bus, 6, D);
                8
            }
            0xF3 => {
                self.bit_set(bus, 6, E);
                8
            }
            0xF4 => {
                self.bit_set(bus, 6, H);
                8
            }
            0xF5 => {
                self.bit_set(bus, 6, L);
                8
            }
            0xF6 => {
                self.bit_set(bus, 6, Direct(HL));
                16
            }
            0xF7 => {
                self.bit_set(bus, 6, A);
                8
            }
            0xF8 => {
                self.bit_set(bus, 7, B);
                8
            }
            0xF9 => {
                self.bit_set(bus, 7, C);
                8
            }
            0xFA => {
                self.bit_set(bus, 7, D);
                8
            }
            0xFB => {
                self.bit_set(bus, 7, E);
                8
            }
            0xFC => {
                self.bit_set(bus, 7, H);
                8
            }
            0xFD => {
                self.bit_set(bus, 7, L);
                8
            }
            0xFE => {
                self.bit_set(bus, 7, Direct(HL));
                16
            }
            0xFF => {
                self.bit_set(bus, 7, A);
                8
            }
        }
    }
}
