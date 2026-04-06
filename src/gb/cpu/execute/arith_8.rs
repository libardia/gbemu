use crate::gb::GameBoy;

#[inline(always)]
fn do_add(ctx: &mut GameBoy, value: u8) {
    let lhs = ctx.cpu.a;
    let rhs = value;
    let (result, overflow) = lhs.overflowing_add(rhs);

    ctx.cpu.a = result;
    ctx.cpu.f.z = result == 0;
    ctx.cpu.f.n = false;
    ctx.cpu.f.h = (lhs & 0xF) + (rhs & 0xF) > 0xF;
    ctx.cpu.f.c = overflow;
}

#[inline(always)]
fn do_add_carry(ctx: &mut GameBoy, value: u8) {
    let lhs = ctx.cpu.a;
    let rhs = value;
    let c = ctx.cpu.f.c as u8;
    let (result, overflow1) = lhs.overflowing_add(rhs);
    let (result, overflow2) = result.overflowing_add(c);

    ctx.cpu.a = result;
    ctx.cpu.f.z = result == 0;
    ctx.cpu.f.n = false;
    ctx.cpu.f.h = (lhs & 0xF) + (rhs & 0xF) + c > 0xF;
    ctx.cpu.f.c = overflow1 || overflow2;
}
