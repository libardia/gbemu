use crate::gb::{
    GameBoy,
    cpu::{CPU, access::ByteLoc},
};

pub fn add_r8(ctx: &mut GameBoy, reg: ByteLoc, carry: bool) {
    let lhs = ctx.cpu.a;
    let rhs = CPU::get_location(ctx, reg);
    let c = (carry && ctx.cpu.f.c) as u8;
    let (result, overflow1) = lhs.overflowing_add(rhs);
    let (result, overflow2) = result.overflowing_add(c);

    ctx.cpu.a = result;
    ctx.cpu.f.z = result == 0;
    ctx.cpu.f.n = false;
    ctx.cpu.f.h = (lhs & 0xF) + (rhs & 0xF) + c > 0xF;
    ctx.cpu.f.c = overflow1 || overflow2;
}
