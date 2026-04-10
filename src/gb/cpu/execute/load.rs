use log::{debug, warn};

use crate::gb::{
    GameBoy,
    cpu::{
        CPU,
        access::{ByteLoc, WordLoc},
        execute::debug::*,
    },
};

pub fn ld_r8_r8(ctx: &mut GameBoy, dest: ByteLoc, src: ByteLoc) {
    if src == dest {
        if ctx.debug_isntructions {
            match src {
                // ByteLoc::B => todo!(),
                // ByteLoc::C => todo!(),
                ByteLoc::D => print_cpu(ctx),
                ByteLoc::E => exit(ctx),
                // ByteLoc::H => todo!(),
                // ByteLoc::L => todo!(),
                // ByteLoc::A => todo!(),
                _ => warn!(
                    "executed LD {src:?}, {dest:?}: loading a register to itself is a no-op, but may be given debug meaning in the future."
                ),
            }
        } else {
            debug!("executed LD {src:?}, {dest:?}: loading a register to itself is a no-op");
        }
        return;
    }

    let byte = CPU::get_byte_at(ctx, src);
    CPU::set_byte_at(ctx, dest, byte);
}

pub fn ld_a_high(ctx: &mut GameBoy, src: ByteLoc) {
    ctx.cpu.a = CPU::get_hbyte_at(ctx, src);
}

pub fn ld_high_a(ctx: &mut GameBoy, dest: ByteLoc) {
    CPU::set_hbyte_at(ctx, dest, ctx.cpu.a);
}

pub fn ld_r16_n16(ctx: &mut GameBoy, dest: WordLoc) {
    let word = CPU::next_word(ctx);
    CPU::set_word_at(ctx, dest, word);
}

#[cfg(test)]
mod tests {
    use test_log::test;

    use crate::{
        gb::mmu::MMU,
        testutil::{INSTRUCTION_ADDRESS, registers_equal, step_test},
    };

    use super::*;

    const INST_ADD: u16 = 0xDF00;
    const MEM_ADD: u16 = 0xDFF0;
    const HIGH_ADD: u16 = 0xFF00 + HIGH_ADD_LOW as u16;
    const HIGH_ADD_LOW: u8 = 0xC4;
    const VAL: u8 = 0xA4;
    const VAL16: u16 = 0xBEEF;

