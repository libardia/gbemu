use crate::{
    gb::{
        GameBoy,
        cpu::{CPU, Flags, access::ByteLoc},
    },
    macros::select,
};

/* #region Rotate right */
pub fn rra(ctx: &mut GameBoy) {
    let byte = CPU::get_byte_at(ctx, ByteLoc::A);
    let rotated_out = byte & 1 != 0;
    let result = byte >> 1 | select!(ctx.cpu.f.c; 0x80, 0);

    CPU::set_byte_at(ctx, ByteLoc::A, result);
    ctx.cpu.f = Flags {
        z: false,
        n: false,
        h: false,
        c: rotated_out,
    };
}

pub fn rrca(ctx: &mut GameBoy) {
    let byte = CPU::get_byte_at(ctx, ByteLoc::A);
    let rotated_out = byte & 1 != 0;
    let result = byte >> 1 | select!(rotated_out; 0x80, 0);

    CPU::set_byte_at(ctx, ByteLoc::A, result);
    ctx.cpu.f = Flags {
        z: false,
        n: false,
        h: false,
        c: rotated_out,
    };
}

pub fn rr_r8(ctx: &mut GameBoy, target: ByteLoc) {
    let byte = CPU::get_byte_at(ctx, target);
    let rotated_out = byte & 1 != 0;
    let result = byte >> 1 | select!(ctx.cpu.f.c; 0x80, 0);

    CPU::set_byte_at(ctx, target, result);
    ctx.cpu.f = Flags {
        z: result == 0,
        n: false,
        h: false,
        c: rotated_out,
    };
}

pub fn rrc_r8(ctx: &mut GameBoy, target: ByteLoc) {
    let byte = CPU::get_byte_at(ctx, target);
    let rotated_out = byte & 1 != 0;
    let result = byte >> 1 | select!(rotated_out; 0x80, 0);

    CPU::set_byte_at(ctx, target, result);
    ctx.cpu.f = Flags {
        z: result == 0,
        n: false,
        h: false,
        c: rotated_out,
    };
}
/* #endregion */

/* #region Rotate left */
pub fn rla(ctx: &mut GameBoy) {
    let byte = CPU::get_byte_at(ctx, ByteLoc::A);
    let rotated_out = byte & 0x80 != 0;
    let result = byte << 1 | (ctx.cpu.f.c as u8);

    CPU::set_byte_at(ctx, ByteLoc::A, result);
    ctx.cpu.f = Flags {
        z: false,
        n: false,
        h: false,
        c: rotated_out,
    };
}

pub fn rlca(ctx: &mut GameBoy) {
    let byte = CPU::get_byte_at(ctx, ByteLoc::A);
    let rotated_out = byte & 0x80 != 0;
    let result = byte << 1 | (rotated_out as u8);

    CPU::set_byte_at(ctx, ByteLoc::A, result);
    ctx.cpu.f = Flags {
        z: false,
        n: false,
        h: false,
        c: rotated_out,
    };
}

pub fn rl_r8(ctx: &mut GameBoy, target: ByteLoc) {
    let byte = CPU::get_byte_at(ctx, target);
    let rotated_out = byte & 0x80 != 0;
    let result = byte << 1 | (ctx.cpu.f.c as u8);

    CPU::set_byte_at(ctx, target, result);
    ctx.cpu.f = Flags {
        z: result == 0,
        n: false,
        h: false,
        c: rotated_out,
    };
}

pub fn rlc_r8(ctx: &mut GameBoy, target: ByteLoc) {
    let byte = CPU::get_byte_at(ctx, target);
    let rotated_out = byte & 0x80 != 0;
    let result = byte << 1 | (rotated_out as u8);

    CPU::set_byte_at(ctx, target, result);
    ctx.cpu.f = Flags {
        z: result == 0,
        n: false,
        h: false,
        c: rotated_out,
    };
}
/* #endregion */

/* #region Shift right */
pub fn srl_r8(ctx: &mut GameBoy, target: ByteLoc) {
    let byte = CPU::get_byte_at(ctx, target);
    let rotated_out = byte & 1 != 0;
    let result = byte >> 1;

    CPU::set_byte_at(ctx, target, result);
    ctx.cpu.f = Flags {
        z: result == 0,
        n: false,
        h: false,
        c: rotated_out,
    };
}

