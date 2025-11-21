use super::*;
use crate::{
    gb::cpu::instruction::{
        Arg::{self, *},
        Instruction::{self, *},
    },
    macros::error_panic,
};

impl CPU {
    pub fn execute(&mut self, mmu: &mut MMU, inst: Instruction) -> MachineCycles {
        match inst {
            // Load
            LD(dest, source) => todo!(),
            LD_16(dest, source) => todo!(),
            LDH(arg, arg1) => todo!(),

            // 8-bit arithmetic
            ADD(arg) => todo!(),
            ADC(arg) => todo!(),
            SUB(arg) => todo!(),
            SBC(arg) => todo!(),
            INC(arg) => todo!(),
            DEC(arg) => todo!(),
            CP(arg) => todo!(),

            // 16-bit arithmetic
            ADD_16(arg) => todo!(),
            INC_16(arg) => todo!(),
            DEC_16(arg) => todo!(),

            // Bitwise logic
            CPL => todo!(),
            AND(arg) => todo!(),
            OR(arg) => todo!(),
            XOR(arg) => todo!(),

            // Bit flag
            BIT(arg, arg1) => todo!(),
            RES(arg, arg1) => todo!(),
            SET(arg, arg1) => todo!(),

            // Shift
            RL(arg) => todo!(),
            RLC(arg) => todo!(),
            RLA => todo!(),
            RLCA => todo!(),
            RR(arg) => todo!(),
            RRC(arg) => todo!(),
            RRA => todo!(),
            RRCA => todo!(),
            SLA(arg) => todo!(),
            SRA(arg) => todo!(),
            SRL(arg) => todo!(),
            SWAP(arg) => todo!(),

            // Jumps & subroutines
            JP(arg, arg1) => todo!(),
            JR(arg, arg1) => todo!(),
            CALL(arg, arg1) => todo!(),
            RST(arg) => todo!(),
            RET(arg) => todo!(),
            RETI => todo!(),

            // Carry flag
            CCF => todo!(),
            SCF => todo!(),

            // Stack manip
            LD_HL_SP_E8(arg, arg1) => todo!(),
            ADD_STK(arg, arg1) => todo!(),
            POP(arg) => todo!(),
            PUSH(arg) => todo!(),

            // Interrupts
            DI => todo!(),
            EI => todo!(),
            HALT => todo!(),

            // Misc
            NOP => todo!(),
            DAA => todo!(),
            STOP(arg) => todo!(),

            // Invalid!
            PREFIX => todo!(),
            INVALID(meta_instruction) => todo!(),
        }
    }

    fn get_byte_at(&mut self, mmu: &MMU, source: Arg) -> u8 {
        match source {
            R8_B => self.b,
            R8_C => self.c,
            R8_D => self.d,
            R8_E => self.e,
            R8_H => self.h,
            R8_L => self.l,
            R8_A => self.a,
            M_BC => mmu.get(self.get_bc()),
            M_DE => mmu.get(self.get_de()),
            M_HL => mmu.get(self.get_hl()),
            M_HLI => mmu.get(self.get_hli()),
            M_HLD => mmu.get(self.get_hld()),
            CONST_8(byte) => byte,
            CONST_16(address) => mmu.get(address),

            _ => error_panic!("get_byte_at() was called with an invalid argument: {source:X?}"),
        }
    }

    fn set_byte_at(&mut self, mmu: &mut MMU, dest: Arg, value: u8) {
        match dest {
            R8_B => self.b = value,
            R8_C => self.c = value,
            R8_D => self.d = value,
            R8_E => self.e = value,
            R8_H => self.h = value,
            R8_L => self.l = value,
            R8_A => self.a = value,
            M_BC => mmu.set(self.get_bc(), value),
            M_DE => mmu.set(self.get_de(), value),
            M_HL => mmu.set(self.get_hl(), value),
            M_HLI => mmu.set(self.get_hli(), value),
            M_HLD => mmu.set(self.get_hld(), value),
            CONST_16(address) => mmu.set(address, value),

            _ => error_panic!("set_byte_at() was called with an invalid argument: {dest:X?}"),
        }
    }

    fn get_word_at(&self, source: Arg) -> u16 {
        match source {
            R16_BC => self.get_bc(),
            R16_DE => self.get_de(),
            R16_HL => self.get_hl(),
            R16_AF => self.get_af(),
            R16_SP => self.sp,
            CONST_16(word) => word,

            _ => error_panic!("get_word_at() was called with an invalid argument: {source:X?}"),
        }
    }

    fn set_word_at(&mut self, dest: Arg, value: u16) {
        match dest {
            R16_BC => self.set_bc(value),
            R16_DE => self.set_de(value),
            R16_HL => self.set_hl(value),
            R16_AF => self.set_af(value),
            R16_SP => self.sp = value,

            _ => error_panic!("get_word_at() was called with an invalid argument: {dest:X?}"),
        }
    }
}

mod load;
