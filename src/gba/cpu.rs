mod registers;
pub mod instructions;

use registers::Registers;
use instructions::*;
use std::{fmt, fmt::Display, fmt::Formatter};
use super::MMU;

#[derive(Debug)]
pub struct CPU  {
    // Program counter
    pub pc: u16,
    // Stack pointer
    pub sp: u16,
    // The 8 main registers
    pub regs: Registers,
}

macro_rules! match_all_r8 {
    ($match_on:ident, $for_b:block, $for_c:block, $for_d:block, $for_e:block, $for_h:block, $for_l:block, $for_mhl:block, $for_a:block) => {
        match $match_on {
            ArgR8::B => $for_b
            ArgR8::C => $for_c
            ArgR8::D => $for_d
            ArgR8::E => $for_e
            ArgR8::H => $for_h
            ArgR8::L => $for_l
            ArgR8::MHL => $for_mhl
            ArgR8::A => $for_a
        }
    };
}

macro_rules! add_target_r8 {
    ($callr:ident, $target:ident) => {
        $callr.regs.a = $callr.add_8bit($callr.regs.$target)
    };
    ($callr:ident, mhl, $gba_ref:expr) => {
        $callr.regs.a = $callr.add_8bit_at_hl($gba_ref, $callr.regs.get_hl());
    };
}

impl CPU {
    pub fn new() -> Self {
        CPU { pc: 0, sp: 0, regs: Registers::new() }
    }

    pub fn reset(&mut self) {
        self.pc = 0;
        self.sp = 0;
        self.regs = Registers::new();
    }

    pub fn execute(&mut self, mmu: &mut MMU, instruction: Instruction) {
        match instruction {
            Instruction::NOP => {}
            Instruction::ADD(target) => {
                match_all_r8!(target,
                    { add_target_r8!(self, b); },
                    { add_target_r8!(self, c); },
                    { add_target_r8!(self, d); },
                    { add_target_r8!(self, e); },
                    { add_target_r8!(self, h); },
                    { add_target_r8!(self, l); },
                    { add_target_r8!(self, mhl, mmu); },
                    { add_target_r8!(self, a); }
                );
            }
            _ => todo!()
        }
    }

    fn add_8bit(&mut self, value: u8) -> u8 {
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

    fn add_8bit_at_hl(&mut self, mmu: &MMU, address: u16) -> u8 {
        // TODO: Add value pointed to by HL to A
        todo!();
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