pub fn sra_r8(ctx: &mut GameBoy, target: ByteLoc) {
    let byte = CPU::get_byte_at(ctx, target);
    let rotated_out = byte & 1 != 0;
    let leading_bit = byte & 0x80;
    let result = byte >> 1 | leading_bit;

    CPU::set_byte_at(ctx, target, result);
    ctx.cpu.f = Flags {
        z: result == 0,
        n: false,
        h: false,
        c: rotated_out,
    };
}
/* #endregion */

/* #region Shift left */
pub fn sla_r8(ctx: &mut GameBoy, target: ByteLoc) {
    let byte = CPU::get_byte_at(ctx, target);
    let rotated_out = byte & 0x80 != 0;
    let result = byte << 1;

    CPU::set_byte_at(ctx, target, result);
    ctx.cpu.f = Flags {
        z: result == 0,
        n: false,
        h: false,
        c: rotated_out,
    };
}
/* #endregion */

/* #region Swap */
pub fn swap_r8(ctx: &mut GameBoy, target: ByteLoc) {
    let byte = CPU::get_byte_at(ctx, target);
    let result = ((byte & 0xF) << 4) | (byte >> 4);

    CPU::set_byte_at(ctx, target, result);
    ctx.cpu.f = Flags {
        z: result == 0,
        n: false,
        h: false,
        c: false,
    };
}
/* #endregion */

#[cfg(test)]
mod tests {
    use crate::{gb::mmu::MMU, testutil::step_test};

    use super::*;
    use test_log::test;

    #[test]
    fn rra() {
        let ctx = &mut GameBoy::new();
        step_test! {
            ctx: ctx;
            code: 0x1F, length: 1, cycles: 1
            setup {
                ctx.cpu.a = 0b10000001;
                ctx.cpu.f.c = false;
            }
            after {
                assert_eq!(ctx.cpu.a, 0b01000000);
                assert!(!ctx.cpu.f.z);
                assert!(!ctx.cpu.f.n);
                assert!(!ctx.cpu.f.h);
                assert!( ctx.cpu.f.c);
            }
        }
    }

    #[test]
    fn rrca() {
        let ctx = &mut GameBoy::new();
        step_test! {
            ctx: ctx;
            code: 0x0F, length: 1, cycles: 1
            setup {
                ctx.cpu.a = 0b10000001;
                ctx.cpu.f.c = false;
            }
            after {
                assert_eq!(ctx.cpu.a, 0b11000000);
                assert!(!ctx.cpu.f.z);
                assert!(!ctx.cpu.f.n);
                assert!(!ctx.cpu.f.h);
                assert!( ctx.cpu.f.c);
            }
        }
    }

    #[test]
    fn rla() {
        let ctx = &mut GameBoy::new();
        step_test! {
            ctx: ctx;
            code: 0x17, length: 1, cycles: 1
            setup {
                ctx.cpu.a = 0b10000001;
                ctx.cpu.f.c = false;
            }
            after {
                assert_eq!(ctx.cpu.a, 0b00000010);
                assert!(!ctx.cpu.f.z);
                assert!(!ctx.cpu.f.n);
                assert!(!ctx.cpu.f.h);
                assert!( ctx.cpu.f.c);
            }
        }
    }

    #[test]
    fn rlca() {
        let ctx = &mut GameBoy::new();
        step_test! {
            ctx: ctx;
            code: 0x07, length: 1, cycles: 1
            setup {
                ctx.cpu.a = 0b10000001;
                ctx.cpu.f.c = false;
            }
            after {
                assert_eq!(ctx.cpu.a, 0b00000011);
                assert!(!ctx.cpu.f.z);
                assert!(!ctx.cpu.f.n);
                assert!(!ctx.cpu.f.h);
                assert!( ctx.cpu.f.c);
            }
        }
    }

