use crate::cpu::{
    Cpu, Flag, Immediate, JumpCondition, ReadByte, ReadWord, Register16, WriteByte, WriteWord,
};
use crate::hardware::AddressBus;

impl Cpu {
    /// NOP
    /// 1 4
    /// - - - -
    ///
    /// Nothing happens.
    pub(crate) const fn no_operation() {}

    /// STOP n8
    /// 2 4
    /// - - - -
    ///
    /// Stop CPU & display until button pressed.
    pub(crate) fn stop() {
        // No licensed Game Boy games use this instruction
        // It's only used for speed switching Game Boy Color games
        unimplemented!("STOP instruction");
    }

    /// HALT
    /// 1 4
    /// - - - -
    ///
    /// Halt CPU until an interrupt occurs.
    pub(crate) fn halt(&mut self) {
        self.halted = true;
        // TODO: Look into halt bug
    }

    /// LD r8, r8
    /// 1 4
    /// - - - -
    ///
    /// LD
    ///
    /// Load src (right) and copy into dest (left).
    pub(crate) fn load<D, S>(&mut self, bus: &mut AddressBus, dest: D, src: S)
    where
        Self: ReadByte<S> + WriteByte<D>,
    {
        let value = self.read_byte(bus, src);
        self.write_byte(bus, dest, value);
    }

    /// LD r16, n16
    /// 3 12
    /// - - - -
    ///
    /// Load src (right) and copy into dest (left).
    pub(crate) fn load16<D, S>(&mut self, bus: &mut AddressBus, dest: D, src: S)
    where
        Self: ReadWord<S> + WriteWord<D>,
    {
        let value = self.read_word(bus, src);
        self.write_word(bus, dest, value);
    }

    /// LD HL, SP + e8
    /// 2 12
    /// 0 0 H C
    ///
    /// Add the signed value e8 to SP and store the result in HL.
    pub(crate) fn load16_hl_sp_e(&mut self, bus: &mut AddressBus) {
        let sp = self.sp;
        let offset = self.read_next_byte_signed(bus) as i16;

        self.f.set(Flag::Zero, false);
        self.f.set(Flag::Subtract, false);
        // Half-carry from bit 3, carry from bit 7
        // Bits are labeled from 0-15 from least to most significant.
        let half_carry = (sp & 0xF).wrapping_add_signed(offset & 0xF) > 0xF;
        self.f.set(Flag::HalfCarry, half_carry);
        let carry = (sp & 0xFF).wrapping_add_signed(offset & 0xFF) > 0xFF;
        self.f.set(Flag::Carry, carry);

        let new_value = sp.wrapping_add_signed(offset);
        self.write_word(bus, Register16::HL, new_value);
        bus.tick();
    }

    /// ADD A, r8
    /// 1 4
    /// Z 0 H C
    ///
    /// Add the value in r8 to register A.
    pub(crate) fn add<D, S>(&mut self, bus: &mut AddressBus, dest: D, src: S)
    where
        D: Copy,
        Self: ReadByte<S> + ReadByte<D> + WriteByte<D>,
    {
        let lhs = self.read_byte(bus, dest);
        let rhs = self.read_byte(bus, src);
        let (result, did_overflow) = lhs.overflowing_add(rhs);
        self.f.set(Flag::Zero, result == 0);
        self.f.set(Flag::Subtract, false);
        // Half-carry is set if adding the lower bits (0-3) of the value and register A
        // together results in overflowing to bit 4. If the result is larger than 0xF
        // than the addition caused a carry from bit 3 to bit 4.
        let half_carry = (lhs & 0xF) + (rhs & 0xF) > 0xF;
        self.f.set(Flag::HalfCarry, half_carry);
        self.f.set(Flag::Carry, did_overflow);
        self.write_byte(bus, dest, result);
    }

