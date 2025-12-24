use crate::gb::cpu::{instructions::ArgR8, MTime, CPU};

impl CPU {
    fn do_rotate_left(&self, value: u8) -> (u8, bool) {
        // Get value and rotate
        let rotated = value.rotate_left(1);
        // New carry is whatever was rotated
        (rotated, rotated & 1 != 0)
    }

    fn do_rotate_left_carry(&self, value: u8) -> (u8, bool) {
        // Get value as u16
        let value_l = value as u16;
        // Put carry flag at the start, then the value
        let processed = ((self.getf_c() as u16) << 15) | (value_l << 7);
        // Rotate
        let rotated = processed.rotate_left(1);
        // The top 7 bits of the final value are these bits of rotated
        let new_value_top_7 = ((rotated & 0b0111_1111_0000_0000) >> 7) as u8;
        // The last bit of the final value is the last bit of rotated
        let new_value_last = (rotated & 1) as u8;
        // Put the final value together, and new carry is the first bit of rotated
        (new_value_top_7 | new_value_last, rotated & (1 << 15) != 0)
    }

    fn do_rotate_right(&self, value: u8) -> (u8, bool) {
        // New value is rotated, new carry is what was going to be rotated
        (value.rotate_right(1), value & 1 != 0)
    }

    fn do_rotate_right_carry(&self, value: u8) -> (u8, bool) {
        // Get value as u16
        let value_l = value as u16;
        // Put carry flag at the end, then the value
        let processed = (self.getf_c() as u16) | (value_l << 1);
        // Rotate
        let rotated = processed.rotate_right(1);
        // The last 7 bits of the final value are these bits of rotated
        let new_value_last_7 = (rotated & 0b0000_0000_1111_1110) as u8;
        // The top bit of the final value is the top bit of rotated
        let new_value_top = ((rotated & (1 << 15)) >> 8) as u8;
        // Put the final value together, and new carry is the last bit of rotated
        (new_value_last_7 | new_value_top, rotated & 1 != 0)
    }

    // RL r8    (m: 2)
    // RL [HL]  (m: 4)
    // RLC r8   (m: 2)
    // RLC [HL] (m: 4)
    pub(super) fn op_rl_r8(&mut self, r8: ArgR8, through_carry: bool) -> MTime {
        let (new_value, new_carry) = if through_carry {
            self.do_rotate_left(self.get_r8(r8))
        } else {
            self.do_rotate_left_carry(self.get_r8(r8))
        };

        self.set_all_flags(new_value == 0, false, false, new_carry);

        self.set_r8(r8, new_value);

        decide_time_for_r8!(r8; 4, 2)
    }

    // RLA  (m: 1)
    // RLCA (m: 1)
    pub(super) fn op_rla(&mut self, through_carry: bool) -> MTime {
        let (new_value, new_carry) = if through_carry {
            self.do_rotate_left(self.a)
        } else {
            self.do_rotate_left_carry(self.a)
        };

        self.set_all_flags(false, false, false, new_carry);

        self.a = new_value;

        1.into()
    }

    // RR r8    (m: 2)
    // RR [HL]  (m: 4)
    // RRC r8   (m: 2)
    // RRC [HL] (m: 4)
    pub(super) fn op_rr_r8(&mut self, r8: ArgR8, through_carry: bool) -> MTime {
        let (new_value, new_carry) = if through_carry {
            self.do_rotate_right(self.get_r8(r8))
        } else {
            self.do_rotate_right_carry(self.get_r8(r8))
        };

        self.set_all_flags(new_value == 0, false, false, new_carry);

        self.set_r8(r8, new_value);

        decide_time_for_r8!(r8; 4, 2)
    }

    // RRA  (m: 1)
    // RRCA (m: 1)
    pub(super) fn op_rra(&mut self, through_carry: bool) -> MTime {
        let (new_value, new_carry) = if through_carry {
            self.do_rotate_right(self.a)
        } else {
            self.do_rotate_right_carry(self.a)
        };

        self.set_all_flags(false, false, false, new_carry);

        self.a = new_value;

        1.into()
    }

    // SLA r8   (m: 2)
    // SLA [HL] (m: 4)
    pub(super) fn op_sla_r8(&mut self, r8: ArgR8) -> MTime {
        let value = self.get_r8(r8);
        let original_top_bit = value & 0x80;
        let new_value = value << 1;

        self.set_all_flags(new_value == 0, false, false, original_top_bit != 0);

        self.set_r8(r8, new_value);

        decide_time_for_r8!(r8; 4, 2)
    }

    // SRA r8   (m: 2)
    // SRA [HL] (m: 4)
    // SRL r8   (m: 2)
    // SRL [HL] (m: 4)
    pub(super) fn op_sra_r8(&mut self, r8: ArgR8, is_arithmetic: bool) -> MTime {
        let value = self.get_r8(r8);
        let original_bottom_bit = value & 1;
        let shifted_value = value >> 1;

        let new_value = if is_arithmetic {
            (value & 0x80) | shifted_value
        } else {
            shifted_value
        };

        self.set_all_flags(new_value == 0, false, false, original_bottom_bit != 0);

        self.set_r8(r8, new_value);

        decide_time_for_r8!(r8; 4, 2)
    }

    // SWAP r8   (m: 2)
    // SWAP [HL] (m: 4)
    pub(super) fn op_swap_r8(&mut self, r8: ArgR8) -> MTime {
        let value = self.get_r8(r8);

        let new_value = ((value & 0x0F) << 4) | ((value & 0xF0) >> 4);

        self.set_all_flags(new_value == 0, false, false, false);

        self.set_r8(r8, new_value);

        decide_time_for_r8!(r8; 4, 2)
    }
}
