use crate::gb::GameBoy;

// TODO: DAA

pub fn nop(_ctx: &mut GameBoy) {
    // Do nothing
}

// TODO: STOP

pub fn prefix(ctx: &mut GameBoy) {
    // Next byte is interpreted as a prefixed instruction
    ctx.cpu.prefix_mode = true;
}

#[cfg(test)]
mod tests {
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