    /// ADC A, r8
    /// 1 4
    /// Z 0 H C
    ///
    /// Add the value in r8 plus the carry flag to register A.
    pub(crate) fn add_with_carry<D, S>(&mut self, bus: &mut AddressBus, dest: D, src: S)
    where
        D: Copy,
        Self: ReadByte<S> + ReadByte<D> + WriteByte<D>,
    {
        let lhs = self.read_byte(bus, dest);
        let rhs = self.read_byte(bus, src);
        let cf = self.f.contains(Flag::Carry) as u8;
        let result = lhs.wrapping_add(rhs).wrapping_add(cf);
        self.f.set(Flag::Zero, result == 0);
        self.f.set(Flag::Subtract, false);
        let half_carry = (lhs & 0xF) + (rhs & 0xF) + cf > 0xF;
        self.f.set(Flag::HalfCarry, half_carry);
        let carry = lhs as u16 + rhs as u16 + cf as u16 > 0xFF;
        self.f.set(Flag::Carry, carry);
        self.write_byte(bus, dest, result);
    }

    /// SUB A, r8
    /// 1 4
    /// Z 1 H C
    ///
    /// Subtract the value in r8 from register A.
    pub(crate) fn subtract<D, S>(&mut self, bus: &mut AddressBus, dest: D, src: S)
    where
        D: Copy,
        Self: ReadByte<S> + ReadByte<D> + WriteByte<D>,
    {
        let lhs = self.read_byte(bus, dest);
        let rhs = self.read_byte(bus, src);
        let (result, did_overflow) = lhs.overflowing_sub(rhs);
        self.f.set(Flag::Zero, result == 0);
        self.f.set(Flag::Subtract, true);
        let half_carry = (lhs & 0xF) < (rhs & 0xF);
        self.f.set(Flag::HalfCarry, half_carry);
        self.f.set(Flag::Carry, did_overflow);
        self.write_byte(bus, dest, result);
    }

    /// SBC A, r8
    /// 1 4
    /// Z 1 H C
    ///
    /// Subtract the value in r8 and the carry flag from register A.
    pub(crate) fn subtract_with_carry<D, S>(&mut self, bus: &mut AddressBus, dest: D, src: S)
    where
        D: Copy,
        Self: ReadByte<S> + ReadByte<D> + WriteByte<D>,
    {
        let lhs = self.read_byte(bus, dest);
        let rhs = self.read_byte(bus, src);
        let cf = self.f.contains(Flag::Carry) as u8;
        let result = lhs.wrapping_sub(rhs).wrapping_sub(cf);
        self.f.set(Flag::Zero, result == 0);
        self.f.set(Flag::Subtract, true);
        let half_carry = (lhs & 0xF) < (rhs & 0xF) + cf;
        self.f.set(Flag::HalfCarry, half_carry);
        let carry = (lhs as u16) < (rhs as u16) + (cf as u16);
        self.f.set(Flag::Carry, carry);
        self.write_byte(bus, dest, result);
    }

    /// AND A, r8
    /// 1 4
    /// Z 0 1 0
    ///
    /// Bitwise AND between the value in r8 and register A.
    pub(crate) fn and<D, S>(&mut self, bus: &mut AddressBus, dest: D, src: S)
    where
        D: Copy,
        Self: ReadByte<S> + ReadByte<D> + WriteByte<D>,
    {
        let lhs = self.read_byte(bus, dest);
        let rhs = self.read_byte(bus, src);
        let result = lhs & rhs;
        self.f.set(Flag::Zero, result == 0);
        self.f.set(Flag::Subtract, false);
        self.f.set(Flag::HalfCarry, true);
        self.f.set(Flag::Carry, false);
        self.write_byte(bus, dest, result);
    }

    /// XOR A, r8
    /// 1 4
    /// Z 0 0 0
    ///
    /// Bitwise XOR between the value in r8 and register A.
    pub(crate) fn xor<D, S>(&mut self, bus: &mut AddressBus, dest: D, src: S)
    where
        D: Copy,
        Self: ReadByte<S> + ReadByte<D> + WriteByte<D>,
    {
        let lhs = self.read_byte(bus, dest);
        let rhs = self.read_byte(bus, src);
        let result = lhs ^ rhs;
        self.f.set(Flag::Zero, result == 0);
        self.f.set(Flag::Subtract, false);
        self.f.set(Flag::HalfCarry, false);
        self.f.set(Flag::Carry, false);
        self.write_byte(bus, dest, result);
    }

