#[derive(Debug, Default, PartialEq, Eq)]
struct Regs {
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    h: u8,
    l: u8,
    a: u8,
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

#[derive(Debug, Default, PartialEq, Eq)]
struct Flags {
    z: bool,
    n: bool,
    h: bool,
    c: bool,
}

impl Flags {
    fn to_byte(&self) -> u8 {
        (self.z as u8) << 7 | (self.n as u8) << 6 | (self.h as u8) << 5 | (self.c as u8) << 4
    }

    fn from_byte(&mut self, value: u8) {
        self.z = value & 0x80 != 0;
        self.n = value & 0x40 != 0;
        self.h = value & 0x20 != 0;
        self.c = value & 0x10 != 0;
    }
}

#[derive(Debug, Default, PartialEq, Eq)]
pub struct CPU {
    r: Regs,
    f: Flags,

    ime: bool,
}

impl CPU {}

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
                    let rs_g = Regs { $r1: 0xDE, $r2: 0xAD, ..Default::default() };
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
        for i in 0..=0xF {
            let byte = i << 4;
            let expected = Flags {
                z: i & 0x8 != 0,
                n: i & 0x4 != 0,
                h: i & 0x2 != 0,
                c: i & 0x1 != 0,
            };

            let mut fs = Flags::default();
            fs.from_byte(byte);
            debug!("{byte:0>8b} => {expected:>5?}");
            assert_eq!(fs, expected);
        }
    }

    #[test]
    fn test_flags_to_byte() {
        for i in 0..=0xF {
            let fs = Flags {
                z: i & 0x8 != 0,
                n: i & 0x4 != 0,
                h: i & 0x2 != 0,
                c: i & 0x1 != 0,
            };
            let expected = i << 4;

            debug!("{fs:>5?} => {expected:0>8b}");
            assert_eq!(fs.to_byte(), expected);
        }
    }
    /* #endregion */
}
