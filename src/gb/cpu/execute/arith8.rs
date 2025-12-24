use super::*;

impl CPU {
    pub(super) fn op_add_8(&mut self, mmu: &MMU, op: R8, with_carry: bool) -> MTime {
        let lhs = self.a;
        let rhs = self.get_r8(mmu, op);
        let cv = (with_carry && self.getf_c()) as u8;
        let (result, overflow1) = lhs.overflowing_add(rhs);
        let (result, overflow2) = result.overflowing_add(cv);
        self.a = result;

        self.set_all_flags(
            result == 0,
            false,
            ((lhs & 0xF) + (rhs & 0xF) + cv) > 0xF,
            overflow1 || overflow2,
        );

        match op {
            R8::MHL | R8::IMM(_) => 2,
            _ => 1,
        }
    }

    pub(super) fn op_sub_8(&mut self, mmu: &MMU, op: R8, with_carry: bool) -> MTime {
        // This is a seperate sub-function because "compare" is identical
        self.a = self.subtract_internal(mmu, op, with_carry);

        match op {
            R8::MHL | R8::IMM(_) => 2,
            _ => 1,
        }
    }

    pub(super) fn op_cp_8(&mut self, mmu: &MMU, op: R8) -> MTime {
        // Do a subtraction and set flags accordingly, but ignore the result
        self.subtract_internal(mmu, op, false);

        match op {
            R8::MHL | R8::IMM(_) => 2,
            _ => 1,
        }
    }

    pub(super) fn op_inc_8(&mut self, mmu: &mut MMU, target: R8) -> MTime {
        let before = self.get_r8(mmu, target);
        let after = before.wrapping_add(1);
        self.set_r8(mmu, target, after);

        self.setf_z(after == 0);
        self.setf_n(false);
        self.setf_h(before & 0xF == 0xF);

        match target {
            R8::MHL => 3,
            _ => 1,
        }
    }

    pub(super) fn op_dec_8(&mut self, mmu: &mut MMU, target: R8) -> MTime {
        let before = self.get_r8(mmu, target);
        let after = before.wrapping_sub(1);
        self.set_r8(mmu, target, after);

        self.setf_z(after == 0);
        self.setf_n(true);
        self.setf_h(before & 0xF == 0);

        match target {
            R8::MHL => 3,
            _ => 1,
        }
    }

    fn subtract_internal(&mut self, mmu: &MMU, op: R8, with_carry: bool) -> u8 {
        let lhs = self.a;
        let rhs = self.get_r8(mmu, op);
        let cv = (with_carry && self.getf_c()) as u8;
        let (result, overflow1) = lhs.overflowing_sub(rhs);
        let (result, overflow2) = result.overflowing_sub(cv);

        self.set_all_flags(
            result == 0,
            true,
            ((lhs & 0xF) as i8 - (rhs & 0xF) as i8 - cv as i8) < 0,
            overflow1 || overflow2,
        );

        result
    }
}