    /// OR A, r8
    /// 1 4
    /// Z 0 0 0
    ///
    /// Bitwise OR between the value in r8 and register A.
    pub(crate) fn or<D, S>(&mut self, bus: &mut AddressBus, dest: D, src: S)
    where
        D: Copy,
        Self: ReadByte<S> + ReadByte<D> + WriteByte<D>,
    {
        let lhs = self.read_byte(bus, dest);
        let rhs = self.read_byte(bus, src);
        let result = lhs | rhs;
        self.f.set(Flag::Zero, result == 0);
        self.f.set(Flag::Subtract, false);
        self.f.set(Flag::HalfCarry, false);
        self.f.set(Flag::Carry, false);
        self.write_byte(bus, dest, result);
    }

    /// CP A, r8
    /// 1 4
    /// Z 1 H C
    ///
    /// Subtract the value in r8 from register A and set flags accordingly, but don't store the result.
    pub(crate) fn compare<D, S>(&mut self, bus: &mut AddressBus, dest: D, src: S)
    where
        Self: ReadByte<S> + ReadByte<D>,
    {
        let lhs = self.read_byte(bus, dest);
        let rhs = self.read_byte(bus, src);
        let (result, did_overflow) = lhs.overflowing_sub(rhs);
        self.f.set(Flag::Zero, result == 0);
        self.f.set(Flag::Subtract, true);
        let half_carry = (lhs & 0xF) < (rhs & 0xF);
        self.f.set(Flag::HalfCarry, half_carry);
        self.f.set(Flag::Carry, did_overflow);
    }

    /// INC r8
    /// 1 4
    /// Z 0 H -
    ///
    /// Increment value in register r8 by 1.
    pub(crate) fn increment<T>(&mut self, bus: &mut AddressBus, target: T)
    where
        T: Copy,
        Self: ReadByte<T> + WriteByte<T>,
    {
        let value = self.read_byte(bus, target);
        let result = value.wrapping_add(1);
        self.f.set(Flag::Zero, result == 0);
        self.f.set(Flag::Subtract, false);
        let half_carry = value & 0xF == 0xF;
        self.f.set(Flag::HalfCarry, half_carry);
        // CARRY is left untouched
        self.write_byte(bus, target, result);
    }

    /// DEC r8
    /// 1 4
    /// Z 1 H -
    ///
    /// Decrement value in register r8 by 1.
    pub(crate) fn decrement<T>(&mut self, bus: &mut AddressBus, target: T)
    where
        T: Copy,
        Self: ReadByte<T> + WriteByte<T>,
    {
        let value = self.read_byte(bus, target);
        let result = value.wrapping_sub(1);
        self.f.set(Flag::Zero, result == 0);
        self.f.set(Flag::Subtract, true);
        let half_carry = value & 0xF == 0;
        self.f.set(Flag::HalfCarry, half_carry);
        // CARRY is left untouched
        self.write_byte(bus, target, result);
    }

    /// ADD HL, r16
    /// 1 8
    /// - 0 H C
    ///
    /// Add the value in r16 to register HL.
    pub(crate) fn add16<D, S>(&mut self, bus: &mut AddressBus, dest: D, src: S)
    where
        D: Copy,
        Self: ReadWord<S> + ReadWord<D> + WriteWord<D>,
    {
        let lhs = self.read_word(bus, dest);
        let rhs = self.read_word(bus, src);
        let (result, did_overflow) = lhs.overflowing_add(rhs);
        // ZERO is left untouched
        self.f.set(Flag::Subtract, false);
        // Half-carry from bit 11, carry from bit 15
        // Bits are labeled from 0-15 from least to most significant.
        let half_carry = (lhs & 0xFFF) + (rhs & 0xFFF) > 0xFFF;
        self.f.set(Flag::HalfCarry, half_carry);
        self.f.set(Flag::Carry, did_overflow);
        self.write_word(bus, dest, result);
        bus.tick();
    }

