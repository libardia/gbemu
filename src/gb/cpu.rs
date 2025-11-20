mod instruction;
mod optable;

use crate::{
    gb::{
        mmu::{AccessMode, MMU},
        MachineCycles,
    },
    macros::{address_fmt, byte_fmt, new},
};
use instruction::{Arg::*, Instruction::*, MetaInstruction::NONE, *};
use log::error;
use optable::*;

#[derive(Debug, Default)]
pub struct CPU {
    // Registers
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    h: u8,
    l: u8,
    a: u8,
    f: u8,
    pc: u16,
    sp: u16,

    // Flags
    pub ime: bool,

    // Helper
    pub enable_meta_instructions: bool,
}

impl CPU {
    new!();

    // Accessors
    getset_r16!(get_bc, set_bc, b, c);
    getset_r16!(get_de, set_de, d, e);
    getset_r16!(get_hl, set_hl, h, l);
    getset_r16!(get_af, set_af, a, f);

    pub fn step(&mut self, mmu: &mut MMU) -> MachineCycles {
        // Tell the MMU that the CPU is accessing it
        mmu.access_mode = AccessMode::CPU;

        // Decode instruction at PC
        let inst = self.decode(mmu);

        self.execute(mmu)
    }

    fn decode(&mut self, mmu: &MMU) -> Instruction {
        /* #region Decode helpers */
        fn next_byte(cpu: &mut CPU, mmu: &MMU) -> u8 {
            let byte = mmu.get(cpu.pc);
            cpu.pc += 1;
            byte
        }

        fn next_word(cpu: &mut CPU, mmu: &MMU) -> u16 {
            let word = mmu.get_word(cpu.pc);
            cpu.pc += 2;
            word
        }

        fn next_const8(cpu: &mut CPU, mmu: &MMU) -> Arg {
            CONST_8(next_byte(cpu, mmu))
        }

        fn next_consti8(cpu: &mut CPU, mmu: &MMU) -> Arg {
            CONST_i8(next_byte(cpu, mmu) as i8)
        }

        fn next_const16(cpu: &mut CPU, mmu: &MMU) -> Arg {
            CONST_16(next_word(cpu, mmu))
        }
        /* #endregion */

        let starting_pc = self.pc;
        let first_byte = next_byte(self, mmu);

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
            inst = PREFIX_TABLE[next_byte(self, mmu) as usize];
        }

        match inst {
            // 0x
            LD(first, IMM_16) => LD(first, next_const16(self, mmu)),
            LD(first, IMM_8) => LD(first, next_const8(self, mmu)),
            LD(IMM_16, second) => LD(next_const16(self, mmu), second),

            // 1x
            STOP(IMM_8) => STOP(next_const8(self, mmu)),
            JR(first, IMM_i8) => JR(first, next_consti8(self, mmu)),

            // Cx
            JP(first, IMM_16) => JP(first, next_const16(self, mmu)),
            CALL(first, IMM_16) => CALL(first, next_const16(self, mmu)),
            ADD(IMM_8) => ADD(next_const8(self, mmu)),
            ADC(IMM_8) => ADC(next_const8(self, mmu)),

            // Dx
            SUB(IMM_8) => SUB(next_const8(self, mmu)),
            SBC(IMM_8) => SBC(next_const8(self, mmu)),

            // Ex
            LDH(IMM_8, second) => LDH(next_const8(self, mmu), second),
            AND(IMM_8) => AND(next_const8(self, mmu)),
            ADD_STK(first, IMM_i8) => ADD_STK(first, next_consti8(self, mmu)),
            XOR(IMM_8) => XOR(next_const8(self, mmu)),

            // Fx
            LDH(first, IMM_8) => LDH(first, next_const8(self, mmu)),
            OR(IMM_8) => OR(next_const8(self, mmu)),
            LD(first, IMM_i8) => LD(first, next_consti8(self, mmu)),
            CP(IMM_8) => CP(next_const8(self, mmu)),

            // Any other instruction
            _ => inst,
        }
    }

    fn execute(&mut self, mmu: &mut MMU) -> MachineCycles {
        // TODO: Execute
        0
    }
}

macro_rules! getset_r16 {
    ($getname:ident, $setname:ident, $r1:ident, $r2:ident) => {
        fn $getname(&self) -> u16 {
            let r1 = self.$r1 as u16;
            let r2 = self.$r2 as u16;
            (r1 << 8 | r2)
        }

        fn $setname(&mut self, value: u16) {
            self.$r1 = crate::macros::byte_of!(value, 1);
            self.$r2 = crate::macros::byte_of!(value, 0);
        }
    };
}
use getset_r16;
