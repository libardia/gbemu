pub mod instructions;
mod registers;

use super::MMU;
use instructions::{Instruction::*, *};
use registers::Registers;
use std::fmt::{self, Display, Formatter};

#[derive(Debug)]
pub struct CPU {
    // Program counter
    pub pc: u16,
    // Stack pointer
    pub sp: u16,
    // The 8 main registers
    pub regs: Registers,
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

    fn get_value_at_r8(&self, mmu: &MMU, target: ArgR8) -> u8 {
        match target {
            ArgR8::B => self.regs.b,
            ArgR8::C => self.regs.c,
            ArgR8::D => self.regs.d,
            ArgR8::E => self.regs.e,
            ArgR8::H => self.regs.h,
            ArgR8::L => self.regs.l,
            ArgR8::MHL => mmu.read_byte(self.regs.get_hl()),
            ArgR8::A => self.regs.a,
            ArgR8::CONST(value) => value,
        }
    }

    fn set_value_at_r8(&mut self, mmu: &mut MMU, target: ArgR8, value: u8) {
        match target {
            ArgR8::B => self.regs.b = value,
            ArgR8::C => self.regs.c = value,
            ArgR8::D => self.regs.d = value,
            ArgR8::E => self.regs.e = value,
            ArgR8::H => self.regs.h = value,
            ArgR8::L => self.regs.l = value,
            ArgR8::MHL => mmu.write_byte(self.regs.get_hl(), value),
            ArgR8::A => self.regs.a = value,
            ArgR8::CONST(_) => panic!("Constant not allowed here"),
        }
    }

    fn guard_no_const(arg_r8: ArgR8) {
        match arg_r8 {
            ArgR8::CONST(_) => panic!("Constant not allowed here"),
            _ => (),
        }
    }

    fn add_8bit(&mut self, mmu: &MMU, operand: ArgR8, with_carry: bool) {
        // Load value from target
        let value = self.get_value_at_r8(mmu, operand);

        // Calculate
        let cv = if with_carry && self.regs.getf_carry() {1} else {0};
        let (result, overflow1) = self.regs.a.overflowing_add(value);
        let (result, overflow2) = result.overflowing_add(cv);
        let nibble_sum = (self.regs.a & 0xF) + (value & 0xF) + cv;

        // Set flags
        self.regs.set_all_flags(
            result == 0,
            false,
            nibble_sum > 0xF,
            overflow1 || overflow2
        );

        // Write result to A
        self.regs.a = result;
    }

    fn sub_8bit(&mut self, mmu: &MMU, operand: ArgR8, with_carry: bool) {
        // Load value from target
        let value = self.get_value_at_r8(mmu, operand);

        // Calculate
        let cv = if with_carry && self.regs.getf_carry() {1} else {0};
        let (result, overflow1) = self.regs.a.overflowing_sub(value);
        let (result, overflow2) = result.overflowing_sub(cv);
        let nibble_diff = ((self.regs.a & 0xF) as i8) - ((value & 0xF) as i8) - (cv as i8);

        // Set flags
        self.regs.set_all_flags(
            result == 0,
            true,
            nibble_diff < 0,
            overflow1 || overflow2
        );

        // Write result to A
        self.regs.a = result;
    }
}

impl CPU {
    pub fn execute(&mut self, mmu: &mut MMU, instruction: Instruction) {
        match instruction {
            // Load (LD_dest_source)
            LD_r8_r8(dest, src) => todo!(),
            LD_r16_n16(dest, value) => todo!(),
            LD_mr16_a(dest_address) => todo!(),
            LDH_mn8_a(dest_address) => todo!(),
            LDH_mc_a => todo!(),
            LD_a_mr16(src_address) => todo!(),
            LDH_a_mn16(src_address) => todo!(),
            LDH_a_mc => todo!(),

            // 8-bit arithmetic
            ADC_a_r8(operand) => self.add_8bit(mmu, operand, true),
            ADD_a_r8(operand) => self.add_8bit(mmu, operand, false),
            CP_a_r8(operand) => todo!(),
            DEC_r8(target) => todo!(),
            INC_r8(target) => todo!(),
            SBC_a_r8(operand) => self.sub_8bit(mmu, operand, true),
            SUB_a_r8(operand) => self.sub_8bit(mmu, operand, false),

            // 16-bit arithmetic
            ADD_hl_r16(operand) => todo!(),
            DEC_r16(target) => todo!(),
            INC_r16(target) => todo!(),

            // Bitwise logic
            AND_a_r8(operand) => todo!(),
            CPL => todo!(),
            OR_a_r8(operand) => todo!(),
            XOR_a_r8(operand) => todo!(),

            // Bit flags
            BIT_u3_r8(bit, operand) => todo!(),
            RES_u3_r8(bit, operand) => todo!(),
            SET_u3_r8(bit, operand) => todo!(),

            // Bit shift
            RL_r8(target) => todo!(),
            RLA => todo!(),
            RLC_r8(target) => todo!(),
            RLCA => todo!(),
            RR_r8(target) => todo!(),
            RRA => todo!(),
            RRC_r8(target) => todo!(),
            RRCA => todo!(),
            SLA_r8(target) => todo!(),
            SRA_r8(target) => todo!(),
            SRL_r8(target) => todo!(),
            SWAP_r8(target) => todo!(),

            // Jumps and subroutines
            CALL_n16(address) => todo!(),
            CALL_cc_n16(condition, address) => todo!(),
            JP_hl => todo!(),
            JP_n16(address) => todo!(),
            JP_cc_n16(condition, address) => todo!(),
            JR_n16(offset) => todo!(),
            JR_cc_n16(condition, offset) => todo!(),
            RET_cc(condition) => todo!(),
            RET => todo!(),
            RETI => todo!(),
            RST_vec(vec_address) => todo!(),

            // Carry flag
            CCF => todo!(),
            SCF => todo!(),

            // Stack manipulation
            ADD_hl_sp => todo!(),
            ADD_sp_e8(operand) => todo!(),
            DEC_sp => todo!(),
            INC_sp => todo!(),
            LD_sp_n16(value) => todo!(),
            LD_mn16_sp(address) => todo!(),
            LD_hl_sp_plus_e8(offset) => todo!(),
            LD_sp_hl => todo!(),
            POP_r16(target) => todo!(),
            PUSH_r16(target) => todo!(),

            // Interrupt-related
            DI => todo!(),
            EI => todo!(),
            HALT => todo!(),

            // Miscellaneous
            DAA => todo!(),
            NOP => (),
            STOP => todo!(),
        }
    }
}

// Display ========================================================================================

impl Display for CPU {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        let Registers { a, f, b, c, d, e, h, l } = &self.regs;
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