    /// ADD SP, e8
    /// 2 16
    /// 0 0 H C
    ///
    /// Add the signed value e8 to SP.
    pub(crate) fn add16_sp_e(&mut self, bus: &mut AddressBus) {
        let sp = self.sp;
        let offset = self.read_next_byte_signed(bus) as i16;

        self.f.set(Flag::Zero, false);
        self.f.set(Flag::Subtract, false);
        // Half-carry from bit 3, carry from bit 7
        // Bits are labeled from 0-15 from least to most significant.
        let half_carry = (sp & 0xF).wrapping_add_signed(offset & 0xF) > 0xF;
        self.f.set(Flag::HalfCarry, half_carry);
        let carry = (sp & 0xFF).wrapping_add_signed(offset & 0xFF) > 0xFF;
        self.f.set(Flag::Carry, carry);
        let result = sp.wrapping_add_signed(offset);
        self.sp = result;
        bus.tick();
        bus.tick();
    }

    /// INC r16
    /// 1 8
    /// - - - -
    ///
    /// Increment value in register r16 by 1.
    pub(crate) fn increment16<T>(&mut self, bus: &mut AddressBus, target: T)
    where
        T: Copy,
        Self: ReadWord<T> + WriteWord<T>,
    {
        let value = self.read_word(bus, target);
        let result = value.wrapping_add(1);
        self.write_word(bus, target, result);
        bus.tick();
    }

    /// DEC r16
    /// 1 8
    /// - - - -
    ///
    /// Decrement value in register r16 by 1.
    pub(crate) fn decrement16<T>(&mut self, bus: &mut AddressBus, target: T)
    where
        T: Copy,
        Self: ReadWord<T> + WriteWord<T>,
    {
        let value = self.read_word(bus, target);
        let result = value.wrapping_sub(1);
        self.write_word(bus, target, result);
        bus.tick();
    }

    /// RLCA
    /// 1 4
    /// 0 0 0 C
    ///
    /// Rotate register A left.
    pub(crate) fn rotate_left_circular_accumulator(&mut self) {
        let value = self.a;
        let result = value.rotate_left(1);
        self.f.set(Flag::Zero, false);
        self.f.set(Flag::Subtract, false);
        self.f.set(Flag::HalfCarry, false);
        let carry = value & 0x80 != 0;
        self.f.set(Flag::Carry, carry);
        self.a = result;
    }

    /// RLA
    /// 1 4
    /// 0 0 0 C
    ///
    /// Rotate register A left, through the carry flag.
    pub(crate) fn rotate_left_accumulator(&mut self) {
        let value = self.a;
        let cf = self.f.contains(Flag::Carry) as u8;
        let result = (value << 1) | cf;
        self.f.set(Flag::Zero, false);
        self.f.set(Flag::Subtract, false);
        self.f.set(Flag::HalfCarry, false);
        let carry = value & 0x80 != 0;
        self.f.set(Flag::Carry, carry);
        self.a = result;
    }

    /// RRCA
    /// 1 4
    /// 0 0 0 C
    ///
    /// Rotate register A right.
    pub(crate) fn rotate_right_circular_accumulator(&mut self) {
        let value = self.a;
        let result = value.rotate_right(1);
        self.f.set(Flag::Zero, false);
        self.f.set(Flag::Subtract, false);
        self.f.set(Flag::HalfCarry, false);
        let carry = value & 0x01 != 0;
        self.f.set(Flag::Carry, carry);
        self.a = result;
    }

    /// RRA
    /// 1 4
    /// 0 0 0 C
    ///
    /// Rotate register A right, through the carry flag.
    pub(crate) fn rotate_right_accumulator(&mut self) {
        let value = self.a;
        let cf = self.f.contains(Flag::Carry) as u8;
        let result = (value >> 1) | (cf << 7);
        self.f.set(Flag::Zero, false);
        self.f.set(Flag::Subtract, false);
        self.f.set(Flag::HalfCarry, false);
        let carry = value & 0x01 != 0;
        self.f.set(Flag::Carry, carry);
        self.a = result;
    }

