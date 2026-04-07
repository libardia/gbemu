use crate::{
    gb::{
        GameBoy,
        cpu::{CPU, access::ByteLoc},
    },
    step_test,
};

pub fn add_r8(ctx: &mut GameBoy, target: ByteLoc, carry: bool) {
    let lhs = ctx.cpu.a;
    let rhs = CPU::get_byte_at(ctx, target);
    let c = (carry && ctx.cpu.f.c) as u8;
    let (result, overflow1) = lhs.overflowing_add(rhs);
    let (result, overflow2) = result.overflowing_add(c);

    ctx.cpu.a = result;
    ctx.cpu.f.z = result == 0;
    ctx.cpu.f.n = false;
    ctx.cpu.f.h = ((lhs & 0xF) + (rhs & 0xF)) + c > 0xF;
    ctx.cpu.f.c = overflow1 || overflow2;
}

pub fn sub_r8(ctx: &mut GameBoy, target: ByteLoc, carry: bool) {
    ctx.cpu.a = sub_internal(ctx, target, carry);
}

pub fn inc_r8(ctx: &mut GameBoy, target: ByteLoc) {
    let orig = CPU::get_byte_at(ctx, target);
    let result = orig.wrapping_add(1);
    CPU::set_byte_at(ctx, target, result);

    ctx.cpu.f.z = result == 0;
    ctx.cpu.f.n = false;
    ctx.cpu.f.h = orig & 0xF == 0xF
}

pub fn dec_r8(ctx: &mut GameBoy, target: ByteLoc) {
    let orig = CPU::get_byte_at(ctx, target);
    let result = orig.wrapping_sub(1);
    CPU::set_byte_at(ctx, target, result);

    ctx.cpu.f.z = result == 0;
    ctx.cpu.f.n = true;
    ctx.cpu.f.h = orig & 0xF == 0
}

pub fn cp_r8(ctx: &mut GameBoy, target: ByteLoc) {
    // Flags set the same as a subtraction, but the result is ignored
    sub_internal(ctx, target, false);
}

fn sub_internal(ctx: &mut GameBoy, target: ByteLoc, carry: bool) -> u8 {
    let lhs = ctx.cpu.a;
    let rhs = CPU::get_byte_at(ctx, target);
    let c = (carry && ctx.cpu.f.c) as u8;
    let (result, overflow1) = lhs.overflowing_sub(rhs);
    let (result, overflow2) = result.overflowing_sub(c);

    ctx.cpu.f.z = result == 0;
    ctx.cpu.f.n = false;
    ctx.cpu.f.h = (lhs & 0xF) < ((rhs & 0xF) + c);
    ctx.cpu.f.c = overflow1 || overflow2;

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! arith_tests {
        (@inner $code:literal add n8) => {
            paste::paste! {
                #[test]
                fn [<add_ $reg>]() {
                    let ctx = &mut GameBoy::new();
                    step_test! {
                        ctx: ctx;

                        code: $code, length: 1, cycles: 1
                        setup {
                        }
                        after {
                        }
                    }
                }
            }
        };

        (@inner $code:literal add *hl) => {
            paste::paste! {
                #[test]
                fn [<add_ $reg>]() {
                    let ctx = &mut GameBoy::new();
                    step_test! {
                        ctx: ctx;

                        code: $code, length: 1, cycles: 1
                        setup {
                        }
                        after {
                        }
                    }
                }
            }
        };

        (@inner $code:literal add $reg:ident) => {
            paste::paste! {
                #[test]
                fn [<add_ $reg>]() {
                    let ctx = &mut GameBoy::new();
                    step_test! {
                        ctx: ctx;

                        code: $code, length: 1, cycles: 1
                        setup {
                        }
                        after {
                        }
                    }
                }
            }
        };

        (@inner $code:literal add $reg:ident) => {
            paste::paste! {
                #[test]
                fn [<add_ $reg>]() {
                    let ctx = &mut GameBoy::new();
                    step_test! {
                        ctx: ctx;

                        code: $code, length: 1, cycles: 1
                        setup {
                        }
                        after {
                        }
                    }
                }
            }
        };

        ($(($($arg:tt)*))*) => {
            $(
                arith_tests!(@inner $($arg)*);
            )*
        };
    }

    arith_tests! {
        (0x80 add b)
    }
}
