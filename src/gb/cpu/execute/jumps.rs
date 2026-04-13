use crate::{
    gb::{
        GameBoy,
        cpu::{CPU, access::Condition, debug_interrupts},
    },
    macros::hex,
};

pub fn jr_cc_e8(ctx: &mut GameBoy, cond: Condition) {
    let relative_from: u16 = ctx.cpu.pc;

    // Read always happens first (1 tick)
    let offset = CPU::next_signed(ctx) as i16;
    println!("Hex of offset: {}", hex!(offset, 4));
    let address = relative_from.wrapping_add_signed(offset);

    if CPU::test_condition(ctx, cond) {
        ctx.m_tick(); // 1 tick longer if branch
        ctx.cpu.pc = address;
    }
}

pub fn jp_hl(ctx: &mut GameBoy) {
    ctx.cpu.pc = ctx.cpu.get_hl();
}

pub fn jp_cc_a16(ctx: &mut GameBoy, cond: Condition) {
    // Read always happens first (2 ticks)
    let address = CPU::next_word(ctx);

    if CPU::test_condition(ctx, cond) {
        ctx.m_tick(); // 1 tick longer if branch
        ctx.cpu.pc = address;
    }
}

pub fn call_cc_a16(ctx: &mut GameBoy, cond: Condition) {
    // Read always happens first (2 ticks)
    let address = CPU::next_word(ctx);

    if CPU::test_condition(ctx, cond) {
        ctx.m_tick(); // One tick happens here, not sure why
        CPU::push_stack(ctx, ctx.cpu.pc); // 2 ticks
        ctx.cpu.pc = address;
    }
}

pub fn rst(ctx: &mut GameBoy, address: u16) {
    ctx.m_tick(); // One tick here
    CPU::push_stack(ctx, ctx.cpu.pc); // 2 ticks
    ctx.cpu.pc = address;
}

pub fn ret(ctx: &mut GameBoy, enable_interrupts: bool) {
    let address = CPU::pop_stack(ctx); // 2 ticks
    ctx.m_tick(); // 1 tick here?
    ctx.cpu.pc = address;

    if enable_interrupts {
        debug_interrupts!(on);
        ctx.cpu.ime = true;
    }
}

