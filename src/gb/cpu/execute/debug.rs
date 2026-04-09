use log::debug;

use crate::gb::GameBoy;

pub fn exit(ctx: &mut GameBoy) {
    ctx.exit = true;
}

pub fn print_cpu(ctx: &mut GameBoy) {
    debug!("{}", ctx.cpu.debug_str());
}
