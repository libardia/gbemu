use crate::gb::{
    GameBoy,
    hardware::processor::{Processor, instructions::R8},
};

pub fn bit(ctx: &mut GameBoy, bit: u8, target: R8) -> u16 {
    let mask = 1 << bit;
    let byte = Processor::get_r8(ctx, target);

    ctx.cpu.f.z = byte & mask == 0;
    ctx.cpu.f.n = false;
    ctx.cpu.f.h = true;

    match target {
        R8::MHL => 3,
        _ => 2,
    }
}

pub fn set(ctx: &mut GameBoy, bit: u8, target: R8) -> u16 {
    let mask = 1 << bit;
    let byte = Processor::get_r8(ctx, target);
    Processor::set_r8(ctx, target, byte | mask);

    match target {
        R8::MHL => 4,
        _ => 2,
    }
}

pub fn res(ctx: &mut GameBoy, bit: u8, target: R8) -> u16 {
    let mask = 1 << bit;
    let byte = Processor::get_r8(ctx, target);
    Processor::set_r8(ctx, target, byte & !mask);

    match target {
        R8::MHL => 4,
        _ => 2,
    }
}
