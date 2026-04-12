use crate::gb::{
    GameBoy,
    cpu::{CPU, Flags, access::WordLoc},
};

pub fn push_r16(ctx: &mut GameBoy, src: WordLoc) {
    ctx.m_tick(); // 1 tick
    let word = CPU::get_word_at(ctx, src);
    CPU::push_stack(ctx, word); // 2 ticks
}

pub fn pop_r16(ctx: &mut GameBoy, dest: WordLoc) {
    let word = CPU::pop_stack(ctx);
    CPU::set_word_at(ctx, dest, word); // 2 ticks
}

pub fn add_sp_e8(ctx: &mut GameBoy) {
    let off = CPU::next_signed(ctx); // 1 tick

    // The flags here are extremely confusing. Here's how they actually work (I hope, this was
    // from someone on r/EmuDev):
    // https://www.reddit.com/r/EmuDev/comments/y51i1c/game_boy_dealing_with_carry_flags_when_handling/
    // In the real hardware, this operation is done by first doing an unsigned 8 bit addition
    // between LOW(SP) and off, and the carry and half carry flags are set from that, while Z and N
    // are always reset. Then it translates that to a signed addition to SP.
    weird_flags(ctx, off);

    // Okay, now do the real thing lol
    ctx.m_tick();
    ctx.m_tick(); // 2 ticks
    ctx.cpu.sp = ctx.cpu.sp.wrapping_add_signed(off as i16);
}

pub fn ld_hl_sp_e8(ctx: &mut GameBoy) {
    let off = CPU::next_signed(ctx); // 1 tick

    // This instruction wasn't mentioned in the same post as 'ADD SP e8', but by context it's
    // clear that the flags are handled the same way.
    weird_flags(ctx, off);

    // Now do the real thing
    ctx.m_tick(); // 1 tick
    ctx.cpu.set_hl(ctx.cpu.sp.wrapping_add_signed(off as i16));
}

pub fn ld_sp_n16(ctx: &mut GameBoy) {
    let word = CPU::next_word(ctx); // 2 ticks
    ctx.cpu.sp = word;
}

pub fn ld_sp_hl(ctx: &mut GameBoy) {
    ctx.m_tick(); // 1 tick
    ctx.cpu.sp = ctx.cpu.get_hl();
}

pub fn ld_ma16_sp(ctx: &mut GameBoy) {
    let address = CPU::next_word(ctx); // 2 ticks
    CPU::write_tick(ctx, address, (ctx.cpu.sp & 0xFF) as u8); // 1 tick
    CPU::write_tick(ctx, address + 1, (ctx.cpu.sp >> 8) as u8); // 1 tick
}

fn weird_flags(ctx: &mut GameBoy, off: i8) {
    let low_sp = (ctx.cpu.sp & 0xFF) as u8;
    let uoff = off as u8;

    ctx.cpu.f = Flags {
        z: false,
        n: false,
        h: ((low_sp & 0xF) + (uoff & 0xF)) > 0xF,
        c: low_sp.overflowing_add(uoff).1,
    };
}

#[cfg(test)]
mod tests {
    use crate::{
        gb::mmu::MMU,
        testutil::{INSTRUCTION_ADDRESS, step_test},
    };

    use super::*;
    use test_log::test;

    fn test_push_stack(ctx: &mut GameBoy, word: u16) {
        MMU::write(ctx, ctx.cpu.sp - 1, (word >> 8) as u8);
        MMU::write(ctx, ctx.cpu.sp - 2, (word & 0xFF) as u8);
        ctx.cpu.sp -= 2;
    }

    fn test_peek_stack(ctx: &mut GameBoy) -> u16 {
        let low = MMU::read(ctx, ctx.cpu.sp);
        let high = MMU::read(ctx, ctx.cpu.sp + 1);

        ((high as u16) << 8) | (low as u16)
    }

    #[test]
    fn ld_sp_n16() {
        let ctx = &mut GameBoy::new();
        step_test! {
            ctx: ctx;
            code: 0x31, length: 3, cycles: 3
            setup {
                MMU::write(ctx, INSTRUCTION_ADDRESS + 1, 0xAD);
                MMU::write(ctx, INSTRUCTION_ADDRESS + 2, 0xDE);
            }
            after {
                assert_eq!(ctx.cpu.sp, 0xDEAD);
            }
        }
    }

    #[test]
    fn ld_sp_hl() {
        let ctx = &mut GameBoy::new();
        step_test! {
            ctx: ctx;
            code: 0xF9, length: 1, cycles: 2
            setup {
                ctx.cpu.set_hl(0xDEAD);
            }
            after {
                assert_eq!(ctx.cpu.sp, 0xDEAD);
            }
        }
    }

