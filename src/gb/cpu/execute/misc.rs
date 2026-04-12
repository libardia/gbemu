use log::warn;

use crate::gb::GameBoy;

// TODO: DAA

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
    use test_log::test;

    use crate::testutil::step_test;

    use super::*;

    #[test]
    fn nop() {
        let ctx = &mut GameBoy::new();
        step_test! {
            ctx: ctx;
            code: 0x00, length: 1, cycles: 1
            // Nothing else to do!
        }
    }

    #[test]
    fn prefix() {
        let ctx = &mut GameBoy::new();
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
