use crate::gb::cpu::{instructions::ArgR8, MTime, CPU};

impl CPU {
    fn do_sub_r8(&mut self, r8: ArgR8, with_carry: bool) -> u8 {
        let value = self.get_r8(r8);
        let cv = (with_carry && self.getf_c()) as u8;
        let (result, overflow1) = self.a.overflowing_sub(value);
        let (result, overflow2) = result.overflowing_sub(cv);
        let nibble_diff = ((self.a & 0xF) as i8) - ((value & 0xF) as i8) - (cv as i8);

        self.set_all_flags(result == 0, true, nibble_diff < 0, overflow1 || overflow2);

        result
    }

    // ADC A,r8   (m: 1)
    // ADC A,[HL] (m: 2)
    // ADC A,n8   (m: 2)
    // ADD A,r8   (m: 1)
    // ADD A,[HL] (m: 2)
    // ADD A,n8   (m: 2)
    pub(super) fn op_add_r8(&mut self, r8: ArgR8, with_carry: bool) -> MTime {
        let value = self.get_r8(r8);
        let cv = (with_carry && self.getf_c()) as u8;
        let (result, overflow1) = self.a.overflowing_add(value);
        let (result, overflow2) = result.overflowing_add(cv);
        let nibble_sum = (self.a & 0xF) + (value & 0xF) + cv;

        self.set_all_flags(result == 0, false, nibble_sum > 0xF, overflow1 || overflow2);

        self.a = result;

        decide_time_for_r8!(r8; 2, 1)
    }

    // CP A,r8   (m: 1)
    // CP A,[HL] (m: 2)
    // CP A,n8   (m: 2)
    pub(super) fn op_cp_r8(&mut self, r8: ArgR8) -> MTime {
        self.do_sub_r8(r8, false);
        decide_time_for_r8!(r8; 2, 1)
    }

    // DEC r8   (m: 1)
    // DEC [HL] (m: 3)
    pub(super) fn op_dec_r8(&mut self, r8: ArgR8) -> MTime {
        let value = self.get_r8(r8);
        let new_value = value.wrapping_sub(1);

        self.setf_z(new_value == 0);
        self.setf_s(true);
        self.setf_hc(value & 0xF == 0);

        self.set_r8(r8, new_value);

        decide_time_for_r8!(r8; 3, 1)
    }

    // INC r8   (m: 1)
    // INC [HL] (m: 3)
    pub(super) fn op_inc_r8(&mut self, r8: ArgR8) -> MTime {
        let value = self.get_r8(r8);
        let new_value = value.overflowing_add(1).0;

        self.setf_z(new_value == 0);
        self.setf_s(false);
        self.setf_hc(value & 0xF == 0xF);

        self.set_r8(r8, new_value);

        decide_time_for_r8!(r8; 3, 1)
    }

    // SBC A,r8   (m: 1)
    // SBC A,[HL] (m: 2)
    // SBC A,n8   (m: 2)
    // SUB A,r8   (m: 1)
    // SUB A,[HL] (m: 2)
    // SUB A,n8   (m: 2)
    pub(super) fn op_sub_r8(&mut self, r8: ArgR8, with_carry: bool) -> MTime {
        self.a = self.do_sub_r8(r8, with_carry);
        decide_time_for_r8!(r8; 2, 1)
    }
}
