use super::*;

impl CPU {
    pub(super) fn op_and(&mut self, mmu: &mut MMU, op: R8) -> MTime {
        self.a &= self.get_r8(mmu, op);
        self.set_all_flags(self.a == 0, false, true, false);

        match op {
            R8::MHL | R8::IMM(_) => 2,
            _ => 1,
        }
    }

    pub(super) fn op_or(&mut self, mmu: &mut MMU, op: R8) -> MTime {
        self.a |= self.get_r8(mmu, op);
        self.set_all_flags(self.a == 0, false, false, false);

        match op {
            R8::MHL | R8::IMM(_) => 2,
            _ => 1,
        }
    }

    pub(super) fn op_xor(&mut self, mmu: &mut MMU, op: R8) -> MTime {
        self.a ^= self.get_r8(mmu, op);
        self.set_all_flags(self.a == 0, false, false, false);

        match op {
            R8::MHL | R8::IMM(_) => 2,
            _ => 1,
        }
    }

    pub(super) fn op_cpl(&mut self) -> MTime {
        self.a = !self.a;
        self.setf_n(true);
        self.setf_h(true);

        // Always 1 cycle
        1
    }
}
