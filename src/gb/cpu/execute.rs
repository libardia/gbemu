use super::*;
use crate::gb::{
    cpu::instruction::{
        Cond,
        Instruction::{self, *},
        Mem, R16, R8,
    },
    macros::error_panic,
};

impl CPU {
    pub fn execute(&mut self, mmu: &mut MMU, inst: Instruction) -> u16 {
        match inst {
            // Load
            LD_r8_r8(dest, src) => self.op_load_r8_r8(mmu, dest, src),
            LD_r8_mem(dest, src) => self.op_load_r8_mem(mmu, dest, src),
            LD_mem_r8(dest, src) => self.op_load_mem_r8(mmu, dest, src),
            LD_r16_r16(dest, src) => self.op_load_r16_r16(dest, src),

            // Load high
            LDH_A_mem(src) => self.op_loadhigh_a_mem(mmu, src),
            LDH_mem_A(dest) => self.op_loadhigh_mem_a(mmu, dest),

            // 8-bit arithmetic
            ADD_r8(op) => self.op_add_8(mmu, op, false),
            ADC_r8(op) => self.op_add_8(mmu, op, true),
            SUB_r8(op) => self.op_sub_8(mmu, op, false),
            SBC_r8(op) => self.op_sub_8(mmu, op, true),
            INC_r8(target) => self.op_inc_8(mmu, target),
            DEC_r8(target) => self.op_dec_8(mmu, target),
            CP_r8(op) => self.op_cp_8(mmu, op),

            // 16-bit arithmetic
            ADD_r16(op) => self.op_add_16(op),
            INC_r16(target) => self.op_inc_16(target),
            DEC_r16(target) => self.op_dec_16(target),

            // Logic
            AND(op) => self.op_and(mmu, op),
            OR(op) => self.op_or(mmu, op),
            XOR(op) => self.op_xor(mmu, op),
            CPL => self.op_cpl(),

            // Bit flags
            BIT(bit, target) => self.op_bit(mmu, bit, target),
            SET(bit, target) => self.op_set(mmu, bit, target),
            RES(bit, target) => self.op_res(mmu, bit, target),

            // Bit shifts
            RL(target) => self.op_rl(mmu, target, true, false),
            RLA => self.op_rl(mmu, R8::A, true, true),
            RLC(target) => self.op_rl(mmu, target, false, false),
            RLCA => self.op_rl(mmu, R8::A, false, true),
            RR(target) => self.op_rr(mmu, target, true, false),
            RRA => self.op_rr(mmu, R8::A, true, true),
            RRC(target) => self.op_rr(mmu, target, false, false),
            RRCA => self.op_rr(mmu, R8::A, false, true),
            SLA(target) => self.op_sl(mmu, target),
            SRA(target) => self.op_sr(mmu, target, true),
            SRL(target) => self.op_sr(mmu, target, false),
            SWAP(target) => self.op_swap(mmu, target),

            // Jumps and subroutines
            CALL(cond, address) => self.op_call(mmu, cond, address.0),
            JP(cond, address) => self.op_jump(cond, address),
            JR(cond, off) => self.op_jump_rel(cond, off),
            RET(cond) => self.op_ret(mmu, cond, false),
            RETI => self.op_ret(mmu, Cond::ALWAYS, true),
            RST(address) => self.op_rst(mmu, address.0),

            // Carry flag
            CCF => self.op_ccf(),
            SCF => self.op_scf(),

            // Stack manipulation
            ADD_SP_e8(off) => self.op_add_sp_e8(off),
            LD_a16_SP(address) => self.op_ld_a16_sp(mmu, address.0),
            LD_HL_SPe8(off) => self.op_ld_hl_sp_e8(off),
            POP(target) => self.op_pop(mmu, target),
            PUSH(target) => self.op_push(mmu, target),

            // Interrupts
            DI => self.op_di(),
            EI => self.op_ei(),
            HALT => self.op_halt(mmu),

            // Misc
            DAA => self.op_daa(),
            NOP => 1, // Do nothing for 1 MTime
            STOP(_) => self.op_stop(),

            // Meta
            INVALID(meta) => todo!(),

            _ => error_panic!("Tried to execute invalid instruction: {inst:X?}"),
        }
    }

    fn get_r8(&self, mmu: &MMU, src: R8) -> u8 {
        match src {
            R8::B => self.b,
            R8::C => self.c,
            R8::D => self.d,
            R8::E => self.e,
            R8::H => self.h,
            R8::L => self.l,
            R8::MHL => mmu.get(self.get_hl()),
            R8::A => self.a,
            R8::IMM(byte) => byte.0,
        }
    }

    fn set_r8(&mut self, mmu: &mut MMU, dest: R8, value: u8) {
        match dest {
            R8::B => self.b = value,
            R8::C => self.c = value,
            R8::D => self.d = value,
            R8::E => self.e = value,
            R8::H => self.h = value,
            R8::L => self.l = value,
            R8::MHL => mmu.set(self.get_hl(), value),
            R8::A => self.a = value,
            R8::IMM(value) => error_panic!(
                "Tried to set a value into the constant {value:?}, which doesn't make sense.",
            ),
        }
    }

    fn get_r16(&self, src: R16) -> u16 {
        match src {
            R16::BC => self.get_bc(),
            R16::DE => self.get_de(),
            R16::HL => self.get_hl(),
            R16::SP => self.sp,
            R16::AF => self.get_af(),
            R16::IMM(word) => word.0,
        }
    }

    fn set_r16(&mut self, dest: R16, value: u16) {
        match dest {
            R16::BC => self.set_bc(value),
            R16::DE => self.set_de(value),
            R16::HL => self.set_hl(value),
            R16::SP => self.sp = value,
            R16::AF => self.set_af(value),
            R16::IMM(value) => error_panic!(
                "Tried to set a value into the constant {value:?}, which doesn't make sense.",
            ),
        }
    }

    fn address_from_mem(&mut self, mem: Mem) -> u16 {
        match mem {
            Mem::BC => self.get_bc(),
            Mem::DE => self.get_de(),
            Mem::HL => self.get_hl(),
            Mem::HLI => self.get_hli(),
            Mem::HLD => self.get_hld(),
            Mem::IMM(address) => address.0,
            Mem::HIGH_C => self.c as u16 + 0xFF00,
            Mem::HIGH_IMM(half_address) => half_address.0 as u16 + 0xFF00,
        }
    }

    fn get_mem(&mut self, mmu: &MMU, src: Mem) -> u8 {
        mmu.get(self.address_from_mem(src))
    }

    fn set_mem(&mut self, mmu: &mut MMU, dest: Mem, value: u8) {
        mmu.set(self.address_from_mem(dest), value);
    }

    fn test_condition(&self, cond: Cond) -> bool {
        match cond {
            Cond::NZ => !self.getf_z(),
            Cond::Z => self.getf_z(),
            Cond::NC => !self.getf_c(),
            Cond::C => self.getf_c(),
            Cond::ALWAYS => true,
        }
    }
}

mod arith16;
mod arith8;
mod bitflags;
mod bitshifts;
mod jumps;
mod load;
mod logic;
mod misc;
mod stack;
