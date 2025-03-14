use crate::gb::cpu::{instructions::ArgR8, MTime, CPU};

impl CPU {
    // AND A,r8   (m: 1)
    // AND A,[HL] (m: 2)
    // AND A,n8   (m: 2)
    pub(super) fn op_and_r8(&mut self, r8: ArgR8) -> MTime {
        let value = self.get_r8(r8);
        let result = self.a & value;

        self.set_all_flags(result == 0, false, true, false);

        self.a = result;

        decide_time_for_r8!(r8; 2, 1)
    }

    // CPL (m: 1)
    pub(super) fn op_cpl(&mut self) -> MTime {
        self.a = !self.a;

        self.setf_s(true);
        self.setf_hc(true);

        1.into()
    }

    // OR A,r8   (m: 1)
    // OR A,[HL] (m: 2)
    // OR A,n8   (m: 2)
    pub(super) fn op_or_r8(&mut self, r8: ArgR8) -> MTime {
        let value = self.get_r8(r8);
        let result = self.a | value;

        self.set_all_flags(result == 0, false, false, false);

        self.a = result;

        decide_time_for_r8!(r8; 2, 1)
    }

    // XOR A,r8   (m: 1)
    // XOR A,[HL] (m: 2)
    // XOR A,n8   (m: 2)
    pub(super) fn op_xor_r8(&mut self, r8: ArgR8) -> MTime {
        let value = self.get_r8(r8);
        let result = self.a ^ value;

        self.set_all_flags(result == 0, false, false, false);

        self.a = result;

        decide_time_for_r8!(r8; 2, 1)
    }
}
