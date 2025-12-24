use log::debug;

use crate::gb::cpu::{
    instructions::{ArgCOND, ArgVEC},
    MTime, CPU,
};

impl CPU {
    fn eval_condition(&self, condition: ArgCOND) -> bool {
        match condition {
            ArgCOND::NZ => !self.getf_z(),
            ArgCOND::Z => self.getf_z(),
            ArgCOND::NC => !self.getf_c(),
            ArgCOND::C => self.getf_c(),
            ArgCOND::ALWAYS => true,
        }
    }

    // CALL n16    (m:   6)
    // CALL cc,n16 (m: 6/3)
    pub(super) fn op_call_cc_n16(&mut self, cond: ArgCOND, n16: u16) -> MTime {
        if self.eval_condition(cond) {
            self.push_word(self.pc);
            self.pc = n16;
            6.into()
        } else {
            3.into()
        }
    }

    // JP HL (m: 1)
    pub(super) fn op_jp_hl(&mut self) -> MTime {
        self.pc = self.get_hl();
        1.into()
    }

    // JP n16    (m:   4)
    // JP cc,n16 (m: 4/3)
    pub(super) fn op_jp_cc_n16(&mut self, cond: ArgCOND, n16: u16) -> MTime {
        if self.eval_condition(cond) {
            self.pc = n16;
            4.into()
        } else {
            3.into()
        }
    }

    // JR n16    (m:   3)
    // JR cc,n16 (m: 3/2)
    pub(super) fn op_jr_cc_n16(&mut self, cond: ArgCOND, offset: i8) -> MTime {
        if self.eval_condition(cond) {
            // When offset is converted to u16, it will be filled with the same bits as an i16.
            // Because of two's complement, adding the reults (allowing for overflow) is exactly
            // the same as subtracting, if offset was negative.
            self.pc = self.pc.wrapping_add(offset as u16);
            3.into()
        } else {
            2.into()
        }
    }

    // RET cc (m: 5/2)
    pub(super) fn op_ret_cc(&mut self, cond: ArgCOND) -> MTime {
        if self.eval_condition(cond) {
            self.pc = self.pop_word();
            5.into()
        } else {
            2.into()
        }
    }

    // RET  (m: 4)
    // RETI (m: 4)
    pub(super) fn op_ret(&mut self, enable_interrupts: bool) -> MTime {
        if enable_interrupts {
            debug!(
                "[PC {:?}] RETI: Interrupts enabled immediately.",
                self.this_instruction_pc
            );
            // Because this is equivalent to EI then RET, the IME flag is actually set at the end
            // of this insctruction
            self.ime = true;
        }
        self.pc = self.pop_word();

        4.into()
    }

    // RST vec (m: 4)
    pub(super) fn op_rst(&mut self, vec: ArgVEC) -> MTime {
        self.push_word(self.pc);
        self.pc = vec as u16;

        4.into()
    }
}
