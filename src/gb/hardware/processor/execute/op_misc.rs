use crate::gb::{
    GameBoy,
    hardware::{
        memory::Memory,
        processor::{EIState, ProcessorMode},
    },
    registers::{IO_IE, IO_IF},
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
    ctx.cpu.ime = false;

    1
}

pub fn ei(ctx: &mut GameBoy) -> u16 {
    // The interrupt flag isn't set until AFTER THE NEXT INSTRUCTION.
    ctx.cpu.ei_state = EIState::Waiting;

    1
}

pub fn halt(ctx: &mut GameBoy) -> u16 {
    let int_pending = (Memory::read(ctx, IO_IE) & Memory::read(ctx, IO_IF)) != 0;

    if !ctx.cpu.ime && int_pending {
        // Halt bug is triggered!
        ctx.cpu.halt_bug = true;
    } else {
        // Enter halt mode
        ctx.cpu.mode = ProcessorMode::Halt;
    }

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
        ctx.cpu.r.a = ctx.cpu.r.a.wrapping_sub(adj);
    } else {
        if f.h || (ctx.cpu.r.a & 0xF) > 0x9 {
            adj += 0x6;
        }
        if f.c || ctx.cpu.r.a > 0x99 {
            adj += 0x60;
            f.c = true;
        }
        ctx.cpu.r.a = ctx.cpu.r.a.wrapping_add(adj);
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
