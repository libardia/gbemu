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
    const HIGH_ADD: u16 = 0xFF00 + HIGH_ADD_LOW as u16;
    const HIGH_ADD_LOW: u8 = 0xC4;
    const VAL: u8 = 0xA4;
    const VAL16: u16 = 0xBEEF;

    fn set_instruction_at(ctx: &mut GameBoy, address: u16, byte: u8) {
        MMU::write(ctx, address, byte);
        ctx.cpu.pc = address;
    }

    macro_rules! load_test {
        ($byte:literal $dest:ident n8) => {
            paste::paste! {
                #[test]
                fn [<load_ $dest _n8>]() {
                    let gb = &mut GameBoy::new();
                    set_instruction_at(gb, INST_ADD, $byte);
                    MMU::write(gb, INST_ADD + 1, VAL);
                    CPU::step(gb);
                    assert_eq!(gb.cpu.$dest, VAL);
                    assert_eq!(gb.debug_timer, 8); // 2 mtime
                    assert_eq!(gb.cpu.pc, INST_ADD + 2); // 2 bytes long
                }
            }
        };

        ($byte:literal *$dest:ident n8) => {
            paste::paste! {
                #[test]
                fn [<load_m $dest _n8>]() {
                    let gb = &mut GameBoy::new();
                    set_instruction_at(gb, INST_ADD, $byte);
                    MMU::write(gb, INST_ADD + 1, VAL);
                    gb.cpu.[<set_ $dest>](MEM_ADD);
                    CPU::step(gb);
                    assert_eq!(MMU::read(gb, MEM_ADD), VAL);
                    assert_eq!(gb.debug_timer, 12); // 3 mtime
                }
            }
        };

        ($byte:literal $reg:ident _) => {
            paste::paste! {
                #[test]
                fn [<load_ $reg _ $reg>]() {
                    let gb = &mut GameBoy::new();
                    set_instruction_at(gb, INST_ADD, $byte);
                    CPU::step(gb);
                    assert_eq!(gb.debug_timer, 4); // 1 mtime
                }
            }
        };

        ($byte:literal *a16 $src:ident) => {
            paste::paste! {
                #[test]
                fn [<load_ma16_ $src>]() {
                    let gb = &mut GameBoy::new();
                    set_instruction_at(gb, INST_ADD, $byte);
                    MMU::write(gb, INST_ADD + 1, (MEM_ADD & 0xFF) as u8);
                    MMU::write(gb, INST_ADD + 2, (MEM_ADD >> 8) as u8);
                    gb.cpu.$src = VAL;
                    CPU::step(gb);
                    assert_eq!(MMU::read(gb, MEM_ADD), VAL);
                    assert_eq!(gb.debug_timer, 16); // 4 mtime
                }
            }
        };

        ($byte:literal *$dest:ident $src:ident) => {
            paste::paste! {
                #[test]
                fn [<load_m $dest _ $src>]() {
                    let gb = &mut GameBoy::new();
                    set_instruction_at(gb, INST_ADD, $byte);
                    gb.cpu.$src = VAL; // if $src is one of $dest, this will be overwritten (that's ok)
                    gb.cpu.[<set_ $dest>](MEM_ADD);
                    CPU::step(gb);
                    assert!(gb.cpu.$src != 0xFF); // Make sure the load was in the correct direction
                    assert_eq!(MMU::read(gb, MEM_ADD), gb.cpu.$src); // In case of overwrite
                    assert_eq!(gb.debug_timer, 8); // 2 mtime
                }
            }
        };

        ($byte:literal $dest:ident *a16) => {
            paste::paste! {
                #[test]
                fn [<load_ $dest _ma16>]() {
                    let gb = &mut GameBoy::new();
                    set_instruction_at(gb, INST_ADD, $byte);
                    MMU::write(gb, INST_ADD + 1, (MEM_ADD & 0xFF) as u8);
                    MMU::write(gb, INST_ADD + 2, (MEM_ADD >> 8) as u8);
                    MMU::write(gb, MEM_ADD, VAL);
                    CPU::step(gb);
                    assert_eq!(gb.cpu.$dest, VAL);
                    assert_eq!(gb.debug_timer, 16); // 4 mtime
                }
            }
        };

        ($byte:literal $dest:ident *$src:ident) => {
            paste::paste! {
                #[test]
                fn [<load_ $dest _m $src>]() {
                    let gb = &mut GameBoy::new();
                    set_instruction_at(gb, INST_ADD, $byte);
                    gb.cpu.[<set_ $src>](MEM_ADD);
                    MMU::write(gb, MEM_ADD, VAL);
                    CPU::step(gb);
                    assert_eq!(gb.cpu.$dest, VAL);
                    assert_eq!(gb.debug_timer, 8); // 2 mtime
                }
            }
        };

        ($byte:literal $dest:ident $src:ident) => {
            paste::paste! {
                #[test]
                fn [<load_ $dest _ $src>]() {
                    let gb = &mut GameBoy::new();
                    set_instruction_at(gb, INST_ADD, $byte);
                    gb.cpu.$src = VAL;
                    CPU::step(gb);
                    assert_eq!(gb.cpu.$dest, VAL);
                    assert_eq!(gb.debug_timer, 4); // 1 mtime
                }
            }
        };
    }

    macro_rules! load_tests {
        ($(($($arg:tt)*))*) => { $(load_test!($($arg)*);)* };
    }

    macro_rules! load16_test {
        ($byte:literal $dest:ident n16) => {
            paste::paste! {
                #[test]
                fn [<load_ $dest _n16>]() {
                    let gb = &mut GameBoy::new();
                    set_instruction_at(gb, INST_ADD, $byte);
                    MMU::write(gb, INST_ADD + 1, (VAL16 & 0xFF) as u8);
                    MMU::write(gb, INST_ADD + 2, (VAL16 >> 8) as u8);
                    CPU::step(gb);
                    assert_eq!(gb.cpu.[<get_ $dest>](), VAL16);
                    assert_eq!(gb.debug_timer, 12); // 3 mtime
                }
            }
        };
    }

    macro_rules! load16_tests {
        ($(($($arg:tt)*))*) => { $(load16_test!($($arg)*);)* };
    }

    load_tests! {
        // Simple reg-to-reg loads
        (0x40 b _) (0x41 b c) (0x42 b d) (0x43 b e) (0x44 b h) (0x45 b l) (0x47 b a)
        (0x48 c b) (0x49 c _) (0x4A c d) (0x4B c e) (0x4C c h) (0x4D c l) (0x4F c a)
        (0x50 d b) (0x51 d c) (0x52 d _) (0x53 d e) (0x54 d h) (0x55 d l) (0x57 d a)
        (0x58 e b) (0x59 e c) (0x5A e d) (0x5B e _) (0x5C e h) (0x5D e l) (0x5F e a)
        (0x60 h b) (0x61 h c) (0x62 h d) (0x63 h e) (0x64 h _) (0x65 h l) (0x67 h a)
        (0x68 l b) (0x69 l c) (0x6A l d) (0x6B l e) (0x6C l h) (0x6D l _) (0x6F l a)
        (0x78 a b) (0x79 a c) (0x7A a d) (0x7B a e) (0x7C a h) (0x7D a l) (0x7F a _)

        // Memory loads
        (0x46 b *hl) (0x4E c *hl) (0x56 d *hl) (0x5E e *hl) (0x66 h *hl) (0x6E l *hl) (0x7E a *hl)
        (0x70 *hl b) (0x71 *hl c) (0x72 *hl d) (0x73 *hl e) (0x74 *hl h) (0x75 *hl l) (0x77 *hl a)

        // Other memory <=> A loads
        (0x0A a *bc) (0x1A a *de) (0xFA a *a16)
        (0x02 *bc a) (0x12 *de a) (0xEA *a16 a)

        // Constant loads
        (0x06 b n8) (0x0E c n8) (0x16 d n8) (0x1E e n8) (0x26 h n8) (0x2E l n8) (0x36 *hl n8) (0x3E a n8)
    }

    load16_tests! {
        (0x01 bc n16) (0x11 de n16) (0x21 hl n16)
    }

    // HL+ and HL- loads
    #[test]
    fn ld_mhli_a() {
        let gb = &mut GameBoy::new();
        set_instruction_at(gb, INST_ADD, 0x22); // Instruction code is $22
        gb.cpu.a = VAL;
        gb.cpu.set_hl(MEM_ADD);
        CPU::step(gb);
        assert_eq!(MMU::read(gb, MEM_ADD), VAL);
        assert_eq!(gb.cpu.get_hl(), MEM_ADD + 1);
        assert_eq!(gb.debug_timer, 8); // 2 mtime
    }

    #[test]
    fn ld_mhld_a() {
        let gb = &mut GameBoy::new();
        set_instruction_at(gb, INST_ADD, 0x32); // Instruction code is $32
        gb.cpu.a = VAL;
        gb.cpu.set_hl(MEM_ADD);
        CPU::step(gb);
        assert_eq!(MMU::read(gb, MEM_ADD), VAL);
        assert_eq!(gb.cpu.get_hl(), MEM_ADD - 1);
        assert_eq!(gb.debug_timer, 8); // 2 mtime
    }

    #[test]
    fn ld_a_mhli() {
        let gb = &mut GameBoy::new();
        set_instruction_at(gb, INST_ADD, 0x2A); // Instruction code is $2A
        MMU::write(gb, MEM_ADD, VAL);
        gb.cpu.set_hl(MEM_ADD);
        CPU::step(gb);
        assert_eq!(gb.cpu.a, VAL);
        assert_eq!(gb.cpu.get_hl(), MEM_ADD + 1);
        assert_eq!(gb.debug_timer, 8); // 2 mtime
    }

    #[test]
    fn ld_a_mhld() {
        let gb = &mut GameBoy::new();
        set_instruction_at(gb, INST_ADD, 0x3A); // Instruction code is $3A
        MMU::write(gb, MEM_ADD, VAL);
        gb.cpu.set_hl(MEM_ADD);
        CPU::step(gb);
        assert_eq!(gb.cpu.a, VAL);
        assert_eq!(gb.cpu.get_hl(), MEM_ADD - 1);
        assert_eq!(gb.debug_timer, 8); // 2 mtime
    }

    // High loads
    #[test]
    fn ldh_ma8_a() {
        let gb = &mut GameBoy::new();
        set_instruction_at(gb, INST_ADD, 0xE0); // Instruction code is $E0
        MMU::write(gb, INST_ADD + 1, HIGH_ADD_LOW);
        gb.cpu.a = VAL;
        CPU::step(gb);
        assert_eq!(MMU::read(gb, HIGH_ADD), VAL);
        assert_eq!(gb.debug_timer, 12); // 3 mtime
        assert_eq!(gb.cpu.pc, INST_ADD + 2); // 2 bytes long
    }
}