    #[test]
    fn ld_hl_sp_e8() {
        let ctx = &mut GameBoy::new();
        step_test! {
            ctx: ctx;
            code: 0xF8, length: 2, cycles: 3
            setup {
                ctx.cpu.sp = 0xDEAD;
                MMU::write(ctx, INSTRUCTION_ADDRESS + 1, (-0x15i8) as u8);
            }
            after {
                assert_eq!(ctx.cpu.get_hl(), 0xDE98);
                assert!(!ctx.cpu.f.z);
                assert!(!ctx.cpu.f.n);
                assert!( ctx.cpu.f.h);
                assert!( ctx.cpu.f.c);
            }
        }
    }

    #[test]
    fn ld_ma16_sp() {
        let ctx = &mut GameBoy::new();
        step_test! {
            ctx: ctx;
            code: 0x08, length: 3, cycles: 5
            setup {
                ctx.cpu.sp = 0xDEAD;
                MMU::write(ctx, INSTRUCTION_ADDRESS + 1, 0x00);
                MMU::write(ctx, INSTRUCTION_ADDRESS + 2, 0xDF);
            }
            after {
                assert_eq!(MMU::read(ctx, 0xDF00), 0xAD);
                assert_eq!(MMU::read(ctx, 0xDF01), 0xDE);
            }
        }
    }

    #[test]
    fn add_sp_e8() {
        let ctx = &mut GameBoy::new();
        step_test! {
            ctx: ctx;
            code: 0xE8, length: 2, cycles: 4
            setup {
                ctx.cpu.sp = 0xDEAD;
                MMU::write(ctx, INSTRUCTION_ADDRESS + 1, (-0x15i8) as u8);
            }
            after {
                assert_eq!(ctx.cpu.sp, 0xDE98);
                assert!(!ctx.cpu.f.z);
                assert!(!ctx.cpu.f.n);
                assert!( ctx.cpu.f.h);
                assert!( ctx.cpu.f.c);
            }
        }
    }

    macro_rules! pop_tests {
        ($($code:literal $dest:ident;)*) => {
            paste::paste! {$(
                #[test]
                fn [<pop_ $dest>]() {
                    let ctx = &mut GameBoy::new();
                    step_test! {
                        ctx: ctx;
                        code: $code, length: 1, cycles: 3
                        setup {
                            ctx.cpu.sp = 0xFFFE;
                            test_push_stack(ctx, 0xDEAD);
                        }
                        after {
                            assert_eq!(ctx.cpu.[<get_ $dest>](), 0xDEAD);
                        }
                    }
                }
            )*}
        };
    }

    pop_tests! {
        0xC1 bc;
        0xD1 de;
        0xE1 hl;
    }

    #[test]
    fn pop_af() {
        let ctx = &mut GameBoy::new();
        step_test! {
            ctx: ctx;
            code: 0xF1, length: 1, cycles: 3
            setup {
                ctx.cpu.sp = 0xFFFE;
                test_push_stack(ctx, 0xDEAD);
            }
            after {
                assert_eq!(ctx.cpu.get_af(), 0xDEA0);
                assert!( ctx.cpu.f.z);
                assert!(!ctx.cpu.f.n);
                assert!( ctx.cpu.f.h);
                assert!(!ctx.cpu.f.c);
            }
        }
    }

    macro_rules! push_tests {
        ($($code:literal $src:ident;)*) => {
            paste::paste! {$(
                #[test]
                fn [<push_ $src>]() {
                    let ctx = &mut GameBoy::new();
                    step_test! {
                        ctx: ctx;
                        code: $code, length: 1, cycles: 4
                        setup {
                            ctx.cpu.sp = 0xFFFE;
                            ctx.cpu.[<set_ $src>](0xDEAD);
                        }
                        after {
                            assert_eq!(test_peek_stack(ctx), 0xDEAD);
                        }
                    }
                }
            )*}
        };
    }

    push_tests! {
        0xC5 bc;
        0xD5 de;
        0xE5 hl;
    }

    #[test]
    fn push_af() {
        let ctx = &mut GameBoy::new();
        step_test! {
            ctx: ctx;
            code: 0xF5, length: 1, cycles: 4
            setup {
                ctx.cpu.sp = 0xFFFE;
                ctx.cpu.set_af(0xDEAD);
            }
            after {
                assert_eq!(test_peek_stack(ctx), 0xDEA0);
            }
        }
    }
}
