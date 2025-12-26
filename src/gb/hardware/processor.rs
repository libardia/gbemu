use crate::gb::{GameBoy, hardware::memory::Memory};

mod decode;
mod execute;
mod instructions;
mod optable;

const Z_FLAG_MASK: u8 = 0x80;
const N_FLAG_MASK: u8 = 0x40;
const H_FLAG_MASK: u8 = 0x20;
const C_FLAG_MASK: u8 = 0x10;

/* #region Registers */

#[derive(Debug, Default, PartialEq, Eq)]
struct Regs {
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    h: u8,
    l: u8,
    a: u8,
    f: u8,
}

macro_rules! getset_r16 {
    ($r1:ident + $r2:ident) => {
        paste::paste! {
            pub fn [<get_ $r1 $r2>](&self) -> u16 {
                (self.$r1 as u16) << 8 | self.$r2 as u16
            }

            pub fn [<set_ $r1 $r2>](&mut self, value: u16) {
                self.$r1 = ((value & 0xFF00) >> 8) as u8;
                self.$r2 = ( value & 0x00FF ) as u8;
            }
        }
    };
}

impl Regs {
    getset_r16!(b + c);
    getset_r16!(d + e);
    getset_r16!(h + l);
    getset_r16!(a + f);

    pub fn get_hli(&mut self) -> u16 {
        let before = self.get_hl();
        self.set_hl(before.wrapping_add(1));
        before
    }

    pub fn get_hld(&mut self) -> u16 {
        let before = self.get_hl();
        self.set_hl(before.wrapping_sub(1));
        before
    }

    pub fn get_flags(&self) -> Flags {
        self.f.into()
    }

    pub fn set_flags(&mut self, flags: Flags) {
        self.f = flags.into();
    }
}

/* #endregion */

/* #region Flags */

#[derive(Debug, Default, PartialEq, Eq, Clone, Copy)]
struct Flags {
    z: bool,
    n: bool,
    h: bool,
    c: bool,
}

impl From<u8> for Flags {
    fn from(value: u8) -> Self {
        Flags {
            z: value & Z_FLAG_MASK != 0,
            n: value & N_FLAG_MASK != 0,
            h: value & H_FLAG_MASK != 0,
            c: value & C_FLAG_MASK != 0,
        }
    }
}

impl Into<u8> for Flags {
    fn into(self) -> u8 {
        (if self.z { Z_FLAG_MASK } else { 0 })
            | (if self.n { N_FLAG_MASK } else { 0 })
            | (if self.h { H_FLAG_MASK } else { 0 })
            | (if self.c { C_FLAG_MASK } else { 0 })
    }
}

/* #endregion */

#[derive(Debug, Default)]
enum EIState {
    #[default]
    Idle,
    Waiting,
    Now,
}

#[derive(Debug, Default)]
enum ProcessorMode {
    #[default]
    Normal,
    Halt,
    Stop,
}

#[derive(Default, Debug)]
pub struct Processor {
    r: Regs,
    f: Flags,

    pc: u16,
    sp: u16,

    mode: ProcessorMode,
    meta_inst: bool,

    halt_bug: bool,
    ei_state: EIState,
    ime: bool,
}

impl Processor {
    pub fn step(ctx: &mut GameBoy) -> u16 {
        // TODO: CPU step
        0
    }

    // Stack
    fn push_stack(ctx: &mut GameBoy, value: u16) {
        let high = (value >> 8) as u8;
        let low = (value & 0xFF) as u8;

        ctx.cpu.sp = ctx.cpu.sp.wrapping_sub(1);
        Memory::write(ctx, ctx.cpu.sp, high);
        ctx.cpu.sp = ctx.cpu.sp.wrapping_sub(1);
        Memory::write(ctx, ctx.cpu.sp, low);
    }

    fn pop_stack(ctx: &mut GameBoy) -> u16 {
        let low = Memory::read(ctx, ctx.cpu.sp) as u16;
        ctx.cpu.sp = ctx.cpu.sp.wrapping_add(1);
        let high = Memory::read(ctx, ctx.cpu.sp) as u16;
        ctx.cpu.sp = ctx.cpu.sp.wrapping_add(1);

        (high << 8) | low
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use log::debug;
    use test_log::test;

    /* #region Regs */
    #[test]
    fn test_r16s() {
        macro_rules! test_r16 {
            ($r1:ident + $r2:ident) => {
                paste::paste! {
                    // Get
                    let rs_g = Regs { $r1: 0xDE, $r2: 0xAD, ..Regs::default() };
                    debug!("raw: {rs_g:x?}");
                    assert_eq!(rs_g.[<get_ $r1 $r2>](), 0xDEAD);

                    // Set
                    let mut rs_s = Regs::default();
                    debug!("before: {rs_s:x?}");
                    rs_s.[<set_ $r1 $r2>](0xBEEF);
                    debug!("after:  {rs_s:x?}");
                    assert_eq!(rs_s.$r1, 0xBE);
                    assert_eq!(rs_s.$r2, 0xEF);
                }
            };
        }

        test_r16!(b + c);
        test_r16!(d + e);
        test_r16!(h + l);
    }
    /* #endregion */

    /* #region Flags */
    #[test]
    fn test_byte_to_flags() {
        for byte in 0..=0xFF {
            let expected = Flags {
                z: byte & 0b1000_0000 != 0,
                n: byte & 0b0100_0000 != 0,
                h: byte & 0b0010_0000 != 0,
                c: byte & 0b0001_0000 != 0,
            };
            let fs: Flags = byte.into();

            debug!("{byte:0>8b} => {expected:>5?}");
            assert_eq!(fs, expected);
        }
    }

    #[test]
    fn test_flags_to_byte() {
        for i in 0..=0xF {
            let fs = Flags {
                z: i & 0b1000 != 0,
                n: i & 0b0100 != 0,
                h: i & 0b0010 != 0,
                c: i & 0b0001 != 0,
            };
            let fs_byte: u8 = fs.into();
            let expected: u8 = i << 4;

            debug!("{fs:>5?} => {expected:0>8b}");
            assert_eq!(fs_byte, expected);
        }
    }
    /* #endregion */
}