    /// SCF
    /// 1 4
    /// - 0 0 1
    ///
    /// Set the carry flag.
    pub(crate) fn set_carry_flag(&mut self) {
        // ZERO left untouched
        self.f.set(Flag::Subtract, false);
        self.f.set(Flag::HalfCarry, false);
        self.f.set(Flag::Carry, true);
    }

    /// CPL
    /// 1 4
    /// - 1 1 -
    ///
    /// Flip the bits in register A.
    pub(crate) fn complement_accumulator(&mut self) {
        let value = self.a;
        // ZERO left untouched
        self.f.set(Flag::Subtract, true);
        self.f.set(Flag::HalfCarry, true);
        // CARRY left untouched
        self.a = !value;
    }

    /// CCF
    /// 1 4
    /// - 0 0 C
    ///
    /// Complement the carry flag.
    pub(crate) fn complement_carry_flag(&mut self) {
        let cf = self.f.contains(Flag::Carry);
        // ZERO left untouched
        self.f.set(Flag::Subtract, false);
        self.f.set(Flag::HalfCarry, false);
        self.f.set(Flag::Carry, !cf);
    }

    /// DAA
    /// 1 4
    /// Z - 0 C
    ///
    /// Decimal Adjust register A to get a correct BCD representation after an arithmetic instruction.
    pub(crate) fn decimal_adjust_accumulator(&mut self) {
        let mut value = self.a;

        let nf = self.f.contains(Flag::Subtract);
        let hf = self.f.contains(Flag::HalfCarry);
        let mut cf = self.f.contains(Flag::Carry);

        if nf {
            // After a subtraction, only adjust if (half-)carry occurred
            if cf {
                value = value.wrapping_sub(0x60);
            }
            if hf {
                value = value.wrapping_sub(0x06);
            }
        } else {
            // After an addition, adjust if (half-)carry occurred or if out of bounds
            if cf || value > 0x99 {
                value = value.wrapping_add(0x60);
                cf = true;
            }
            if hf || (value & 0x0F) > 0x09 {
                value = value.wrapping_add(0x06);
            }
        }

        self.f.set(Flag::Zero, value == 0);
        // SUBTRACT left untouched
        self.f.set(Flag::HalfCarry, false);
        self.f.set(Flag::Carry, cf);

        self.a = value;
    }

    /// RLC r8
    /// 2 8
    /// Z 0 0 C
    ///
    /// Rotate register r8 left.
    pub(crate) fn rotate_left_circular<S>(&mut self, bus: &mut AddressBus, src: S)
    where
        S: Copy,
        Self: ReadByte<S> + WriteByte<S>,
    {
        let value = self.read_byte(bus, src);
        let result = value.rotate_left(1);
        self.f.set(Flag::Zero, result == 0);
        self.f.set(Flag::Subtract, false);
        self.f.set(Flag::HalfCarry, false);
        let carry = value & 0x80 != 0;
        self.f.set(Flag::Carry, carry);
        self.write_byte(bus, src, result);
    }

    /// RRC r8
    /// 2 8
    /// Z 0 0 C
    ///
    /// Rotate register r8 right.
    pub(crate) fn rotate_right_circular<S>(&mut self, bus: &mut AddressBus, src: S)
    where
        S: Copy,
        Self: ReadByte<S> + WriteByte<S>,
    {
        let value = self.read_byte(bus, src);
        let result = value.rotate_right(1);
        self.f.set(Flag::Zero, result == 0);
        self.f.set(Flag::Subtract, false);
        self.f.set(Flag::HalfCarry, false);
        let carry = value & 0x01 != 0;
        self.f.set(Flag::Carry, carry);
        self.write_byte(bus, src, result);
    }

