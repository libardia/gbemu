use crate::{
    gb::{
        GameBoy,
        hardware::processor::{Processor, instructions::R16},
    },
    wrapping_add_warn, wrapping_sub_warn,
};

pub fn add(ctx: &mut GameBoy, op: R16) -> u16 {
    let lhs = ctx.cpu.r.get_hl();
    let rhs = Processor::get_r16(ctx, op);
    let (result, overflow) = ctx.cpu.r.get_hl().overflowing_add(rhs);
    ctx.cpu.r.set_hl(result);

    ctx.cpu.f.n = false;
    ctx.cpu.f.h = lhs & 0xFFF + rhs & 0xFFF > 0xFFF;
    ctx.cpu.f.c = overflow;

    // Always 2 cycles
    2
}

pub fn inc(ctx: &mut GameBoy, target: R16) -> u16 {
    let before = Processor::get_r16(ctx, target);
    let after = wrapping_add_warn!(before, 1, "Increment caused r16 {target:?} to overflow");
    Processor::set_r16(ctx, target, after);

    // Always 2 cycles
    2
}

pub fn dec(ctx: &mut GameBoy, target: R16) -> u16 {
    let before = Processor::get_r16(ctx, target);
    let after = wrapping_sub_warn!(before, 1, "Decrement caused r16 {target:?} to underflow");
    Processor::set_r16(ctx, target, after);

    // Always 2 cycles
    2
}
