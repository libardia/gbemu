use crate::util::{Hex16, Hex8};

use super::{
    instructions::{
        ArgR16MEM, ArgR8,
        Instruction::{self, *},
    },
    optables::{OP_TABLE, PREFIX_TABLE},
    CPU,
};

impl CPU {
    fn get_instruction(&self, table: &[[Instruction; 16]; 16], code: u8) -> Instruction {
        let upper = ((code & 0xF0) >> 4) as usize;
        let lower = (code & 0xF) as usize;
        table[upper][lower]
    }

    fn next_byte(&mut self) -> u8 {
        let byte = self.mmu_read(self.pc);
        self.pc += 1;
        byte
    }

    fn next_word(&mut self) -> u16 {
        let word = self.mmu_read_word(self.pc);
        self.pc += 2;
        word
    }

    fn next_hex8(&mut self) -> Hex8 {
        self.next_byte().into()
    }

    fn next_hex16(&mut self) -> Hex16 {
        self.next_word().into()
    }

    pub(super) fn decode(&mut self) -> Instruction {
        let code = self.next_byte();
        let mut instruction = self.get_instruction(&OP_TABLE, code);

        if instruction == PREFIX {
            let second = self.next_byte();
            instruction = self.get_instruction(&PREFIX_TABLE, second);
        } else {
            // Fill in constants from following bytes, if applicable
            instruction = match instruction {
                // 0x
                LD_r16_n16(x, _) => LD_r16_n16(x, self.next_hex16()),
                LD_r8_r8(x, ArgR8::CONST(_)) => LD_r8_r8(x, ArgR8::CONST(self.next_hex8())),
                LD_mn16_sp(_) => LD_mn16_sp(self.next_hex16()),

                // 1x
                STOP(_) => STOP(self.next_hex8()),
                JR_e8(_) => JR_e8(self.next_byte() as i8),

                // 2x
                JR_cc_e8(x, _) => JR_cc_e8(x, self.next_byte() as i8),

                // 3x
                LD_sp_n16(_) => LD_sp_n16(self.next_hex16()),

                // Cx
                JP_cc_n16(x, _) => JP_cc_n16(x, self.next_hex16()),
                JP_n16(_) => JP_n16(self.next_hex16()),
                CALL_cc_n16(x, _) => CALL_cc_n16(x, self.next_hex16()),
                ADD_a_r8(ArgR8::CONST(_)) => ADD_a_r8(ArgR8::CONST(self.next_hex8())),
                CALL_n16(_) => CALL_n16(self.next_hex16()),
                ADC_a_r8(ArgR8::CONST(_)) => ADC_a_r8(ArgR8::CONST(self.next_hex8())),

                // Dx
                SUB_a_r8(ArgR8::CONST(_)) => SUB_a_r8(ArgR8::CONST(self.next_hex8())),
                SBC_a_r8(ArgR8::CONST(_)) => SBC_a_r8(ArgR8::CONST(self.next_hex8())),

                // Ex
                LDH_mn16_a(_) => LDH_mn16_a(self.next_hex8()),
                AND_a_r8(ArgR8::CONST(_)) => AND_a_r8(ArgR8::CONST(self.next_hex8())),
                ADD_sp_e8(_) => ADD_sp_e8(self.next_byte() as i8),
                LD_mr16_a(ArgR16MEM::CONST(_)) => LD_mr16_a(ArgR16MEM::CONST(self.next_hex16())),
                XOR_a_r8(ArgR8::CONST(_)) => XOR_a_r8(ArgR8::CONST(self.next_hex8())),

                // Fx
                LDH_a_mn16(_) => LDH_a_mn16(self.next_hex8()),
                OR_a_r8(ArgR8::CONST(_)) => OR_a_r8(ArgR8::CONST(self.next_hex8())),
                LD_hl_sp_plus_e8(_) => LD_hl_sp_plus_e8(self.next_byte() as i8),
                LD_a_mr16(ArgR16MEM::CONST(_)) => LD_a_mr16(ArgR16MEM::CONST(self.next_hex16())),
                CP_a_r8(ArgR8::CONST(_)) => CP_a_r8(ArgR8::CONST(self.next_hex8())),

                // Nonstandard instructions (in non-debug mode, these are invalid)
                DEBUG_PRINT | TERMINATE => {
                    if self.debug_mode {
                        instruction
                    } else {
                        INVALID
                    }
                }

                // Everything else
                _ => instruction,
            };
        }

        instruction
    }
}
