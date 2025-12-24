use log::debug;

use crate::gb::cpu::{MTime, CPU};

impl CPU {
    // DI (m: 1)
    pub(super) fn op_di(&mut self) -> MTime {
        debug!(
            "[PC {:?}] DI: Interrupts disabled.",
            self.this_instruction_pc
        );
        self.ime = false;
        1.into()
    }

    // EI (m: 1)
    pub(super) fn op_ei(&mut self) -> MTime {
        debug!(
            "[PC {:?}] EI: Interrupts enabled; effect delayed by 1 m-cycle.",
            self.this_instruction_pc
        );
        self.will_set_ime = true;
        1.into()
    }

    // TODO: HALT (m: --)
    pub(super) fn op_halt(&self) -> MTime {
        todo!("HALT")
    }
}
