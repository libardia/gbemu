use super::*;

impl CPU {
    pub(super) fn op_load_r8_r8(&mut self, mmu: &mut MMU, dest: R8, src: R8) -> MTime {
        let value = self.get_r8(mmu, src);
        self.set_r8(mmu, dest, value);

        match (dest, src) {
            (R8::MHL | R8::IMM(_), R8::MHL | R8::IMM(_)) => 3,
            (R8::MHL | R8::IMM(_), _) | (_, R8::MHL | R8::IMM(_)) => 2,
            _ => 1,
        }
    }

    pub(super) fn op_load_r8_mem(&mut self, mmu: &mut MMU, dest: R8, src: Mem) -> MTime {
        let value = self.get_mem(mmu, src);
        self.set_r8(mmu, dest, value);

        match src {
            Mem::IMM(_) => 4,
            _ => 2,
        }
    }

    pub(super) fn op_load_mem_r8(&mut self, mmu: &mut MMU, dest: Mem, src: R8) -> MTime {
        let value = self.get_r8(mmu, src);
        self.set_mem(mmu, dest, value);

        match dest {
            Mem::IMM(_) => 4,
            _ => 2,
        }
    }

    pub(super) fn op_load_r16_r16(&mut self, dest: R16, src: R16) -> MTime {
        let value = self.get_r16(src);
        self.set_r16(dest, value);

        // Always 3 cycles
        3
    }

    pub(super) fn op_loadhigh_a_mem(&mut self, mmu: &mut MMU, src: Mem) -> MTime {
        self.a = self.get_mem(mmu, src);

        match src {
            Mem::HIGH_IMM(_) => 3,
            _ => 2,
        }
    }

    pub(super) fn op_loadhigh_mem_a(&mut self, mmu: &mut MMU, dest: Mem) -> MTime {
        self.set_mem(mmu, dest, self.a);

        match dest {
            Mem::HIGH_IMM(_) => 3,
            _ => 2,
        }
    }
}