    macro_rules! load_test {
        ($code:literal $dest:ident n8) => {
            paste::paste! {
                #[test]
                fn [<ld_ $dest _n8>]() {
                    let ctx = &mut GameBoy::new();
                    step_test! {
                        ctx: ctx;

                        code: $code, length: 2, cycles: 2
                        setup {
                            MMU::write(ctx, INST_ADD + 1, VAL);
                        }
                        after {
                            assert_eq!(ctx.cpu.$dest, VAL);
                        }
                    }
                }
            }
        };

        ($code:literal *$dest:ident n8) => {
            paste::paste! {
                #[test]
                fn [<ld_m $dest _n8>]() {
                    let ctx = &mut GameBoy::new();
                    step_test! {
                        ctx: ctx;

                        code: $code, length: 2, cycles: 3
                        setup {
                            MMU::write(ctx, INST_ADD + 1, VAL);
                            ctx.cpu.[<set_ $dest>](MEM_ADD);
                        }
                        after {
                            assert_eq!(MMU::read(ctx, MEM_ADD), VAL);
                        }
                    }
                }
            }
        };

        ($code:literal $reg:ident _) => {
            paste::paste! {
                #[test]
                fn [<ld_ $reg _ $reg>]() {
                    let ctx = &mut GameBoy::new();
                    let cpu_a: CPU;
                    step_test! {
                        ctx: ctx;

                        code: $code, length: 1, cycles: 1
                        setup {
                            ctx.cpu.$reg = VAL;
                            cpu_a = ctx.cpu.clone();
                        }
                        after {
                            assert!(registers_equal(&cpu_a, &ctx.cpu));
                        }
                    }
                }
            }
        };

        ($code:literal *a16 $src:ident) => {
            paste::paste! {
                #[test]
                fn [<ld_ma16_ $src>]() {
                    let ctx = &mut GameBoy::new();
                    step_test! {
                        ctx: ctx;

                        code: $code, length: 3, cycles: 4
                        setup {
                            MMU::write(ctx, INST_ADD + 1, (MEM_ADD & 0xFF) as u8);
                            MMU::write(ctx, INST_ADD + 2, (MEM_ADD >> 8) as u8);
                            ctx.cpu.$src = VAL;
                        }
                        after {
                            assert_eq!(MMU::read(ctx, MEM_ADD), VAL);
                        }
                    }
                }
            }
        };

        ($code:literal *$dest:ident $src:ident) => {
            paste::paste! {
                #[test]
                fn [<ld_m $dest _ $src>]() {
                    let ctx = &mut GameBoy::new();
                    step_test! {
                        ctx: ctx;

                        code: $code, length: 1, cycles: 2
                        setup {
                            // if $src is one of $dest, this will be overwritten (that's ok)
                            ctx.cpu.$src = VAL;
                            ctx.cpu.[<set_ $dest>](MEM_ADD);
                        }
                        after {
                            // Make sure the load was in the correct direction
                            assert!(ctx.cpu.$src != 0xFF);
                            // Check against $src here in case of overwrite
                            assert_eq!(MMU::read(ctx, MEM_ADD), ctx.cpu.$src);
                        }
                    }
                }
            }
        };

        ($code:literal $dest:ident *a16) => {
            paste::paste! {
                #[test]
                fn [<ld_ $dest _ma16>]() {
                    let ctx = &mut GameBoy::new();
                    step_test! {
                        ctx: ctx;

                        code: $code, length: 3, cycles: 4
                        setup {
                            MMU::write(ctx, INST_ADD + 1, (MEM_ADD & 0xFF) as u8);
                            MMU::write(ctx, INST_ADD + 2, (MEM_ADD >> 8) as u8);
                            MMU::write(ctx, MEM_ADD, VAL);
                        }
                        after {
                            assert_eq!(ctx.cpu.$dest, VAL);
                        }
                    }
                }
            }
        };

        ($code:literal $dest:ident *$src:ident) => {
            paste::paste! {
                #[test]
                fn [<ld_ $dest _m $src>]() {
                    let ctx = &mut GameBoy::new();
                    step_test! {
                        ctx: ctx;

                        code: $code, length: 1, cycles: 2
                        setup {
                            ctx.cpu.[<set_ $src>](MEM_ADD);
                            MMU::write(ctx, MEM_ADD, VAL);
                        }
                        after {
                            assert_eq!(ctx.cpu.$dest, VAL);
                        }
                    }
                }
            }
        };

        ($code:literal $dest:ident $src:ident) => {
            paste::paste! {
                #[test]
                fn [<ld_ $dest _ $src>]() {
                    let ctx = &mut GameBoy::new();
                    step_test! {
                        ctx: ctx;

                        code: $code, length: 1, cycles: 1
                        setup {
                            ctx.cpu.$src = VAL;
                        }
                        after {
                            assert_eq!(ctx.cpu.$dest, VAL);
                        }
                    }
                }
            }
        };
    }

    macro_rules! load_tests {
        ($(($($arg:tt)*))*) => { $(load_test!($($arg)*);)* };
    }

    macro_rules! load16_test {
        ($code:literal $dest:ident n16) => {
            paste::paste! {
                #[test]
                fn [<ld_ $dest _n16>]() {
                    let ctx = &mut GameBoy::new();
                    step_test! {
                        ctx: ctx;

                        code: $code, length: 3, cycles: 3
                        setup {
                            MMU::write(ctx, INST_ADD + 1, (VAL16 & 0xFF) as u8);
                            MMU::write(ctx, INST_ADD + 2, (VAL16 >> 8) as u8);
                        }
                        after {
                            assert_eq!(ctx.cpu.[<get_ $dest>](), VAL16);
                        }
                    }
                }
            }
        };
    }

    macro_rules! load16_tests {
        ($(($($arg:tt)*))*) => { $(load16_test!($($arg)*);)* };
    }

    load_tests! {
        // Simple reg-to-reg loads
        (0x40 b _) (0x41 b c) (0x42 b d) (0x43 b e) (0x44 b h) (0x45 b l) (0x47 b a)
        (0x48 c b) (0x49 c _) (0x4A c d) (0x4B c e) (0x4C c h) (0x4D c l) (0x4F c a)
        (0x50 d b) (0x51 d c) (0x52 d _) (0x53 d e) (0x54 d h) (0x55 d l) (0x57 d a)
        (0x58 e b) (0x59 e c) (0x5A e d) (0x5B e _) (0x5C e h) (0x5D e l) (0x5F e a)
        (0x60 h b) (0x61 h c) (0x62 h d) (0x63 h e) (0x64 h _) (0x65 h l) (0x67 h a)
        (0x68 l b) (0x69 l c) (0x6A l d) (0x6B l e) (0x6C l h) (0x6D l _) (0x6F l a)
        (0x78 a b) (0x79 a c) (0x7A a d) (0x7B a e) (0x7C a h) (0x7D a l) (0x7F a _)

        // Memory loads
        (0x46 b *hl) (0x4E c *hl) (0x56 d *hl) (0x5E e *hl) (0x66 h *hl) (0x6E l *hl) (0x7E a *hl)
        (0x70 *hl b) (0x71 *hl c) (0x72 *hl d) (0x73 *hl e) (0x74 *hl h) (0x75 *hl l) (0x77 *hl a)

        // Other memory <=> A loads
        (0x0A a *bc) (0x1A a *de) (0xFA a *a16)
        (0x02 *bc a) (0x12 *de a) (0xEA *a16 a)

        // Constant loads
        (0x06 b n8) (0x0E c n8) (0x16 d n8) (0x1E e n8) (0x26 h n8) (0x2E l n8) (0x36 *hl n8) (0x3E a n8)
    }