    macro_rules! rr_r8 {
        ($($code:literal $target:ident;)*) => {
            paste::paste! {$(
                #[test]
                fn [<rr_ $target>]() {
                    let ctx = &mut GameBoy::new();
                    step_test! {
                        ctx: ctx;
                        code: 0xCB $code, length: 2, cycles: 2
                        setup {
                            ctx.cpu.$target = 0b10000001;
                            ctx.cpu.f.c = false;
                        }
                        after {
                            assert_eq!(ctx.cpu.$target, 0b01000000);
                            assert!(!ctx.cpu.f.z);
                            assert!(!ctx.cpu.f.n);
                            assert!(!ctx.cpu.f.h);
                            assert!( ctx.cpu.f.c);
                        }
                    }
                }
            )*}
        };
    }

    rr_r8! {
        0x18 b; 0x19 c; 0x1A d; 0x1B e; 0x1C h; 0x1D l; 0x1F a;
    }

    #[test]
    fn rr_mhl() {
        let ctx = &mut GameBoy::new();
        step_test! {
            ctx: ctx;
            code: 0xCB 0x1E, length: 2, cycles: 4
            setup {
                MMU::write(ctx, 0xDEAD, 0b10000001);
                ctx.cpu.set_hl(0xDEAD);
                ctx.cpu.f.c = false;
            }
            after {
                assert_eq!(MMU::read(ctx, 0xDEAD), 0b01000000);
                assert!(!ctx.cpu.f.z);
                assert!(!ctx.cpu.f.n);
                assert!(!ctx.cpu.f.h);
                assert!( ctx.cpu.f.c);
            }
        }
    }

    macro_rules! rrc_r8 {
        ($($code:literal $target:ident;)*) => {
            paste::paste! {$(
                #[test]
                fn [<rrc_ $target>]() {
                    let ctx = &mut GameBoy::new();
                    step_test! {
                        ctx: ctx;
                        code: 0xCB $code, length: 2, cycles: 2
                        setup {
                            ctx.cpu.$target = 0b10000001;
                            ctx.cpu.f.c = false;
                        }
                        after {
                            assert_eq!(ctx.cpu.$target, 0b11000000);
                            assert!(!ctx.cpu.f.z);
                            assert!(!ctx.cpu.f.n);
                            assert!(!ctx.cpu.f.h);
                            assert!( ctx.cpu.f.c);
                        }
                    }
                }
            )*}
        };
    }

    rrc_r8! {
        0x08 b; 0x09 c; 0x0A d; 0x0B e; 0x0C h; 0x0D l; 0x0F a;
    }

    #[test]
    fn rrc_mhl() {
        let ctx = &mut GameBoy::new();
        step_test! {
            ctx: ctx;
            code: 0xCB 0x0E, length: 2, cycles: 4
            setup {
                MMU::write(ctx, 0xDEAD, 0b10000001);
                ctx.cpu.set_hl(0xDEAD);
                ctx.cpu.f.c = false;
            }
            after {
                assert_eq!(MMU::read(ctx, 0xDEAD), 0b11000000);
                assert!(!ctx.cpu.f.z);
                assert!(!ctx.cpu.f.n);
                assert!(!ctx.cpu.f.h);
                assert!( ctx.cpu.f.c);
            }
        }
    }

    macro_rules! rl_r8 {
        ($($code:literal $target:ident;)*) => {
            paste::paste! {$(
                #[test]
                fn [<rl_ $target>]() {
                    let ctx = &mut GameBoy::new();
                    step_test! {
                        ctx: ctx;
                        code: 0xCB $code, length: 2, cycles: 2
                        setup {
                            ctx.cpu.$target = 0b10000001;
                            ctx.cpu.f.c = false;
                        }
                        after {
                            assert_eq!(ctx.cpu.$target, 0b00000010);
                            assert!(!ctx.cpu.f.z);
                            assert!(!ctx.cpu.f.n);
                            assert!(!ctx.cpu.f.h);
                            assert!( ctx.cpu.f.c);
                        }
                    }
                }
            )*}
        };
    }

    rl_r8! {
        0x10 b; 0x11 c; 0x12 d; 0x13 e; 0x14 h; 0x15 l; 0x17 a;
    }

    #[test]
    fn rl_mhl() {
        let ctx = &mut GameBoy::new();
        step_test! {
            ctx: ctx;
            code: 0xCB 0x16, length: 2, cycles: 4
            setup {
                MMU::write(ctx, 0xDEAD, 0b10000001);
                ctx.cpu.set_hl(0xDEAD);
                ctx.cpu.f.c = false;
            }
            after {
                assert_eq!(MMU::read(ctx, 0xDEAD), 0b00000010);
                assert!(!ctx.cpu.f.z);
                assert!(!ctx.cpu.f.n);
                assert!(!ctx.cpu.f.h);
                assert!( ctx.cpu.f.c);
            }
        }
    }

