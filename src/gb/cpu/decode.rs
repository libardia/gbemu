use super::*;
use crate::gb::{
    cpu::{
        instruction::{
            Instruction::{self, *},
            Mem,
            MetaInstruction::*,
            R16, R8,
        },
        optable::*,
    },
    macros::{address_fmt, error_panic},
    types::{Byte, Word},
};

impl CPU {
    pub(super) fn decode(&mut self, mmu: &MMU) -> Instruction {
        let starting_pc = self.pc;
        let first_byte = self.next_byte(mmu);

        let mut inst = OP_TABLE[first_byte.0 as usize];

        // Check for validity
        match inst {
            INVALID(meta_inst) => {
                if meta_inst == NONE {
                    error_panic!(
                        "Byte {first_byte:?} at address {} is an invalid instruction.",
                        address_fmt!(starting_pc)
                    )
                } else if !self.enable_meta_instructions {
                    error_panic!(
                        "Byte {first_byte:?} at address {} is an invalid instruction (but would be {meta_inst:?} if meta instructions were enabled).",
                        address_fmt!(starting_pc)
                    )
                }
            }
            _ => (),
        }

        if inst == PREFIX {
            inst = PREFIX_TABLE[self.next_byte(mmu).0 as usize];
        }

        // Fill any constants in the instruction
        match inst {
            // 0x
            LD_r16_r16(first, R16::IMM(_)) => LD_r16_r16(first, R16::IMM(self.next_word(mmu))),
            LD_r8_r8(first, R8::IMM(_)) => LD_r8_r8(first, R8::IMM(self.next_byte(mmu))),
            LD_a16_SP(_) => LD_a16_SP(self.next_word(mmu)),

            // 1x
            STOP(_) => STOP(self.next_byte(mmu)),
            JR(first, _) => JR(first, self.next_signed(mmu)),

            // Cx
            JP(first, Mem::IMM(_)) => JP(first, Mem::IMM(self.next_word(mmu))),
            CALL(first, _) => CALL(first, self.next_word(mmu)),
            ADD_r8(R8::IMM(_)) => ADD_r8(R8::IMM(self.next_byte(mmu))),
            ADC_r8(R8::IMM(_)) => ADC_r8(R8::IMM(self.next_byte(mmu))),

            // Dx
            SUB_r8(R8::IMM(_)) => SUB_r8(R8::IMM(self.next_byte(mmu))),
            SBC_r8(R8::IMM(_)) => SBC_r8(R8::IMM(self.next_byte(mmu))),

            // Ex
            LDH_mem_A(Mem::HIGH_IMM(_)) => LDH_mem_A(Mem::HIGH_IMM(self.next_byte(mmu))),
            AND(R8::IMM(_)) => AND(R8::IMM(self.next_byte(mmu))),
            ADD_SP_e8(_) => ADD_SP_e8(self.next_signed(mmu)),
            LD_mem_r8(Mem::IMM(_), second) => LD_mem_r8(Mem::IMM(self.next_word(mmu)), second),
            XOR(R8::IMM(_)) => XOR(R8::IMM(self.next_byte(mmu))),

            // Fx
            LDH_A_mem(Mem::HIGH_IMM(_)) => LDH_A_mem(Mem::HIGH_IMM(self.next_byte(mmu))),
            OR(R8::IMM(_)) => OR(R8::IMM(self.next_byte(mmu))),
            LD_HL_SPe8(_) => LD_HL_SPe8(self.next_signed(mmu)),
            LD_r8_mem(first, Mem::IMM(_)) => LD_r8_mem(first, Mem::IMM(self.next_word(mmu))),
            CP_r8(R8::IMM(_)) => CP_r8(R8::IMM(self.next_byte(mmu))),

            // Any other instruction
            _ => inst,
        }
    }

    fn next_byte(&mut self, mmu: &MMU) -> Byte {
        let byte = mmu.get(self.pc);
        if self.halt_bug {
            // Don't increment PC, whoops!
            self.halt_bug = false
        } else {
            self.pc = self.pc.wrapping_add(1);
        }
        Byte(byte)
    }

    fn next_signed(&mut self, mmu: &MMU) -> i8 {
        self.next_byte(mmu).0 as i8
    }

    fn next_word(&mut self, mmu: &MMU) -> Word {
        let low = self.next_byte(mmu).0 as u16;
        let high = self.next_byte(mmu).0 as u16;
        Word((high << 8) | low)
    }
}