    load16_tests! {
        (0x01 bc n16) (0x11 de n16) (0x21 hl n16)
    }

    // HL+ and HL- loads
    #[test]
    fn ld_mhli_a() {
        let ctx = &mut GameBoy::new();
        step_test! {
            ctx: ctx;

            code: 0x22, length: 1, cycles: 2
            setup {
                ctx.cpu.a = VAL;
                ctx.cpu.set_hl(MEM_ADD);
            }
            after {
                assert_eq!(MMU::read(ctx, MEM_ADD), VAL);
                assert_eq!(ctx.cpu.get_hl(), MEM_ADD + 1);
            }
        }
    }

    #[test]
    fn ld_mhld_a() {
        let ctx = &mut GameBoy::new();
        step_test! {
            ctx: ctx;

            code: 0x32, length: 1, cycles: 2
            setup {
                ctx.cpu.a = VAL;
                ctx.cpu.set_hl(MEM_ADD);
            }
            after {
                assert_eq!(MMU::read(ctx, MEM_ADD), VAL);
                assert_eq!(ctx.cpu.get_hl(), MEM_ADD - 1);
            }
        }
    }

    #[test]
    fn ld_a_mhli() {
        let ctx = &mut GameBoy::new();
        step_test! {
            ctx: ctx;

            code: 0x2A, length: 1, cycles: 2
            setup {
                MMU::write(ctx, MEM_ADD, VAL);
                ctx.cpu.set_hl(MEM_ADD);
            }
            after {
                assert_eq!(ctx.cpu.a, VAL);
                assert_eq!(ctx.cpu.get_hl(), MEM_ADD + 1);
            }
        }
    }

    #[test]
    fn ld_a_mhld() {
        let ctx = &mut GameBoy::new();
        step_test! {
            ctx: ctx;

            code: 0x3A, length: 1, cycles: 2
            setup {
                MMU::write(ctx, MEM_ADD, VAL);
                ctx.cpu.set_hl(MEM_ADD);
            }
            after {
                assert_eq!(ctx.cpu.a, VAL);
                assert_eq!(ctx.cpu.get_hl(), MEM_ADD - 1);
            }
        }
    }

    // High loads
    #[test]
    fn ldh_ma8_a() {
        let ctx = &mut GameBoy::new();
        step_test! {
            ctx: ctx;

            code: 0xE0, length: 2, cycles: 3
            setup {
                MMU::write(ctx, INSTRUCTION_ADDRESS + 1, HIGH_ADD_LOW);
                ctx.cpu.a = VAL;
            }
            after {
                assert_eq!(MMU::read(ctx, HIGH_ADD), VAL);
            }
        }
    }

    #[test]
    fn ldh_mc_a() {
        let ctx = &mut GameBoy::new();
        step_test! {
            ctx: ctx;

            code: 0xE2, length: 1, cycles: 2
            setup {
                ctx.cpu.c = HIGH_ADD_LOW;
                ctx.cpu.a = VAL;
            }
            after {
                assert_eq!(MMU::read(ctx, HIGH_ADD), VAL);
            }
        }
    }

    #[test]
    fn ldh_a_ma8() {
        let ctx = &mut GameBoy::new();
        step_test! {
            ctx: ctx;

            code: 0xF0, length: 2, cycles: 3
            setup {
                MMU::write(ctx, INSTRUCTION_ADDRESS + 1, HIGH_ADD_LOW);
                MMU::write(ctx, HIGH_ADD, VAL);
            }
            after {
                assert_eq!(ctx.cpu.a, VAL);
            }
        }
    }

    #[test]
    fn ldh_a_mc() {
        let ctx = &mut GameBoy::new();
        step_test! {
            ctx: ctx;

            code: 0xF2, length: 1, cycles: 2
            setup {
                ctx.cpu.c = HIGH_ADD_LOW;
                MMU::write(ctx, HIGH_ADD, VAL);
            }
            after {
                assert_eq!(ctx.cpu.a, VAL);
            }
        }
    }
}
