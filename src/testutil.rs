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

#[macro_export]
macro_rules! step_test {
    (
        ctx: $ctx:expr;
        code: $code:literal, length: $length:literal, cycles: $cycles:literal
        $(setup $setup:block)?
        $(after $after:block)?
    ) => {{
        crate::testutil::prepare_instruction(
            $ctx,
            crate::testutil::INSTRUCTION_ADDRESS,
            $code
        );

        $($setup)?

        crate::gb::cpu::CPU::step($ctx);

        $($after)?

        assert_eq!(
            $ctx.cpu.pc,
            crate::testutil::INSTRUCTION_ADDRESS + $length
        );
        assert_eq!($ctx.debug_timer, $cycles*4);
    }};
}
