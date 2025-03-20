use instructions::Instruction;
use std::{cell::RefCell, fmt::Display, rc::Rc};

use crate::{
    mem_region::io_regs::{REG_IE, REG_IF, REG_TIMA},
    util::{bit_flag, byte_of, either, input, new, Hex16, Hex8},
};

use super::{mmu::MMU, time_types::MTime};

pub mod instructions;
pub mod optables;

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
    this_instruction_pc: Hex16,
    this_instruction_code: Hex8,
    this_instruction: Instruction,
    // Control
    terminate: bool,
    // For debugging
    break_once_on: Option<u16>,
    pub breakpoints: Vec<u16>,
    pub debug_mode: bool,
    pub break_mode: bool,
}

impl CPU {
    new!(mmu: Rc<RefCell<MMU>>);

    pub fn should_terminate(&self) -> bool {
        self.terminate
    }

    pub fn step(&mut self) -> MTime {
        const INTERRUPT_TIME: MTime = MTime::make(5);

        // Update interrupt registers
        self.int_enabled = self.mmu_read(REG_IE);
        self.int_flags = self.mmu_read(REG_IF);

        // Breakpoint?
        if self.debug_mode {
            if self.break_once_on == Some(self.pc) {
                self.break_once_on = None;
                self.break_mode = true;
            } else if self.breakpoints.contains(&self.pc) {
                self.break_mode = true;
            }
        }

        // Record current PC (for logging)
        self.this_instruction_pc = self.pc.into();
        self.this_instruction_code = self.mmu_read(self.pc).into();

        if self.ime {
            // Check for interrupt
            let did_interrupt = self.maybe_interrupt();

            // If we did interrupt, return the time it took to hand off to the interrupt. This
            // always takes the same amount of time.
            if did_interrupt {
                self.mmu_write(REG_IF, self.int_flags);
                return INTERRUPT_TIME;
            }
        }

        // Decode the instruction at PC
        let instruction = self.decode();

        // Record current instruction (for logging)
        self.this_instruction = instruction;

        // Break, if we're in debug mode and breakpoint mode
        if self.debug_mode && self.break_mode {
            self.debug_break();
        }

        // Execute the instruction
        let cycles_elapsed = self.execute(instruction);

        // Return how many cycles the instruction took
        cycles_elapsed
    }

    /* #region Registers as hex ================================================================= */

    get_as_hex!(hpc, pc, Hex16);
    get_as_hex!(hsp, sp, Hex16);

    get_as_hex!(ha, a, Hex8);
    get_as_hex!(hf, f, Hex8);
    get_as_hex!(hb, b, Hex8);
    get_as_hex!(hc, c, Hex8);
    get_as_hex!(hd, d, Hex8);
    get_as_hex!(he, e, Hex8);
    get_as_hex!(hh, h, Hex8);
    get_as_hex!(hl, l, Hex8);

    /* #endregion */

    /* #region MMU convenience ================================================================== */

    fn mmu_read(&self, address: u16) -> u8 {
        self.mmu.borrow().cpu_read(address)
    }

    fn mmu_write(&self, address: u16, value: u8) {
        self.mmu.borrow_mut().cpu_write(address, value);
    }

    fn mmu_read_word(&self, address: u16) -> u16 {
        self.mmu.borrow().cpu_read_word(address)
    }

    fn mmu_write_word(&self, address: u16, value: u16) {
        self.mmu.borrow_mut().cpu_write_word(address, value);
    }

    /* #endregion */

    /* #region Registers ======================================================================== */

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

    /* #region Stack ============================================================================ */

    fn push_word(&mut self, value: u16) {
        self.sp = self.sp.wrapping_sub(2);
        self.mmu_write_word(self.sp, value);
    }

    fn pop_word(&mut self) -> u16 {
        let word = self.mmu_read_word(self.sp);
        self.sp = self.sp.wrapping_add(2);
        word
    }

    /* #endregion */

    /* #region Debugging ======================================================================== */

    fn debug_break(&mut self) {
        println!("BEFORE INSTRUCTION:");
        self.debug_print();
        println!("c: continue one step; x: exit break mode; bXXXX: add a breakpoint at address;");
        println!("wXXXX: wait until address but do not add breakpoint. Default 'c'.");
        let mut invalid_input = true;
        while invalid_input {
            invalid_input = false;
            let msg = input("> ");
            let trimmed = msg.trim();

            if trimmed == "x" {
                self.break_mode = false;
            } else if let Some(rest) = trimmed.strip_prefix("b") {
                if let Ok(a) = u16::from_str_radix(rest, 16) {
                    self.breakpoints.push(a);
                } else {
                    invalid_input = true;
                }
            } else if let Some(rest) = trimmed.strip_prefix("w") {
                if let Ok(a) = u16::from_str_radix(rest, 16) {
                    self.break_once_on = Some(a);
                    self.break_mode = false;
                } else {
                    invalid_input = true;
                }
            } else if trimmed == "c" || trimmed.is_empty() {
                // Do nothing
            } else {
                invalid_input = true;
            }
        }
    }

    fn debug_print(&self) {
        println!(
            "[PC {:?}]: {:?}\n{}\n",
            self.this_instruction_pc, self.this_instruction, self
        );
    }

    /* #endregion */
}

#[rustfmt::skip]
impl Display for CPU {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "+--------------------------+\n")?;
        write!(f, "| PC: {:?}    SP: {:?} | IME: {}\n", self.hpc(), self.hsp(), self.ime)?;
        write!(f, "| A:  {:?}      F:  {:0>4b}   | IE: {:0>5b}\n", self.ha(), self.f >> 4, self.int_enabled & 0x1F)?;
        write!(f, "| B:  {:?}      C:  {:?}   | IF: {:0>5b}\n", self.hb(), self.hc(), self.int_flags & 0x1F)?;
        write!(f, "| D:  {:?}      E:  {:?}   |\n", self.hd(), self.he())?;
        write!(f, "| H:  {:?}      L:  {:?}   | TIMA: {:?}\n", self.hh(), self.hl(), Hex8::make(self.mmu_read(REG_TIMA)))?;
        write!(f, "+--------------------------+")
    }
}

macro_rules! getset_r16 {
    ($getname:ident, $setname:ident, $r1:ident, $r2:ident) => {
        pub fn $getname(&self) -> u16 {
            let r1 = self.$r1 as u16;
            let r2 = self.$r2 as u16;
            (r1 << 8 | r2)
        }

        pub fn $setname(&mut self, value: u16) {
            self.$r1 = byte_of!(value, 1);
            self.$r2 = byte_of!(value, 0);
        }
    };
}
pub(self) use getset_r16;

macro_rules! get_as_hex {
    ($fn_name:ident, $field:ident, Hex8) => {
        pub fn $fn_name(&self) -> Hex8 {
            Hex8::make(self.$field)
        }
    };
    ($fn_name:ident, $field:ident, Hex16) => {
        pub fn $fn_name(&self) -> Hex16 {
            Hex16::make(self.$field)
        }
    };
}
pub(self) use get_as_hex;

mod cpu_decode;
mod cpu_execute;
mod cpu_interrupt;
