use crate::gb::cpu::{instructions::ArgR16STK, MTime, CPU};

impl CPU {
    // ADD SP,e8 (m: 4)
    pub(super) fn op_add_sp_e8(&mut self, offset: i8) -> MTime {
        let osp = self.sp;
        let asu16 = offset as u16;

        // I really don't understand how the flags are supposed to work here. This is the best I
        // could figure from the docs I could find. Hopefully Blaarg's test ROMs will make this
        // clear.
        let (nhc, nc) = if offset > 0 {
            let nibble_sum = (asu16 & 0xF) + (osp & 0xF);
            let byte_sum = (asu16 & 0xFF) + (osp & 0xFF);
            (nibble_sum > 0xF, byte_sum > 0xFF)
        } else {
            (false, false)
        };

        self.set_all_flags(false, false, nhc, nc);

        self.sp = osp.wrapping_add(asu16);

        4.into()
    }

    // LD [n16],SP (m: 5)
    pub(super) fn op_ld_mn16_sp(&self, mn16: u16) -> MTime {
        self.mmu_write_word(mn16, self.sp);
        5.into()
    }

    // LD HL,SP+e8 (m: 3)
    pub(super) fn op_ld_hl_sp_plus_e8(&mut self, offset: i8) -> MTime {
        let osp = self.sp;
        let asu16 = offset as u16;

        // I really don't understand how the flags are supposed to work here. This is the best I
        // could figure from the docs I could find. Hopefully Blaarg's test ROMs will make this
        // clear.
        let (nhc, nc) = if offset > 0 {
            let nibble_sum = (asu16 & 0xF) + (osp & 0xF);
            let byte_sum = (asu16 & 0xFF) + (osp & 0xFF);
            (nibble_sum > 0xF, byte_sum > 0xFF)
        } else {
            (false, false)
        };

        self.set_all_flags(false, false, nhc, nc);

        self.set_hl(osp.wrapping_add(asu16));

        3.into()
    }

    // LD SP,HL (m: 2)
    pub(super) fn op_ld_sp_hl(&mut self) -> MTime {
        self.sp = self.get_hl();
        2.into()
    }

    // POP AF      (m: 3)
    // POP r16     (m: 3)
    pub(super) fn op_pop_r16(&mut self, r16: ArgR16STK) -> MTime {
        let value = self.pop_word();
        self.set_r16stk(r16, value);
        3.into()
    }

    // PUSH AF  (m: 4)
    // PUSH r16 (m: 4)
    pub(super) fn op_push_r16(&mut self, r16: ArgR16STK) -> MTime {
        self.push_word(self.get_r16stk(r16));
        4.into()
    }
}
