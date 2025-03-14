use crate::gb::cpu::{instructions::ArgR16, MTime, CPU};

impl CPU {
    // ADD HL,r16 (m: 2)
    // ADD HL,SP  (m: 2)
    pub(super) fn op_add_hl_r16(&mut self, r16: ArgR16) -> MTime {
        let lhs = self.get_hl();
        let rhs = self.get_r16(r16);
        let (result, overflow) = lhs.overflowing_add(rhs);
        let half_carry = (lhs & 0x0FFF) + (rhs & 0x0FFF) > 0x0FFF;

        self.setf_s(false);
        self.setf_hc(half_carry);
        self.setf_c(overflow);

        self.set_hl(result);

        2.into()
    }

    // DEC r16 (m: 2)
    // DEC SP  (m: 2)
    pub(super) fn op_dec_r16(&mut self, r16: ArgR16) -> MTime {
        let value = self.get_r16(r16);
        self.set_r16(r16, value.overflowing_sub(1).0);

        2.into()
    }

    // INC r16 (m: 2)
    // INC SP  (m: 2)
    pub(super) fn op_inc_r16(&mut self, r16: ArgR16) -> MTime {
        let value = self.get_r16(r16);
        self.set_r16(r16, value.overflowing_add(1).0);

        2.into()
    }
}
