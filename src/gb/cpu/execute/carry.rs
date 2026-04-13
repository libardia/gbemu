use crate::gb::GameBoy;

pub fn scf(ctx: &mut GameBoy) {
    ctx.cpu.f.c = true;
}

pub fn ccf(ctx: &mut GameBoy) {
    ctx.cpu.f.c = !ctx.cpu.f.c;
}

#[cfg(test)]
mod tests {
    use test_log::test;

    use crate::testutil::{dummy_ctx, step_test};

    use super::*;

    #[test]
    fn scf_0() {
        let ctx = &mut dummy_ctx();
        step_test! {
            ctx: ctx;

            code: 0x37, length: 1, cycles: 1
            setup {
                ctx.cpu.f.c = false;
            }
            after {
                assert!(ctx.cpu.f.c);
            }
        }
    }

    #[test]
    fn scf_1() {
        let ctx = &mut dummy_ctx();
        step_test! {
            ctx: ctx;

            code: 0x37, length: 1, cycles: 1
            setup {
                ctx.cpu.f.c = true;
            }
            after {
                assert!(ctx.cpu.f.c);
            }
        }
    }

    #[test]
    fn ccf_0() {
        let ctx = &mut dummy_ctx();
        step_test! {
            ctx: ctx;

            code: 0x3F, length: 1, cycles: 1
            setup {
                ctx.cpu.f.c = false;
            }
            after {
                assert!(ctx.cpu.f.c);
            }
        }
    }

    #[test]
    fn ccf_1() {
        let ctx = &mut dummy_ctx();
        step_test! {
            ctx: ctx;

            code: 0x3F, length: 1, cycles: 1
            setup {
                ctx.cpu.f.c = true;
            }
            after {
                assert!(!ctx.cpu.f.c);
            }
        }
    }
}
