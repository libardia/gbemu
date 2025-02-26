pub mod registers;
pub mod instructions;

use registers::*;
use instructions::*;
use std::{fmt, fmt::Display, fmt::Formatter};

#[derive(Default, Debug)]
pub struct CPU  {
    // Program counter
    pub pc: u16,
    // Stack pointer
    pub sp: u16,
    // The 8 main registers
    pub regs: Registers,
}

macro_rules! match_all_targets {
    ($match_on:ident, $for_a:block, $for_b:block, $for_c:block, $for_d:block, $for_e:block, $for_h:block, $for_l:block) => {
        match $match_on {
            Target::A => $for_a
            Target::B => $for_b
            Target::C => $for_c
            Target::D => $for_d
            Target::E => $for_e
            Target::H => $for_h
            Target::L => $for_l
        }
    };
}

macro_rules! add_target {
    ($callr:ident, $target:ident) => {
        $callr.regs.a = $callr.add($callr.regs.$target)
    };
}

impl CPU {
    pub fn execute(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::ADD(target) => {
                match_all_targets!(target,
                    { add_target!(self, a); },
                    { add_target!(self, b); },
                    { add_target!(self, c); },
                    { add_target!(self, d); },
                    { add_target!(self, e); },
                    { add_target!(self, h); },
                    { add_target!(self, l); }
                );
            }
            _ => todo!()
        }
    }

    fn add(&mut self, value: u8) -> u8 {
        let (new_value, did_overflow) = self.regs.a.overflowing_add(value);
        self.regs.set_all_flags(
            // Set if the result of the operation was zero
            new_value == 0,
            // Set if the operation was a subtraction
            false,
            // Set if adding the lower nibbles of the value and register A together result in a
            // value bigger than 0xF. If the result is larger than 0xF than the addition caused a
            // carry from the lower nibble to the upper nibble.
            (self.regs.a & 0xF) + (value & 0xF) > 0xF,
            // Set if the operation fully overflowed a u8
            did_overflow
        );
        new_value
    }
}

// Display ========================================================================================

impl Display for CPU {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        let Registers {a, f, b, c, d, e, h, l} = &self.regs;
        write!(formatter, "+--------------------------+\n")?;
        write!(formatter, "| PC: 0x{:0>4X}    SP: 0x{:0>4X} |\n", self.pc, self.sp)?;
        write!(formatter, "| A:  0x{a:0>2X}      F:  0x{f:0>2X}   |\n")?;
        write!(formatter, "| B:  0x{b:0>2X}      C:  0x{c:0>2X}   |\n")?;
        write!(formatter, "| D:  0x{d:0>2X}      E:  0x{e:0>2X}   |\n")?;
        write!(formatter, "| H:  0x{h:0>2X}      L:  0x{l:0>2X}   |\n")?;
        write!(formatter, "+--------------------------+")
    }
}

// Tests ==========================================================================================
#[cfg(test)]
mod cpu_tests;