    macro_rules! rlc_r8 {
        ($($code:literal $target:ident;)*) => {
            paste::paste! {$(
                #[test]
                fn [<rlc_ $target>]() {
                    let ctx = &mut GameBoy::new();
                    step_test! {
                        ctx: ctx;
                        code: 0xCB $code, length: 2, cycles: 2
                        setup {
                            ctx.cpu.$target = 0b10000001;
                            ctx.cpu.f.c = false;
                        }
                        after {
                            assert_eq!(ctx.cpu.$target, 0b00000011);
                            assert!(!ctx.cpu.f.z);
                            assert!(!ctx.cpu.f.n);
                            assert!(!ctx.cpu.f.h);
                            assert!( ctx.cpu.f.c);
                        }
                    }
                }
            )*}
        };
    }

    rlc_r8! {
        0x00 b; 0x01 c; 0x02 d; 0x03 e; 0x04 h; 0x05 l; 0x07 a;
    }

    #[test]
    fn rlc_mhl() {
        let ctx = &mut GameBoy::new();
        step_test! {
            ctx: ctx;
            code: 0xCB 0x06, length: 2, cycles: 4
            setup {
                MMU::write(ctx, 0xDEAD, 0b10000001);
                ctx.cpu.set_hl(0xDEAD);
                ctx.cpu.f.c = false;
            }
            after {
                assert_eq!(MMU::read(ctx, 0xDEAD), 0b00000011);
                assert!(!ctx.cpu.f.z);
                assert!(!ctx.cpu.f.n);
                assert!(!ctx.cpu.f.h);
                assert!( ctx.cpu.f.c);
            }
        }
    }

    macro_rules! srl_r8 {
        ($($code:literal $target:ident;)*) => {
            paste::paste! {$(
                #[test]
                fn [<srl_ $target>]() {
                    let ctx = &mut GameBoy::new();
                    step_test! {
                        ctx: ctx;
                        code: 0xCB $code, length: 2, cycles: 2
                        setup {
                            ctx.cpu.$target = 0b10000001;
                            ctx.cpu.f.c = false;
                        }
                        after {
                            assert_eq!(ctx.cpu.$target, 0b01000000);
                            assert!(!ctx.cpu.f.z);
                            assert!(!ctx.cpu.f.n);
                            assert!(!ctx.cpu.f.h);
                            assert!( ctx.cpu.f.c);
                        }
                    }
                }
            )*}
        };
    }

    srl_r8! {
        0x38 b; 0x39 c; 0x3A d; 0x3B e; 0x3C h; 0x3D l; 0x3F a;
    }

    #[test]
    fn srl_mhl() {
        let ctx = &mut GameBoy::new();
        step_test! {
            ctx: ctx;
            code: 0xCB 0x3E, length: 2, cycles: 4
            setup {
                MMU::write(ctx, 0xDEAD, 0b10000001);
                ctx.cpu.set_hl(0xDEAD);
                ctx.cpu.f.c = false;
            }
            after {
                assert_eq!(MMU::read(ctx, 0xDEAD), 0b01000000);
                assert!(!ctx.cpu.f.z);
                assert!(!ctx.cpu.f.n);
                assert!(!ctx.cpu.f.h);
                assert!( ctx.cpu.f.c);
            }
        }
    }

    macro_rules! sra_r8 {
        ($($code:literal $target:ident;)*) => {
            paste::paste! {$(
                #[test]
                fn [<sra_ $target>]() {
                    let ctx = &mut GameBoy::new();
                    step_test! {
                        ctx: ctx;
                        code: 0xCB $code, length: 2, cycles: 2
                        setup {
                            ctx.cpu.$target = 0b10000001;
                            ctx.cpu.f.c = false;
                        }
                        after {
                            assert_eq!(ctx.cpu.$target, 0b11000000);
                            assert!(!ctx.cpu.f.z);
                            assert!(!ctx.cpu.f.n);
                            assert!(!ctx.cpu.f.h);
                            assert!( ctx.cpu.f.c);
                        }
                    }
                }
            )*}
        };
    }

