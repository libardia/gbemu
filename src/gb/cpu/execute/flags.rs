use crate::gb::{GameBoy, cpu::CPU};

macro_rules! operations {
    (@inner $bit:literal $target:ident) => {
        paste::paste! {
            #[inline(always)]
            pub fn [<bit_ $bit _ $target>](ctx: &mut GameBoy) {
                ctx.cpu.f.z = ctx.cpu.$target & (1 << $bit) == 0;
                ctx.cpu.f.n = false;
                ctx.cpu.f.h = true;
            }

            #[inline(always)]
            pub fn [<set_ $bit _ $target>](ctx: &mut GameBoy) {
                ctx.cpu.$target |= 1 << $bit;
            }

            #[inline(always)]
            pub fn [<res_ $bit _ $target>](ctx: &mut GameBoy) {
                ctx.cpu.$target &= !(1 << $bit);
            }
        }
    };

    (@inner $bit:literal *hl) => {
        paste::paste! {
            #[inline(always)]
            pub fn [<bit_ $bit _mhl>](ctx: &mut GameBoy) {
                let address = ctx.cpu.get_hl();
                let byte = CPU::read_tick(ctx, address);
                ctx.cpu.f.z = byte & (1 << $bit) == 0;
                ctx.cpu.f.n = false;
                ctx.cpu.f.h = true;
            }

            #[inline(always)]
            pub fn [<set_ $bit _mhl>](ctx: &mut GameBoy) {
                let address = ctx.cpu.get_hl();
                let byte = CPU::read_tick(ctx, address);
                CPU::write_tick(ctx, address, byte | (1 << $bit));
            }

            #[inline(always)]
            pub fn [<res_ $bit _mhl>](ctx: &mut GameBoy) {
                let address = ctx.cpu.get_hl();
                let byte = CPU::read_tick(ctx, address);
                CPU::write_tick(ctx, address, byte & !(1 << $bit));
            }
        }
    };

    ($(($($arg:tt)*))*) => {
        $(
            operations!(@inner $($arg)*);
        )*
    };
}

operations! {
    (0 b) (0 c) (0 d) (0 e) (0 h) (0 l) (0 *hl) (0 a)
    (1 b) (1 c) (1 d) (1 e) (1 h) (1 l) (1 *hl) (1 a)
    (2 b) (2 c) (2 d) (2 e) (2 h) (2 l) (2 *hl) (2 a)
    (3 b) (3 c) (3 d) (3 e) (3 h) (3 l) (3 *hl) (3 a)
    (4 b) (4 c) (4 d) (4 e) (4 h) (4 l) (4 *hl) (4 a)
    (5 b) (5 c) (5 d) (5 e) (5 h) (5 l) (5 *hl) (5 a)
    (6 b) (6 c) (6 d) (6 e) (6 h) (6 l) (6 *hl) (6 a)
    (7 b) (7 c) (7 d) (7 e) (7 h) (7 l) (7 *hl) (7 a)
}

#[cfg(test)]
mod tests {
    use crate::gb::mmu::MMU;
    use crate::step_test;

    use super::*;

    const MEM_ADD: u16 = 0xDFE0;

