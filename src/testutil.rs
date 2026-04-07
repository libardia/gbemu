use crate::gb::{GameBoy, cpu::CPU, mmu::MMU};

pub const INSTRUCTION_ADDRESS: u16 = 0xDF00;

pub fn prepare_instruction(ctx: &mut GameBoy, address: u16, byte: u8) {
    MMU::write(ctx, address, byte);
    ctx.cpu.pc = address;
}

pub fn registers_equal(cpu_a: &CPU, cpu_b: &CPU) -> bool {
    cpu_a.b == cpu_b.b
        && cpu_a.c == cpu_b.c
        && cpu_a.d == cpu_b.d
        && cpu_a.e == cpu_b.e
        && cpu_a.h == cpu_b.h
        && cpu_a.l == cpu_b.l
        && cpu_a.a == cpu_b.a
        && cpu_a.f == cpu_b.f
}

macro_rules! step_test {
    (
        ctx: $ctx:expr;
        code: $code:literal, length: $length:literal, cycles: $cycles:literal
        $(setup $setup:block)?
        $(after $after:block)?
    ) => {{
        step_test! { @inner1 $ctx, [$code] $($setup)? }
        crate::gb::cpu::CPU::step($ctx);
        step_test! { @inner2 $ctx, $length, $cycles $($after)? }
    }};

    (
        ctx: $ctx:expr;
        code: $code:literal $code_ex:literal, length: $length:literal, cycles: $cycles:literal
        $(setup $setup:block)?
        $(after $after:block)?
    ) => {{
        step_test! { @inner1 $ctx, [$code $code_ex] $($setup)? }
        crate::gb::cpu::CPU::step($ctx);
        crate::gb::cpu::CPU::step($ctx);
        step_test! { @inner2 $ctx, $length, $cycles $($after)? }
    }};

    (@inner1 $ctx:expr, [$code:literal $($code_ex:literal)?] $($setup:block)?) => {
        crate::testutil::prepare_instruction(
            $ctx,
            crate::testutil::INSTRUCTION_ADDRESS,
            $code
        );
        $(
            crate::gb::mmu::MMU::write(
                $ctx,
                crate::testutil::INSTRUCTION_ADDRESS + 1,
                $code_ex
            );
        )?

        $($setup)?
    };

    (@inner2 $ctx:expr, $length:literal, $cycles:literal $($after:block)?) => {
        $($after)?

        assert_eq!(
            $ctx.cpu.pc,
            crate::testutil::INSTRUCTION_ADDRESS + $length
        );
        assert_eq!($ctx.debug_timer, $cycles*4);
    };
}
pub(crate) use step_test;
