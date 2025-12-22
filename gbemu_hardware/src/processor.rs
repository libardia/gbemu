const Z_FLAG_MASK: u8 = 0x80;
const N_FLAG_MASK: u8 = 0x40;
const H_FLAG_MASK: u8 = 0x20;
const C_FLAG_MASK: u8 = 0x10;

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
}

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

#[derive(Default, Debug)]
pub struct Processor {
    r: Regs,
    f: Flags,

    ime: bool,
}

impl Processor {
    //TODO: CPU
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