    /// RL r8
    /// 2 8
    /// Z 0 0 C
    ///
    /// Rotate bits in register r8 left, through the carry flag.
    pub(crate) fn rotate_left<S>(&mut self, bus: &mut AddressBus, src: S)
    where
        S: Copy,
        Self: ReadByte<S> + WriteByte<S>,
    {
        let value = self.read_byte(bus, src);
        let cf = self.f.contains(Flag::Carry) as u8;
        let result = (value << 1) | cf;
        self.f.set(Flag::Zero, result == 0);
        self.f.set(Flag::Subtract, false);
        self.f.set(Flag::HalfCarry, false);
        let carry = value & 0x80 != 0;
        self.f.set(Flag::Carry, carry);
        self.write_byte(bus, src, result);
    }

    /// RR r8
    /// 2 8
    /// Z 0 0 C
    ///
    /// Rotate register r8 right, through the carry flag.
    pub(crate) fn rotate_right<S>(&mut self, bus: &mut AddressBus, src: S)
    where
        S: Copy,
        Self: ReadByte<S> + WriteByte<S>,
    {
        let value = self.read_byte(bus, src);
        let cf = self.f.contains(Flag::Carry) as u8;
        let result = (value >> 1) | (cf << 7);
        self.f.set(Flag::Zero, result == 0);
        self.f.set(Flag::Subtract, false);
        self.f.set(Flag::HalfCarry, false);
        let carry = value & 0x01 != 0;
        self.f.set(Flag::Carry, carry);
        self.write_byte(bus, src, result);
    }

    /// SLA r8
    /// 2 8
    /// Z 0 0 C
    ///
    /// Shift register r8 left.
    pub(crate) fn shift_left<S>(&mut self, bus: &mut AddressBus, src: S)
    where
        S: Copy,
        Self: ReadByte<S> + WriteByte<S>,
    {
        let value = self.read_byte(bus, src);
        let result = value << 1;
        self.f.set(Flag::Zero, result == 0);
        self.f.set(Flag::Subtract, false);
        self.f.set(Flag::HalfCarry, false);
        let carry = value & 0x80 != 0;
        self.f.set(Flag::Carry, carry);
        self.write_byte(bus, src, result);
    }

    /// SRA r8
    /// 2 8
    /// Z 0 0 C
    ///
    /// Shift register r8 right (bit 7 of r8 is unchanged).
    pub(crate) fn shift_right_arithmetic<S>(&mut self, bus: &mut AddressBus, src: S)
    where
        S: Copy,
        Self: ReadByte<S> + WriteByte<S>,
    {
        let value = self.read_byte(bus, src);
        let result = (value >> 1) | (value & 0x80);
        self.f.set(Flag::Zero, result == 0);
        self.f.set(Flag::Subtract, false);
        self.f.set(Flag::HalfCarry, false);
        let carry = value & 0x01 != 0;
        self.f.set(Flag::Carry, carry);
        self.write_byte(bus, src, result);
    }

    /// SWAP r8
    /// 2 8
    /// Z 0 0 0
    ///
    /// Swap the upper 4 bits in register r8 and the lower 4 ones.
    pub(crate) fn swap<S>(&mut self, bus: &mut AddressBus, src: S)
    where
        S: Copy,
        Self: ReadByte<S> + WriteByte<S>,
    {
        let value = self.read_byte(bus, src);
        // Rotating by 4 swaps the upper bits with the lower bits
        let result = value.rotate_left(4);
        self.f.set(Flag::Zero, result == 0);
        self.f.set(Flag::Subtract, false);
        self.f.set(Flag::HalfCarry, false);
        self.f.set(Flag::Carry, false);
        self.write_byte(bus, src, result);
    }

    /// SRL r8
    /// 2 8
    /// Z 0 0 C
    ///
    /// Shift register r8 right.
    pub(crate) fn shift_right_logical<S>(&mut self, bus: &mut AddressBus, src: S)
    where
        S: Copy,
        Self: ReadByte<S> + WriteByte<S>,
    {
        let value = self.read_byte(bus, src);
        let result = value >> 1;
        self.f.set(Flag::Zero, result == 0);
        self.f.set(Flag::Subtract, false);
        self.f.set(Flag::HalfCarry, false);
        let carry = value & 0x01 != 0;
        self.f.set(Flag::Carry, carry);
        self.write_byte(bus, src, result);
    }

