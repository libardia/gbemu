use crate::gb::{GameBoy, cpu::CPU};

macro_rules! load_r8 {
    ($ctx:expr, $dest:ident, n8) => {{
        $ctx.cpu.$dest = CPU::next_byte($ctx);
    }};

    ($ctx:expr, $dest:ident, $src:ident) => {{
        $ctx.cpu.$dest = $ctx.cpu.$src;
    }};

    ($ctx:expr, $dest:ident, [a16]) => {{
        let address = CPU::next_word($ctx);
        $ctx.cpu.$dest = CPU::read_tick($ctx, address);
    }};

    ($ctx:expr, $dest:ident, [$r16:ident]) => {{
        paste::paste! {
            let address = $ctx.cpu.[<get_ $r16>]();
            $ctx.cpu.$dest = CPU::read_tick($ctx, address);
        }
    }};

    ($ctx:expr, [a16], $src:ident) => {{
        paste::paste! {
            let address = CPU::next_word($ctx);
            CPU::write_tick($ctx, address, $ctx.cpu.$src);
        }
    }};

    ($ctx:expr, [$r16:ident], n8) => {{
        paste::paste! {
            let byte = CPU::next_byte($ctx);
            let address = $ctx.cpu.[<get_ $r16>]();
            CPU::write_tick($ctx, address, byte);
        }
    }};

    ($ctx:expr, [$r16:ident], $src:ident) => {{
        paste::paste! {
            let address = $ctx.cpu.[<get_ $r16>]();
            CPU::write_tick($ctx, address, $ctx.cpu.$src);
        }
    }};
}
pub(super) use load_r8;

pub fn loadh_ma8_a(ctx: &mut GameBoy) {
    let byte = CPU::next_byte(ctx);
    loadh_m_a(ctx, byte);
}

pub fn loadh_mc_a(ctx: &mut GameBoy) {
    loadh_m_a(ctx, ctx.cpu.c);
}

fn loadh_m_a(ctx: &mut GameBoy, half: u8) {
    let address = 0xFF00 + half as u16;
    CPU::write_tick(ctx, address, ctx.cpu.a);
}

pub fn loadh_a_ma8(ctx: &mut GameBoy) {
    let byte = CPU::next_byte(ctx);
    loadh_a_m(ctx, byte);
}

pub fn loadh_a_mc(ctx: &mut GameBoy) {
    loadh_a_m(ctx, ctx.cpu.c);
}

fn loadh_a_m(ctx: &mut GameBoy, half: u8) {
    let address = 0xFF00 + half as u16;
    ctx.cpu.a = CPU::read_tick(ctx, address);
}

macro_rules! load_r16 {
    ($ctx:expr, $dest:ident) => {{
        paste::paste! {
            let word = CPU::next_word($ctx);
            $ctx.cpu.[<set_ $dest>](word);
        }
    }};
}
pub(super) use load_r16;
