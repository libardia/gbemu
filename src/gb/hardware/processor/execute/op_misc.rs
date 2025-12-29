use crate::{
    cpu_log,
    gb::{
        GameBoy,
        hardware::processor::{EIState, Processor, ProcessorMode},
    },
    wrapping_add_warn, wrapping_sub_warn,
};

/* #region Carry flag */
pub fn ccf(ctx: &mut GameBoy) -> u16 {
    ctx.cpu.f.n = false;
    ctx.cpu.f.h = false;
    ctx.cpu.f.c = !ctx.cpu.f.c;

    1
}

pub fn scf(ctx: &mut GameBoy) -> u16 {
    ctx.cpu.f.n = false;
    ctx.cpu.f.h = false;
    ctx.cpu.f.c = true;

    1
}
/* #endregion */

/* #region Interrupts */
pub fn di(ctx: &mut GameBoy) -> u16 {
    cpu_log!(debug, ctx, "Interrupts disabled");
    ctx.cpu.ime = false;

    1
}

pub fn ei(ctx: &mut GameBoy) -> u16 {
    // The interrupt flag isn't set until AFTER THE NEXT INSTRUCTION.
    cpu_log!(
        debug,
        ctx,
        "Interrupts will be enabled after next instruction"
    );
    ctx.cpu.ei_state = EIState::Waiting;

    1
}

pub fn halt(ctx: &mut GameBoy) -> u16 {
    if !ctx.cpu.ime && (Processor::pending_interrupts(ctx) != 0) {
        // Halt bug is triggered!
        ctx.cpu.halt_bug = true;
    }
    // Enter halt mode
    cpu_log!(debug, ctx, "Entering HALT mode");
    ctx.cpu.mode = ProcessorMode::Halt;

    1
}
/* #endregion */

/* #region Misc */
pub fn daa(ctx: &mut GameBoy) -> u16 {
    let mut adj = 0u8;
    let mut f = ctx.cpu.f;

    if f.n {
        if f.h {
            adj += 0x6;
        }
        if f.c {
            adj += 0x60;
        }
        ctx.cpu.r.a = wrapping_sub_warn!(ctx.cpu.r.a, adj, "DAA caused reg A to underflow");
    } else {
        if f.h || (ctx.cpu.r.a & 0xF) > 0x9 {
            adj += 0x6;
        }
        if f.c || ctx.cpu.r.a > 0x99 {
            adj += 0x60;
            f.c = true;
        }
        ctx.cpu.r.a = wrapping_add_warn!(ctx.cpu.r.a, adj, "DAA caused reg A to overflow");
    }

    f.z = ctx.cpu.r.a == 0;
    f.h = false;
    ctx.cpu.f = f;

    1
}

pub fn stop(ctx: &mut GameBoy) -> u16 {
    // STOP is completely insane
    // https://gbdev.io/pandocs/Reducing_Power_Consumption.html#the-bizarre-case-of-the-game-boy-stop-instruction-before-even-considering-timing
    // TODO: STOP
    todo!()
}
/* #endregion */
