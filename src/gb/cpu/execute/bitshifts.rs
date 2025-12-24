use super::*;

impl CPU {
    pub(super) fn op_rl(
        &mut self,
        mmu: &mut MMU,
        target: R8,
        through_carry: bool,
        fast: bool,
    ) -> MTime {
        let before = self.get_r8(mmu, target);
        let rotated_out = (before & 0b1000_0000) != 0;
        let mut result = before << 1;
        if through_carry {
            result |= self.getf_c() as u8;
        } else {
            result |= rotated_out as u8;
        }
        self.set_r8(mmu, target, result);

        self.set_all_flags(
            if fast { false } else { result == 0 },
            false,
            false,
            rotated_out,
        );

        match target {
            _ if fast => 1,
            R8::MHL => 4,
            _ => 2,
        }
    }

    pub(super) fn op_rr(
        &mut self,
        mmu: &mut MMU,
        target: R8,
        through_carry: bool,
        fast: bool,
    ) -> MTime {
        let before = self.get_r8(mmu, target);
        let rotated_out = (before & 0b1) << 7;
        let mut result = before >> 1;
        if through_carry {
            result |= (self.getf_c() as u8) << 7;
        } else {
            result |= rotated_out;
        }
        self.set_r8(mmu, target, result);

        self.set_all_flags(
            if fast { false } else { result == 0 },
            false,
            false,
            rotated_out != 0,
        );

        match target {
            _ if fast => 1,
            R8::MHL => 4,
            _ => 2,
        }
    }

    pub(super) fn op_sl(&mut self, mmu: &mut MMU, target: R8) -> MTime {
        let before = self.get_r8(mmu, target);
        let shifted_out = (before & 0b1000_0000) != 0;
        let result = before << 1;
        self.set_r8(mmu, target, result);

        self.set_all_flags(result == 0, false, false, shifted_out);

        match target {
            R8::MHL => 4,
            _ => 2,
        }
    }

    pub(super) fn op_sr(&mut self, mmu: &mut MMU, target: R8, arith: bool) -> MTime {
        let before = self.get_r8(mmu, target);
        let shifted_out = (before & 0b1) != 0;
        let leftmost = before & 0b1000_0000;
        let mut result = before >> 1;
        if arith {
            result |= leftmost;
        }
        self.set_r8(mmu, target, result);

        self.set_all_flags(result == 0, false, false, shifted_out);

        match target {
            R8::MHL => 4,
            _ => 2,
        }
    }

    pub(super) fn op_swap(&mut self, mmu: &mut MMU, target: R8) -> MTime {
        let before = self.get_r8(mmu, target);
        let upper = before & 0xF0;
        let lower = before & 0x0F;
        let result = (lower << 4) | (upper >> 4);
        self.set_r8(mmu, target, result);

        self.set_all_flags(result == 0, false, false, false);

        match target {
            R8::MHL => 4,
            _ => 2,
        }
    }
}
