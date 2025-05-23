use crate::cpu::{
    AccessReadByte, AccessReadWord, AccessWriteByte, AccessWriteWord, Cpu, FlagsRegister,
    JumpCondition, Register16,
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
    pub(crate) fn stop(&mut self, bus: &AddressBus) {
        let _ = self.read_next_byte(bus);
        loop {
            // TODO: Add sleeping to save CPU usage
            let joypad = bus.get_joypad();
            if joypad.is_any_pressed() {
                break;
            }
        }
        // TODO: look into strange stop behavior
    }

    /// HALT
    /// 1 4
    /// - - - -
    ///
    /// Halt CPU until an interrupt occurs.
    pub(crate) const fn halt(&mut self) {
        self.halted = true;
        // TODO: Look into halt bug
    }

    /// LD r8, r8
    /// 1 4
    /// - - - -
    ///
    /// Load src (right) and copy into dst (left).
    pub(crate) fn load<D, S>(&mut self, bus: &mut AddressBus, dst: D, src: S)
    where
        Self: AccessReadByte<S> + AccessWriteByte<D>,
    {
        let value = self.read_byte(bus, src);
        self.write_byte(bus, dst, value);
    }

    /// LD r16, n16
    /// 3 12
    /// - - - -
    ///
    /// Load src (right) and copy into dst (left).
    pub(crate) fn load16<D, S>(&mut self, bus: &AddressBus, dst: D, src: S)
    where
        Self: AccessReadWord<S> + AccessWriteWord<D>,
    {
        let value = self.read_word(bus, src);
        self.write_word(dst, value);
    }

    /// LD \[a16\], SP
    /// 3 20
    /// - - - -
    ///
    /// Load SP at address a16.
    pub(crate) fn load16_a16_sp(&mut self, bus: &mut AddressBus) {
        let value = self.registers.sp;
        let [low, high] = value.to_le_bytes();
        let addr = self.read_next_word(bus);
        bus.write_byte(addr, low);
        bus.write_byte(addr.wrapping_add(1), high);
    }

    /// LD HL, SP + e8
    /// 2 12
    /// 0 0 H C
    ///
    /// Add the signed value e8 to SP and store the result in HL.
    pub(crate) fn load16_hl_sp(&mut self, bus: &AddressBus) {
        let sp = self.registers.sp;
        let offset = self.read_next_byte_signed(bus) as i16;
        self.registers.f.set(FlagsRegister::ZERO, false);
        self.registers.f.set(FlagsRegister::SUBTRACT, false);
        // Half-carry from bit 3, carry from bit 7
        // Bits are labeled from 0-15 from least to most significant.
        let half_carry = (sp & 0xF).wrapping_add_signed(offset & 0xF) > 0xF;
        self.registers.f.set(FlagsRegister::HALF_CARRY, half_carry);
        let carry = (sp & 0xFF).wrapping_add_signed(offset & 0xFF) > 0xFF;
        self.registers.f.set(FlagsRegister::CARRY, carry);
        let new_value = sp.wrapping_add_signed(offset);
        self.registers.write_word(Register16::HL, new_value);
    }

    /// ADD A, r8
    /// 1 4
    /// Z 0 H C
    ///
    /// Add the value in r8 to register A.
    pub(crate) fn add<S>(&mut self, bus: &AddressBus, src: S)
    where
        Self: AccessReadByte<S>,
    {
        let value = self.read_byte(bus, src);
        let a = self.registers.a;
        let (new_value, did_overflow) = a.overflowing_add(value);
        self.registers.f.set(FlagsRegister::ZERO, new_value == 0);
        self.registers.f.set(FlagsRegister::SUBTRACT, false);
        // Half carry is set if adding the lower bits (0-3) of the value and register A
        // together result in overflowing to bit 4. If the result is larger than 0xF
        // than the addition caused a carry from bit 3 to bit 4.
        let half_carry = (a & 0xF) + (value & 0xF) > 0xF;
        self.registers.f.set(FlagsRegister::HALF_CARRY, half_carry);
        self.registers.f.set(FlagsRegister::CARRY, did_overflow);
        self.registers.a = new_value;
    }

    /// ADC A, r8
    /// 1 4
    /// Z 0 H C
    ///
    /// Add the value in r8 plus the carry flag to register A.
    pub(crate) fn add_with_carry<S>(&mut self, bus: &AddressBus, src: S)
    where
        Self: AccessReadByte<S>,
    {
        let value = self.read_byte(bus, src);
        let a = self.registers.a;
        let cf = self.registers.f.contains(FlagsRegister::CARRY) as u8;
        let new_value = a.wrapping_add(value).wrapping_add(cf);
        self.registers.f.set(FlagsRegister::ZERO, new_value == 0);
        self.registers.f.set(FlagsRegister::SUBTRACT, false);
        let half_carry = (a & 0xF) + (value & 0xF) + cf > 0xF;
        self.registers.f.set(FlagsRegister::HALF_CARRY, half_carry);
        let carry = a as u16 + value as u16 + cf as u16 > 0xFF;
        self.registers.f.set(FlagsRegister::CARRY, carry);
        self.registers.a = new_value;
    }

    /// SUB A, r8
    /// 1 4
    /// Z 1 H C
    ///
    /// Subtract the value in r8 from register A.
    pub(crate) fn subtract<S>(&mut self, bus: &AddressBus, src: S)
    where
        Self: AccessReadByte<S>,
    {
        let value = self.read_byte(bus, src);
        let a = self.registers.a;
        let (new_value, did_overflow) = a.overflowing_sub(value);
        self.registers.f.set(FlagsRegister::ZERO, new_value == 0);
        self.registers.f.set(FlagsRegister::SUBTRACT, true);
        let half_carry = (a & 0xF) < (value & 0xF);
        self.registers.f.set(FlagsRegister::HALF_CARRY, half_carry);
        self.registers.f.set(FlagsRegister::CARRY, did_overflow);
        self.registers.a = new_value;
    }

    /// SBC A, r8
    /// 1 4
    /// Z 1 H C
    ///
    /// Subtract the value in r8 and the carry flag from register A.
    pub(crate) fn subtract_with_carry<S>(&mut self, bus: &AddressBus, src: S)
    where
        Self: AccessReadByte<S>,
    {
        let value = self.read_byte(bus, src);
        let a = self.registers.a;
        let cf = self.registers.f.contains(FlagsRegister::CARRY) as u8;
        let new_value = a.wrapping_sub(value).wrapping_sub(cf);
        self.registers.f.set(FlagsRegister::ZERO, new_value == 0);
        self.registers.f.set(FlagsRegister::SUBTRACT, true);
        let half_carry = (a & 0xF) < (value & 0xF) + cf;
        self.registers.f.set(FlagsRegister::HALF_CARRY, half_carry);
        let carry = (a as u16) < (value as u16) + (cf as u16);
        self.registers.f.set(FlagsRegister::CARRY, carry);
        self.registers.a = new_value;
    }

    /// AND A, r8
    /// 1 4
    /// Z 0 1 0
    ///
    /// Bitwise AND between the value in r8 and register A.
    pub(crate) fn and<S>(&mut self, bus: &AddressBus, src: S)
    where
        Self: AccessReadByte<S>,
    {
        let value = self.read_byte(bus, src);
        let new_value = self.registers.a & value;
        self.registers.f.set(FlagsRegister::ZERO, new_value == 0);
        self.registers.f.set(FlagsRegister::SUBTRACT, false);
        self.registers.f.set(FlagsRegister::HALF_CARRY, true);
        self.registers.f.set(FlagsRegister::CARRY, false);
        self.registers.a = new_value;
    }

    /// XOR A, r8
    /// 1 4
    /// Z 0 0 0
    ///
    /// Bitwise XOR between the value in r8 and register A.
    pub(crate) fn xor<S>(&mut self, bus: &AddressBus, src: S)
    where
        Self: AccessReadByte<S>,
    {
        let value = self.read_byte(bus, src);
        let new_value = self.registers.a ^ value;
        self.registers.f.set(FlagsRegister::ZERO, new_value == 0);
        self.registers.f.set(FlagsRegister::SUBTRACT, false);
        self.registers.f.set(FlagsRegister::HALF_CARRY, false);
        self.registers.f.set(FlagsRegister::CARRY, false);
        self.registers.a = new_value;
    }

    /// OR A, r8
    /// 1 4
    /// Z 0 0 0
    ///
    /// Bitwise OR between the value in r8 and register A.
    pub(crate) fn or<S>(&mut self, bus: &AddressBus, src: S)
    where
        Self: AccessReadByte<S>,
    {
        let value = self.read_byte(bus, src);
        let new_value = self.registers.a | value;
        self.registers.f.set(FlagsRegister::ZERO, new_value == 0);
        self.registers.f.set(FlagsRegister::SUBTRACT, false);
        self.registers.f.set(FlagsRegister::HALF_CARRY, false);
        self.registers.f.set(FlagsRegister::CARRY, false);
        self.registers.a = new_value;
    }

    /// CP A, r8
    /// 1 4
    /// Z 1 H C
    ///
    /// Subtract the value in r8 from register A and set flags accordingly, but don't store the result.
    pub(crate) fn compare<S>(&mut self, bus: &AddressBus, src: S)
    where
        Self: AccessReadByte<S>,
    {
        let value = self.read_byte(bus, src);
        let a = self.registers.a;
        let (new_value, did_overflow) = a.overflowing_sub(value);
        self.registers.f.set(FlagsRegister::ZERO, new_value == 0);
        self.registers.f.set(FlagsRegister::SUBTRACT, true);
        let half_carry = (a & 0xF) < (value & 0xF);
        self.registers.f.set(FlagsRegister::HALF_CARRY, half_carry);
        self.registers.f.set(FlagsRegister::CARRY, did_overflow);
    }

    /// INC r8
    /// 1 4
    /// Z 0 H -
    ///
    /// Increment value in register r8 by 1.
    pub(crate) fn increment<S>(&mut self, bus: &mut AddressBus, src: S)
    where
        S: Copy,
        Self: AccessReadByte<S> + AccessWriteByte<S>,
    {
        let value = self.read_byte(bus, src);
        let new_value = value.wrapping_add(1);
        self.registers.f.set(FlagsRegister::ZERO, new_value == 0);
        self.registers.f.set(FlagsRegister::SUBTRACT, false);
        let half_carry = value & 0xF == 0xF;
        self.registers.f.set(FlagsRegister::HALF_CARRY, half_carry);
        // CARRY is left untouched
        self.write_byte(bus, src, new_value);
    }

    /// DEC r8
    /// 1 4
    /// Z 1 H -
    ///
    /// Decrement value in register r8 by 1.
    pub(crate) fn decrement<S>(&mut self, bus: &mut AddressBus, src: S)
    where
        S: Copy,
        Self: AccessReadByte<S> + AccessWriteByte<S>,
    {
        let value = self.read_byte(bus, src);
        let new_value = value.wrapping_sub(1);
        self.registers.f.set(FlagsRegister::ZERO, new_value == 0);
        self.registers.f.set(FlagsRegister::SUBTRACT, true);
        let half_carry = value & 0xF == 0;
        self.registers.f.set(FlagsRegister::HALF_CARRY, half_carry);
        // CARRY is left untouched
        self.write_byte(bus, src, new_value);
    }

    /// ADD HL, r16
    /// 1 8
    /// - 0 H C
    ///
    /// Add the value in r16 to register HL.
    pub(crate) const fn add16_hl(&mut self, src: Register16) {
        let value = self.registers.read_word(src);
        let hl = self.registers.read_word(Register16::HL);
        let (new_value, did_overflow) = hl.overflowing_add(value);
        // ZERO is left untouched
        self.registers.f.set(FlagsRegister::SUBTRACT, false);
        // Half-carry from bit 11, carry from bit 15
        // Bits are labeled from 0-15 from least to most significant.
        let half_carry = (hl & 0xFFF) + (value & 0xFFF) > 0xFFF;
        self.registers.f.set(FlagsRegister::HALF_CARRY, half_carry);
        self.registers.f.set(FlagsRegister::CARRY, did_overflow);
        self.registers.write_word(Register16::HL, new_value);
    }

    /// ADD SP, e8
    /// 2 16
    /// 0 0 H C
    ///
    /// Add the signed value e8 to SP.
    pub(crate) fn add16_sp(&mut self, bus: &AddressBus) {
        let offset = self.read_next_byte_signed(bus) as i16;
        let sp = self.registers.sp;
        self.registers.f.set(FlagsRegister::ZERO, false);
        self.registers.f.set(FlagsRegister::SUBTRACT, false);
        // Half-carry from bit 3, carry from bit 7
        // Bits are labeled from 0-15 from least to most significant.
        let half_carry = (sp & 0xF).wrapping_add_signed(offset & 0xF) > 0xF;
        self.registers.f.set(FlagsRegister::HALF_CARRY, half_carry);
        let carry = (sp & 0xFF).wrapping_add_signed(offset & 0xFF) > 0xFF;
        self.registers.f.set(FlagsRegister::CARRY, carry);
        let new_value = sp.wrapping_add_signed(offset);
        self.registers.sp = new_value;
    }

    /// INC r16
    /// 1 8
    /// - - - -
    ///
    /// Increment value in register r16 by 1.
    pub(crate) const fn increment16(&mut self, src: Register16) {
        let value = self.registers.read_word(src);
        let new_value = value.wrapping_add(1);
        self.registers.write_word(src, new_value);
    }

    /// DEC r16
    /// 1 8
    /// - - - -
    ///
    /// Decrement value in register r16 by 1.
    pub(crate) const fn decrement16(&mut self, src: Register16) {
        let value = self.registers.read_word(src);
        let new_value = value.wrapping_sub(1);
        self.registers.write_word(src, new_value);
    }

    /// RLCA
    /// 1 4
    /// 0 0 0 C
    ///
    /// Rotate register A left.
    pub(crate) const fn rotate_left_circular_accumulator(&mut self) {
        let value = self.registers.a;
        let new_value = self.registers.a.rotate_left(1);
        self.registers.f.set(FlagsRegister::ZERO, false);
        self.registers.f.set(FlagsRegister::SUBTRACT, false);
        self.registers.f.set(FlagsRegister::HALF_CARRY, false);
        let carry = value & 0x80 != 0;
        self.registers.f.set(FlagsRegister::CARRY, carry);
        self.registers.a = new_value;
    }

    /// RLA
    /// 1 4
    /// 0 0 0 C
    ///
    /// Rotate register A left, through the carry flag.
    pub(crate) const fn rotate_left_accumulator(&mut self) {
        let value = self.registers.a;
        let cf = self.registers.f.contains(FlagsRegister::CARRY) as u8;
        let new_value = (value << 1) | cf;
        self.registers.f.set(FlagsRegister::ZERO, false);
        self.registers.f.set(FlagsRegister::SUBTRACT, false);
        self.registers.f.set(FlagsRegister::HALF_CARRY, false);
        let carry = value & 0x80 != 0;
        self.registers.f.set(FlagsRegister::CARRY, carry);
        self.registers.a = new_value;
    }

    /// RRCA
    /// 1 4
    /// 0 0 0 C
    ///
    /// Rotate register A right.
    pub(crate) const fn rotate_right_circular_accumulator(&mut self) {
        let value = self.registers.a;
        let new_value = self.registers.a.rotate_right(1);
        self.registers.f.set(FlagsRegister::ZERO, false);
        self.registers.f.set(FlagsRegister::SUBTRACT, false);
        self.registers.f.set(FlagsRegister::HALF_CARRY, false);
        let carry = value & 0x01 != 0;
        self.registers.f.set(FlagsRegister::CARRY, carry);
        self.registers.a = new_value;
    }

    /// RRA
    /// 1 4
    /// 0 0 0 C
    ///
    /// Rotate register A right, through the carry flag.
    pub(crate) const fn rotate_right_accumulator(&mut self) {
        let value = self.registers.a;
        let cf = self.registers.f.contains(FlagsRegister::CARRY) as u8;
        let new_value = (value >> 1) | (cf << 7);
        self.registers.f.set(FlagsRegister::ZERO, false);
        self.registers.f.set(FlagsRegister::SUBTRACT, false);
        self.registers.f.set(FlagsRegister::HALF_CARRY, false);
        let carry = value & 0x01 != 0;
        self.registers.f.set(FlagsRegister::CARRY, carry);
        self.registers.a = new_value;
    }

    /// SCF
    /// 1 4
    /// - 0 0 1
    ///
    /// Set the carry flag.
    pub(crate) const fn set_carry_flag(&mut self) {
        // ZERO left untouched
        self.registers.f.set(FlagsRegister::SUBTRACT, false);
        self.registers.f.set(FlagsRegister::HALF_CARRY, false);
        self.registers.f.set(FlagsRegister::CARRY, true);
    }

    /// CPL
    /// 1 4
    /// - 1 1 -
    ///
    /// Flip the bits in register A.
    pub(crate) const fn complement_accumulator(&mut self) {
        let value = self.registers.a;
        // ZERO left untouched
        self.registers.f.set(FlagsRegister::SUBTRACT, true);
        self.registers.f.set(FlagsRegister::HALF_CARRY, true);
        // CARRY left untouched
        self.registers.a = !value;
    }

    /// CCF
    /// 1 4
    /// - 0 0 C
    ///
    /// Complement the carry flag.
    pub(crate) const fn complement_carry_flag(&mut self) {
        let cf = self.registers.f.contains(FlagsRegister::CARRY);
        // ZERO left untouched
        self.registers.f.set(FlagsRegister::SUBTRACT, false);
        self.registers.f.set(FlagsRegister::HALF_CARRY, false);
        self.registers.f.set(FlagsRegister::CARRY, !cf);
    }

    /// DAA
    /// 1 4
    /// Z - 0 C
    ///
    /// Decimal Adjust register A to get a correct BCD representation after an arithmetic instruction.
    pub(crate) const fn decimal_adjust_accumulator(&mut self) {
        let mut value = self.registers.a;

        let nf = self.registers.f.contains(FlagsRegister::SUBTRACT);
        let hf = self.registers.f.contains(FlagsRegister::HALF_CARRY);
        let mut cf = self.registers.f.contains(FlagsRegister::CARRY);

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

        self.registers.f.set(FlagsRegister::ZERO, value == 0);
        // SUBTRACT left untouched
        self.registers.f.set(FlagsRegister::HALF_CARRY, false);
        self.registers.f.set(FlagsRegister::CARRY, cf);

        self.registers.a = value;
    }

    /// RLC r8
    /// 2 8
    /// Z 0 0 C
    ///
    /// Rotate register r8 left.
    pub(crate) fn rotate_left_circular<S>(&mut self, bus: &mut AddressBus, src: S)
    where
        S: Copy,
        Self: AccessReadByte<S> + AccessWriteByte<S>,
    {
        let value = self.read_byte(bus, src);
        let new_value = value.rotate_left(1);
        self.registers.f.set(FlagsRegister::ZERO, new_value == 0);
        self.registers.f.set(FlagsRegister::SUBTRACT, false);
        self.registers.f.set(FlagsRegister::HALF_CARRY, false);
        let carry = value & 0x80 != 0;
        self.registers.f.set(FlagsRegister::CARRY, carry);
        self.write_byte(bus, src, new_value);
    }

    /// RRC r8
    /// 2 8
    /// Z 0 0 C
    ///
    /// Rotate register r8 right.
    pub(crate) fn rotate_right_circular<S>(&mut self, bus: &mut AddressBus, src: S)
    where
        S: Copy,
        Self: AccessReadByte<S> + AccessWriteByte<S>,
    {
        let value = self.read_byte(bus, src);
        let new_value = value.rotate_right(1);
        self.registers.f.set(FlagsRegister::ZERO, new_value == 0);
        self.registers.f.set(FlagsRegister::SUBTRACT, false);
        self.registers.f.set(FlagsRegister::HALF_CARRY, false);
        let carry = value & 0x01 != 0;
        self.registers.f.set(FlagsRegister::CARRY, carry);
        self.write_byte(bus, src, new_value);
    }

    /// RL r8
    /// 2 8
    /// Z 0 0 C
    ///
    /// Rotate bits in register r8 left, through the carry flag.
    pub(crate) fn rotate_left<S>(&mut self, bus: &mut AddressBus, src: S)
    where
        S: Copy,
        Self: AccessReadByte<S> + AccessWriteByte<S>,
    {
        let value = self.read_byte(bus, src);
        let cf = self.registers.f.contains(FlagsRegister::CARRY) as u8;
        let new_value = (value << 1) | cf;
        self.registers.f.set(FlagsRegister::ZERO, new_value == 0);
        self.registers.f.set(FlagsRegister::SUBTRACT, false);
        self.registers.f.set(FlagsRegister::HALF_CARRY, false);
        let carry = value & 0x80 != 0;
        self.registers.f.set(FlagsRegister::CARRY, carry);
        self.write_byte(bus, src, new_value);
    }

    /// RR r8
    /// 2 8
    /// Z 0 0 C
    ///
    /// Rotate register r8 right, through the carry flag.
    pub(crate) fn rotate_right<S>(&mut self, bus: &mut AddressBus, src: S)
    where
        S: Copy,
        Self: AccessReadByte<S> + AccessWriteByte<S>,
    {
        let value = self.read_byte(bus, src);
        let cf = self.registers.f.contains(FlagsRegister::CARRY) as u8;
        let new_value = (value >> 1) | (cf << 7);
        self.registers.f.set(FlagsRegister::ZERO, new_value == 0);
        self.registers.f.set(FlagsRegister::SUBTRACT, false);
        self.registers.f.set(FlagsRegister::HALF_CARRY, false);
        let carry = value & 0x01 != 0;
        self.registers.f.set(FlagsRegister::CARRY, carry);
        self.write_byte(bus, src, new_value);
    }

    /// SLA r8
    /// 2 8
    /// Z 0 0 C
    ///
    /// Shift Left Arithmetically register r8.
    pub(crate) fn shift_left_arithmetic<S>(&mut self, bus: &mut AddressBus, src: S)
    where
        S: Copy,
        Self: AccessReadByte<S> + AccessWriteByte<S>,
    {
        let value = self.read_byte(bus, src);
        let new_value = value << 1;
        self.registers.f.set(FlagsRegister::ZERO, new_value == 0);
        self.registers.f.set(FlagsRegister::SUBTRACT, false);
        self.registers.f.set(FlagsRegister::HALF_CARRY, false);
        let carry = value & 0x80 != 0;
        self.registers.f.set(FlagsRegister::CARRY, carry);
        self.write_byte(bus, src, new_value);
    }

    /// SRA r8
    /// 2 8
    /// Z 0 0 C
    ///
    /// Shift Right Arithmetically register r8 (bit 7 of r8 is unchanged).
    pub(crate) fn shift_right_arithmetic<S>(&mut self, bus: &mut AddressBus, src: S)
    where
        S: Copy,
        Self: AccessReadByte<S> + AccessWriteByte<S>,
    {
        let value = self.read_byte(bus, src);
        let new_value = (value >> 1) | (value & 0x80);
        self.registers.f.set(FlagsRegister::ZERO, new_value == 0);
        self.registers.f.set(FlagsRegister::SUBTRACT, false);
        self.registers.f.set(FlagsRegister::HALF_CARRY, false);
        let carry = value & 0x01 != 0;
        self.registers.f.set(FlagsRegister::CARRY, carry);
        self.write_byte(bus, src, new_value);
    }

    /// SWAP r8
    /// 2 8
    /// Z 0 0 0
    ///
    /// Swap the upper 4 bits in register r8 and the lower 4 ones.
    pub(crate) fn swap<S>(&mut self, bus: &mut AddressBus, src: S)
    where
        S: Copy,
        Self: AccessReadByte<S> + AccessWriteByte<S>,
    {
        let value = self.read_byte(bus, src);
        // Rotating by 4 swaps the upper bits with the lower bits
        let new_value = value.rotate_left(4);
        self.registers.f.set(FlagsRegister::ZERO, new_value == 0);
        self.registers.f.set(FlagsRegister::SUBTRACT, false);
        self.registers.f.set(FlagsRegister::HALF_CARRY, false);
        self.registers.f.set(FlagsRegister::CARRY, false);
        self.write_byte(bus, src, new_value);
    }

    /// SRL r8
    /// 2 8
    /// Z 0 0 C
    ///
    /// Shift Right Logically register r8.
    pub(crate) fn shift_right_logical<S>(&mut self, bus: &mut AddressBus, src: S)
    where
        S: Copy,
        Self: AccessReadByte<S> + AccessWriteByte<S>,
    {
        let value = self.read_byte(bus, src);
        let new_value = value >> 1;
        self.registers.f.set(FlagsRegister::ZERO, new_value == 0);
        self.registers.f.set(FlagsRegister::SUBTRACT, false);
        self.registers.f.set(FlagsRegister::HALF_CARRY, false);
        let carry = value & 0x01 != 0;
        self.registers.f.set(FlagsRegister::CARRY, carry);
        self.write_byte(bus, src, new_value);
    }

    /// BIT u3, r8
    /// 2 8
    /// Z 0 1 -
    ///
    /// Test bit u3 in register r8, set the zero flag if bit not set.
    pub(crate) fn bit_test<S>(&mut self, bus: &AddressBus, bit: u8, src: S)
    where
        Self: AccessReadByte<S>,
    {
        let value = self.read_byte(bus, src);
        let new_value = value & (1 << bit);
        self.registers.f.set(FlagsRegister::ZERO, new_value == 0);
        self.registers.f.set(FlagsRegister::SUBTRACT, false);
        self.registers.f.set(FlagsRegister::HALF_CARRY, true);
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
        Self: AccessReadByte<S> + AccessWriteByte<S>,
    {
        let value = self.read_byte(bus, src);
        let new_value = value & !(1 << bit);
        // Flags left untouched
        self.write_byte(bus, src, new_value);
    }

    /// SET u3, r8
    /// 2 8
    /// - - - -
    ///
    /// Set bit u3 in register r8 to 1. Bit 0 is the rightmost one, bit 7 the leftmost one.
    pub(crate) fn bit_set<S>(&mut self, bus: &mut AddressBus, bit: u8, src: S)
    where
        S: Copy,
        Self: AccessReadByte<S> + AccessWriteByte<S>,
    {
        let value = self.read_byte(bus, src);
        let new_value = value | (1 << bit);
        // Flags left untouched
        self.write_byte(bus, src, new_value);
    }

    /// JP HL
    /// 1 4
    /// - - - -
    ///
    /// Jump to address in HL; effectively, load PC with value in register HL.
    pub(crate) const fn jump_to_hl(&mut self) {
        self.registers.pc = self.registers.read_word(Register16::HL);
    }

    /// JP cc, n16
    /// 3 16/12
    /// - - - -
    ///
    /// Jump to address n16 if condition cc is met.
    pub(crate) fn jump(&mut self, bus: &AddressBus, condition: JumpCondition) -> usize {
        let should_jump = self.registers.f.test(condition);
        let addr = self.read_next_word(bus);
        if should_jump {
            self.registers.pc = addr;
            return 16;
        }
        12
    }

    /// JR cc, e8
    /// 2 12/8
    /// - - - -
    ///
    /// Relative Jump to current address plus e8 offset if condition cc is met.
    pub(crate) fn jump_relative(&mut self, bus: &AddressBus, condition: JumpCondition) -> usize {
        let should_jump = self.registers.f.test(condition);
        let offset = self.read_next_byte_signed(bus) as i16;
        if should_jump {
            self.registers.pc = self.registers.pc.wrapping_add_signed(offset);
            return 12;
        }
        8
    }

    /// PUSH r16
    /// 1 16
    /// - - - -
    ///
    /// Push register r16 into the stack.
    pub(crate) fn push(&mut self, bus: &mut AddressBus, register: Register16) {
        let value = self.registers.read_word(register);
        let [low, high] = value.to_le_bytes();
        self.registers.sp = self.registers.sp.wrapping_sub(1);
        bus.write_byte(self.registers.sp, high);

        self.registers.sp = self.registers.sp.wrapping_sub(1);
        bus.write_byte(self.registers.sp, low);
    }

    /// POP r16
    /// 1 12
    /// - - - -
    ///
    /// Pop register r16 from the stack.
    ///
    /// NOTE: POP AF affects all flags.
    pub(crate) fn pop(&mut self, bus: &AddressBus, register: Register16) {
        let low = bus.read_byte(self.registers.sp);
        self.registers.sp = self.registers.sp.wrapping_add(1);

        let high = bus.read_byte(self.registers.sp);
        self.registers.sp = self.registers.sp.wrapping_add(1);

        let value = u16::from_le_bytes([low, high]);
        self.registers.write_word(register, value);
    }

    /// CALL cc, n16
    /// 3 24/12
    /// - - - -
    ///
    /// Call address n16 if condition cc is met.
    pub(crate) fn call(&mut self, bus: &mut AddressBus, condition: JumpCondition) -> usize {
        let should_jump = self.registers.f.test(condition);
        let addr = self.read_next_word(bus);
        if should_jump {
            self.push(bus, Register16::PC);
            self.registers.pc = addr;
            return 24;
        }
        12
    }

    /// RET cc
    /// 1 20/8
    /// - - - -
    ///
    /// Return from subroutine if condition cc is met.
    pub(crate) fn return_(&mut self, bus: &AddressBus, condition: JumpCondition) -> usize {
        let should_jump = self.registers.f.test(condition);
        if should_jump {
            self.pop(bus, Register16::PC);
            return 20;
        }
        8
    }

    /// RETI
    /// 1 16
    /// - - - -
    ///
    /// Return from subroutine and enable interrupts.
    /// This is basically equivalent to executing EI then RET, meaning that IME is set right after this instruction.
    pub(crate) fn return_from_interrupt_handler(&mut self, bus: &AddressBus) {
        self.return_(bus, JumpCondition::Always);
        self.ime = true;
    }

    /// RST u8
    /// 1 16
    /// - - - -
    ///
    /// Push current address onto stack, and jump to address u8.
    pub(crate) fn restart(&mut self, bus: &mut AddressBus, addr: u16) {
        self.push(bus, Register16::PC);
        self.registers.pc = addr;
    }

    /// DI
    /// 1 4
    /// - - - -
    ///
    /// Disable Interrupts by clearing the IME flag.
    pub(crate) const fn disable_interrupt(&mut self) {
        self.ime_delay_counter = None;
        self.ime = false;
    }

    /// EI
    /// 1 4
    /// - - - -
    ///
    /// Enable Interrupts by setting the IME flag.
    /// The flag is only set after the instruction following EI.
    pub(crate) const fn enable_interrupt(&mut self) {
        self.ime_delay_counter = Some(2);
    }
}