    /// BIT u3, r8
    /// 2 8
    /// Z 0 1 -
    ///
    /// Test bit u3 in register r8, set the zero flag if bit not set.
    pub(crate) fn bit_test<S>(&mut self, bus: &mut AddressBus, bit: u8, src: S)
    where
        Self: ReadByte<S>,
    {
        let value = self.read_byte(bus, src);
        let result = value & (1 << bit);
        self.f.set(Flag::Zero, result == 0);
        self.f.set(Flag::Subtract, false);
        self.f.set(Flag::HalfCarry, true);
        // CARRY left untouched
    }

    /// RES u3, r8
    /// 2 8
    /// - - - -
    ///
    /// Set bit u3 in register r8 to 0. Bit 0 is the rightmost one, bit 7 the leftmost one.
    pub(crate) fn bit_reset<S>(&mut self, bus: &mut AddressBus, bit: u8, src: S)
    where
        S: Copy,
        Self: ReadByte<S> + WriteByte<S>,
    {
        let value = self.read_byte(bus, src);
        let result = value & !(1 << bit);
        // Flags left untouched
        self.write_byte(bus, src, result);
    }

    /// SET u3, r8
    /// 2 8
    /// - - - -
    ///
    /// Set bit u3 in register r8 to 1. Bit 0 is the rightmost one, bit 7 the leftmost one.
    pub(crate) fn bit_set<S>(&mut self, bus: &mut AddressBus, bit: u8, src: S)
    where
        S: Copy,
        Self: ReadByte<S> + WriteByte<S>,
    {
        let value = self.read_byte(bus, src);
        let result = value | (1 << bit);
        // Flags left untouched
        self.write_byte(bus, src, result);
    }

    /// JP HL
    /// 1 4
    /// - - - -
    ///
    /// Jump to address in HL; effectively, load PC with value in register HL.
    pub(crate) fn jump_hl(&mut self, bus: &mut AddressBus) {
        self.pc = self.read_word(bus, Register16::HL);
    }

    /// JP cc, n16
    /// 3 16/12
    /// - - - -
    ///
    /// Jump to address n16 if condition cc is met.
    pub(crate) fn jump(&mut self, bus: &mut AddressBus, condition: JumpCondition) {
        let should_jump = self.f.test(condition);
        let addr = self.read_word(bus, Immediate);
        if should_jump {
            self.pc = addr;
            bus.tick();
        }
    }

    /// JR cc, e8
    /// 2 12/8
    /// - - - -
    ///
    /// Relative Jump to current address plus e8 offset if condition cc is met.
    pub(crate) fn jump_relative(&mut self, bus: &mut AddressBus, condition: JumpCondition) {
        let should_jump = self.f.test(condition);
        let offset = self.read_next_byte_signed(bus) as i16;
        if should_jump {
            self.pc = self.pc.wrapping_add_signed(offset);
            bus.tick();
        }
    }

    /// PUSH r16
    /// 1 16
    /// - - - -
    ///
    /// Push register r16 into the stack.
    pub(crate) fn push<T>(&mut self, bus: &mut AddressBus, target: T)
    where
        Self: ReadWord<T>,
    {
        bus.tick();
        let value = self.read_word(bus, target);
        let [low, high] = value.to_le_bytes();
        self.sp = self.sp.wrapping_sub(1);
        bus.write_byte(self.sp, high);
        bus.tick();

        self.sp = self.sp.wrapping_sub(1);
        bus.write_byte(self.sp, low);
        bus.tick();
    }