    macro_rules! bit_tests {
        (@inner $code:literal $bit:literal ? $target:ident) => {
            paste::paste! {
                #[test]
                fn [<bit_on_ $bit _ $target>]() {
                    let ctx = &mut GameBoy::new();
                    step_test! {
                        ctx: ctx;

                        code: 0xCB $code, length: 2, cycles: 2
                        setup {
                            ctx.cpu.$target = 1 << $bit;
                        }
                        after {
                            assert!(!ctx.cpu.f.z);
                            assert!(!ctx.cpu.f.n);
                            assert!( ctx.cpu.f.h);
                        }
                    }
                }

                #[test]
                fn [<bit_off_ $bit _ $target>]() {
                    let ctx = &mut GameBoy::new();
                    step_test! {
                        ctx: ctx;

                        code: 0xCB $code, length: 2, cycles: 2
                        setup {
                            ctx.cpu.$target = !(1 << $bit);
                        }
                        after {
                            assert!( ctx.cpu.f.z);
                            assert!(!ctx.cpu.f.n);
                            assert!( ctx.cpu.f.h);
                        }
                    }
                }
            }
        };

        (@inner $code:literal $bit:literal ? *hl) => {
            paste::paste! {
                #[test]
                fn [<bit_on_ $bit _mhl>]() {
                    let ctx = &mut GameBoy::new();
                    step_test! {
                        ctx: ctx;

                        code: 0xCB $code, length: 2, cycles: 3
                        setup {
                            MMU::write(ctx, MEM_ADD, 1 << $bit);
                            ctx.cpu.set_hl(MEM_ADD);
                        }
                        after {
                            assert!(!ctx.cpu.f.z);
                            assert!(!ctx.cpu.f.n);
                            assert!( ctx.cpu.f.h);
                        }
                    }
                }

                #[test]
                fn [<bit_off_ $bit _mhl>]() {
                    let ctx = &mut GameBoy::new();
                    step_test! {
                        ctx: ctx;

                        code: 0xCB $code, length: 2, cycles: 3
                        setup {
                            MMU::write(ctx, MEM_ADD, !(1 << $bit));
                            ctx.cpu.set_hl(MEM_ADD);
                        }
                        after {
                            assert!( ctx.cpu.f.z);
                            assert!(!ctx.cpu.f.n);
                            assert!( ctx.cpu.f.h);
                        }
                    }
                }
            }
        };

        (@inner $code:literal $bit:literal r $target:ident) => {
            paste::paste! {
                #[test]
                fn [<res_off_ $bit _ $target>]() {
                    let ctx = &mut GameBoy::new();
                    step_test! {
                        ctx: ctx;

                        code: 0xCB $code, length: 2, cycles: 2
                        setup {
                            ctx.cpu.$target = 1 << $bit;
                        }
                        after {
                            assert_eq!(ctx.cpu.$target, 0);
                        }
                    }
                }

                #[test]
                fn [<res_nop_ $bit _ $target>]() {
                    let ctx = &mut GameBoy::new();
                    step_test! {
                        ctx: ctx;

                        code: 0xCB $code, length: 2, cycles: 2
                        setup {
                            ctx.cpu.$target = !(1 << $bit);
                        }
                        after {
                            assert_eq!(ctx.cpu.$target, !(1 << $bit));
                        }
                    }
                }
            }
        };

        (@inner $code:literal $bit:literal r *hl) => {
            paste::paste! {
                #[test]
                fn [<res_off_ $bit _mhl>]() {
                    let ctx = &mut GameBoy::new();
                    step_test! {
                        ctx: ctx;

                        code: 0xCB $code, length: 2, cycles: 4
                        setup {
                            MMU::write(ctx, MEM_ADD, 1 << $bit);
                            ctx.cpu.set_hl(MEM_ADD);
                        }
                        after {
                            assert_eq!(MMU::read(ctx, MEM_ADD), 0);
                        }
                    }
                }

                #[test]
                fn [<res_nop_ $bit _mhl>]() {
                    let ctx = &mut GameBoy::new();
                    step_test! {
                        ctx: ctx;

                        code: 0xCB $code, length: 2, cycles: 4
                        setup {
                            MMU::write(ctx, MEM_ADD, !(1 << $bit));
                            ctx.cpu.set_hl(MEM_ADD);
                        }
                        after {
                            assert_eq!(MMU::read(ctx, MEM_ADD), !(1 << $bit));
                        }
                    }
                }
            }
        };

        (@inner $code:literal $bit:literal s $target:ident) => {
            paste::paste! {
                #[test]
                fn [<set_on_ $bit _ $target>]() {
                    let ctx = &mut GameBoy::new();
                    step_test! {
                        ctx: ctx;

                        code: 0xCB $code, length: 2, cycles: 2
                        setup {
                            ctx.cpu.$target = !(1 << $bit);
                        }
                        after {
                            assert_eq!(ctx.cpu.$target, 0xFF);
                        }
                    }
                }

                #[test]
                fn [<set_nop_ $bit _ $target>]() {
                    let ctx = &mut GameBoy::new();
                    step_test! {
                        ctx: ctx;

                        code: 0xCB $code, length: 2, cycles: 2
                        setup {
                            ctx.cpu.$target = 1 << $bit;
                        }
                        after {
                            assert_eq!(ctx.cpu.$target, 1 << $bit);
                        }
                    }
                }
            }
        };

        (@inner $code:literal $bit:literal s *hl) => {
            paste::paste! {
                #[test]
                fn [<set_on_ $bit _mhl>]() {
                    let ctx = &mut GameBoy::new();
                    step_test! {
                        ctx: ctx;

                        code: 0xCB $code, length: 2, cycles: 4
                        setup {
                            MMU::write(ctx, MEM_ADD, !(1 << $bit));
                            ctx.cpu.set_hl(MEM_ADD);
                        }
                        after {
                            assert_eq!(MMU::read(ctx, MEM_ADD), 0xFF);
                        }
                    }
                }

                #[test]
                fn [<set_nop_ $bit _mhl>]() {
                    let ctx = &mut GameBoy::new();
                    step_test! {
                        ctx: ctx;

                        code: 0xCB $code, length: 2, cycles: 4
                        setup {
                            MMU::write(ctx, MEM_ADD, 1 << $bit);
                            ctx.cpu.set_hl(MEM_ADD);
                        }
                        after {
                            assert_eq!(MMU::read(ctx, MEM_ADD), 1 << $bit);
                        }
                    }
                }
            }
        };

        ($(($($arg:tt)*))*) => {
            $(
                bit_tests!(@inner $($arg)*);
            )*
        };
    }

