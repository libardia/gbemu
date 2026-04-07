use crate::gb::{
    GameBoy,
    cpu::{CPU, access::ByteLoc},
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
    ctx.cpu.f.n = true;
    ctx.cpu.f.h = (lhs & 0xF) < ((rhs & 0xF) + c);
    ctx.cpu.f.c = overflow1 || overflow2;

    result
}

#[cfg(test)]
mod tests {
    use crate::{
        gb::mmu::MMU,
        testutil::{INSTRUCTION_ADDRESS, step_test},
    };

    use super::*;

    const MEM_ADD: u16 = 0xDD00;

    const A: u8 = 0x15;
    const B: u8 = 0x23;

    const AI: u8 = 0x16;
    const AD: u8 = 0x14;

    macro_rules! arith_tests {
        (@inner $code:literal add n8) => {
            #[test]
            fn add_n8() {
                let ctx = &mut GameBoy::new();
                step_test! {
                    ctx: ctx;

                    code: $code, length: 2, cycles: 2
                    setup {
                        ctx.cpu.a = A;
                        MMU::write(ctx, INSTRUCTION_ADDRESS + 1, B);
                    }
                    after {
                        assert_eq!(ctx.cpu.a, A+B);
                        assert!(!ctx.cpu.f.z);
                        assert!(!ctx.cpu.f.n);
                        assert!(!ctx.cpu.f.h);
                        assert!(!ctx.cpu.f.c);
                    }
                }
            }
        };

        (@inner $code:literal add *hl) => {
            #[test]
            fn add_mhl() {
                let ctx = &mut GameBoy::new();
                step_test! {
                    ctx: ctx;

                    code: $code, length: 1, cycles: 2
                    setup {
                        ctx.cpu.a = A;
                        ctx.cpu.set_hl(MEM_ADD);
                        MMU::write(ctx, MEM_ADD, B)
                    }
                    after {
                        assert_eq!(ctx.cpu.a, A+B);
                        assert!(!ctx.cpu.f.z);
                        assert!(!ctx.cpu.f.n);
                        assert!(!ctx.cpu.f.h);
                        assert!(!ctx.cpu.f.c);
                    }
                }
            }
        };

        (@inner $code:literal add a) => {
            #[test]
            fn add_a() {
                let ctx = &mut GameBoy::new();
                step_test! {
                    ctx: ctx;

                    code: $code, length: 1, cycles: 1
                    setup {
                        ctx.cpu.a = A;
                    }
                    after {
                        assert_eq!(ctx.cpu.a, A+A);
                        assert!(!ctx.cpu.f.z);
                        assert!(!ctx.cpu.f.n);
                        assert!(!ctx.cpu.f.h);
                        assert!(!ctx.cpu.f.c);
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
                            ctx.cpu.a = A;
                            ctx.cpu.$reg = B;
                        }
                        after {
                            assert_eq!(ctx.cpu.a, A+B);
                            assert!(!ctx.cpu.f.z);
                            assert!(!ctx.cpu.f.n);
                            assert!(!ctx.cpu.f.h);
                            assert!(!ctx.cpu.f.c);
                        }
                    }
                }
            }
        };

        (@inner $code:literal adc n8) => {
            #[test]
            fn adc_n8() {
                let ctx = &mut GameBoy::new();
                step_test! {
                    ctx: ctx;

                    code: $code, length: 2, cycles: 2
                    setup {
                        ctx.cpu.a = A;
                        ctx.cpu.f.c = true;
                        MMU::write(ctx, INSTRUCTION_ADDRESS + 1, B);
                    }
                    after {
                        assert_eq!(ctx.cpu.a, A+B+1);
                        assert!(!ctx.cpu.f.z);
                        assert!(!ctx.cpu.f.n);
                        assert!(!ctx.cpu.f.h);
                        assert!(!ctx.cpu.f.c);
                    }
                }
            }
        };

