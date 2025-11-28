use super::*;

impl CPU {
    pub(super) fn op_add_sp_e8(&mut self, off: i8) -> MTime {
        // The flags here are extremely confusing. Here's how they actually work (I hope, this was
        // from someone on r/EmuDev):
        // https://www.reddit.com/r/EmuDev/comments/y51i1c/game_boy_dealing_with_carry_flags_when_handling/
        // In the real hardware, this operation is done by first doing an unsigned 8 bit addition
        // between LOW(SP) and off, and all flags are set from that. Then it translates that to a
        // signed addition to SP.
        self.weird_flags(off);

        // Okay, now do the real thing lol
        self.sp = self.sp.wrapping_add_signed(off as i16);
        4
    }

    pub(super) fn op_ld_hl_sp_e8(&mut self, off: i8) -> MTime {
        // This instruction wasn't mentioned in the same post as 'ADD SP e8', but by context it's
        // clear that the flags are handled the same way.
        self.weird_flags(off);

        // Now do the real thing
        self.set_hl(self.sp.wrapping_add_signed(off as i16));
        3
    }

    pub(super) fn op_ld_a16_sp(&self, mmu: &mut MMU, address: u16) -> MTime {
        mmu.set(address, (self.sp & 0xFF) as u8);
        mmu.set(address.wrapping_add(1), (self.sp >> 8) as u8);

        // Always takes 5 cycles
        5
    }

    pub(super) fn op_push(&mut self, mmu: &mut MMU, target: R16) -> MTime {
        self.push_stack(mmu, self.get_r16(target));
        4
    }

    pub(super) fn op_pop(&mut self, mmu: &mut MMU, target: R16) -> MTime {
        let value = self.pop_stack(mmu);
        self.set_r16(target, value);
        3
    }

    fn weird_flags(&mut self, off: i8) {
        let low_sp = (self.sp & 0xFF) as u8;
        let uoff = off as u8;

        self.set_all_flags(
            false,
            false,
            ((low_sp & 0xF) + (uoff & 0xF)) > 0xF,
            low_sp.overflowing_add(uoff).1,
        );
    }
}
