use instructions::Instruction;
use log::{debug, trace};
use std::{cell::RefCell, rc::Rc};

use crate::util::{bit_flag, either, input, new};

use super::{mmu::MMU, time_types::MTime};

mod instructions;
mod optables;

#[derive(Debug, Default)]
pub struct CPU {
    // Reference to MMU
    mmu: Rc<RefCell<MMU>>,
    // Registers
    pc: u16,   // Program counter
    sp: u16,   // Stack pointer
    ime: bool, // Interrupt master enable
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    a: u8,
    h: u8,
    l: u8,
    f: u8,
    // Flags
    int_enabled: u8,
    int_flags: u8,
    // Needed to emulate IME flag delay
    will_set_ime: bool,
    setting_ime: bool,
    // For logging
    this_instruction_pc: u16,
    this_instruction_code: u8,
    this_instruction: Instruction,
    // For debugging
    pub debug_mode: bool,
    pub terminate: bool,
    pub debug_print: bool,
}

impl CPU {
    new!(mmu: Rc<RefCell<MMU>>);

    pub fn step(&mut self) -> MTime {
        const INTERRUPT_TIME: MTime = MTime::make(5);

        // Record current PC (for logging)
        self.this_instruction_pc = self.pc;
        self.this_instruction_code = self.mmu_read_byte(self.pc);

        if self.ime {
            // Check for interrupt
            let did_interrupt = self.maybe_interrupt();

            // If we did interrupt, return the time it took to hand off to the interrupt. This
            // always takes the same amount of time.
            if did_interrupt {
                return INTERRUPT_TIME;
            }
        }

        // Decode the instruction at PC
        let instruction = self.decode();

        // Record current instruction (for logging)
        self.this_instruction = instruction;

        // println!(
        //     "[PC 0x{:0>4X}] {:?}",
        //     self.this_instruction_pc, self.this_instruction
        // );
        // self.pretty_print();
        // input("");

        // Execute the instruction
        let cycles_elapsed = self.execute(instruction);

        // Return how many cycles the instruction took
        cycles_elapsed
    }

    /* #region MMU convenience ================================================================= */

    fn mmu_read_byte(&self, address: u16) -> u8 {
        self.mmu.borrow().read_byte(address)
    }

    fn mmu_write_byte(&self, address: u16, value: u8) {
        self.mmu.borrow_mut().write_byte(address, value);
    }

    fn mmu_read_word(&self, address: u16) -> u16 {
        self.mmu.borrow().read_word(address)
    }

    fn mmu_write_word(&self, address: u16, value: u16) {
        self.mmu.borrow_mut().write_word(address, value);
    }

    /* #endregion */

    /* #region Registers ======================================================================= */

    fn get_hl_then_inc(&mut self) -> u16 {
        let before = self.get_hl();
        self.set_hl(before.wrapping_add(1));
        before
    }

    fn get_hl_then_dec(&mut self) -> u16 {
        let before = self.get_hl();
        self.set_hl(before.wrapping_sub(1));
        before
    }

    fn set_all_flags(&mut self, z: bool, s: bool, hc: bool, c: bool) {
        self.f = either!(z => 1 << 7; 0)
            | either!(s => 1 << 6; 0)
            | either!(hc => 1 << 5; 0)
            | either!(c => 1 << 4; 0)
    }

    bit_flag!(getf_z, setf_z, f, 7);
    bit_flag!(getf_s, setf_s, f, 6);
    bit_flag!(getf_hc, setf_hc, f, 5);
    bit_flag!(getf_c, setf_c, f, 4);

    getset_r16!(get_bc, set_bc, b, c);
    getset_r16!(get_de, set_de, d, e);
    getset_r16!(get_hl, set_hl, h, l);
    getset_r16!(get_af, set_af, a, f);

    /* #endregion */

    /* #region Stack =========================================================================== */

    fn push_word(&mut self, value: u16) {
        self.sp = self.sp.wrapping_sub(2);
        self.mmu_write_word(self.sp, value);
    }

    fn pop_word(&mut self) -> u16 {
        let word = self.mmu_read_word(self.sp);
        self.sp = self.sp.wrapping_add(2);
        word
    }

    #[rustfmt::skip]
    pub fn pretty_print(&self) {
        println!("+--------------------------+");
        println!("| PC: 0x{:0>4X}    SP: 0x{:0>4X} | IME: {}", self.pc, self.sp, self.ime);
        println!("| A:  0x{:0>2X}      F:  {:0>4b}   |", self.a, self.f >> 4);
        println!("| B:  0x{:0>2X}      C:  0x{:0>2X}   |", self.b, self.c);
        println!("| D:  0x{:0>2X}      E:  0x{:0>2X}   |", self.d, self.e);
        println!("| H:  0x{:0>2X}      L:  0x{:0>2X}   |", self.h, self.l);
        println!("+--------------------------+");
    }

    /* #endregion */
}

macro_rules! getset_r16 {
    ($getname:ident, $setname:ident, $r1:ident, $r2:ident) => {
        pub fn $getname(&self) -> u16 {
            let r1 = self.$r1 as u16;
            let r2 = self.$r2 as u16;
            (r1 << 8 | r2)
        }

        pub fn $setname(&mut self, value: u16) {
            self.$r1 = ((value & 0xFF00) >> 8) as u8;
            self.$r2 = (value & 0xFF) as u8;
        }
    };
}
pub(self) use getset_r16;

mod cpu_decode;
mod cpu_execute;
mod cpu_interrupt;
