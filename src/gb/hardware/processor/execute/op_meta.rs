use crate::{cpu_log, gb::GameBoy};

pub fn terminate(ctx: &mut GameBoy) -> u16 {
    cpu_log!(info, ctx, "TERMINATE instruction reached.");
    ctx.exit = true;

    // Instant (nothing else updates)
    0
}
