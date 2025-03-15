use log::warn;

use crate::{
    gb::cpu::{MTime, CPU},
    util::error_and_panic,
};

impl CPU {
    // INVALID (m: 1)
    // In debug mode, we print a warning but otherwise treat it as NOP
    pub(super) fn op_invalid(&mut self) -> MTime {
        if self.debug_mode {
            warn!(
                "[PC {:?}] Ignoring an invalid instruction: {:?}",
                self.this_instruction_pc, self.this_instruction_code
            );
            self.op_nop()
        } else {
            error_and_panic!(
                "[PC {:?}] Attempted to execute an invalid instruction: {:?}",
                self.this_instruction_pc,
                self.this_instruction_code
            );
        }
    }

    // TERMINATE (m: 0)
    pub(super) fn op_terminate(&mut self) -> MTime {
        if self.debug_mode {
            self.terminate = true;
            0.into()
        } else {
            // The decoder should return an invalid instruction when not in debug mode, so this
            // should be unreachable.
            self.op_invalid()
        }
    }

    // DEBUG_PRINT (m: 0)
    pub(super) fn op_debug_print(&mut self) -> MTime {
        if self.debug_mode {
            self.debug_print();
            0.into()
        } else {
            // The decoder should return an invalid instruction when not in debug mode, so this
            // should be unreachable.
            self.op_invalid()
        }
    }
}