pub fn ret_cc(ctx: &mut GameBoy, cond: Condition) {
    ctx.m_tick(); // 1 tick: internal branch decision?
    if CPU::test_condition(ctx, cond) {
        let address = CPU::pop_stack(ctx); // 2 ticks
        ctx.m_tick(); // 1 tick: internal?
        ctx.cpu.pc = address;
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        gb::mmu::{MMU, region::HIGH_RAM_END},
        testutil::{INSTRUCTION_ADDRESS, dummy_ctx, jump_test, step_test},
    };

    use super::*;
    use test_log::test;

    const STACK_BEGIN: u16 = HIGH_RAM_END;
    const OFFSET: u8 = -84i8 as u8;
    const FINAL_DEST: u16 = 0xDEAD;
    const FINAL_DEST_HIGH: u8 = 0xDE;
    const FINAL_DEST_LOW: u8 = 0xAD;

    fn test_push_stack(ctx: &mut GameBoy, word: u16) {
        MMU::write(ctx, ctx.cpu.sp - 1, (word >> 8) as u8);
        MMU::write(ctx, ctx.cpu.sp - 2, (word & 0xFF) as u8);
        ctx.cpu.sp -= 2;
    }

    fn test_peek_stack(ctx: &mut GameBoy) -> u16 {
        let low = MMU::read(ctx, ctx.cpu.sp);
        let high = MMU::read(ctx, ctx.cpu.sp + 1);

        ((high as u16) << 8) | (low as u16)
    }

    #[test]
    fn jp_a16() {
        let ctx = &mut dummy_ctx();
        jump_test! {
            ctx: ctx;

            code: 0xC3, pc_after: FINAL_DEST, cycles: 4
            setup {
                MMU::write(ctx, INSTRUCTION_ADDRESS + 1, FINAL_DEST_LOW);
                MMU::write(ctx, INSTRUCTION_ADDRESS + 2, FINAL_DEST_HIGH);
            }
        }
    }

    #[test]
    fn jp_hl() {
        let ctx = &mut dummy_ctx();
        jump_test! {
            ctx: ctx;

            code: 0xE9, pc_after: FINAL_DEST, cycles: 1
            setup {
                ctx.cpu.set_hl(FINAL_DEST);
            }
        }
    }

    macro_rules! jp_cc_a16 {
        ($($code:literal $testname:ident $flag:ident $target_state:literal;)*) => {
            paste::paste! {$(
                #[test]
                fn [<$testname _taken>]() {
                    let ctx = &mut dummy_ctx();
                    jump_test! {
                        ctx: ctx;

                        code: $code, pc_after: FINAL_DEST, cycles: 4
                        setup {
                            ctx.cpu.f.$flag = $target_state;
                            MMU::write(ctx, INSTRUCTION_ADDRESS + 1, FINAL_DEST_LOW);
                            MMU::write(ctx, INSTRUCTION_ADDRESS + 2, FINAL_DEST_HIGH);
                        }
                    }
                }

                #[test]
                fn [<$testname _untaken>]() {
                    let ctx = &mut dummy_ctx();
                    step_test! {
                        ctx: ctx;

                        code: $code, length: 3, cycles: 3
                        setup {
                            ctx.cpu.f.$flag = !$target_state;
                            MMU::write(ctx, INSTRUCTION_ADDRESS + 1, FINAL_DEST_LOW);
                            MMU::write(ctx, INSTRUCTION_ADDRESS + 2, FINAL_DEST_HIGH);
                        }
                    }
                }
            )*}
        };
    }

    jp_cc_a16! {
        0xC2 jp_nz_a16 z false;
        0xCA jp_z_a16 z true;
        0xD2 jp_nc_a16 c false;
        0xDA jp_c_a16 c true;
    }

    #[test]
    fn jr_e8() {
        let ctx = &mut dummy_ctx();
        jump_test! {
            ctx: ctx;

            code: 0x18, pc_after: FINAL_DEST, cycles: 3
            setup {
                MMU::write(ctx, INSTRUCTION_ADDRESS + 1, OFFSET);
            }
        }
    }

    macro_rules! jr_cc_e8 {
        ($($code:literal $testname:ident $flag:ident $target_state:literal;)*) => {
            paste::paste! {$(
                #[test]
                fn [<$testname _taken>]() {
                    let ctx = &mut dummy_ctx();
                    jump_test! {
                        ctx: ctx;

                        code: $code, pc_after: FINAL_DEST, cycles: 3
                        setup {
                            ctx.cpu.f.$flag = $target_state;
                            MMU::write(ctx, INSTRUCTION_ADDRESS + 1, OFFSET);
                        }
                    }
                }

                #[test]
                fn [<$testname _untaken>]() {
                    let ctx = &mut dummy_ctx();
                    step_test! {
                        ctx: ctx;

                        code: $code, length: 2, cycles: 2
                        setup {
                            ctx.cpu.f.$flag = !$target_state;
                            MMU::write(ctx, INSTRUCTION_ADDRESS + 1, OFFSET);
                        }
                    }
                }
            )*}
        };
    }

    jr_cc_e8! {
        0x20 jr_nz_e8 z false;
        0x28 jr_z_e8 z true;
        0x30 jr_nc_e8 c false;
        0x38 jr_c_e8 c true;
    }

    #[test]
    fn call_a16() {
        let ctx = &mut dummy_ctx();
        jump_test! {
            ctx: ctx;

            code: 0xCD, pc_after: FINAL_DEST, cycles: 6
            setup {
                ctx.cpu.sp = STACK_BEGIN;
                MMU::write(ctx, INSTRUCTION_ADDRESS + 1, FINAL_DEST_LOW);
                MMU::write(ctx, INSTRUCTION_ADDRESS + 2, FINAL_DEST_HIGH);
            }
            after {
                assert_eq!(ctx.cpu.sp, STACK_BEGIN - 2);
                assert_eq!(test_peek_stack(ctx), INSTRUCTION_ADDRESS + 3);
            }
        }
    }

    macro_rules! call_cc_a16 {
        ($($code:literal $testname:ident $flag:ident $target_state:literal;)*) => {
            paste::paste! {$(
                #[test]
                fn [<$testname _taken>]() {
                    let ctx = &mut dummy_ctx();
                    jump_test! {
                        ctx: ctx;

                        code: $code, pc_after: FINAL_DEST, cycles: 6
                        setup {
                            ctx.cpu.sp = STACK_BEGIN;
                            ctx.cpu.f.$flag = $target_state;
                            MMU::write(ctx, INSTRUCTION_ADDRESS + 1, FINAL_DEST_LOW);
                            MMU::write(ctx, INSTRUCTION_ADDRESS + 2, FINAL_DEST_HIGH);
                        }
                        after {
                            assert_eq!(ctx.cpu.sp, STACK_BEGIN - 2);
                            assert_eq!(test_peek_stack(ctx), INSTRUCTION_ADDRESS + 3);
                        }
                    }
                }

                #[test]
                fn [<$testname _untaken>]() {
                    let ctx = &mut dummy_ctx();
                    step_test! {
                        ctx: ctx;

                        code: $code, length: 3, cycles: 3
                        setup {
                            ctx.cpu.sp = STACK_BEGIN;
                            ctx.cpu.f.$flag = !$target_state;
                            MMU::write(ctx, INSTRUCTION_ADDRESS + 1, FINAL_DEST_LOW);
                            MMU::write(ctx, INSTRUCTION_ADDRESS + 2, FINAL_DEST_HIGH);
                        }
                        after {
                            assert_eq!(ctx.cpu.sp, STACK_BEGIN);
                        }
                    }
                }
            )*}
        };
    }

    call_cc_a16! {
        0xC4 call_nz_a16 z false;
        0xCC call_z_a16 z true;
        0xD4 call_nc_a16 c false;
        0xDC call_c_a16 c true;
    }

    #[test]
    fn ret() {
        let ctx = &mut dummy_ctx();
        jump_test! {
            ctx: ctx;

            code: 0xC9, pc_after: FINAL_DEST, cycles: 4
            setup {
                ctx.cpu.sp = STACK_BEGIN;
                test_push_stack(ctx, FINAL_DEST);
                assert_eq!(ctx.cpu.sp, STACK_BEGIN - 2);
            }
            after {
                assert_eq!(ctx.cpu.sp, STACK_BEGIN);
            }
        }
    }

    #[test]
    fn reti() {
        let ctx = &mut dummy_ctx();
        jump_test! {
            ctx: ctx;

            code: 0xD9, pc_after: FINAL_DEST, cycles: 4
            setup {
                ctx.cpu.sp = STACK_BEGIN;
                ctx.cpu.ime = false;
                test_push_stack(ctx, FINAL_DEST);
                assert_eq!(ctx.cpu.sp, STACK_BEGIN - 2);
            }
            after {
                assert!(ctx.cpu.ime);
                assert_eq!(ctx.cpu.sp, STACK_BEGIN);
            }
        }
    }

    macro_rules! ret_cc {
        ($($code:literal $testname:ident $flag:ident $target_state:literal;)*) => {
            paste::paste! {$(
                #[test]
                fn [<$testname _taken>]() {
                    let ctx = &mut dummy_ctx();
                    jump_test! {
                        ctx: ctx;

                        code: $code, pc_after: FINAL_DEST, cycles: 5
                        setup {
                            ctx.cpu.sp = STACK_BEGIN;
                            ctx.cpu.f.$flag = $target_state;
                            test_push_stack(ctx, FINAL_DEST);
                            assert_eq!(ctx.cpu.sp, STACK_BEGIN - 2);
                        }
                        after {
                            assert_eq!(ctx.cpu.sp, STACK_BEGIN);
                        }
                    }
                }

                #[test]
                fn [<$testname _untaken>]() {
                    let ctx = &mut dummy_ctx();
                    step_test! {
                        ctx: ctx;

                        code: $code, length: 1, cycles: 2
                        setup {
                            ctx.cpu.sp = STACK_BEGIN;
                            ctx.cpu.f.$flag = !$target_state;
                            test_push_stack(ctx, FINAL_DEST);
                            assert_eq!(ctx.cpu.sp, STACK_BEGIN - 2);
                        }
                        after {
                            assert_eq!(ctx.cpu.sp, STACK_BEGIN - 2);
                        }
                    }
                }
            )*}
        };
    }

    ret_cc! {
        0xC0 ret_nz z false;
        0xC8 ret_z z true;
        0xD0 ret_nc c false;
        0xD8 ret_c c true;
    }

    macro_rules! rst_vXX {
        ($($code:literal $testname:ident $vec:literal;)*) => {$(
            #[test]
            fn $testname() {
                let ctx = &mut dummy_ctx();
                jump_test! {
                    ctx: ctx;

                    code: $code, pc_after: $vec, cycles: 4
                    setup {
                        ctx.cpu.sp = STACK_BEGIN;
                    }
                    after {
                        assert_eq!(ctx.cpu.sp, STACK_BEGIN - 2);
                        assert_eq!(test_peek_stack(ctx), INSTRUCTION_ADDRESS + 1);
                    }
                }
            }
        )*};
    }

    rst_vXX! {
        0xC7 rst_v00 0x00;
        0xCF rst_v08 0x08;
        0xD7 rst_v10 0x10;
        0xDF rst_v18 0x18;
        0xE7 rst_v20 0x20;
        0xEF rst_v28 0x28;
        0xF7 rst_v30 0x30;
        0xFF rst_v38 0x38;
    }
}
