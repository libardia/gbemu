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

#[cfg(test)]
mod tests {
    use crate::gb::{CPU, GameBoy, MMU};

    const INST_ADD: u16 = 0xDF00;
    const MEM_ADD: u16 = 0xDFF0;
    const VAL: u8 = 0xAA;

    macro_rules! load_test {
        ($byte:literal $dest:ident $src:ident) => {
            paste::paste! {
                #[test]
                fn [<load_ $dest _ $src>]() {
                    let gb = &mut GameBoy::new();
                    CPU::set_instruction_at(gb, INST_ADD, $byte);
                    gb.cpu.$src = VAL;
                    CPU::step(gb);
                    assert_eq!(gb.cpu.$dest, VAL);
                    assert_eq!(gb.debug_timer, 4); // 1 mtime
                }
            }
        };
        ($byte:literal $reg:ident _) => {
            paste::paste! {
                #[test]
                fn [<load_ $reg _ $reg>]() {
                    let gb = &mut GameBoy::new();
                    CPU::set_instruction_at(gb, INST_ADD, $byte);
                    CPU::step(gb);
                    assert_eq!(gb.debug_timer, 4); // 1 mtime
                }
            }
        };
        ($byte:literal *$dest:ident $src:ident) => {
            paste::paste! {
                #[test]
                fn [<load_m $dest _ $src>]() {
                    let gb = &mut GameBoy::new();
                    CPU::set_instruction_at(gb, INST_ADD, $byte);
                    gb.cpu.$src = VAL; // if $src is one of $dest, this will be overwritten (that's ok)
                    gb.cpu.[<set_ $dest>](MEM_ADD);
                    CPU::step(gb);
                    assert_eq!(MMU::force_read(gb, MEM_ADD), gb.cpu.$src); // In case of overwrite
                    assert_eq!(gb.debug_timer, 8); // 2 mtime
                }
            }
        };
        ($byte:literal $dest:ident *$src:ident) => {
            paste::paste! {
                #[test]
                fn [<load_ $dest _m $src>]() {
                    let gb = &mut GameBoy::new();
                    CPU::set_instruction_at(gb, INST_ADD, $byte);
                    gb.cpu.[<set_ $src>](MEM_ADD);
                    MMU::force_write(gb, MEM_ADD, VAL);
                    CPU::step(gb);
                    assert_eq!(gb.cpu.$dest, VAL);
                    assert_eq!(gb.debug_timer, 8); // 2 mtime
                }
            }
        };
    }

    macro_rules! load_tests {
        ($(($($arg:tt)*))*) => { $(load_test!($($arg)*);)* };
    }

    load_tests! {
        (0x40   b _) (0x41   b c) (0x42   b d) (0x43   b e) (0x44   b h) (0x45   b l) (0x46 b *hl) (0x47   b a)
        (0x48   c b) (0x49   c _) (0x4A   c d) (0x4B   c e) (0x4C   c h) (0x4D   c l) (0x4E c *hl) (0x4F   c a)
        (0x50   d b) (0x51   d c) (0x52   d _) (0x53   d e) (0x54   d h) (0x55   d l) (0x56 d *hl) (0x57   d a)
        (0x58   e b) (0x59   e c) (0x5A   e d) (0x5B   e _) (0x5C   e h) (0x5D   e l) (0x5E e *hl) (0x5F   e a)
        (0x60   h b) (0x61   h c) (0x62   h d) (0x63   h e) (0x64   h _) (0x65   h l) (0x66 h *hl) (0x67   h a)
        (0x68   l b) (0x69   l c) (0x6A   l d) (0x6B   l e) (0x6C   l h) (0x6D   l _) (0x6E l *hl) (0x6F   l a)
        (0x70 *hl b) (0x71 *hl c) (0x72 *hl d) (0x73 *hl e) (0x74 *hl h) (0x75 *hl l) /*  N/A   */ (0x77 *hl a)
        (0x78   a b) (0x79   a c) (0x7A   a d) (0x7B   a e) (0x7C   a h) (0x7D   a l) (0x7E a *hl) (0x7F   a _)
    }
}
