use crate::gb::{
    GameBoy,
    cpu::{CPU, Flags, access::ByteLoc},
};

pub fn and_a_r8(ctx: &mut GameBoy, operand: ByteLoc) {
    ctx.cpu.a = ctx.cpu.a & CPU::get_byte_at(ctx, operand);
    ctx.cpu.f = Flags {
        z: ctx.cpu.a == 0,
        n: false,
        h: true,
        c: false,
    }
}

pub fn or_a_r8(ctx: &mut GameBoy, operand: ByteLoc) {
    ctx.cpu.a = ctx.cpu.a | CPU::get_byte_at(ctx, operand);
    ctx.cpu.f = Flags {
        z: ctx.cpu.a == 0,
        n: false,
        h: false,
        c: false,
    }
}

pub fn xor_a_r8(ctx: &mut GameBoy, operand: ByteLoc) {
    ctx.cpu.a = ctx.cpu.a ^ CPU::get_byte_at(ctx, operand);
    ctx.cpu.f = Flags {
        z: ctx.cpu.a == 0,
        n: false,
        h: false,
        c: false,
    }
}

pub fn cpl(ctx: &mut GameBoy) {
    ctx.cpu.a = !ctx.cpu.a;
    ctx.cpu.f.n = true;
    ctx.cpu.f.h = true;
}

#[cfg(test)]
mod tests {
    use crate::gb::MMU;
    use crate::testutil::{INSTRUCTION_ADDRESS, dummy_ctx, step_test};
    use test_log::test;

    const MEM_ADD: u16 = 0xDD00;
    const BASE: u8 = 0b10110001;
    const OTHER: u8 = 0b11110000;
    const AND: u8 = BASE & OTHER;
    const OR: u8 = BASE | OTHER;
    const XOR: u8 = BASE ^ OTHER;
    const NOT: u8 = !BASE;

