use super::*;
use crate::gb::regions::*;

impl CPU {
    /* #region Carry flag */
    pub(super) fn op_ccf(&mut self) -> MTime {
        self.setf_n(false);
        self.setf_h(false);
        self.setf_c(!self.getf_c());

        1
    }

    pub(super) fn op_scf(&mut self) -> MTime {
        self.setf_n(false);
        self.setf_h(false);
        self.setf_c(true);

        1
    }
    /* #endregion */

    /* #region Interrupts */
    pub(super) fn op_di(&mut self) -> MTime {
        self.ime = false;

        1
    }

    pub(super) fn op_ei(&mut self) -> MTime {
        // The interrupt flag isn't set until AFTER THE NEXT INSTRUCTION.
        self.ei_state = EIState::Waiting;

        1
    }

    pub(super) fn op_halt(&mut self, mmu: &MMU) -> MTime {
        let int_pending = (mmu.get(IO_IE) & mmu.get(IO_IF)) != 0;

        if !self.ime && int_pending {
            // Halt bug is triggered!
            self.halt_bug = true;
        } else {
            // Enter halt mode
            self.mode = CPUMode::Halt;
        }

        1
    }
    /* #endregion */

    /* #region Misc */
    pub(super) fn op_daa(&mut self) -> MTime {
        let mut adj = 0u8;

        if self.getf_n() {
            if self.getf_h() {
                adj += 0x6;
            }
            if self.getf_c() {
                adj += 0x60;
            }
            self.a = self.a.wrapping_sub(adj);
        } else {
            if self.getf_h() || (self.a & 0xF) > 0x9 {
                adj += 0x6;
            }
            if self.getf_c() || self.a > 0x99 {
                adj += 0x60;
                self.setf_c(true);
            }
            self.a = self.a.wrapping_add(adj);
        }

        self.setf_z(self.a == 0);
        self.setf_h(false);

        1
    }

    pub(super) fn op_stop(&self) -> MTime {
        // STOP is completely insane
        // https://gbdev.io/pandocs/Reducing_Power_Consumption.html#the-bizarre-case-of-the-game-boy-stop-instruction-before-even-considering-timing
        // TODO: STOP
        todo!()
    }
    /* #endregion */
}
