use crate::gb::{
    GameBoy,
    hardware::processor::{
        Processor,
        instructions::{Cond, Mem},
    },
};

pub fn jump(ctx: &mut GameBoy, cond: Cond, address: Mem) -> u16 {
    if Processor::test_condition(ctx, cond) {
        ctx.cpu.pc = Processor::mem_to_address(ctx, address);

        match cond {
            Cond::ALWAYS => 1, // Special fast version for 'JP HL'
            _ => 4,
        }
    } else {
        // The only situation where the jump isn't taken is when the condition of 'JP cc n16'
        // fails, in which case it takes 3 cycles
        3
    }
}

pub fn jump_rel(ctx: &mut GameBoy, cond: Cond, off: i8) -> u16 {
    if Processor::test_condition(ctx, cond) {
        ctx.cpu.pc = ctx.cpu.pc.wrapping_add_signed(off as i16);
        // 3 cycles if jump
        3
    } else {
        // 2 cycles if no jump
        2
    }
}

pub fn call(ctx: &mut GameBoy, cond: Cond, address: u16) -> u16 {
    if Processor::test_condition(ctx, cond) {
        Processor::push_stack(ctx, ctx.cpu.pc);
        ctx.cpu.pc = address.into();
        // 6 cycles if jump
        6
    } else {
        // 3 cycles if no jump
        3
    }
}

pub fn ret(ctx: &mut GameBoy, cond: Cond, enable_interrupts: bool) -> u16 {
    // This can happen now, because interrupts can only trigger between instructions
    if enable_interrupts {
        ctx.cpu.ime = true;
    }

    if Processor::test_condition(ctx, cond) {
        ctx.cpu.pc = Processor::pop_stack(ctx);

        match cond {
            Cond::ALWAYS => 4,
            _ => 5,
        }
    } else {
        // The only situation where the jump isn't taken is when the condition of 'RET cc'
        // fails, in which case it takes 2 cycles
        2
    }
}

pub fn rst(ctx: &mut GameBoy, address: u16) -> u16 {
    // There's only one path for this one: always jump, always takes 4 cycles.
    Processor::push_stack(ctx, ctx.cpu.pc);
    ctx.cpu.pc = address;
    4
}