    sra_r8! {
        0x28 b; 0x29 c; 0x2A d; 0x2B e; 0x2C h; 0x2D l; 0x2F a;
    }

    #[test]
    fn sra_mhl() {
        let ctx = &mut GameBoy::new();
        step_test! {
            ctx: ctx;
            code: 0xCB 0x2E, length: 2, cycles: 4
            setup {
                MMU::write(ctx, 0xDEAD, 0b10000001);
                ctx.cpu.set_hl(0xDEAD);
                ctx.cpu.f.c = false;
            }
            after {
                assert_eq!(MMU::read(ctx, 0xDEAD), 0b11000000);
                assert!(!ctx.cpu.f.z);
                assert!(!ctx.cpu.f.n);
                assert!(!ctx.cpu.f.h);
                assert!( ctx.cpu.f.c);
            }
        }
    }

    macro_rules! sla_r8 {
        ($($code:literal $target:ident;)*) => {
            paste::paste! {$(
                #[test]
                fn [<sla_ $target>]() {
                    let ctx = &mut GameBoy::new();
                    step_test! {
                        ctx: ctx;
                        code: 0xCB $code, length: 2, cycles: 2
                        setup {
                            ctx.cpu.$target = 0b10000001;
                            ctx.cpu.f.c = false;
                        }
                        after {
                            assert_eq!(ctx.cpu.$target, 0b00000010);
                            assert!(!ctx.cpu.f.z);
                            assert!(!ctx.cpu.f.n);
                            assert!(!ctx.cpu.f.h);
                            assert!( ctx.cpu.f.c);
                        }
                    }
                }
            )*}
        };
    }

    sla_r8! {
        0x20 b; 0x21 c; 0x22 d; 0x23 e; 0x24 h; 0x25 l; 0x27 a;
    }

    #[test]
    fn sla_mhl() {
        let ctx = &mut GameBoy::new();
        step_test! {
            ctx: ctx;
            code: 0xCB 0x26, length: 2, cycles: 4
            setup {
                MMU::write(ctx, 0xDEAD, 0b10000001);
                ctx.cpu.set_hl(0xDEAD);
                ctx.cpu.f.c = false;
            }
            after {
                assert_eq!(MMU::read(ctx, 0xDEAD), 0b00000010);
                assert!(!ctx.cpu.f.z);
                assert!(!ctx.cpu.f.n);
                assert!(!ctx.cpu.f.h);
                assert!( ctx.cpu.f.c);
            }
        }
    }

    macro_rules! swap_r8 {
        ($($code:literal $target:ident;)*) => {
            paste::paste! {$(
                #[test]
                fn [<swap_ $target>]() {
                    let ctx = &mut GameBoy::new();
                    step_test! {
                        ctx: ctx;
                        code: 0xCB $code, length: 2, cycles: 2
                        setup {
                            ctx.cpu.$target = 0b10110000;
                            ctx.cpu.f.c = false;
                        }
                        after {
                            assert_eq!(ctx.cpu.$target, 0b00001011);
                            assert!(!ctx.cpu.f.z);
                            assert!(!ctx.cpu.f.n);
                            assert!(!ctx.cpu.f.h);
                            assert!(!ctx.cpu.f.c);
                        }
                    }
                }
            )*}
        };
    }

    swap_r8! {
        0x30 b; 0x31 c; 0x32 d; 0x33 e; 0x34 h; 0x35 l; 0x37 a;
    }

    #[test]
    fn swap_mhl() {
        let ctx = &mut GameBoy::new();
        step_test! {
            ctx: ctx;
            code: 0xCB 0x36, length: 2, cycles: 4
            setup {
                MMU::write(ctx, 0xDEAD, 0b10110000);
                ctx.cpu.set_hl(0xDEAD);
                ctx.cpu.f.c = false;
            }
            after {
                assert_eq!(MMU::read(ctx, 0xDEAD), 0b00001011);
                assert!(!ctx.cpu.f.z);
                assert!(!ctx.cpu.f.n);
                assert!(!ctx.cpu.f.h);
                assert!(!ctx.cpu.f.c);
            }
        }
    }
}
