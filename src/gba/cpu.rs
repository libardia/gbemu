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
    // Timers (emulation purposes only, not in real hardware)
    pub m_time: u64,
    pub t_time: u64,
}

impl CPU {
    pub fn new() -> Self {
        CPU {
            pc: 0,
            sp: 0,
            regs: Registers::new(),
            m_time: 0,
            t_time: 0,
        }
    }

    pub fn reset(&mut self) {
        self.pc = 0;
        self.sp = 0;
        self.regs = Registers::new();
        self.m_time = 0;
        self.t_time = 0;
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
            ArgR16MEM::HL => self.regs.get_hl(),
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
            ArgR16MEM::HL => self.regs.get_hl(),
            ArgR16MEM::HLI => {
                let address = self.regs.get_hl();
                self.regs.set_hl(address.overflowing_add(1).0);
                address
            }
            ArgR16MEM::HLD => {
                let address = self.regs.get_hl();
                self.regs.set_hl(address.overflowing_sub(1).0);
                address
            }
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

    fn do_sub8(&mut self, mmu: &MMU, operand: ArgR8, with_carry: bool) -> u8 {
        let value = self.get_value_at_r8(mmu, &operand);
        let cv = (with_carry && self.regs.getf_carry()) as u8;
        let (result, overflow1) = self.regs.a.overflowing_sub(value);
        let (result, overflow2) = result.overflowing_sub(cv);
        let nibble_diff = ((self.regs.a & 0xF) as i8) - ((value & 0xF) as i8) - (cv as i8);

        self.regs
            .set_all_flags(result == 0, true, nibble_diff < 0, overflow1 || overflow2);

        result
    }
}

// Instruction functions
impl CPU {
    fn op_nop(&mut self) {
        self.add_m_time(1);
    }

    fn op_add8(&mut self, mmu: &MMU, operand: ArgR8, with_carry: bool) {
        let value = self.get_value_at_r8(mmu, &operand);
        let cv = (with_carry && self.regs.getf_carry()) as u8;
        let (result, overflow1) = self.regs.a.overflowing_add(value);
        let (result, overflow2) = result.overflowing_add(cv);
        let nibble_sum = (self.regs.a & 0xF) + (value & 0xF) + cv;

        self.regs
            .set_all_flags(result == 0, false, nibble_sum > 0xF, overflow1 || overflow2);

        self.regs.a = result;

        self.add_m_time(if matches!(operand, ArgR8::CONST(_)) {
            1
        } else {
            2
        });
    }

    fn op_sub8(&mut self, mmu: &MMU, operand: ArgR8, with_carry: bool) {
        self.regs.a = self.do_sub8(mmu, operand, with_carry);

        self.add_m_time(if matches!(operand, ArgR8::CONST(_)) {
            1
        } else {
            2
        });
    }

    fn op_compare8(&mut self, mmu: &MMU, operand: ArgR8) {
        self.do_sub8(mmu, operand, false);

        self.add_m_time(if matches!(operand, ArgR8::CONST(_) | ArgR8::MHL) {
            1
        } else {
            2
        });
    }

    fn op_inc8(&mut self, mmu: &mut MMU, target: ArgR8) {
        let value = self.get_value_at_r8(mmu, &target);
        let new_value = value.overflowing_add(1).0;

        self.regs.setf_zero(new_value == 0);
        self.regs.setf_subtract(false);
        self.regs.setf_half_carry(value & 0xF == 0xF);

        self.set_value_at_r8(mmu, &target, new_value);

        self.add_m_time(1);
    }

    fn op_dec8(&mut self, mmu: &mut MMU, target: ArgR8) {
        let value = self.get_value_at_r8(mmu, &target);
        let new_value = value.overflowing_sub(1).0;

        self.regs.setf_zero(new_value == 0);
        self.regs.setf_subtract(true);
        self.regs.setf_half_carry(value & 0xF == 0);

        self.set_value_at_r8(mmu, &target, new_value);

        self.add_m_time(1);
    }

    fn op_load8(&mut self, mmu: &mut MMU, dest: ArgR8, src: ArgR8) {
        if dest == src {
            // No op if src == dest
            self.op_nop();
            return;
        }

        let value = self.get_value_at_r8(mmu, &src);

        self.set_value_at_r8(mmu, &dest, value);

        self.add_m_time(if matches!(src, ArgR8::CONST(_) | ArgR8::MHL) {
            1
        } else {
            2
        });
    }

    fn op_load_const_to_r16(&mut self, dest: ArgR16, value: u16) {
        match dest {
            ArgR16::BC => self.regs.set_bc(value),
            ArgR16::DE => self.regs.set_de(value),
            ArgR16::HL => self.regs.set_hl(value),
            ArgR16::CONST(_) => Self::panic_no_const(),
        }

        self.add_m_time(3);
    }

    fn op_load_between_a_mr16(&mut self, mmu: &mut MMU, address: ArgR16MEM, a_is_dest: bool) {
        if a_is_dest {
            self.regs.a = self.get_value_at_mr16(mmu, &address);
        } else {
            self.set_value_at_mr16(mmu, &address, self.regs.a);
        }

        self.add_m_time(if matches!(address, ArgR16MEM::CONST(_)) {
            2
        } else {
            4
        });
    }

    fn op_loadhigh_between_a_mn16(&mut self, mmu: &mut MMU, half_address: u8, a_is_dest: bool) {
        let address = 0xFF + (half_address as u16);

        if a_is_dest {
            self.regs.a = mmu.read_byte(address);
        } else {
            mmu.write_byte(address, self.regs.a);
        }

        self.add_m_time(3);
    }

    fn op_loadhigh_between_a_mc(&mut self, mmu: &mut MMU, a_is_dest: bool) {
        let address = 0xFF00 + (self.regs.c as u16);

        if a_is_dest {
            self.regs.a = mmu.read_byte(address);
        } else {
            mmu.write_byte(address, self.regs.a);
        }

        self.add_m_time(2);
    }
}

// Execute
impl CPU {
    pub fn execute(&mut self, mmu: &mut MMU, instruction: Instruction) {
        match instruction {
            // Load (LD_dest_source)
            LD_r8_r8(dest, src) => self.op_load8(mmu, dest, src),
            LD_r16_n16(dest, value) => self.op_load_const_to_r16(dest, value),
            LD_mr16_a(address) => self.op_load_between_a_mr16(mmu, address, false),
            LDH_mn16_a(half_address) => self.op_loadhigh_between_a_mn16(mmu, half_address, false),
            LDH_mc_a => self.op_loadhigh_between_a_mc(mmu, false),
            LD_a_mr16(address) => self.op_load_between_a_mr16(mmu, address, true),
            LDH_a_mn16(half_address) => self.op_loadhigh_between_a_mn16(mmu, half_address, true),
            LDH_a_mc => self.op_loadhigh_between_a_mc(mmu, true),

            // 8-bit arithmetic
            ADC_a_r8(operand) => self.op_add8(mmu, operand, true),
            ADD_a_r8(operand) => self.op_add8(mmu, operand, false),
            CP_a_r8(operand) => self.op_compare8(mmu, operand),
            DEC_r8(target) => self.op_dec8(mmu, target),
            INC_r8(target) => self.op_inc8(mmu, target),
            SBC_a_r8(operand) => self.op_sub8(mmu, operand, true),
            SUB_a_r8(operand) => self.op_sub8(mmu, operand, false),

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
            NOP => self.op_nop(),
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
