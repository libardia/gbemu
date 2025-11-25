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
    pub fn execute(&mut self, mmu: &mut MMU, inst: Instruction) -> MachineCycles {
        match inst {
            // Load
            LD_r8_r8(dest, source) => todo!(),
            LD_r8_mem(dest, source) => todo!(),
            LD_mem_r8(dest, source) => todo!(),
            LD_r16_r16(dest, source) => todo!(),

            // Load high
            LDH_A_mem(address) => todo!(),
            LDH_mem_A(address) => todo!(),

            // 8-bit arithmetic
            ADD_r8(operand) => todo!(),
            ADC_r8(operand) => todo!(),
            SUB_r8(operand) => todo!(),
            SBC_r8(operand) => todo!(),
            INC_r8(target) => todo!(),
            DEC_r8(target) => todo!(),
            CP_r8(operand) => todo!(),

            // 16-bit arithmetic
            ADD_r16(operand) => todo!(),
            INC_r16(target) => todo!(),
            DEC_r16(target) => todo!(),

            // Logic
            AND_r8(operand) => todo!(),
            OR_r8(operand) => todo!(),
            XOR_r8(operand) => todo!(),
            CPL => todo!(),

            // Bit flags
            BIT_r8(bit_index, target) => todo!(),
            SET_r8(bit_index, target) => todo!(),
            RES_r8(bit_index, target) => todo!(),

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
            CALL(condition, address) => todo!(),
            JP(condition, address) => todo!(),
            JR(condition, offset) => todo!(),
            RET(condition) => todo!(),
            RETI => todo!(),
            RST(Mem::IMM(address)) => todo!(),

            // Carry flag
            CCF => todo!(),
            SCF => todo!(),

            // Stack manipulation
            ADD_SP_e8(offset) => todo!(),
            LD_a16_SP(address) => todo!(),
            LD_HL_SPe8(offset) => todo!(),
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

    fn get_r8(&self, mmu: &MMU, r8: R8) -> u8 {
        match r8 {
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

    fn set_r8(&mut self, mmu: &mut MMU, r8: R8, value: u8) {
        match r8 {
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

    fn get_r16(&self, r16: R16) -> u16 {
        match r16 {
            R16::BC => self.get_bc(),
            R16::DE => self.get_de(),
            R16::HL => self.get_hl(),
            R16::SP => self.sp,
            R16::AF => self.get_af(),
            R16::IMM(word) => word,
        }
    }

    fn set_r16(&mut self, r16: R16, value: u16) {
        match r16 {
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

    fn get_mem(&mut self, mmu: &MMU, mem: Mem) -> u8 {
        mmu.get(self.address_from_mem(mem))
    }

    fn set_mem(&mut self, mmu: &mut MMU, mem: Mem, value: u8) {
        mmu.set(self.address_from_mem(mem), value);
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

mod load;
