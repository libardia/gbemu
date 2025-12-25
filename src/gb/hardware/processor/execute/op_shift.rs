use crate::gb::{
    GameBoy,
    hardware::processor::{Flags, Processor, instructions::R8},
};

pub fn rl(ctx: &mut GameBoy, target: R8, through_carry: bool, fast: bool) -> u16 {
    let before = Processor::get_r8(ctx, target);
    let rotated_out = (before & 0b1000_0000) != 0;
    let mut result = before << 1;
    if through_carry {
        result |= ctx.cpu.f.c as u8;
    } else {
        result |= rotated_out as u8;
    }
    Processor::set_r8(ctx, target, result);

    ctx.cpu.f = Flags {
        z: if fast { false } else { result == 0 },
        n: false,
        h: false,
        c: rotated_out,
    };

    match target {
        _ if fast => 1,
        R8::MHL => 4,
        _ => 2,
    }
}

pub fn rr(ctx: &mut GameBoy, target: R8, through_carry: bool, fast: bool) -> u16 {
    let before = Processor::get_r8(ctx, target);
    let rotated_out = (before & 0b1) << 7;
    let mut result = before >> 1;
    if through_carry {
        result |= (ctx.cpu.f.c as u8) << 7;
    } else {
        result |= rotated_out;
    }
    Processor::set_r8(ctx, target, result);

    ctx.cpu.f = Flags {
        z: if fast { false } else { result == 0 },
        n: false,
        h: false,
        c: rotated_out != 0,
    };

    match target {
        _ if fast => 1,
        R8::MHL => 4,
        _ => 2,
    }
}

pub fn sl(ctx: &mut GameBoy, target: R8) -> u16 {
    let before = Processor::get_r8(ctx, target);
    let shifted_out = (before & 0b1000_0000) != 0;
    let result = before << 1;
    Processor::set_r8(ctx, target, result);

    ctx.cpu.f = Flags {
        z: result == 0,
        n: false,
        h: false,
        c: shifted_out,
    };

    match target {
        R8::MHL => 4,
        _ => 2,
    }
}

pub fn sr(ctx: &mut GameBoy, target: R8, arith: bool) -> u16 {
    let before = Processor::get_r8(ctx, target);
    let shifted_out = (before & 0b1) != 0;
    let leftmost = before & 0b1000_0000;
    let mut result = before >> 1;
    if arith {
        result |= leftmost;
    }
    Processor::set_r8(ctx, target, result);

    ctx.cpu.f = Flags {
        z: result == 0,
        n: false,
        h: false,
        c: shifted_out,
    };

    match target {
        R8::MHL => 4,
        _ => 2,
    }
}

pub fn swap(ctx: &mut GameBoy, target: R8) -> u16 {
    let before = Processor::get_r8(ctx, target);
    let upper = before & 0xF0;
    let lower = before & 0x0F;
    let result = (lower << 4) | (upper >> 4);
    Processor::set_r8(ctx, target, result);

    ctx.cpu.f = Flags {
        z: result == 0,
        n: false,
        h: false,
        c: false,
    };

    match target {
        R8::MHL => 4,
        _ => 2,
    }
}
