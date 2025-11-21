use super::*;
use crate::{
    gb::cpu::{
        instruction::{
            Arg::{self, *},
            Instruction::{self, *},
            MetaInstruction::NONE,
        },
        optable::*,
    },
    macros::{address_fmt, byte_fmt},
};
use log::error;

impl CPU {
    pub(super) fn decode(&mut self, mmu: &MMU) -> Instruction {
        let starting_pc = self.pc;
        let first_byte = self.next_byte(mmu);

        let mut inst = OP_TABLE[first_byte as usize];

        // Check for validity
        match inst {
            INVALID(meta_inst) => {
                if meta_inst == NONE {
                    error!(
                        "Byte {} at address {} is an invalid instruction.",
                        byte_fmt!(first_byte),
                        address_fmt!(starting_pc)
                    )
                } else if !self.enable_meta_instructions {
                    error!(
                        "Byte {} at address {} is an invalid instruction (but would be {meta_inst:?} if meta instructions were enabled).",
                        byte_fmt!(first_byte),
                        address_fmt!(starting_pc)
                    )
                }
            }
            _ => (),
        }

        if inst == PREFIX {
            inst = PREFIX_TABLE[self.next_byte(mmu) as usize];
        }

        match inst {
            // 0x
            LD_16(first, IMM_16) => LD_16(first, self.next_const16(mmu)),
            LD(first, IMM_8) => LD(first, self.next_const8(mmu)),
            LD_16(IMM_16, second) => LD_16(self.next_const16(mmu), second),

            // 1x
            STOP(IMM_8) => STOP(self.next_const8(mmu)),
            JR(first, IMM_i8) => JR(first, self.next_consti8(mmu)),

            // Cx
            JP(first, IMM_16) => JP(first, self.next_const16(mmu)),
            CALL(first, IMM_16) => CALL(first, self.next_const16(mmu)),
            ADD(IMM_8) => ADD(self.next_const8(mmu)),
            ADC(IMM_8) => ADC(self.next_const8(mmu)),

            // Dx
            SUB(IMM_8) => SUB(self.next_const8(mmu)),
            SBC(IMM_8) => SBC(self.next_const8(mmu)),

            // Ex
            LDH(IMM_8, second) => LDH(self.next_const8(mmu), second),
            AND(IMM_8) => AND(self.next_const8(mmu)),
            ADD_STK(first, IMM_i8) => ADD_STK(first, self.next_consti8(mmu)),
            LD(IMM_16, second) => LD(self.next_const16(mmu), second),
            XOR(IMM_8) => XOR(self.next_const8(mmu)),

            // Fx
            LDH(first, IMM_8) => LDH(first, self.next_const8(mmu)),
            OR(IMM_8) => OR(self.next_const8(mmu)),
            LD_HL_SP_E8(first, IMM_i8) => LD_HL_SP_E8(first, self.next_consti8(mmu)),
            LD(first, IMM_16) => LD(first, self.next_const16(mmu)),
            CP(IMM_8) => CP(self.next_const8(mmu)),

            // Any other instruction
            _ => inst,
        }
    }

    fn next_byte(&mut self, mmu: &MMU) -> u8 {
        let byte = mmu.get(self.pc);
        self.pc += 1;
        byte
    }

    fn next_word(&mut self, mmu: &MMU) -> u16 {
        let word = mmu.get_word(self.pc);
        self.pc += 2;
        word
    }

    fn next_const8(&mut self, mmu: &MMU) -> Arg {
        CONST_8(self.next_byte(mmu))
    }

    fn next_consti8(&mut self, mmu: &MMU) -> Arg {
        CONST_i8(self.next_byte(mmu) as i8)
    }

    fn next_const16(&mut self, mmu: &MMU) -> Arg {
        CONST_16(self.next_word(mmu))
    }
}
