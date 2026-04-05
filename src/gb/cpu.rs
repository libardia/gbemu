use log::debug;

use crate::{
    gb::{
        GameBoy,
        cpu::{
            instructions::Instruction,
            optables::{OPTABLE, PREFIX_OPTABLE},
        },
        mmu::MMU,
    },
    hex,
};

pub mod execute;
pub mod instructions;
pub mod optables;

#[derive(Debug, Default, Clone)]
pub struct CPU {
    // Registers
    pub b: u8,
    pub c: u8,

    pub d: u8,
    pub e: u8,

    pub h: u8,
    pub l: u8,

    pub a: u8,
    pub f: u8,

    // Internal
    pub pc: u16,
    pub sp: u16,

    pub prefix_mode: bool,
    pub halt_bug: bool,
}

macro_rules! r16 {
    ($r1:ident + $r2:ident) => {
        paste::paste! {
            pub fn [<get_ $r1 $r2>](&self) -> u16 {
                (self.$r1 as u16) << 8 | self.$r2 as u16
            }

            pub fn [<set_ $r1 $r2>](&mut self, word: u16) {
                self.$r1 = (word >> 8) as u8;
                self.$r2 = (word & 0xFF) as u8
            }
        }
    };
}

macro_rules! flag {
    ($f:ident, $bit:literal) => {
        paste::paste! {
            pub const [<FLAG_ $f:upper _MASK>]: u8 = 1 << $bit;

            pub fn [<get_flag_ $f>](&self) -> bool {
                self.f & Self::[<FLAG_ $f:upper _MASK>] != 0
            }

            pub fn [<set_flag_ $f>](&mut self, value: bool) {
                if value {
                    self.f |= Self::[<FLAG_ $f:upper _MASK>];
                } else {
                    self.f &= !Self::[<FLAG_ $f:upper _MASK>];
                }
            }
        }
    };
}

impl CPU {
    pub fn new() -> Self {
        let cpu = CPU::default();
        // TODO: init
        cpu
    }

    pub fn step(ctx: &mut GameBoy) {
        let inst = CPU::decode(ctx);
        debug!("Instruction: {inst:?}");
        CPU::execute(ctx, inst);
    }

    r16!(b + c);
    r16!(d + e);
    r16!(h + l);
    r16!(a + f);

    pub fn get_hli(&mut self) -> u16 {
        let hl = self.get_hl();
        self.set_hl(hl.wrapping_add(1));
        hl
    }

    pub fn get_hld(&mut self) -> u16 {
        let hl = self.get_hl();
        self.set_hl(hl.wrapping_sub(1));
        hl
    }

    flag!(z, 7);
    flag!(n, 6);
    flag!(h, 5);
    flag!(c, 4);

    pub fn read_tick(ctx: &mut GameBoy, address: u16) -> u8 {
        ctx.m_tick(); // Read takes 1 m-cycle
        MMU::read(ctx, address)
    }

    pub fn write_tick(ctx: &mut GameBoy, address: u16, byte: u8) {
        ctx.m_tick(); // Write takes 1 m-cycle
        MMU::write(ctx, address, byte);
    }

    pub fn next_byte(ctx: &mut GameBoy) -> u8 {
        let byte = CPU::read_tick(ctx, ctx.cpu.pc);
        if ctx.cpu.halt_bug {
            // PC doesn't increment, whoops!
            ctx.cpu.halt_bug = false
        } else {
            ctx.cpu.pc = ctx.cpu.pc.wrapping_add(1)
        }
        byte
    }

    pub fn next_word(ctx: &mut GameBoy) -> u16 {
        let lower = CPU::next_byte(ctx);
        let upper = CPU::next_byte(ctx);
        (upper as u16) << 8 | lower as u16
    }

    pub fn decode(ctx: &mut GameBoy) -> Instruction {
        let byte = CPU::next_byte(ctx) as usize;
        debug!("Byte: {}", hex!(byte, 2));
        if ctx.cpu.prefix_mode {
            ctx.cpu.prefix_mode = false;
            PREFIX_OPTABLE[byte]
        } else {
            OPTABLE[byte]
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! r16_test {
        ($r1:ident + $r2:ident) => {
            paste::paste! {
                #[test]
                fn [<r16_ $r1 $r2 _test>]() {
                    let mut cpu = CPU::default();

                    assert_eq!(cpu.$r1, 0);
                    assert_eq!(cpu.$r2, 0);
                    assert_eq!(cpu.[<get_ $r1 $r2>](), 0);

                    cpu.[<set_ $r1 $r2>](0xDEAD);
                    assert_eq!(cpu.$r1, 0xDE);
                    assert_eq!(cpu.$r2, 0xAD);
                    assert_eq!(cpu.[<get_ $r1 $r2>](), 0xDEAD);

                    cpu.$r1 = 0xBE;
                    cpu.$r2 = 0xEF;
                    assert_eq!(cpu.[<get_ $r1 $r2>](), 0xBEEF);
                }
            }
        };
    }

    r16_test!(b + c);
    r16_test!(d + e);
    r16_test!(h + l);
    r16_test!(a + f);

    #[test]
    pub fn decode_test() {
        let mut gb = GameBoy::new();

        let address = 0xC0CA; // Put instruction in work ram
        let byte = 0xFE; // Instruction CP_A_n8

        gb.cpu.pc = address;
        MMU::write(&mut gb, address, byte);

        assert_eq!(gb.cpu.pc, address);
        let inst = CPU::decode(&mut gb);
        assert_eq!(gb.cpu.pc, address + 1);
        assert_eq!(inst, Instruction::CP_A_n8)
    }

    #[test]
    pub fn decode_prefix_test() {
        let mut gb = GameBoy::new();

        let address = 0xC0BE; // Put instruction in work ram
        let prefix = 0xCB; // Instruction prefix
        let byte = 0xEF; // Instruction SET_5_A

        gb.cpu.pc = address;
        MMU::write(&mut gb, address, prefix);
        MMU::write(&mut gb, address + 1, byte);

        assert!(!gb.cpu.prefix_mode);
        assert_eq!(gb.cpu.pc, address);
        let inst = CPU::decode(&mut gb);
        assert_eq!(gb.cpu.pc, address + 1);
        assert_eq!(inst, Instruction::PREFIX);

        // "Execution" of the prefix instruction really is just a NOP and setting prefix mode
        gb.cpu.prefix_mode = true;

        let inst = CPU::decode(&mut gb);
        assert_eq!(gb.cpu.pc, address + 2);
        assert_eq!(inst, Instruction::SET_5_A);
        assert!(!gb.cpu.prefix_mode);
    }
}
