use super::*;

impl CPU {
    pub(super) fn op_jump(&mut self, cond: Cond, address: Mem) -> MTime {
        if self.test_condition(cond) {
            self.pc = self.address_from_mem(address);

            match cond {
                Cond::ALWAYS => 1, // Special fast version for 'JP HL'
                _ => 4,
            }
        } else {
            // The only situation where the jump isn't taken is when the condition of 'JP cc n16'
            // fails, in which case it takes 3 cycles
            3
        }
    }

    pub(super) fn op_jump_rel(&mut self, cond: Cond, off: i8) -> MTime {
        if self.test_condition(cond) {
            self.pc = self.pc.wrapping_add_signed(off as i16);
            // 3 cycles if jump
            3
        } else {
            // 2 cycles if no jump
            2
        }
    }

    pub(super) fn op_call(&mut self, mmu: &mut MMU, cond: Cond, address: u16) -> MTime {
        if self.test_condition(cond) {
            self.push_stack(mmu, self.pc);
            self.pc = address;
            // 6 cycles if jump
            6
        } else {
            // 3 cycles if no jump
            3
        }
    }

    pub(super) fn op_ret(&mut self, mmu: &mut MMU, cond: Cond, enable_interrupts: bool) -> MTime {
        // This can happen now, because interrupts can only trigger between instructions
        if enable_interrupts {
            self.ime = true;
        }

        if self.test_condition(cond) {
            self.pc = self.pop_stack(mmu);

            match cond {
                Cond::ALWAYS => 4,
                _ => 5,
            }
        } else {
            // The only situation where the jump isn't taken is when the condition of 'RET cc'
            // fails, in which case it takes 2 cycles
            2
        }
    }

    pub(super) fn op_rst(&mut self, mmu: &mut MMU, address: u16) -> MTime {
        // There's only one path for this one: always jump, always takes 4 cycles.
        self.push_stack(mmu, self.pc);
        self.pc = address;
        4
    }
}
