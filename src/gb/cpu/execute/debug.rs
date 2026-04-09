use crate::gb::GameBoy;

pub fn exit(ctx: &mut GameBoy) {
    ctx.exit = true;
}
