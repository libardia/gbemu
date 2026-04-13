use crate::gb::{
    GameBoy,
    cpu::{CPU, Flags, access::ByteLoc},
};

pub fn add_a_r8(ctx: &mut GameBoy, target: ByteLoc) {
    ctx.cpu.a = add_internal(ctx, target, false);
}

pub fn adc_a_r8(ctx: &mut GameBoy, target: ByteLoc) {
    ctx.cpu.a = add_internal(ctx, target, true);
}

pub fn sub_a_r8(ctx: &mut GameBoy, target: ByteLoc) {
    ctx.cpu.a = sub_internal(ctx, target, false);
}

pub fn sbc_a_r8(ctx: &mut GameBoy, target: ByteLoc) {
    ctx.cpu.a = sub_internal(ctx, target, true);
}

pub fn inc_r8(ctx: &mut GameBoy, target: ByteLoc) {
    let orig = CPU::get_byte_at(ctx, target);
    let result = orig.wrapping_add(1);
    CPU::set_byte_at(ctx, target, result);

    ctx.cpu.f = Flags {
        z: result == 0,
        n: false,
        h: orig & 0xF == 0xF,
        c: ctx.cpu.f.c, // Unchanged
    };
}

pub fn dec_r8(ctx: &mut GameBoy, target: ByteLoc) {
    let orig = CPU::get_byte_at(ctx, target);
    let result = orig.wrapping_sub(1);
    CPU::set_byte_at(ctx, target, result);

    ctx.cpu.f = Flags {
        z: result == 0,
        n: true,
        h: orig & 0xF == 0,
        c: ctx.cpu.f.c, // Unchanged
    };
}

pub fn cp_a_r8(ctx: &mut GameBoy, target: ByteLoc) {
    // Flags set the same as a subtraction, but the result is ignored
    sub_internal(ctx, target, false);
}

fn add_internal(ctx: &mut GameBoy, target: ByteLoc, carry: bool) -> u8 {
    let lhs = ctx.cpu.a;
    let rhs = CPU::get_byte_at(ctx, target);
    let c = (carry && ctx.cpu.f.c) as u8;
    let (result, overflow1) = lhs.overflowing_add(rhs);
    let (result, overflow2) = result.overflowing_add(c);

    ctx.cpu.f = Flags {
        z: result == 0,
        n: false,
        h: ((lhs & 0xF) + (rhs & 0xF) + c) > 0xF,
        c: overflow1 || overflow2,
    };

    result
}

fn sub_internal(ctx: &mut GameBoy, target: ByteLoc, carry: bool) -> u8 {
    let lhs = ctx.cpu.a;
    let rhs = CPU::get_byte_at(ctx, target);
    let c = (carry && ctx.cpu.f.c) as u8;
    let (result, overflow1) = lhs.overflowing_sub(rhs);
    let (result, overflow2) = result.overflowing_sub(c);

    ctx.cpu.f = Flags {
        z: result == 0,
        n: true,
        h: (lhs & 0xF) < ((rhs & 0xF) + c),
        c: overflow1 || overflow2,
    };

    result
}

#[cfg(test)]
mod tests {
    use test_log::test;

    use crate::{
        gb::mmu::MMU,
        testutil::{INSTRUCTION_ADDRESS, dummy_ctx, step_test},
    };

    const MEM_ADD: u16 = 0xDD00;

    const A: u8 = 0x15;
    const B: u8 = 0x23;

    const AI: u8 = 0x16;
    const AD: u8 = 0x14;

