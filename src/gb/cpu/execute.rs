use super::*;
use crate::{gb::cpu::instruction::Instruction, macros::error_panic};

impl CPU {
    pub fn execute(&mut self, mmu: &mut MMU, inst: Instruction) -> MachineCycles {
        match inst {
            _ => todo!(),
        }
    }

    // fn get_byte_at(&mut self, mmu: &MMU, source: Arg) -> u8 {
    //     match source {
    //         _ => error_panic!("get_byte_at() was called with an invalid argument: {source:X?}"),
    //     }
    // }

    // fn set_byte_at(&mut self, mmu: &mut MMU, dest: Arg, value: u8) {
    //     match dest {
    //         _ => error_panic!("set_byte_at() was called with an invalid argument: {dest:X?}"),
    //     }
    // }

    // fn get_word_at(&self, source: Arg) -> u16 {
    //     match source {
    //         R16_BC => self.get_bc(),
    //         R16_DE => self.get_de(),
    //         R16_HL => self.get_hl(),
    //         R16_AF => self.get_af(),
    //         R16_SP => self.sp,
    //         CONST_16(word) => word,

    //         _ => error_panic!("get_word_at() was called with an invalid argument: {source:X?}"),
    //     }
    // }

    // fn set_word_at(&mut self, dest: Arg, value: u16) {
    //     match dest {
    //         R16_BC => self.set_bc(value),
    //         R16_DE => self.set_de(value),
    //         R16_HL => self.set_hl(value),
    //         R16_AF => self.set_af(value),
    //         R16_SP => self.sp = value,

    //         _ => error_panic!("get_word_at() was called with an invalid argument: {dest:X?}"),
    //     }
    // }
}

mod load;
