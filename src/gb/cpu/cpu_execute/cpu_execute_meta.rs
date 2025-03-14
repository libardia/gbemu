use log::warn;

use crate::gb::cpu::{MTime, CPU};

impl CPU {
    // INVALID (m: 1)
    // In debug mode, we print a warning but otherwise treat it as NOP
    pub(super) fn op_invalid(&mut self) -> MTime {
        if self.debug_mode {
            warn!(
                "[PC {:?}] Ignoring an invalid instruction: {:?} -> {:?}",
                self.this_instruction_pc, self.this_instruction_code, self.this_instruction
            );
            self.op_nop()
        } else {
            panic!(
                "[PC {:?}] Attempted to execute an invalid instruction: {:?} -> {:?}",
                self.this_instruction_pc, self.this_instruction_code, self.this_instruction
            );
        }
    }

    // TERMINATE (m: 0)
    pub(super) fn op_terminate(&mut self) -> MTime {
        if self.debug_mode {
            self.terminate = true;
            0.into()
        } else {
            self.op_invalid()
        }
    }

    // DEBUG_PRINT (m: 0)
    pub(super) fn op_debug_print(&mut self) -> MTime {
        if self.debug_mode {
            self.debug_print = true;
            0.into()
        } else {
            self.op_invalid()
        }
    }
}
