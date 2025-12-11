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

#[derive(Debug, Default)]
pub struct CPU {
    r: Regs,
    f: Flags,

    ime: bool,
}

impl CPU {}

/* #region Tests */
#[cfg(test)]
mod tests {
    use super::*;
    use log::debug;
    use test_log::test;

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
/* #endregion */
