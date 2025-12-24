use crate::gb::cpu::{MTime, CPU};

impl CPU {
    // DAA (m: 1)
    pub(super) fn op_daa(&mut self) -> MTime {
        let mut adj = 0u8;
        if self.getf_s() {
            if self.getf_hc() {
                adj += 0x6;
            }
            if self.getf_c() {
                adj += 0x60;
            }
            self.a = self.a.wrapping_sub(adj);
        } else {
            if self.getf_hc() || (self.a & 0xF) > 0x9 {
                adj += 0x6;
            }
            if self.getf_c() || self.a > 0x99 {
                adj += 0x60;
                self.setf_c(true);
            }
            self.a = self.a.wrapping_add(adj);
        }

        self.setf_z(self.a == 0);
        self.setf_hc(false);

        1.into()
    }

    // NOP (m: 1)
    pub(super) fn op_nop(&self) -> MTime {
        // Wait one cycle and do nothing
        1.into()
    }

    // TODO: STOP (m: --)
    pub(super) fn op_stop(&self, n8: u8) -> MTime {
        let _ = n8;
        todo!("STOP");
    }
}
