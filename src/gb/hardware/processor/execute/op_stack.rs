use crate::{
    gb::{
        GameBoy,
        hardware::{
            memory::Memory,
            processor::{Flags, Processor, instructions::R16},
        },
    },
    wrapping_add_warn,
};

pub fn offset_sp(ctx: &mut GameBoy, off: i8) -> u16 {
    // The flags here are extremely confusing. Here's how they actually work (I hope, this was
    // from someone on r/EmuDev):
    // https://www.reddit.com/r/EmuDev/comments/y51i1c/game_boy_dealing_with_carry_flags_when_handling/
    // In the real hardware, this operation is done by first doing an unsigned 8 bit addition
    // between LOW(SP) and off, and the carry and half carry flags are set from that, while Z and N
    // are always reset. Then it translates that to a signed addition to SP.
    weird_flags(ctx, off);

    // Okay, now do the real thing lol
    ctx.cpu.sp = ctx.cpu.sp.wrapping_add_signed(off as i16);
    4
}

pub fn offset_sp_to_hl(ctx: &mut GameBoy, off: i8) -> u16 {
    // This instruction wasn't mentioned in the same post as 'ADD SP e8', but by context it's
    // clear that the flags are handled the same way.
    weird_flags(ctx, off);

    // Now do the real thing
    ctx.cpu.r.set_hl(ctx.cpu.sp.wrapping_add_signed(off as i16));
    3
}

pub fn save_sp(ctx: &mut GameBoy, address: u16) -> u16 {
    Memory::write(ctx, address, (ctx.cpu.sp & 0xFF) as u8);
    Memory::write(
        ctx,
        wrapping_add_warn!(
            address,
            1,
            "Writing second byte of SP caused memory address overflow"
        ),
        (ctx.cpu.sp >> 8) as u8,
    );

    // Always takes 5 cycles
    5
}

pub fn push(ctx: &mut GameBoy, target: R16) -> u16 {
    Processor::push_stack(ctx, Processor::get_r16(ctx, target));
    4
}

pub fn pop(ctx: &mut GameBoy, target: R16) -> u16 {
    let value = Processor::pop_stack(ctx);
    Processor::set_r16(ctx, target, value);
    3
}

fn weird_flags(ctx: &mut GameBoy, off: i8) {
    let low_sp = (ctx.cpu.sp & 0xFF) as u8;
    let uoff = off as u8;

    ctx.cpu.f = Flags {
        z: false,
        n: false,
        h: ((low_sp & 0xF) + (uoff & 0xF)) > 0xF,
        c: low_sp.overflowing_add(uoff).1,
    };
}