        (@inner $code:literal adc *hl) => {
            #[test]
            fn adc_mhl() {
                let ctx = &mut GameBoy::new();
                step_test! {
                    ctx: ctx;

                    code: $code, length: 1, cycles: 2
                    setup {
                        ctx.cpu.a = A;
                        ctx.cpu.f.c = true;
                        ctx.cpu.set_hl(MEM_ADD);
                        MMU::write(ctx, MEM_ADD, B)
                    }
                    after {
                        assert_eq!(ctx.cpu.a, A+B+1);
                        assert!(!ctx.cpu.f.z);
                        assert!(!ctx.cpu.f.n);
                        assert!(!ctx.cpu.f.h);
                        assert!(!ctx.cpu.f.c);
                    }
                }
            }
        };

        (@inner $code:literal adc a) => {
            #[test]
            fn adc_a() {
                let ctx = &mut GameBoy::new();
                step_test! {
                    ctx: ctx;

                    code: $code, length: 1, cycles: 1
                    setup {
                        ctx.cpu.a = A;
                        ctx.cpu.f.c = true;
                    }
                    after {
                        assert_eq!(ctx.cpu.a, A+A+1);
                        assert!(!ctx.cpu.f.z);
                        assert!(!ctx.cpu.f.n);
                        assert!(!ctx.cpu.f.h);
                        assert!(!ctx.cpu.f.c);
                    }
                }
            }
        };

        (@inner $code:literal adc $reg:ident) => {
            paste::paste! {
                #[test]
                fn [<adc_ $reg>]() {
                    let ctx = &mut GameBoy::new();
                    step_test! {
                        ctx: ctx;

                        code: $code, length: 1, cycles: 1
                        setup {
                            ctx.cpu.a = A;
                            ctx.cpu.f.c = true;
                            ctx.cpu.$reg = B;
                        }
                        after {
                            assert_eq!(ctx.cpu.a, A+B+1);
                            assert!(!ctx.cpu.f.z);
                            assert!(!ctx.cpu.f.n);
                            assert!(!ctx.cpu.f.h);
                            assert!(!ctx.cpu.f.c);
                        }
                    }
                }
            }
        };

        (@inner $code:literal sub n8) => {
            #[test]
            fn sub_n8() {
                let ctx = &mut GameBoy::new();
                step_test! {
                    ctx: ctx;

                    code: $code, length: 2, cycles: 2
                    setup {
                        ctx.cpu.a = A+B;
                        MMU::write(ctx, INSTRUCTION_ADDRESS + 1, B);
                    }
                    after {
                        assert_eq!(ctx.cpu.a, A);
                        assert!(!ctx.cpu.f.z);
                        assert!( ctx.cpu.f.n);
                        assert!(!ctx.cpu.f.h);
                        assert!(!ctx.cpu.f.c);
                    }
                }
            }
        };

        (@inner $code:literal sub *hl) => {
            #[test]
            fn sub_mhl() {
                let ctx = &mut GameBoy::new();
                step_test! {
                    ctx: ctx;

                    code: $code, length: 1, cycles: 2
                    setup {
                        ctx.cpu.a = A+B;
                        ctx.cpu.set_hl(MEM_ADD);
                        MMU::write(ctx, MEM_ADD, B)
                    }
                    after {
                        assert_eq!(ctx.cpu.a, A);
                        assert!(!ctx.cpu.f.z);
                        assert!( ctx.cpu.f.n);
                        assert!(!ctx.cpu.f.h);
                        assert!(!ctx.cpu.f.c);
                    }
                }
            }
        };

        (@inner $code:literal sub a) => {
            #[test]
            fn sub_a() {
                let ctx = &mut GameBoy::new();
                step_test! {
                    ctx: ctx;

                    code: $code, length: 1, cycles: 1
                    setup {
                        ctx.cpu.a = A;
                    }
                    after {
                        assert_eq!(ctx.cpu.a, 0);
                        assert!( ctx.cpu.f.z);
                        assert!( ctx.cpu.f.n);
                        assert!(!ctx.cpu.f.h);
                        assert!(!ctx.cpu.f.c);
                    }
                }
            }
        };

