use crate::{
    gb::{
        GameBoy,
        hardware::processor::{Flags, Processor, instructions::R8},
    },
    wrapping_add_warn, wrapping_sub_warn,
};

pub fn add(ctx: &mut GameBoy, op: R8, with_carry: bool) -> u16 {
    let lhs = ctx.cpu.r.a;
    let rhs = Processor::get_r8(ctx, op);
    let cv = (with_carry && ctx.cpu.f.c) as u8;
    let (result, overflow1) = lhs.overflowing_add(rhs);
    let (result, overflow2) = result.overflowing_add(cv);
    ctx.cpu.r.a = result;

    ctx.cpu.f = Flags {
        z: result == 0,
        n: false,
        h: ((lhs & 0xF) + (rhs & 0xF) + cv) > 0xF,
        c: overflow1 || overflow2,
    };

    match op {
        R8::MHL | R8::IMM(_) => 2,
        _ => 1,
    }
}

pub fn sub(ctx: &mut GameBoy, op: R8, with_carry: bool) -> u16 {
    // This is a seperate sub-function because "compare" is identical
    ctx.cpu.r.a = subtract_internal(ctx, op, with_carry);

    match op {
        R8::MHL | R8::IMM(_) => 2,
        _ => 1,
    }
}

pub fn cp(ctx: &mut GameBoy, op: R8) -> u16 {
    // Do a subtraction and set flags accordingly, but ignore the result
    subtract_internal(ctx, op, false);

    match op {
        R8::MHL | R8::IMM(_) => 2,
        _ => 1,
    }
}

pub fn inc(ctx: &mut GameBoy, target: R8) -> u16 {
    let before = Processor::get_r8(ctx, target);
    let after = wrapping_add_warn!(before, 1, "Increment caused r8 {target:?} to overflow");
    Processor::set_r8(ctx, target, after);

    ctx.cpu.f.z = after == 0;
    ctx.cpu.f.n = false;
    ctx.cpu.f.h = before & 0xF == 0xF;

    match target {
        R8::MHL => 3,
        _ => 1,
    }
}

pub fn dec(ctx: &mut GameBoy, target: R8) -> u16 {
    let before = Processor::get_r8(ctx, target);
    let after = wrapping_sub_warn!(before, 1, "Decrement caused r8 {target:?} to underflow");
    Processor::set_r8(ctx, target, after);

    ctx.cpu.f.z = after == 0;
    ctx.cpu.f.n = true;
    ctx.cpu.f.h = before & 0xF == 0;

    match target {
        R8::MHL => 3,
        _ => 1,
    }
}

fn subtract_internal(ctx: &mut GameBoy, op: R8, with_carry: bool) -> u8 {
    let lhs = ctx.cpu.r.a;
    let rhs = Processor::get_r8(ctx, op);
    let cv = (with_carry && ctx.cpu.f.c) as u8;
    let (result, overflow1) = lhs.overflowing_sub(rhs);
    let (result, overflow2) = result.overflowing_sub(cv);

    ctx.cpu.f = Flags {
        z: result == 0,
        n: true,
        h: ((lhs & 0xF) as i8 - (rhs & 0xF) as i8 - cv as i8) < 0,
        c: overflow1 || overflow2,
    };

    result
}
