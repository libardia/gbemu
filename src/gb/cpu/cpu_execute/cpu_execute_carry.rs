use crate::gb::cpu::{MTime, CPU};

impl CPU {
    // CCF (m: 1)
    // SCF (m: 1)
    pub(super) fn op_cf(&mut self, is_set: bool) -> MTime {
        self.setf_c(if is_set { true } else { !self.getf_c() });
        self.setf_hc(false);
        self.setf_s(false);
        1.into()
    }
}