        (@inner $code:literal sub $reg:ident) => {
            paste::paste! {
                #[test]
                fn [<sub_ $reg>]() {
                    let ctx = &mut GameBoy::new();
                    step_test! {
                        ctx: ctx;

                        code: $code, length: 1, cycles: 1
                        setup {
                            ctx.cpu.a = A+B;
                            ctx.cpu.$reg = B;
                        }
                        after {
                            assert_eq!(ctx.cpu.a, A);
                            assert!(!ctx.cpu.f.z);
                            assert!( ctx.cpu.f.n);
                            assert!(!ctx.cpu.f.h);
                            assert!(!ctx.cpu.f.c);
                        }
                    }
                }
            }
        };

        (@inner $code:literal sbc n8) => {
            #[test]
            fn sbc_n8() {
                let ctx = &mut GameBoy::new();
                step_test! {
                    ctx: ctx;

                    code: $code, length: 2, cycles: 2
                    setup {
                        ctx.cpu.a = A+B;
                        ctx.cpu.f.c = true;
                        MMU::write(ctx, INSTRUCTION_ADDRESS + 1, B);
                    }
                    after {
                        assert_eq!(ctx.cpu.a, A-1);
                        assert!(!ctx.cpu.f.z);
                        assert!( ctx.cpu.f.n);
                        assert!(!ctx.cpu.f.h);
                        assert!(!ctx.cpu.f.c);
                    }
                }
            }
        };

        (@inner $code:literal sbc *hl) => {
            #[test]
            fn sbc_mhl() {
                let ctx = &mut GameBoy::new();
                step_test! {
                    ctx: ctx;

                    code: $code, length: 1, cycles: 2
                    setup {
                        ctx.cpu.a = A+B;
                        ctx.cpu.f.c = true;
                        ctx.cpu.set_hl(MEM_ADD);
                        MMU::write(ctx, MEM_ADD, B)
                    }
                    after {
                        assert_eq!(ctx.cpu.a, A-1);
                        assert!(!ctx.cpu.f.z);
                        assert!( ctx.cpu.f.n);
                        assert!(!ctx.cpu.f.h);
                        assert!(!ctx.cpu.f.c);
                    }
                }
            }
        };

        (@inner $code:literal sbc a) => {
            #[test]
            fn sbc_a() {
                let ctx = &mut GameBoy::new();
                step_test! {
                    ctx: ctx;

                    code: $code, length: 1, cycles: 1
                    setup {
                        ctx.cpu.a = A;
                        ctx.cpu.f.c = true;
                    }
                    after {
                        assert_eq!(ctx.cpu.a, 0xFF);
                        assert!(!ctx.cpu.f.z);
                        assert!( ctx.cpu.f.n);
                        assert!( ctx.cpu.f.h);
                        assert!( ctx.cpu.f.c);
                    }
                }
            }
        };

        (@inner $code:literal sbc $reg:ident) => {
            paste::paste! {
                #[test]
                fn [<sbc_ $reg>]() {
                    let ctx = &mut GameBoy::new();
                    step_test! {
                        ctx: ctx;

                        code: $code, length: 1, cycles: 1
                        setup {
                            ctx.cpu.a = A+B;
                            ctx.cpu.f.c = true;
                            ctx.cpu.$reg = B;
                        }
                        after {
                            assert_eq!(ctx.cpu.a, A-1);
                            assert!(!ctx.cpu.f.z);
                            assert!( ctx.cpu.f.n);
                            assert!(!ctx.cpu.f.h);
                            assert!(!ctx.cpu.f.c);
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
        (0x80 add b) (0x81 add c) (0x82 add d) (0x83 add e) (0x84 add h) (0x85 add l) (0x86 add *hl) (0x87 add a) (0xC6 add n8)
        (0x88 adc b) (0x89 adc c) (0x8A adc d) (0x8B adc e) (0x8C adc h) (0x8D adc l) (0x8E adc *hl) (0x8F adc a) (0xCE adc n8)
        (0x90 sub b) (0x91 sub c) (0x92 sub d) (0x93 sub e) (0x94 sub h) (0x95 sub l) (0x96 sub *hl) (0x97 sub a) (0xD6 sub n8)
        (0x98 sbc b) (0x99 sbc c) (0x9A sbc d) (0x9B sbc e) (0x9C sbc h) (0x9D sbc l) (0x9E sbc *hl) (0x9F sbc a) (0xDE sbc n8)
    }
}