    /// POP r16
    /// 1 12
    /// - - - -
    ///
    /// Pop register r16 from the stack.
    ///
    /// NOTE: POP AF affects all flags.
    pub(crate) fn pop<T>(&mut self, bus: &mut AddressBus, target: T)
    where
        Self: WriteWord<T>,
    {
        let low = bus.read_byte(self.sp);
        self.sp = self.sp.wrapping_add(1);
        bus.tick();

        let high = bus.read_byte(self.sp);
        self.sp = self.sp.wrapping_add(1);
        bus.tick();

        let value = u16::from_le_bytes([low, high]);
        self.write_word(bus, target, value);
    }

    /// CALL cc, n16
    /// 3 24/12
    /// - - - -
    ///
    /// Call address n16 if condition cc is met.
    pub(crate) fn call(&mut self, bus: &mut AddressBus, condition: JumpCondition) {
        let should_jump = self.f.test(condition);
        let addr = self.read_word(bus, Immediate);
        if should_jump {
            bus.tick();

            let value = self.read_word(bus, Register16::PC);
            let [low, high] = value.to_le_bytes();

            self.sp = self.sp.wrapping_sub(1);
            bus.write_byte(self.sp, high);
            bus.tick();

            self.sp = self.sp.wrapping_sub(1);
            bus.write_byte(self.sp, low);
            bus.tick();

            self.pc = addr;
        }
    }

    /// RET
    /// 1 16
    /// - - - -
    ///
    /// Return from subroutine.
    pub(crate) fn return_(&mut self, bus: &mut AddressBus) {
        let low = bus.read_byte(self.sp);
        self.sp = self.sp.wrapping_add(1);
        bus.tick();

        let high = bus.read_byte(self.sp);
        self.sp = self.sp.wrapping_add(1);
        bus.tick();

        let value = u16::from_le_bytes([low, high]);
        self.write_word(bus, Register16::PC, value);
        bus.tick();
    }

    /// RET cc
    /// 1 20/8
    /// - - - -
    ///
    /// Return from subroutine if condition cc is met.
    pub(crate) fn return_if(&mut self, bus: &mut AddressBus, condition: JumpCondition) {
        let should_jump = self.f.test(condition);
        bus.tick();
        if should_jump {
            let low = bus.read_byte(self.sp);
            self.sp = self.sp.wrapping_add(1);
            bus.tick();

            let high = bus.read_byte(self.sp);
            self.sp = self.sp.wrapping_add(1);
            bus.tick();

            let value = u16::from_le_bytes([low, high]);
            self.write_word(bus, Register16::PC, value);
            bus.tick();
        }
    }

    /// RETI
    /// 1 16
    /// - - - -
    ///
    /// Return from subroutine and enable interrupts.
    /// This is basically equivalent to executing EI then RET, meaning that IME is set right after this instruction.
    pub(crate) fn return_from_interrupt_handler(&mut self, bus: &mut AddressBus) {
        self.return_(bus);
        self.interrupt_enabled = true;
    }

    /// RST u8
    /// 1 16
    /// - - - -
    ///
    /// Push current address onto stack, and jump to address u8.
    pub(crate) fn restart(&mut self, bus: &mut AddressBus, addr: u16) {
        bus.tick();
        let value = self.read_word(bus, Register16::PC);
        let [low, high] = value.to_le_bytes();
        self.sp = self.sp.wrapping_sub(1);
        bus.write_byte(self.sp, high);
        bus.tick();

        self.sp = self.sp.wrapping_sub(1);
        bus.write_byte(self.sp, low);
        self.pc = addr;
        bus.tick();
    }

    /// DI
    /// 1 4
    /// - - - -
    ///
    /// Disable Interrupts by clearing the IME flag.
    pub(crate) fn disable_interrupt(&mut self) {
        self.interrupt_enabled = false;
        self.interrupt_delay = None;
    }

    /// EI
    /// 1 4
    /// - - - -
    ///
    /// Enable Interrupts by setting the IME flag.
    /// The flag is only set after the instruction following EI.
    pub(crate) fn enable_interrupt(&mut self) {
        if self.interrupt_delay.is_none() {
            self.interrupt_delay = Some(1);
        }
    }
}