    macro_rules! logic_tests {
        (@inner $code:literal and mhl) => {
            #[test]
            fn and_a_mhl() {
                let ctx = &mut dummy_ctx();
                step_test! {
                    ctx: ctx;

                    code: $code, length: 1, cycles: 2
                    setup {
                        ctx.cpu.a = BASE;
                        ctx.cpu.set_hl(MEM_ADD);
                        MMU::write(ctx, MEM_ADD, OTHER);
                    }
                    after {
                        assert_eq!(ctx.cpu.a, AND);
                        assert!(!ctx.cpu.f.z);
                        assert!(!ctx.cpu.f.n);
                        assert!( ctx.cpu.f.h);
                        assert!(!ctx.cpu.f.c);
                    }
                }
            }
        };

        (@inner $code:literal and n8) => {
            #[test]
            fn and_a_n8() {
                let ctx = &mut dummy_ctx();
                step_test! {
                    ctx: ctx;

                    code: $code, length: 2, cycles: 2
                    setup {
                        ctx.cpu.a = BASE;
                        MMU::write(ctx, INSTRUCTION_ADDRESS + 1, OTHER);
                    }
                    after {
                        assert_eq!(ctx.cpu.a, AND);
                        assert!(!ctx.cpu.f.z);
                        assert!(!ctx.cpu.f.n);
                        assert!( ctx.cpu.f.h);
                        assert!(!ctx.cpu.f.c);
                    }
                }
            }
        };

        (@inner $code:literal and a) => {
            #[test]
            fn and_a_a() {
                let ctx = &mut dummy_ctx();
                step_test! {
                    ctx: ctx;

                    code: $code, length: 1, cycles: 1
                    setup {
                        ctx.cpu.a = BASE;
                    }
                    after {
                        assert_eq!(ctx.cpu.a, BASE);
                        assert!(!ctx.cpu.f.z);
                        assert!(!ctx.cpu.f.n);
                        assert!( ctx.cpu.f.h);
                        assert!(!ctx.cpu.f.c);
                    }
                }
            }
        };

        (@inner $code:literal and $reg:ident) => {
            paste::paste! {
                #[test]
                fn [<and_a_ $reg>]() {
                    let ctx = &mut dummy_ctx();
                    step_test! {
                        ctx: ctx;

                        code: $code, length: 1, cycles: 1
                        setup {
                            ctx.cpu.a = BASE;
                            ctx.cpu.$reg = OTHER;
                        }
                        after {
                            assert_eq!(ctx.cpu.a, AND);
                            assert!(!ctx.cpu.f.z);
                            assert!(!ctx.cpu.f.n);
                            assert!( ctx.cpu.f.h);
                            assert!(!ctx.cpu.f.c);
                        }
                    }
                }
            }
        };

        (@inner $code:literal or mhl) => {
            #[test]
            fn or_a_mhl() {
                let ctx = &mut dummy_ctx();
                step_test! {
                    ctx: ctx;

                    code: $code, length: 1, cycles: 2
                    setup {
                        ctx.cpu.a = BASE;
                        ctx.cpu.set_hl(MEM_ADD);
                        MMU::write(ctx, MEM_ADD, OTHER);
                    }
                    after {
                        assert_eq!(ctx.cpu.a, OR);
                        assert!(!ctx.cpu.f.z);
                        assert!(!ctx.cpu.f.n);
                        assert!(!ctx.cpu.f.h);
                        assert!(!ctx.cpu.f.c);
                    }
                }
            }
        };

        (@inner $code:literal or n8) => {
            #[test]
            fn or_a_n8() {
                let ctx = &mut dummy_ctx();
                step_test! {
                    ctx: ctx;

                    code: $code, length: 2, cycles: 2
                    setup {
                        ctx.cpu.a = BASE;
                        MMU::write(ctx, INSTRUCTION_ADDRESS + 1, OTHER);
                    }
                    after {
                        assert_eq!(ctx.cpu.a, OR);
                        assert!(!ctx.cpu.f.z);
                        assert!(!ctx.cpu.f.n);
                        assert!(!ctx.cpu.f.h);
                        assert!(!ctx.cpu.f.c);
                    }
                }
            }
        };

        (@inner $code:literal or a) => {
            #[test]
            fn or_a_a() {
                let ctx = &mut dummy_ctx();
                step_test! {
                    ctx: ctx;

                    code: $code, length: 1, cycles: 1
                    setup {
                        ctx.cpu.a = BASE;
                    }
                    after {
                        assert_eq!(ctx.cpu.a, BASE);
                        assert!(!ctx.cpu.f.z);
                        assert!(!ctx.cpu.f.n);
                        assert!(!ctx.cpu.f.h);
                        assert!(!ctx.cpu.f.c);
                    }
                }
            }
        };

        (@inner $code:literal or $reg:ident) => {
            paste::paste! {
                #[test]
                fn [<or_a_ $reg>]() {
                    let ctx = &mut dummy_ctx();
                    step_test! {
                        ctx: ctx;

                        code: $code, length: 1, cycles: 1
                        setup {
                            ctx.cpu.a = BASE;
                            ctx.cpu.$reg = OTHER;
                        }
                        after {
                            assert_eq!(ctx.cpu.a, OR);
                            assert!(!ctx.cpu.f.z);
                            assert!(!ctx.cpu.f.n);
                            assert!(!ctx.cpu.f.h);
                            assert!(!ctx.cpu.f.c);
                        }
                    }
                }
            }
        };

        (@inner $code:literal xor mhl) => {
            #[test]
            fn xor_a_mhl() {
                let ctx = &mut dummy_ctx();
                step_test! {
                    ctx: ctx;

                    code: $code, length: 1, cycles: 2
                    setup {
                        ctx.cpu.a = BASE;
                        ctx.cpu.set_hl(MEM_ADD);
                        MMU::write(ctx, MEM_ADD, OTHER);
                    }
                    after {
                        assert_eq!(ctx.cpu.a, XOR);
                        assert!(!ctx.cpu.f.z);
                        assert!(!ctx.cpu.f.n);
                        assert!(!ctx.cpu.f.h);
                        assert!(!ctx.cpu.f.c);
                    }
                }
            }
        };

        (@inner $code:literal xor n8) => {
            #[test]
            fn xor_a_n8() {
                let ctx = &mut dummy_ctx();
                step_test! {
                    ctx: ctx;

                    code: $code, length: 2, cycles: 2
                    setup {
                        ctx.cpu.a = BASE;
                        MMU::write(ctx, INSTRUCTION_ADDRESS + 1, OTHER);
                    }
                    after {
                        assert_eq!(ctx.cpu.a, XOR);
                        assert!(!ctx.cpu.f.z);
                        assert!(!ctx.cpu.f.n);
                        assert!(!ctx.cpu.f.h);
                        assert!(!ctx.cpu.f.c);
                    }
                }
            }
        };

        (@inner $code:literal xor a) => {
            #[test]
            fn xor_a_a() {
                let ctx = &mut dummy_ctx();
                step_test! {
                    ctx: ctx;

                    code: $code, length: 1, cycles: 1
                    setup {
                        ctx.cpu.a = BASE;
                    }
                    after {
                        assert_eq!(ctx.cpu.a, 0);
                        assert!( ctx.cpu.f.z);
                        assert!(!ctx.cpu.f.n);
                        assert!(!ctx.cpu.f.h);
                        assert!(!ctx.cpu.f.c);
                    }
                }
            }
        };

        (@inner $code:literal xor $reg:ident) => {
            paste::paste! {
                #[test]
                fn [<xor_a_ $reg>]() {
                    let ctx = &mut dummy_ctx();
                    step_test! {
                        ctx: ctx;

                        code: $code, length: 1, cycles: 1
                        setup {
                            ctx.cpu.a = BASE;
                            ctx.cpu.$reg = OTHER;
                        }
                        after {
                            assert_eq!(ctx.cpu.a, XOR);
                            assert!(!ctx.cpu.f.z);
                            assert!(!ctx.cpu.f.n);
                            assert!(!ctx.cpu.f.h);
                            assert!(!ctx.cpu.f.c);
                        }
                    }
                }
            }
        };

        ($(($($arg:tt)*))*) => {
            $(
                logic_tests!(@inner $($arg)*);
            )*
        };
    }

    logic_tests! {
        (0xA0 and b) (0xA1 and c) (0xA2 and d) (0xA3 and e) (0xA4 and h) (0xA5 and l) (0xA6 and mhl) (0xA7 and a) (0xE6 and n8)
        (0xB0 or  b) (0xB1 or  c) (0xB2 or  d) (0xB3 or  e) (0xB4 or  h) (0xB5 or  l) (0xB6 or  mhl) (0xB7 or  a) (0xF6 or  n8)
        (0xA8 xor b) (0xA9 xor c) (0xAA xor d) (0xAB xor e) (0xAC xor h) (0xAD xor l) (0xAE xor mhl) (0xAF xor a) (0xEE xor n8)
    }

    #[test]
    fn cpl() {
        let ctx = &mut dummy_ctx();
        step_test! {
            ctx: ctx;

            code: 0x2F, length: 1, cycles: 1
            setup {
                ctx.cpu.a = BASE;
            }
            after {
                assert_eq!(ctx.cpu.a, NOT);
                assert!(ctx.cpu.f.n);
                assert!(ctx.cpu.f.h);
            }
        }
    }
}