    macro_rules! arith_tests {
        (@inner $code:literal add n8) => {
            #[test]
            fn add_n8() {
                let ctx = &mut dummy_ctx();
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
                let ctx = &mut dummy_ctx();
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
                let ctx = &mut dummy_ctx();
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
                    let ctx = &mut dummy_ctx();
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
                let ctx = &mut dummy_ctx();
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
                let ctx = &mut dummy_ctx();
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
                let ctx = &mut dummy_ctx();
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
                    let ctx = &mut dummy_ctx();
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
                let ctx = &mut dummy_ctx();
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
                let ctx = &mut dummy_ctx();
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
                let ctx = &mut dummy_ctx();
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
                    let ctx = &mut dummy_ctx();
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
                let ctx = &mut dummy_ctx();
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
                let ctx = &mut dummy_ctx();
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
                let ctx = &mut dummy_ctx();
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
                    let ctx = &mut dummy_ctx();
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

        (@inner $code:literal cp n8) => {
            #[test]
            fn cp_eq_n8() {
                let ctx = &mut dummy_ctx();
                step_test! {
                    ctx: ctx;

                    code: $code, length: 2, cycles: 2
                    setup {
                        ctx.cpu.a = A;
                        MMU::write(ctx, INSTRUCTION_ADDRESS + 1, A);
                    }
                    after {
                        assert_eq!(ctx.cpu.a, A);
                        assert!( ctx.cpu.f.z);
                        assert!( ctx.cpu.f.n);
                        assert!(!ctx.cpu.f.h);
                        assert!(!ctx.cpu.f.c);
                    }
                }
            }

            #[test]
            fn cp_neq_n8() {
                let ctx = &mut dummy_ctx();
                step_test! {
                    ctx: ctx;

                    code: $code, length: 2, cycles: 2
                    setup {
                        ctx.cpu.a = A;
                        MMU::write(ctx, INSTRUCTION_ADDRESS + 1, B);
                    }
                    after {
                        assert_eq!(ctx.cpu.a, A);
                        assert!(!ctx.cpu.f.z);
                        assert!( ctx.cpu.f.n);
                        assert!(!ctx.cpu.f.h);
                        assert!( ctx.cpu.f.c);
                    }
                }
            }
        };

        (@inner $code:literal cp *hl) => {
            #[test]
            fn cp_eq_mhl() {
                let ctx = &mut dummy_ctx();
                step_test! {
                    ctx: ctx;

                    code: $code, length: 1, cycles: 2
                    setup {
                        ctx.cpu.a = A;
                        ctx.cpu.set_hl(MEM_ADD);
                        MMU::write(ctx, MEM_ADD, A);
                    }
                    after {
                        assert_eq!(ctx.cpu.a, A);
                        assert!( ctx.cpu.f.z);
                        assert!( ctx.cpu.f.n);
                        assert!(!ctx.cpu.f.h);
                        assert!(!ctx.cpu.f.c);
                    }
                }
            }

            #[test]
            fn cp_neq_mhl() {
                let ctx = &mut dummy_ctx();
                step_test! {
                    ctx: ctx;

                    code: $code, length: 1, cycles: 2
                    setup {
                        ctx.cpu.a = A;
                        ctx.cpu.set_hl(MEM_ADD);
                        MMU::write(ctx, MEM_ADD, B);
                    }
                    after {
                        assert_eq!(ctx.cpu.a, A);
                        assert!(!ctx.cpu.f.z);
                        assert!( ctx.cpu.f.n);
                        assert!(!ctx.cpu.f.h);
                        assert!( ctx.cpu.f.c);
                    }
                }
            }
        };

        (@inner $code:literal cp a) => {
            #[test]
            fn cp_a() {
                let ctx = &mut dummy_ctx();
                step_test! {
                    ctx: ctx;

                    code: $code, length: 1, cycles: 1
                    setup {
                        ctx.cpu.a = A;
                    }
                    after {
                        assert_eq!(ctx.cpu.a, A);
                        assert!( ctx.cpu.f.z);
                        assert!( ctx.cpu.f.n);
                        assert!(!ctx.cpu.f.h);
                        assert!(!ctx.cpu.f.c);
                    }
                }
            }
        };

        (@inner $code:literal cp $reg:ident) => {
            paste::paste! {
                #[test]
                fn [<cp_eq_ $reg>]() {
                    let ctx = &mut dummy_ctx();
                    step_test! {
                        ctx: ctx;

                        code: $code, length: 1, cycles: 1
                        setup {
                            ctx.cpu.a = A;
                            ctx.cpu.$reg = A;
                        }
                        after {
                            assert_eq!(ctx.cpu.a, A);
                            assert!( ctx.cpu.f.z);
                            assert!( ctx.cpu.f.n);
                            assert!(!ctx.cpu.f.h);
                            assert!(!ctx.cpu.f.c);
                        }
                    }
                }

                #[test]
                fn [<cp_neq_ $reg>]() {
                    let ctx = &mut dummy_ctx();
                    step_test! {
                        ctx: ctx;

                        code: $code, length: 1, cycles: 1
                        setup {
                            ctx.cpu.a = A;
                            ctx.cpu.$reg = B;
                        }
                        after {
                            assert_eq!(ctx.cpu.a, A);
                            assert!(!ctx.cpu.f.z);
                            assert!( ctx.cpu.f.n);
                            assert!(!ctx.cpu.f.h);
                            assert!( ctx.cpu.f.c);
                        }
                    }
                }
            }
        };

        (@inner $code:literal inc *hl) => {
            #[test]
            fn inc_mhl() {
                let ctx = &mut dummy_ctx();
                step_test! {
                    ctx: ctx;

                    code: $code, length: 1, cycles: 3
                    setup {
                        MMU::write(ctx, MEM_ADD, A);
                        ctx.cpu.set_hl(MEM_ADD);
                    }
                    after {
                        assert_eq!(MMU::read(ctx, MEM_ADD), AI);
                        assert!(!ctx.cpu.f.z);
                        assert!(!ctx.cpu.f.n);
                        assert!(!ctx.cpu.f.h);
                    }
                }
            }
        };

        (@inner $code:literal inc $reg:ident) => {
            paste::paste! {
                #[test]
                fn [<inc_ $reg>]() {
                    let ctx = &mut dummy_ctx();
                    step_test! {
                        ctx: ctx;

                        code: $code, length: 1, cycles: 1
                        setup {
                            ctx.cpu.$reg = A;
                        }
                        after {
                            assert_eq!(ctx.cpu.$reg, AI);
                            assert!(!ctx.cpu.f.z);
                            assert!(!ctx.cpu.f.n);
                            assert!(!ctx.cpu.f.h);
                        }
                    }
                }
            }
        };

        (@inner $code:literal dec *hl) => {
            #[test]
            fn dec_mhl() {
                let ctx = &mut dummy_ctx();
                step_test! {
                    ctx: ctx;

                    code: $code, length: 1, cycles: 3
                    setup {
                        MMU::write(ctx, MEM_ADD, A);
                        ctx.cpu.set_hl(MEM_ADD);
                    }
                    after {
                        assert_eq!(MMU::read(ctx, MEM_ADD), AD);
                        assert!(!ctx.cpu.f.z);
                        assert!( ctx.cpu.f.n);
                        assert!(!ctx.cpu.f.h);
                    }
                }
            }
        };

        (@inner $code:literal dec $reg:ident) => {
            paste::paste! {
                #[test]
                fn [<dec_ $reg>]() {
                    let ctx = &mut dummy_ctx();
                    step_test! {
                        ctx: ctx;

                        code: $code, length: 1, cycles: 1
                        setup {
                            ctx.cpu.$reg = A;
                        }
                        after {
                            assert_eq!(ctx.cpu.$reg, AD);
                            assert!(!ctx.cpu.f.z);
                            assert!( ctx.cpu.f.n);
                            assert!(!ctx.cpu.f.h);
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
        (0xB8 cp  b) (0xB9 cp  c) (0xBA cp  d) (0xBB cp  e) (0xBC cp  h) (0xBD cp  l) (0xBE cp  *hl) (0xBF cp  a) (0xFE cp  n8)

        (0x04 inc b) (0x0C inc c) (0x14 inc d) (0x1C inc e) (0x24 inc h) (0x2C inc l) (0x34 inc *hl) (0x3C inc a)
        (0x05 dec b) (0x0D dec c) (0x15 dec d) (0x1D dec e) (0x25 dec h) (0x2D dec l) (0x35 dec *hl) (0x3D dec a)
    }
}
