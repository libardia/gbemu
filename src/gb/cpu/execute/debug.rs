use log::debug;

use crate::gb::GameBoy;

pub fn exit(ctx: &mut GameBoy) {
    ctx.exit = true;
}

pub fn print_cpu(ctx: &mut GameBoy) {
    debug!("{}", ctx.cpu.debug_str());
}

#[cfg(test)]
mod tests {
    use test_log::test;

    use crate::{
        gb::{GameBoy, cpu::Flags},
        testutil::{dummy_ctx, step_test},
    };

    #[test]
    fn test_print_cpu() {
        let ctx = &mut dummy_ctx();
        ctx.debug_isntructions = true;
        step_test!(
            ctx: ctx;

            code: 0x52, length: 1, cycles: 1
            setup {
                ctx.cpu.b = 0xBB;
                ctx.cpu.c = 0xCC;
                ctx.cpu.d = 0xDD;
                ctx.cpu.e = 0xEE;
                ctx.cpu.h = 0xFF;
                ctx.cpu.l = 0x99;
                ctx.cpu.a = 0xAA;
                ctx.cpu.f = Flags { z: true, n: false, h: true, c: false };
                ctx.cpu.sp = 0xBEEF;
            }
            after {
                // assert!(false);
            }
        )
    }
}
