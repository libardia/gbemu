use crate::gb::cpu::{
    instructions::{ArgR8, ArgU3},
    MTime, CPU,
};

impl CPU {
    // BIT u3,r8   (m: 2)
    // BIT u3,[HL] (m: 3)
    pub(super) fn op_bit_u3_r8(&mut self, bit: ArgU3, r8: ArgR8) -> MTime {
        if matches!(r8, ArgR8::CONST(_)) {
            self.panic_no_const();
        }

        let value = self.get_r8(r8);

        self.setf_z(value & (bit as u8) == 0);
        self.setf_s(false);
        self.setf_hc(true);

        decide_time_for_r8!(r8; 3, 2)
    }

    // RES u3,r8   (m: 2)
    // RES u3,[HL] (m: 4)
    // SET u3,r8   (m: 2)
    // SET u3,[HL] (m: 4)
    pub(super) fn op_set_u3_r8(&mut self, bit: ArgU3, r8: ArgR8, set: bool) -> MTime {
        if matches!(r8, ArgR8::CONST(_)) {
            self.panic_no_const();
        }

        let value = self.get_r8(r8);

        let new_value = if set {
            value | (bit as u8)
        } else {
            value & !(bit as u8)
        };

        self.set_r8(r8, new_value);

        decide_time_for_r8!(r8; 4, 2)
    }
}
