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
    SP,
}

impl CPU {
    pub fn get_byte_at(ctx: &mut GameBoy, loc: ByteLoc) -> u8 {
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

    pub fn set_byte_at(ctx: &mut GameBoy, loc: ByteLoc, byte: u8) {
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

    pub fn get_hbyte_at(ctx: &mut GameBoy, loc: ByteLoc) -> u8 {
        match loc {
            ByteLoc::C => CPU::read_tick(ctx, 0xFF00 + ctx.cpu.c as u16),
            ByteLoc::N8 => {
                let half_address = CPU::next_byte(ctx) as u16;
                CPU::read_tick(ctx, 0xFF00 + half_address)
            }

            other => unimplemented!("can't get_high from {other:?}, only C or N8"),
        }
    }

    pub fn set_hbyte_at(ctx: &mut GameBoy, loc: ByteLoc, byte: u8) {
        match loc {
            ByteLoc::C => {
                let half_address = ctx.cpu.c as u16;
                CPU::write_tick(ctx, 0xFF00 + half_address, byte);
            }
            ByteLoc::N8 => {
                let half_address = CPU::next_byte(ctx) as u16;
                CPU::write_tick(ctx, 0xFF00 + half_address, byte);
            }

            other => unimplemented!("can't set_high from {other:?}, only C or N8"),
        }
    }

    pub fn get_word_at(ctx: &mut GameBoy, loc: WordLoc) -> u16 {
        match loc {
            WordLoc::BC => ctx.cpu.get_bc(),
            WordLoc::DE => ctx.cpu.get_de(),
            WordLoc::HL => ctx.cpu.get_hl(),
            WordLoc::SP => ctx.cpu.sp,
        }
    }

    pub fn set_word_at(ctx: &mut GameBoy, loc: WordLoc, word: u16) {
        match loc {
            WordLoc::BC => ctx.cpu.set_bc(word),
            WordLoc::DE => ctx.cpu.set_de(word),
            WordLoc::HL => ctx.cpu.set_hl(word),
            WordLoc::SP => ctx.cpu.sp = word,
        }
    }
}

#[cfg(test)]
mod tests {
    use test_log::test;

    use crate::gb::mmu::MMU;
    use ByteLoc::*;

    use super::*;

    #[test]
    fn getset_byte_at() {
        let ctx = &mut GameBoy::new();

        CPU::set_byte_at(ctx, B, 0xBB);
        CPU::set_byte_at(ctx, C, 0xCC);
        CPU::set_byte_at(ctx, D, 0xDD);
        CPU::set_byte_at(ctx, E, 0xEE);
        CPU::set_byte_at(ctx, H, 0xFF);
        CPU::set_byte_at(ctx, L, 0x99);
        CPU::set_byte_at(ctx, A, 0xAA);

        assert_eq!(CPU::get_byte_at(ctx, B), 0xBB);
        assert_eq!(ctx.cpu.b, 0xBB);
        assert_eq!(CPU::get_byte_at(ctx, C), 0xCC);
        assert_eq!(ctx.cpu.c, 0xCC);
        assert_eq!(CPU::get_byte_at(ctx, D), 0xDD);
        assert_eq!(ctx.cpu.d, 0xDD);
        assert_eq!(CPU::get_byte_at(ctx, E), 0xEE);
        assert_eq!(ctx.cpu.e, 0xEE);
        assert_eq!(CPU::get_byte_at(ctx, H), 0xFF);
        assert_eq!(ctx.cpu.h, 0xFF);
        assert_eq!(CPU::get_byte_at(ctx, L), 0x99);
        assert_eq!(ctx.cpu.l, 0x99);
        assert_eq!(CPU::get_byte_at(ctx, A), 0xAA);
        assert_eq!(ctx.cpu.a, 0xAA);

        ctx.cpu.set_bc(0xCA01);
        ctx.cpu.set_de(0xCB02);
        ctx.cpu.set_hl(0xCC03);

        ctx.cpu.pc = 0xDF00;
        MMU::write(ctx, 0xDF00, 0x04);
        MMU::write(ctx, 0xDF01, 0xCD);

        CPU::set_byte_at(ctx, MBC, 0xBC);
        CPU::set_byte_at(ctx, MDE, 0xDE);
        CPU::set_byte_at(ctx, MHL, 0xF9);
        CPU::set_byte_at(ctx, MA16, 0x16);
        ctx.cpu.pc = 0xDF00; // reset PC (because it will change after last call)

        assert_eq!(CPU::get_byte_at(ctx, MBC), 0xBC);
        assert_eq!(MMU::read(ctx, 0xCA01), 0xBC);
        assert_eq!(CPU::get_byte_at(ctx, MDE), 0xDE);
        assert_eq!(MMU::read(ctx, 0xCB02), 0xDE);
        assert_eq!(CPU::get_byte_at(ctx, MHL), 0xF9);
        assert_eq!(MMU::read(ctx, 0xCC03), 0xF9);
        assert_eq!(CPU::get_byte_at(ctx, MA16), 0x16);
        assert_eq!(MMU::read(ctx, 0xCD04), 0x16);
    }

    #[test]
    #[should_panic]
    fn set_byte_at_n8() {
        let ctx = &mut GameBoy::new();
        CPU::set_byte_at(ctx, N8, 0x08);
    }

    #[test]
    fn getset_hbyte_at() {
        let ctx = &mut GameBoy::new();

        ctx.cpu.c = 0x81;
        ctx.cpu.pc = 0xDF00;
        MMU::write(ctx, 0xDF00, 0x82);

        CPU::set_hbyte_at(ctx, C, 0xCC);
        CPU::set_hbyte_at(ctx, N8, 0x08);
        ctx.cpu.pc = 0xDF00; // reset PC (because it will change after last call)

        assert_eq!(CPU::get_hbyte_at(ctx, C), 0xCC);
        assert_eq!(MMU::read(ctx, 0xFF81), 0xCC);
        assert_eq!(CPU::get_hbyte_at(ctx, N8), 0x08);
        assert_eq!(MMU::read(ctx, 0xFF82), 0x08);
    }

    macro_rules! panic_set_hbyte {
        ($($loc:expr),*) => {
            paste::paste! {$(
                #[test]
                #[should_panic]
                fn [<panic_test_set_hbyte_ $loc:lower>]() {
                    CPU::set_hbyte_at(&mut GameBoy::new(), $loc, 0x00);
                }
            )*}
        };
    }

    macro_rules! panic_get_hbyte {
        ($($loc:expr),*) => {
            paste::paste! {$(
                #[test]
                #[should_panic]
                fn [<panic_test_get_hbyte_ $loc:lower>]() {
                    CPU::get_hbyte_at(&mut GameBoy::new(), $loc);
                }
            )*}
        };
    }

    panic_set_hbyte!(B, D, E, H, L, A, MBC, MDE, MHL, MHLI, MHLD, MA16);
    panic_get_hbyte!(B, D, E, H, L, A, MBC, MDE, MHL, MHLI, MHLD, MA16);
}
