use crate::gb::{
    GameBoy,
    hardware::processor::{
        Processor,
        instructions::{Mem, R8, R16},
    },
};

pub fn r8_r8(ctx: &mut GameBoy, dest: R8, src: R8) -> u16 {
    let value = Processor::get_r8(ctx, src);
    Processor::set_r8(ctx, dest, value);

    match (dest, src) {
        (R8::MHL | R8::IMM(_), R8::MHL | R8::IMM(_)) => 3,
        (R8::MHL | R8::IMM(_), _) | (_, R8::MHL | R8::IMM(_)) => 2,
        _ => 1,
    }
}

pub fn r8_mem(ctx: &mut GameBoy, dest: R8, src: Mem) -> u16 {
    let value = Processor::get_mem(ctx, src);
    Processor::set_r8(ctx, dest, value);

    match src {
        Mem::IMM(_) => 4,
        _ => 2,
    }
}

pub fn mem_r8(ctx: &mut GameBoy, dest: Mem, src: R8) -> u16 {
    let value = Processor::get_r8(ctx, src);
    Processor::set_mem(ctx, dest, value);

    match dest {
        Mem::IMM(_) => 4,
        _ => 2,
    }
}

pub fn r16_r16(ctx: &mut GameBoy, dest: R16, src: R16) -> u16 {
    let value = Processor::get_r16(ctx, src);
    Processor::set_r16(ctx, dest, value);

    // Always 3 cycles
    3
}

pub fn high_a_mem(ctx: &mut GameBoy, src: Mem) -> u16 {
    ctx.cpu.r.a = Processor::get_mem(ctx, src);

    match src {
        Mem::HIGH_IMM(_) => 3,
        _ => 2,
    }
}

pub fn high_mem_a(ctx: &mut GameBoy, dest: Mem) -> u16 {
    Processor::set_mem(ctx, dest, ctx.cpu.r.a);

    match dest {
        Mem::HIGH_IMM(_) => 3,
        _ => 2,
    }
}
