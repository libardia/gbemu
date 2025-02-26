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

macro_rules! ArgR8_to_a {
    ($callr:expr, $mmu:expr, $func:ident, $match_on:expr $(, $other_args:expr)*) => {
        match $match_on {
            ArgR8::B => $callr.regs.a = $callr.$func($callr.regs.b $(, $other_args)*),
            ArgR8::C => $callr.regs.a = $callr.$func($callr.regs.c $(, $other_args)*),
            ArgR8::D => $callr.regs.a = $callr.$func($callr.regs.d $(, $other_args)*),
            ArgR8::E => $callr.regs.a = $callr.$func($callr.regs.e $(, $other_args)*),
            ArgR8::H => $callr.regs.a = $callr.$func($callr.regs.h $(, $other_args)*),
            ArgR8::L => $callr.regs.a = $callr.$func($callr.regs.l $(, $other_args)*),
            ArgR8::MHL => $callr.regs.a = $callr.$func($mmu.read_byte($callr.regs.get_hl()) $(, $other_args)*),
            ArgR8::A => $callr.regs.a = $callr.$func($callr.regs.a $(, $other_args)*),
        }
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

    fn add_8bit(&mut self, value: u8, with_carry: bool) -> u8 {
        let carry_value = if with_carry && self.regs.getf_carry() {1u16} else {0u16};
        let mod_value = value as u16 + carry_value;
        let add_result = self.regs.a as u16 + mod_value;
        let final_value = (add_result & 0xFF) as u8;
        let nibble_sum = (self.regs.a & 0xF) + ((mod_value & 0xF) as u8);
        self.regs.set_all_flags(
            final_value == 0,
            false,
            nibble_sum > 0xF,
            add_result > 0xFF
        );
        final_value
    }

    fn sub_8bit(&mut self, value: u8, with_carry: bool) -> u8 {
        let carry_value = if with_carry && self.regs.getf_carry() {1i16} else {0i16};
        let mod_value = value as i16 - carry_value;
        let sub_result = self.regs.a as i16 - mod_value;
        let final_value = (sub_result & 0xFF) as u8;
        let nibble_diff = ((self.regs.a as i8) & 0xF) - ((mod_value & 0xF) as i8);
        self.regs.set_all_flags(
            final_value == 0,
            true,
            nibble_diff < 0,
            sub_result < 0
        );
        final_value
    }
}

impl CPU {
    pub fn execute(&mut self, mmu: &mut MMU, instruction: Instruction) {
        match instruction {
            Instruction::NOP => (),
            Instruction::ADD_a_r8(target) => ArgR8_to_a!(self, mmu, add_8bit, target, false),
            Instruction::ADC_a_r8(target) => ArgR8_to_a!(self, mmu, add_8bit, target, true),
            _ => todo!()
        }
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
