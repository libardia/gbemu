use crate::gb::{
    GameBoy,
    hardware::processor::{Flags, Processor, instructions::R8},
};

pub fn and(ctx: &mut GameBoy, op: R8) -> u16 {
    ctx.cpu.r.a &= Processor::get_r8(ctx, op);
    ctx.cpu.f = Flags {
        z: ctx.cpu.r.a == 0,
        n: false,
        h: true,
        c: false,
    };

    match op {
        R8::MHL | R8::IMM(_) => 2,
        _ => 1,
    }
}

pub fn or(ctx: &mut GameBoy, op: R8) -> u16 {
    ctx.cpu.r.a |= Processor::get_r8(ctx, op);
    ctx.cpu.f = Flags {
        z: ctx.cpu.r.a == 0,
        n: false,
        h: false,
        c: false,
    };

    match op {
        R8::MHL | R8::IMM(_) => 2,
        _ => 1,
    }
}

pub fn xor(ctx: &mut GameBoy, op: R8) -> u16 {
    ctx.cpu.r.a ^= Processor::get_r8(ctx, op);
    ctx.cpu.f = Flags {
        z: ctx.cpu.r.a == 0,
        n: false,
        h: false,
        c: false,
    };

    match op {
        R8::MHL | R8::IMM(_) => 2,
        _ => 1,
    }
}

pub fn cpl(ctx: &mut GameBoy) -> u16 {
    ctx.cpu.r.a = !ctx.cpu.r.a;

    ctx.cpu.f.n = true;
    ctx.cpu.f.h = true;

    // Always 1 cycle
    1
}
