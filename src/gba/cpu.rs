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
    // Timers
    pub m_time: u64,
    pub t_time: u64,
}

impl CPU {
    #[rustfmt::skip]
    pub fn new() -> Self {
        CPU { pc: 0, sp: 0, regs: Registers::new(), m_time: 0, t_time: 0 }
    }

    pub fn reset(&mut self) {
        self.pc = 0;
        self.sp = 0;
        self.regs = Registers::new();
    }

    fn add_m_time(&mut self, m: u64) {
        self.m_time += m;
        self.t_time = self.m_time * 4;
    }

    fn panic_no_const() -> ! {
        panic!("Constant not allowed here")
    }

    fn get_value_at_r8(&self, mmu: &MMU, target: &ArgR8) -> u8 {
        match target {
            ArgR8::B => self.regs.b,
            ArgR8::C => self.regs.c,
            ArgR8::D => self.regs.d,
            ArgR8::E => self.regs.e,
            ArgR8::H => self.regs.h,
            ArgR8::L => self.regs.l,
            ArgR8::MHL => mmu.read_byte(self.regs.get_hl()),
            ArgR8::A => self.regs.a,
            ArgR8::CONST(c) => *c,
        }
    }

    fn get_value_at_r16(&self, target: &ArgR16) -> u16 {
        match target {
            ArgR16::BC => self.regs.get_bc(),
            ArgR16::DE => self.regs.get_de(),
            ArgR16::HL => self.regs.get_hl(),
            ArgR16::CONST(c) => *c,
        }
    }

    fn get_value_at_mr16(&mut self, mmu: &mut MMU, target: &ArgR16MEM) -> u8 {
        let address = match target {
            ArgR16MEM::BC => self.regs.get_bc(),
            ArgR16MEM::DE => self.regs.get_de(),
            ArgR16MEM::HLI => self.regs.get_hl(),
            ArgR16MEM::HLD => self.regs.get_hl(),
            ArgR16MEM::CONST(c) => *c,
        };
        if matches!(target, ArgR16MEM::HLI) {
            let (v, _) = self.regs.get_hl().overflowing_add(1);
            self.regs.set_hl(v);
        } else if matches!(target, ArgR16MEM::HLD) {
            let (v, _) = self.regs.get_hl().overflowing_sub(1);
            self.regs.set_hl(v);
        }

        mmu.read_byte(address)
    }

    fn set_value_at_mr16(&mut self, mmu: &mut MMU, target: &ArgR16MEM, value: u8) {
        let address = match target {
            ArgR16MEM::BC => self.regs.get_bc(),
            ArgR16MEM::DE => self.regs.get_de(),
            ArgR16MEM::HLI => {
                let address = self.regs.get_hl();
                self.regs.set_hl(address.overflowing_add(1).0);
                address
            },
            ArgR16MEM::HLD => {
                let address = self.regs.get_hl();
                self.regs.set_hl(address.overflowing_sub(1).0);
                address
            },
            ArgR16MEM::CONST(c) => *c,
        };

        mmu.write_byte(address, value);
    }

    fn set_value_at_r8(&mut self, mmu: &mut MMU, target: &ArgR8, value: u8) {
        match target {
            ArgR8::B => self.regs.b = value,
            ArgR8::C => self.regs.c = value,
            ArgR8::D => self.regs.d = value,
            ArgR8::E => self.regs.e = value,
            ArgR8::H => self.regs.h = value,
            ArgR8::L => self.regs.l = value,
            ArgR8::MHL => mmu.write_byte(self.regs.get_hl(), value),
            ArgR8::A => self.regs.a = value,
            ArgR8::CONST(_) => Self::panic_no_const(),
        }
    }

    fn set_value_at_r16(&mut self, target: &ArgR16, value: u16) {
        match target {
            ArgR16::BC => self.regs.set_bc(value),
            ArgR16::DE => self.regs.set_de(value),
            ArgR16::HL => self.regs.set_hl(value),
            ArgR16::CONST(_) => Self::panic_no_const(),
        }
    }

    #[rustfmt::skip]
    fn do_sub8(&mut self, mmu: &MMU, operand: ArgR8, with_carry: bool) -> u8 {
        let value = self.get_value_at_r8(mmu, &operand);
        let cv = if with_carry && self.regs.getf_carry() {1} else {0};
        let (result, overflow1) = self.regs.a.overflowing_sub(value);
        let (result, overflow2) = result.overflowing_sub(cv);
        let nibble_diff = ((self.regs.a & 0xF) as i8) - ((value & 0xF) as i8) - (cv as i8);
        self.regs.set_all_flags(
            result == 0,
            true,
            nibble_diff < 0,
            overflow1 || overflow2
        );
        result
    }
}

