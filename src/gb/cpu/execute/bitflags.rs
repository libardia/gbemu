use super::*;

impl CPU {
    pub(super) fn op_bit(&mut self, mmu: &MMU, bit: u8, target: R8) -> MTime {
        let mask = 1 << bit;
        let byte = self.get_r8(mmu, target);

        self.setf_z(byte & mask == 0);
        self.setf_n(false);
        self.setf_h(true);

        match target {
            R8::MHL => 3,
            _ => 2,
        }
    }

    pub(super) fn op_set(&mut self, mmu: &mut MMU, bit: u8, target: R8) -> MTime {
        let mask = 1 << bit;
        let byte = self.get_r8(mmu, target);
        self.set_r8(mmu, target, byte | mask);

        match target {
            R8::MHL => 4,
            _ => 2,
        }
    }

    pub(super) fn op_res(&mut self, mmu: &mut MMU, bit: u8, target: R8) -> MTime {
        let mask = 1 << bit;
        let byte = self.get_r8(mmu, target);
        self.set_r8(mmu, target, byte & !mask);

        match target {
            R8::MHL => 4,
            _ => 2,
        }
    }
}