    bit_tests! {
        (0x40 0 ? b) (0x41 0 ? c) (0x42 0 ? d) (0x43 0 ? e) (0x44 0 ? h) (0x45 0 ? l) (0x46 0 ? *hl) (0x47 0 ? a)
        (0x48 1 ? b) (0x49 1 ? c) (0x4A 1 ? d) (0x4B 1 ? e) (0x4C 1 ? h) (0x4D 1 ? l) (0x4E 1 ? *hl) (0x4F 1 ? a)
        (0x50 2 ? b) (0x51 2 ? c) (0x52 2 ? d) (0x53 2 ? e) (0x54 2 ? h) (0x55 2 ? l) (0x56 2 ? *hl) (0x57 2 ? a)
        (0x58 3 ? b) (0x59 3 ? c) (0x5A 3 ? d) (0x5B 3 ? e) (0x5C 3 ? h) (0x5D 3 ? l) (0x5E 3 ? *hl) (0x5F 3 ? a)
        (0x60 4 ? b) (0x61 4 ? c) (0x62 4 ? d) (0x63 4 ? e) (0x64 4 ? h) (0x65 4 ? l) (0x66 4 ? *hl) (0x67 4 ? a)
        (0x68 5 ? b) (0x69 5 ? c) (0x6A 5 ? d) (0x6B 5 ? e) (0x6C 5 ? h) (0x6D 5 ? l) (0x6E 5 ? *hl) (0x6F 5 ? a)
        (0x70 6 ? b) (0x71 6 ? c) (0x72 6 ? d) (0x73 6 ? e) (0x74 6 ? h) (0x75 6 ? l) (0x76 6 ? *hl) (0x77 6 ? a)
        (0x78 7 ? b) (0x79 7 ? c) (0x7A 7 ? d) (0x7B 7 ? e) (0x7C 7 ? h) (0x7D 7 ? l) (0x7E 7 ? *hl) (0x7F 7 ? a)
        (0x80 0 r b) (0x81 0 r c) (0x82 0 r d) (0x83 0 r e) (0x84 0 r h) (0x85 0 r l) (0x86 0 r *hl) (0x87 0 r a)
        (0x88 1 r b) (0x89 1 r c) (0x8A 1 r d) (0x8B 1 r e) (0x8C 1 r h) (0x8D 1 r l) (0x8E 1 r *hl) (0x8F 1 r a)
        (0x90 2 r b) (0x91 2 r c) (0x92 2 r d) (0x93 2 r e) (0x94 2 r h) (0x95 2 r l) (0x96 2 r *hl) (0x97 2 r a)
        (0x98 3 r b) (0x99 3 r c) (0x9A 3 r d) (0x9B 3 r e) (0x9C 3 r h) (0x9D 3 r l) (0x9E 3 r *hl) (0x9F 3 r a)
        (0xA0 4 r b) (0xA1 4 r c) (0xA2 4 r d) (0xA3 4 r e) (0xA4 4 r h) (0xA5 4 r l) (0xA6 4 r *hl) (0xA7 4 r a)
        (0xA8 5 r b) (0xA9 5 r c) (0xAA 5 r d) (0xAB 5 r e) (0xAC 5 r h) (0xAD 5 r l) (0xAE 5 r *hl) (0xAF 5 r a)
        (0xB0 6 r b) (0xB1 6 r c) (0xB2 6 r d) (0xB3 6 r e) (0xB4 6 r h) (0xB5 6 r l) (0xB6 6 r *hl) (0xB7 6 r a)
        (0xB8 7 r b) (0xB9 7 r c) (0xBA 7 r d) (0xBB 7 r e) (0xBC 7 r h) (0xBD 7 r l) (0xBE 7 r *hl) (0xBF 7 r a)
        (0xC0 0 s b) (0xC1 0 s c) (0xC2 0 s d) (0xC3 0 s e) (0xC4 0 s h) (0xC5 0 s l) (0xC6 0 s *hl) (0xC7 0 s a)
        (0xC8 1 s b) (0xC9 1 s c) (0xCA 1 s d) (0xCB 1 s e) (0xCC 1 s h) (0xCD 1 s l) (0xCE 1 s *hl) (0xCF 1 s a)
        (0xD0 2 s b) (0xD1 2 s c) (0xD2 2 s d) (0xD3 2 s e) (0xD4 2 s h) (0xD5 2 s l) (0xD6 2 s *hl) (0xD7 2 s a)
        (0xD8 3 s b) (0xD9 3 s c) (0xDA 3 s d) (0xDB 3 s e) (0xDC 3 s h) (0xDD 3 s l) (0xDE 3 s *hl) (0xDF 3 s a)
        (0xE0 4 s b) (0xE1 4 s c) (0xE2 4 s d) (0xE3 4 s e) (0xE4 4 s h) (0xE5 4 s l) (0xE6 4 s *hl) (0xE7 4 s a)
        (0xE8 5 s b) (0xE9 5 s c) (0xEA 5 s d) (0xEB 5 s e) (0xEC 5 s h) (0xED 5 s l) (0xEE 5 s *hl) (0xEF 5 s a)
        (0xF0 6 s b) (0xF1 6 s c) (0xF2 6 s d) (0xF3 6 s e) (0xF4 6 s h) (0xF5 6 s l) (0xF6 6 s *hl) (0xF7 6 s a)
        (0xF8 7 s b) (0xF9 7 s c) (0xFA 7 s d) (0xFB 7 s e) (0xFC 7 s h) (0xFD 7 s l) (0xFE 7 s *hl) (0xFF 7 s a)
    }
}
