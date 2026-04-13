use log::warn;

use crate::gb::GameBoy;

pub fn daa(ctx: &mut GameBoy) {
    let mut f = ctx.cpu.f;
    let mut a = ctx.cpu.a;
    let mut adj = 0;

    if f.n {
        if f.h {
            adj += 0x6;
        }
        if f.c {
            adj += 0x60;
        }
        a = a.wrapping_sub(adj);
    } else {
        if f.h || (a & 0xF) > 0x9 {
            adj += 0x6;
        }
        if f.c || a > 0x99 {
            adj += 0x60;
            f.c = true;
        }
        a = a.wrapping_add(adj);
    }

    f.z = a == 0;
    f.h = false;

    ctx.cpu.a = a;
    ctx.cpu.f = f;
}

pub fn nop(_ctx: &mut GameBoy) {
    // Do nothing
}

pub fn stop(ctx: &mut GameBoy) {
    // The STOP instruction is INSANE. It's full behavior can be found here
    // https://gbdev.io/pandocs/Reducing_Power_Consumption.html#the-bizarre-case-of-the-game-boy-stop-instruction-before-even-considering-timing

    //TODO: maybe try to actually emulate STOP.
    warn!("STOP instruction is not yet implemented and will be ignored");

    // the byte after stop is ignored (unless we're actually trying to emulate it)
    ctx.cpu.pc += 1;
}

pub fn prefix(ctx: &mut GameBoy) {
    // Next byte is interpreted as a prefixed instruction
    ctx.cpu.prefix_mode = true;
}

#[cfg(test)]
mod tests {
    use crate::testutil::{dummy_ctx, step_test};

    use test_log::test;

    #[test]
    fn nop() {
        let ctx = &mut dummy_ctx();
        step_test! {
            ctx: ctx;
            code: 0x00, length: 1, cycles: 1
            // Nothing else to do!
        }
    }

    #[test]
    fn stop() {
        let ctx = &mut dummy_ctx();
        step_test! {
            ctx:ctx;
            code: 0x10, length: 2, cycles: 1
            // TODO: this test will need to change if STOP is ever actually implemented
        }
    }

    #[test]
    fn prefix() {
        let ctx = &mut dummy_ctx();
        step_test! {
            ctx: ctx;
            code: 0xCB, length: 1, cycles: 1
            setup {
                assert!(!ctx.cpu.prefix_mode);
            }
            after {
                assert!(ctx.cpu.prefix_mode);
            }
        }
    }
}
