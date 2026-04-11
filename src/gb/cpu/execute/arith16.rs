use crate::gb::{
    GameBoy,
    cpu::{CPU, Flags, access::WordLoc},
};

pub fn add_hl_r16(ctx: &mut GameBoy, target: WordLoc) {
    let lhs = ctx.cpu.get_hl();
    let rhs = CPU::get_word_at(ctx, target);
    let (result, overflow) = ctx.cpu.get_hl().overflowing_add(rhs);

    ctx.m_tick();
    ctx.cpu.set_hl(result);

    ctx.cpu.f = Flags {
        z: ctx.cpu.f.z,
        n: false,
        h: (lhs & 0xFFF) + (rhs & 0xFFF) > 0xFFF,
        c: overflow,
    }
}

pub fn inc_r16(ctx: &mut GameBoy, target: WordLoc) {
    let before = CPU::get_word_at(ctx, target);
    let after = before.wrapping_add(1);

    ctx.m_tick(); // Takes 2 ticks total
    CPU::set_word_at(ctx, target, after);
}

pub fn dec_r16(ctx: &mut GameBoy, target: WordLoc) {
    let before = CPU::get_word_at(ctx, target);
    let after = before.wrapping_sub(1);

    ctx.m_tick(); // Takes 2 ticks total
    CPU::set_word_at(ctx, target, after);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testutil::step_test;
    use test_log::test;

    const HL: u16 = 0x1234;
    const VAL: u16 = 0xFEDC;
    const VALI: u16 = 0xFEDD;
    const VALD: u16 = 0xFEDB;
    const RESULT: u16 = 0x1110;

    /* #region Addition */
    #[test]
    fn add_hl_bc() {
        let ctx = &mut GameBoy::new();
        step_test! {
            ctx: ctx;

            code: 0x09, length: 1, cycles: 2
            setup {
                ctx.cpu.set_hl(HL);
                ctx.cpu.set_bc(VAL);
            }
            after {
                assert_eq!(ctx.cpu.get_hl(), RESULT);
                assert!(!ctx.cpu.f.n);
                assert!( ctx.cpu.f.h);
                assert!( ctx.cpu.f.c);
            }
        }
    }

    #[test]
    fn add_hl_de() {
        let ctx = &mut GameBoy::new();
        step_test! {
            ctx: ctx;

            code: 0x19, length: 1, cycles: 2
            setup {
                ctx.cpu.set_hl(HL);
                ctx.cpu.set_de(VAL);
            }
            after {
                assert_eq!(ctx.cpu.get_hl(), RESULT);
                assert!(!ctx.cpu.f.n);
                assert!( ctx.cpu.f.h);
                assert!( ctx.cpu.f.c);
            }
        }
    }

    #[test]
    fn add_hl_hl() {
        let ctx = &mut GameBoy::new();
        step_test! {
            ctx: ctx;

            code: 0x29, length: 1, cycles: 2
            setup {
                ctx.cpu.set_hl(HL);
            }
            after {
                assert_eq!(ctx.cpu.get_hl(), 0x2468);
                assert!(!ctx.cpu.f.n);
                assert!(!ctx.cpu.f.h);
                assert!(!ctx.cpu.f.c);
            }
        }
    }

    #[test]
    fn add_hl_sp() {
        let ctx = &mut GameBoy::new();
        step_test! {
            ctx: ctx;

            code: 0x39, length: 1, cycles: 2
            setup {
                ctx.cpu.set_hl(HL);
                ctx.cpu.sp = VAL;
            }
            after {
                assert_eq!(ctx.cpu.get_hl(), RESULT);
                assert!(!ctx.cpu.f.n);
                assert!( ctx.cpu.f.h);
                assert!( ctx.cpu.f.c);
            }
        }
    }
    /* #endregion */

    /* #region Increment */
    #[test]
    fn inc_bc() {
        let ctx = &mut GameBoy::new();
        step_test! {
            ctx: ctx;

            code: 0x03, length: 1, cycles: 2
            setup {
                ctx.cpu.set_bc(VAL);
            }
            after {
                assert_eq!(ctx.cpu.get_bc(), VALI);
            }
        }
    }

    #[test]
    fn inc_de() {
        let ctx = &mut GameBoy::new();
        step_test! {
            ctx: ctx;

            code: 0x13, length: 1, cycles: 2
            setup {
                ctx.cpu.set_de(VAL);
            }
            after {
                assert_eq!(ctx.cpu.get_de(), VALI);
            }
        }
    }

    #[test]
    fn inc_hl() {
        let ctx = &mut GameBoy::new();
        step_test! {
            ctx: ctx;

            code: 0x23, length: 1, cycles: 2
            setup {
                ctx.cpu.set_hl(VAL);
            }
            after {
                assert_eq!(ctx.cpu.get_hl(), VALI);
            }
        }
    }

    #[test]
    fn inc_sp() {
        let ctx = &mut GameBoy::new();
        step_test! {
            ctx: ctx;

            code: 0x33, length: 1, cycles: 2
            setup {
                ctx.cpu.sp = VAL;
            }
            after {
                assert_eq!(ctx.cpu.sp, VALI);
            }
        }
    }
    /* #endregion */

    /* #region Decrement */
    #[test]
    fn dec_bc() {
        let ctx = &mut GameBoy::new();
        step_test! {
            ctx: ctx;

            code: 0x0B, length: 1, cycles: 2
            setup {
                ctx.cpu.set_bc(VAL);
            }
            after {
                assert_eq!(ctx.cpu.get_bc(), VALD);
            }
        }
    }

    #[test]
    fn dec_de() {
        let ctx = &mut GameBoy::new();
        step_test! {
            ctx: ctx;

            code: 0x1B, length: 1, cycles: 2
            setup {
                ctx.cpu.set_de(VAL);
            }
            after {
                assert_eq!(ctx.cpu.get_de(), VALD);
            }
        }
    }

    #[test]
    fn dec_hl() {
        let ctx = &mut GameBoy::new();
        step_test! {
            ctx: ctx;

            code: 0x2B, length: 1, cycles: 2
            setup {
                ctx.cpu.set_hl(VAL);
            }
            after {
                assert_eq!(ctx.cpu.get_hl(), VALD);
            }
        }
    }

    #[test]
    fn dec_sp() {
        let ctx = &mut GameBoy::new();
        step_test! {
            ctx: ctx;

            code: 0x3B, length: 1, cycles: 2
            setup {
                ctx.cpu.sp = VAL;
            }
            after {
                assert_eq!(ctx.cpu.sp, VALD);
            }
        }
    }
    /* #endregion */
}
