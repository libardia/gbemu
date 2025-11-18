use crate::gb::cpu::instruction::Arg;
use crate::gb::cpu::instruction::{Arg::*, Instruction, Instruction::*};
use crate::gb::cpu::optable::*;

use super::CPU;

impl CPU {
    pub fn decode(&mut self, bytes: &[u8]) -> Instruction {
        // TODO: "bytes" is a stand in for the MMU. Refit to use MMU when that's done.

        let mut inst = OP_TABLE[self.read8_and_inc(bytes) as usize];
        if inst == PREFIX {
            inst = PREFIX_TABLE[self.read8_and_inc(bytes) as usize];
        }
        match inst {
            // 0x
            LD(first, IMM_16) => LD(first, self.read_const16(bytes)),
            LD(first, IMM_8) => LD(first, self.read_const8(bytes)),
            LD(IMM_16, second) => LD(self.read_const16(bytes), second),

            // 1x
            STOP(IMM_8) => STOP(self.read_const8(bytes)),
            JR(first, IMM_i8) => JR(first, self.read_consti8(bytes)),

            // Cx
            JP(first, IMM_16) => JP(first, self.read_const16(bytes)),
            CALL(first, IMM_16) => CALL(first, self.read_const16(bytes)),
            ADD(IMM_8) => ADD(self.read_const8(bytes)),
            ADC(IMM_8) => ADC(self.read_const8(bytes)),

            // Dx
            SUB(IMM_8) => SUB(self.read_const8(bytes)),
            SBC(IMM_8) => SBC(self.read_const8(bytes)),

            // Ex
            LDH(IMM_8, second) => LDH(self.read_const8(bytes), second),
            AND(IMM_8) => AND(self.read_const8(bytes)),
            ADD_STK(first, IMM_i8) => ADD_STK(first, self.read_consti8(bytes)),
            XOR(IMM_8) => XOR(self.read_const8(bytes)),

            // Fx
            LDH(first, IMM_8) => LDH(first, self.read_const8(bytes)),
            OR(IMM_8) => OR(self.read_const8(bytes)),
            LD(first, IMM_i8) => LD(first, self.read_consti8(bytes)),
            CP(IMM_8) => CP(self.read_const8(bytes)),

            // Any other instruction
            _ => inst,
        }
    }

    fn read8_and_inc(&mut self, bytes: &[u8]) -> u8 {
        let byte = bytes[self.pc as usize];
        self.pc += 1;
        byte
    }

    fn read_const8(&mut self, bytes: &[u8]) -> Arg {
        CONST_8(self.read8_and_inc(bytes))
    }

    fn read_consti8(&mut self, bytes: &[u8]) -> Arg {
        CONST_i8(self.read8_and_inc(bytes) as i8)
    }

    fn read16_and_inc(&mut self, bytes: &[u8]) -> u16 {
        let low = self.read8_and_inc(bytes) as u16;
        let high = self.read8_and_inc(bytes) as u16;
        let word = (high << 8) | low;
        word
    }

    fn read_const16(&mut self, bytes: &[u8]) -> Arg {
        CONST_16(self.read16_and_inc(bytes))
    }
}
