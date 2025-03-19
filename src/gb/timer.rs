use std::{cell::RefCell, rc::Rc};

use log::trace;

use crate::{
    mem_region::io_regs::{REG_DIV, REG_IF, REG_TAC, REG_TIMA, REG_TMA},
    util::{bit_flag, byte_of, new},
};

use super::{mmu::MMU, time_types::TTime};

#[derive(Debug, Default)]
pub struct Timer {
    mmu: Rc<RefCell<MMU>>,
    div: u16,
    div_last: u16,
    div_overflow: bool,
    tima: u8,
    tma: u8,
    tac: u8,
    interrupt_flags: u8,
}

impl Timer {
    new!(mmu: Rc<RefCell<MMU>>);

    pub fn step(&mut self, dt: TTime) {
        let dt_u16 = dt.to() as u16;
        let should_reset = self.mmu.borrow_mut().should_reset_div();

        // Get registers
        self.tima = self.mmu_get(REG_TIMA);
        self.tma = self.mmu_get(REG_TMA);
        self.tac = self.mmu_get(REG_TAC);
        self.interrupt_flags = self.mmu_get(REG_IF);

        // Update or reset div
        self.div_last = self.div;
        if should_reset {
            self.div = 0;
            self.div_overflow = false;
        } else {
            let (newdiv, overflow) = self.div.overflowing_add(dt_u16);
            self.div = newdiv;
            self.div_overflow = overflow;
        }

        // Test for TIMA increment
        if !should_reset && self.timer_enabled() {
            let mask = self.timer_clock_mask();
            let masked_before = self.div_last & mask;
            if masked_before + dt_u16 > mask {
                // masked_before + dt > mask means it overflowed past `mask`, so TIMA should inc
                if self.tima == 0xFF {
                    // TIMA overflowed
                    trace!("Requesting TIMA interrupt");
                    self.tima = self.tma;
                    self.set_timer_interrupt(true);
                } else {
                    self.tima += 1;
                }
            }
        }

        // Set registers
        self.mmu_set(REG_DIV, byte_of!(self.div, 1));
        self.mmu_set(REG_TIMA, self.tima);
    }

    fn mmu_get(&self, address: u16) -> u8 {
        self.mmu.borrow().get(address)
    }

    fn mmu_set(&self, address: u16, value: u8) {
        self.mmu.borrow_mut().set(address, value);
    }

    fn timer_clock_mask(&self) -> u16 {
        let b1 = self.tac & 0b10 != 0;
        let b2 = self.tac & 0b1 != 0;
        match (b1, b2) {
            (false, false) => 1023,
            (false, true) => 15,
            (true, false) => 63,
            (true, true) => 255,
        }
    }

    bit_flag!(get => timer_enabled, tac, 2);

    bit_flag!(set => set_timer_interrupt, interrupt_flags, 2);
}
