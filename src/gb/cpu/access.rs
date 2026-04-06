use crate::gb::{GameBoy, cpu::CPU};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ByteLoc {
    // Registers
    B,
    C,
    D,
    E,
    H,
    L,
    A,

    // Memory locations
    MBC,
    MDE,
    MHL,
    MHLI,
    MHLD,

    // Constants
    MA16,
    N8,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum WordLoc {
    BC,
    DE,
    HL,
    N16,
}

impl CPU {
    pub(super) fn get_location(ctx: &mut GameBoy, loc: ByteLoc) -> u8 {
        match loc {
            ByteLoc::B => ctx.cpu.b,
            ByteLoc::C => ctx.cpu.c,
            ByteLoc::D => ctx.cpu.d,
            ByteLoc::E => ctx.cpu.e,
            ByteLoc::H => ctx.cpu.h,
            ByteLoc::L => ctx.cpu.l,
            ByteLoc::A => ctx.cpu.a,

            ByteLoc::MBC => CPU::read_tick(ctx, ctx.cpu.get_bc()),
            ByteLoc::MDE => CPU::read_tick(ctx, ctx.cpu.get_de()),
            ByteLoc::MHL => CPU::read_tick(ctx, ctx.cpu.get_hl()),
            ByteLoc::MHLI => {
                let address = ctx.cpu.get_hli();
                CPU::read_tick(ctx, address)
            }
            ByteLoc::MHLD => {
                let address = ctx.cpu.get_hld();
                CPU::read_tick(ctx, address)
            }

            ByteLoc::MA16 => {
                let address = CPU::next_word(ctx);
                CPU::read_tick(ctx, address)
            }
            ByteLoc::N8 => CPU::next_byte(ctx),
        }
    }

    pub(super) fn set_location(ctx: &mut GameBoy, loc: ByteLoc, byte: u8) {
        match loc {
            ByteLoc::B => ctx.cpu.b = byte,
            ByteLoc::C => ctx.cpu.c = byte,
            ByteLoc::D => ctx.cpu.d = byte,
            ByteLoc::E => ctx.cpu.e = byte,
            ByteLoc::H => ctx.cpu.h = byte,
            ByteLoc::L => ctx.cpu.l = byte,
            ByteLoc::A => ctx.cpu.a = byte,

            ByteLoc::MBC => CPU::write_tick(ctx, ctx.cpu.get_bc(), byte),
            ByteLoc::MDE => CPU::write_tick(ctx, ctx.cpu.get_de(), byte),
            ByteLoc::MHL => CPU::write_tick(ctx, ctx.cpu.get_hl(), byte),
            ByteLoc::MHLI => {
                let address = ctx.cpu.get_hli();
                CPU::write_tick(ctx, address, byte);
            }
            ByteLoc::MHLD => {
                let address = ctx.cpu.get_hld();
                CPU::write_tick(ctx, address, byte);
            }

            ByteLoc::MA16 => {
                let address = CPU::next_word(ctx);
                CPU::write_tick(ctx, address, byte);
            }
            ByteLoc::N8 => unimplemented!("can't write to a constant"),
        }
    }

    pub(super) fn get_high_location(ctx: &mut GameBoy, loc: ByteLoc) -> u8 {
        match loc {
            ByteLoc::C => CPU::read_tick(ctx, 0xFF00 + ctx.cpu.c as u16),
            ByteLoc::N8 => {
                let half_address = CPU::next_byte(ctx) as u16;
                CPU::read_tick(ctx, 0xFF00 + half_address)
            }

            other => unimplemented!("can't get_high from {other:?}, only C or N8"),
        }
    }

    pub(super) fn set_high_location(ctx: &mut GameBoy, loc: ByteLoc, byte: u8) {
        match loc {
            ByteLoc::C => CPU::write_tick(ctx, 0xFF00 + ctx.cpu.c as u16, byte),
            ByteLoc::N8 => {
                let half_address = CPU::next_byte(ctx) as u16;
                CPU::write_tick(ctx, 0xFF00 + half_address, byte);
            }

            other => unimplemented!("can't set_high from {other:?}, only C or N8"),
        }
    }

    pub(super) fn set_word_location(ctx: &mut GameBoy, loc: WordLoc, word: u16) {
        match loc {
            WordLoc::BC => ctx.cpu.set_bc(word),
            WordLoc::DE => ctx.cpu.set_de(word),
            WordLoc::HL => ctx.cpu.set_hl(word),
            WordLoc::N16 => unimplemented!("can't write to a constant"),
        }
    }
}
