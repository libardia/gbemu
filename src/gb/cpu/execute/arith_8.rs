use crate::gb::GameBoy;

fn do_add(ctx: &mut GameBoy, value: u8, carry: bool) {
    let lhs = ctx.cpu.a;
    let rhs = value;
    let c = (carry && ctx.cpu.get_flag_c()) as u8;
}
