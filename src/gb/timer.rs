use std::{cell::RefCell, rc::Rc};

use crate::{
    mem_region::io_regs::{REG_DIV, REG_IF, REG_TAC, REG_TIMA, REG_TMA},
    util::{bit_flag, byte_of, new},
};

use super::{mmu::MMU, time_types::TTime};

#[derive(Debug, Default)]
pub struct Timer {
    mmu: Rc<RefCell<MMU>>,
    div: u16,
    tick_last: bool,
    div_overflow: bool,
    tima: u8,
    tma: u8,
    tac: u8,
    interrupt_flags: u8,
}

impl Timer {
    new!(mmu: Rc<RefCell<MMU>>);

    pub fn step(&mut self, dt: TTime) {
        // Get registers
        self.tima = self.mmu_get(REG_TIMA);
        self.tma = self.mmu_get(REG_TMA);
        self.tac = self.mmu_get(REG_TAC);
        self.interrupt_flags = self.mmu_get(REG_IF);

        // Reset div if it was written to
        if self.mmu.borrow_mut().should_reset_div() {
            self.div = 0;
        }

        for _ in 0..dt.to() {
            // Increment div
            self.div = self.div.wrapping_add(1);

            let tick = self.timer_enabled() && self.timer_clock_bit();
            // Falling edge of tick
            if !tick && self.tick_last {
                // Increment TIMA
                if self.tima == 0xFF {
                    // If TIMA would overflow, reset to TMA and request interrupt
                    self.tima = self.tma;
                    self.set_timer_interrupt(true);
                } else {
                    self.tima += 1;
                }
            }

            self.tick_last = tick;
        }

        // Set registers
        self.mmu_set(REG_DIV, byte_of!(self.div, 1));
        self.mmu_set(REG_TIMA, self.tima);
        self.mmu_set(REG_IF, self.interrupt_flags);
    }

    fn mmu_get(&self, address: u16) -> u8 {
        self.mmu.borrow().get(address)
    }

    fn mmu_set(&self, address: u16, value: u8) {
        self.mmu.borrow_mut().set(address, value);
    }

    fn timer_clock_bit(&self) -> bool {
        let b1 = self.tac & 0b10 != 0;
        let b2 = self.tac & 0b1 != 0;
        let mask = match (b1, b2) {
            (false, false) => 1 << 9,
            (false, true) => 1 << 3,
            (true, false) => 1 << 5,
            (true, true) => 1 << 7,
        };
        self.div & mask != 0
    }

    bit_flag!(get => timer_enabled, tac, 2);

    bit_flag!(set => set_timer_interrupt, interrupt_flags, 2);
}
