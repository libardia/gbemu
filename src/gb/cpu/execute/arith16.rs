use super::*;

impl CPU {
    pub(super) fn op_add_16(&mut self, op: R16) -> MTime {
        let lhs = self.get_hl();
        let rhs = self.get_r16(op);
        let (result, overflow) = self.get_hl().overflowing_add(rhs);
        self.set_hl(result);

        self.setf_n(false);
        self.setf_h(lhs & 0xFFF + rhs & 0xFFF > 0xFFF);
        self.setf_c(overflow);

        // Always 2 cycles
        2
    }

    pub(super) fn op_inc_16(&mut self, target: R16) -> MTime {
        let before = self.get_r16(target);
        let after = before.wrapping_add(1);
        self.set_r16(target, after);

        // Always 2 cycles
        2
    }

    pub(super) fn op_dec_16(&mut self, target: R16) -> MTime {
        let before = self.get_r16(target);
        let after = before.wrapping_sub(1);
        self.set_r16(target, after);

        // Always 2 cycles
        2
    }
}
