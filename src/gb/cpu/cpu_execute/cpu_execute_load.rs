use crate::gb::cpu::{
    instructions::{ArgR16, ArgR16MEM, ArgR8},
    MTime, CPU,
};

impl CPU {
    // LD r8,r8   (m: 1)
    // LD r8,n8   (m: 2)
    // LD [HL],r8 (m: 2)
    // LD [HL],n8 (m: 3)
    // LD r8,[HL] (m: 2)
    pub(super) fn op_ld_r8_r8(&mut self, r8_dest: ArgR8, r8_src: ArgR8) -> MTime {
        if r8_dest == ArgR8::MHL && r8_src == ArgR8::MHL {
            self.panic_impossible_arguments();
        } else if r8_dest == r8_src {
            // Self-assignment is a nop (1 m-cycle)
            return self.op_nop();
        }

        let value = self.get_r8(r8_src);
        self.set_r8(r8_dest, value);

        match (r8_dest, r8_src) {
            (ArgR8::MHL, ArgR8::CONST(_)) => 3,
            (ArgR8::MHL, _) => 2,
            (_, ArgR8::MHL) => 2,
            (_, ArgR8::CONST(_)) => 2,
            (_, _) => 1,
        }
        .into()
    }

    // LD r16,n16 (m: 3)
    // LD SP,n16  (m: 3)
    pub(super) fn op_ld_r16_n16(&mut self, r16: ArgR16, n16: u16) -> MTime {
        self.set_r16(r16, n16);
        3.into()
    }

    // LD [r16],A (m: 2)
    // LD [n16],A (m: 4)
    // LD A,[r16] (m: 2)
    // LD A,[n16] (m: 4)
    // LD [HLI],A (m: 2)
    // LD [HLD],A (m: 2)
    // LD A,[HLI] (m: 2)
    // LD A,[HLD] (m: 2)
    pub(super) fn op_ld_a_mr16(&mut self, mr16: ArgR16MEM, a_is_dest: bool) -> MTime {
        if a_is_dest {
            self.a = self.get_mr16(mr16);
        } else {
            self.set_mr16(mr16, self.a);
        }

        match mr16 {
            ArgR16MEM::CONST(_) => 4,
            _ => 2,
        }
        .into()
    }

    // LDH [n16],A (m: 3)
    // LDH A,[n16] (m: 3)
    pub(super) fn op_ldh_a_mn16(&mut self, n8: u8, a_is_dest: bool) -> MTime {
        let address = 0xFF00 + (n8 as u16);

        if a_is_dest {
            self.a = self.mmu_read(address);
        } else {
            self.mmu_write(address, self.a);
        }

        3.into()
    }

    // LDH [C],A (m: 2)
    // LDH A,[C] (m: 2)
    pub(super) fn op_ldh_a_mc(&mut self, a_is_dest: bool) -> MTime {
        let address = 0xFF00 + (self.c as u16);

        if a_is_dest {
            self.a = self.mmu_read(address);
        } else {
            self.mmu_write(address, self.a);
        }

        2.into()
    }
}
