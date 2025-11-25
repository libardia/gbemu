use super::*;
use crate::{
    gb::cpu::instruction::{
        Cond,
        Instruction::{self, *},
        Mem, R16, R8,
    },
    macros::{address_fmt, byte_fmt, error_panic},
};

impl CPU {
    pub fn execute(&mut self, mmu: &mut MMU, inst: Instruction) -> u16 {
        match inst {
            // Load
            LD_r8_r8(dest, src) => self.load_r8_r8(mmu, dest, src),
            LD_r8_mem(dest, src) => self.load_r8_mem(mmu, dest, src),
            LD_mem_r8(dest, src) => self.load_mem_r8(mmu, dest, src),
            LD_r16_r16(dest, src) => self.load_r16_r16(dest, src),

            // Load high
            LDH_A_mem(src) => self.loadhigh_a_mem(mmu, src),
            LDH_mem_A(dest) => self.loadhigh_mem_a(mmu, dest),

            // 8-bit arithmetic
            ADD_r8(op) => self.add_8(mmu, op, false),
            ADC_r8(op) => self.add_8(mmu, op, true),
            SUB_r8(op) => self.sub_8(mmu, op, false),
            SBC_r8(op) => self.sub_8(mmu, op, true),
            INC_r8(target) => self.inc_8(mmu, target),
            DEC_r8(target) => self.dec_8(mmu, target),
            CP_r8(op) => self.compare_8(mmu, op),

            // 16-bit arithmetic
            ADD_r16(op) => self.add_16(op),
            INC_r16(target) => self.inc_16(target),
            DEC_r16(target) => self.dec_16(target),

            // Logic
            AND(op) => todo!(),
            OR(op) => todo!(),
            XOR(op) => todo!(),
            CPL => todo!(),

            // Bit flags
            BIT(bit, target) => todo!(),
            SET(bit, target) => todo!(),
            RES(bit, target) => todo!(),

            // Bit shifts
            RL(target) => todo!(),
            RLA => todo!(),
            RLC(target) => todo!(),
            RLCA => todo!(),
            RR(target) => todo!(),
            RRA => todo!(),
            RRC(target) => todo!(),
            RRCA => todo!(),
            SLA(target) => todo!(),
            SRA(target) => todo!(),
            SRL(target) => todo!(),
            SWAP(target) => todo!(),

            // Jumps and subroutines
            CALL(cond, address) => todo!(),
            JP(cond, address) => todo!(),
            JR(cond, off) => todo!(),
            RET(cond) => todo!(),
            RETI => todo!(),
            RST(Mem::IMM(address)) => todo!(),

            // Carry flag
            CCF => todo!(),
            SCF => todo!(),

            // Stack manipulation
            ADD_SP_e8(off) => todo!(),
            LD_a16_SP(address) => todo!(),
            LD_HL_SPe8(off) => todo!(),
            POP(target) => todo!(),
            PUSH(target) => todo!(),

            // Interrupts
            DI => todo!(),
            EI => todo!(),
            HALT => todo!(),

            // Misc
            DAA => todo!(),
            NOP => todo!(),
            STOP(_) => todo!(),

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
            R8::IMM(byte) => byte,
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
                "Tried to set a value into the constant {}, which doesn't make sense.",
                byte_fmt!(value)
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
            R16::IMM(word) => word,
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
                "Tried to set a value into the constant {}, which doesn't make sense.",
                address_fmt!(value)
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
            Mem::IMM(address) => address,
            Mem::HIGH_C => self.c as u16 + 0xFF00,
            Mem::HIGH_IMM(half_address) => half_address as u16 + 0xFF00,
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
mod load;