// Instruction functions
impl CPU {
    #[rustfmt::skip]
    fn add8(&mut self, mmu: &MMU, operand: ArgR8, with_carry: bool) {
        let value = self.get_value_at_r8(mmu, &operand);
        let cv = if with_carry && self.regs.getf_carry() {1} else {0};
        let (result, overflow1) = self.regs.a.overflowing_add(value);
        let (result, overflow2) = result.overflowing_add(cv);
        let nibble_sum = (self.regs.a & 0xF) + (value & 0xF) + cv;
        self.regs.set_all_flags(
            result == 0,
            false,
            nibble_sum > 0xF,
            overflow1 || overflow2
        );
        self.regs.a = result;
        self.add_m_time(if matches!(operand, ArgR8::CONST(_) | ArgR8::MHL) {2} else {1})
    }

    #[rustfmt::skip]
    fn sub8(&mut self, mmu: &MMU, operand: ArgR8, with_carry: bool) {
        self.regs.a = self.do_sub8(mmu, operand, with_carry);
        self.add_m_time(if matches!(operand, ArgR8::CONST(_) | ArgR8::MHL) {2} else {1})
    }

    #[rustfmt::skip]
    fn compare8(&mut self, mmu: &MMU, operand: ArgR8) {
        self.do_sub8(mmu, operand, false);
        self.add_m_time(if matches!(operand, ArgR8::CONST(_) | ArgR8::MHL) {2} else {1})
    }

    fn inc8(&mut self, mmu: &mut MMU, target: ArgR8) {
        let value = self.get_value_at_r8(mmu, &target);
        let (new_value, _) = value.overflowing_add(1);
        self.regs.setf_zero(new_value == 0);
        self.regs.setf_subtract(false);
        self.regs.setf_half_carry(value & 0xF == 0xF);
        self.set_value_at_r8(mmu, &target, new_value);
        self.add_m_time(1);
    }

    fn dec8(&mut self, mmu: &mut MMU, target: ArgR8) {
        let value = self.get_value_at_r8(mmu, &target);
        let (new_value, _) = value.overflowing_sub(1);
        self.regs.setf_zero(new_value == 0);
        self.regs.setf_subtract(true);
        self.regs.setf_half_carry(value & 0xF == 0);
        self.set_value_at_r8(mmu, &target, new_value);
        self.add_m_time(1);
    }

    fn load8(&mut self, mmu: &mut MMU, dest: ArgR8, src: ArgR8) {
        let value = self.get_value_at_r8(mmu, &src);
        self.set_value_at_r8(mmu, &dest, value);
        
    }

    fn load_mr16_to_a(&mut self, mmu: &mut MMU, src_address: ArgR16MEM) {
        let value = self.get_value_at_mr16(mmu, &src_address);
        self.set_value_at_r8(mmu, &ArgR8::A, value);
    }

    fn load_a_to_mr16(&mut self, mmu: &mut MMU, dest_address: ArgR16MEM) {
        self.set_value_at_mr16(mmu, &dest_address, self.regs.a);
    }
}


// Execute
impl CPU {
    pub fn execute(&mut self, mmu: &mut MMU, instruction: Instruction) {
        match instruction {
            // Load (LD_dest_source)
            LD_r8_r8(dest, src) => self.load8(mmu, dest, src),
            LD_r16_n16(dest, value) => todo!(),
            LD_mr16_a(dest_address) => self.load_a_to_mr16(mmu, dest_address),
            LDH_mn16_a(dest_address) => todo!(),
            LDH_mc_a => todo!(),
            LD_a_mr16(src_address) => self.load_mr16_to_a(mmu, src_address),
            LDH_a_mn16(src_address) => todo!(),
            LDH_a_mc => todo!(),

            // 8-bit arithmetic
            ADC_a_r8(operand) => self.add8(mmu, operand, true),
            ADD_a_r8(operand) => self.add8(mmu, operand, false),
            CP_a_r8(operand) => self.compare8(mmu, operand),
            DEC_r8(target) => self.dec8(mmu, target),
            INC_r8(target) => self.inc8(mmu, target),
            SBC_a_r8(operand) => self.sub8(mmu, operand, true),
            SUB_a_r8(operand) => self.sub8(mmu, operand, false),

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
            JR_e8(offset) => todo!(),
            JR_cc_e8(condition, offset) => todo!(),
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
    #[rustfmt::skip]
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        let Registers { a, f, b, c, d, e, h, l } = &self.regs;
        write!(formatter, "+--------------------------+\n")?;
        write!(formatter, "| PC: 0x{:0>4X}    SP: 0x{:0>4X} | M-time: {}\n", self.pc, self.sp, self.m_time)?;
        write!(formatter, "| A:  0x{:0>2X}      F:  {:0>4b}   | T-time: {}\n", a, f >> 4, self.t_time)?;
        write!(formatter, "| B:  0x{b:0>2X}      C:  0x{c:0>2X}   |\n")?;
        write!(formatter, "| D:  0x{d:0>2X}      E:  0x{e:0>2X}   |\n")?;
        write!(formatter, "| H:  0x{h:0>2X}      L:  0x{l:0>2X}   |\n")?;
        write!(formatter, "+--------------------------+\n")
    }
}

// Tests ==========================================================================================
#[cfg(test)]
mod cpu_tests;
