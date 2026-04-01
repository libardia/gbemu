pub mod instructions;

#[derive(Default)]
pub struct CPU {
    // Registers
    pub b: u8,
    pub c: u8,

    pub d: u8,
    pub e: u8,

    pub h: u8,
    pub l: u8,

    pub a: u8,
    pub f: u8,
}

macro_rules! r16 {
    ($r1:ident + $r2:ident) => {
        paste::paste! {
            pub fn [<get_ $r1 $r2>](&self) -> u16 {
                (self.$r1 as u16) << 8 | self.$r2 as u16
            }

            pub fn [<set_ $r1 $r2>](&mut self, value: u16) {
                self.$r1 = (value >> 8) as u8;
                self.$r2 = (value & 0xFF) as u8
            }
        }
    };
}

macro_rules! flag {
    ($f:ident, $bit:literal) => {
        paste::paste! {
            const [<FLAG_ $f:upper _MASK>]: u8 = 1 << $bit;

            pub fn [<get_flag_ $f>](&self) -> bool {
                self.f & Self::[<FLAG_ $f:upper _MASK>] != 0
            }

            pub fn [<set_flag_ $f>](&mut self, value: bool) {
                if value {
                    self.f |= Self::[<FLAG_ $f:upper _MASK>];
                } else {
                    self.f &= !Self::[<FLAG_ $f:upper _MASK>];
                }
            }
        }
    };
}

impl CPU {
    pub fn new() -> Self {
        let cpu = Default::default();
        // TODO: init
        cpu
    }

    r16!(b + c);
    r16!(d + e);
    r16!(h + l);
    r16!(a + f);

    flag!(z, 7);
    flag!(n, 6);
    flag!(h, 5);
    flag!(c, 4);
}

#[cfg(test)]
mod tests {
    use crate::gb::cpu::CPU;

    macro_rules! r16_test {
        ($r1:ident + $r2:ident) => {
            paste::paste! {
                #[test]
                fn [<r16_ $r1 $r2 _test>]() {
                    let mut cpu = CPU::default();

                    assert_eq!(cpu.$r1, 0);
                    assert_eq!(cpu.$r2, 0);
                    assert_eq!(cpu.[<get_ $r1 $r2>](), 0);

                    cpu.[<set_ $r1 $r2>](0xDEAD);
                    assert_eq!(cpu.$r1, 0xDE);
                    assert_eq!(cpu.$r2, 0xAD);
                    assert_eq!(cpu.[<get_ $r1 $r2>](), 0xDEAD);

                    cpu.$r1 = 0xBE;
                    cpu.$r2 = 0xEF;
                    assert_eq!(cpu.[<get_ $r1 $r2>](), 0xBEEF);
                }
            }
        };
    }

    r16_test!(b + c);
    r16_test!(d + e);
    r16_test!(h + l);
    r16_test!(a + f);
}
