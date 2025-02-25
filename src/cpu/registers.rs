const ZERO_FLAG_MASK: u8        = 0b_1000_0000;
const SUBTRACT_FLAG_MASK: u8    = 0b_0100_0000;
const HALF_CARRY_FLAG_MASK: u8  = 0b_0010_0000;
const CARRY_FLAG_MASK: u8       = 0b_0001_0000;

#[derive(Default, Debug)]
pub struct Registers {
    pub a: u8,
    pub f: u8,

    pub b: u8,
    pub c: u8,

    pub d: u8,
    pub e: u8,

    pub h: u8,
    pub l: u8,
}

macro_rules! make_r16_getset {
    ($get_name:ident, $set_name:ident, $r1:ident, $r2:ident) => {
        pub fn $get_name(&self) -> u16 {
            ((self.$r1 as u16) << 8) | (self.$r2 as u16)
        }

        pub fn $set_name(&mut self, value: u16) {
            self.$r1 = ((value & 0xFF00) >> 8) as u8;
            self.$r2 = (value & 0x00FF) as u8;
        }
    };
}

macro_rules! make_flag_getset {
    ($get_name:ident, $set_name:ident, $mask:expr) => {
        pub fn $get_name(&self) -> bool {
            self.f & $mask != 0
        }

        pub fn $set_name(&mut self, value: bool) {
            if value {
                self.f |= $mask;
            } else {
                self.f &= !$mask
            }
        }
    };
}

impl Registers {
    // Getters and setters for the combined "16 bit" registers
    make_r16_getset!(get_af, set_af, a, f);
    make_r16_getset!(get_bc, set_bc, b, c);
    make_r16_getset!(get_de, set_de, d, e);
    make_r16_getset!(get_hl, set_hl, h, l);

    // Getters and setters for the flags stored in the F register
    make_flag_getset!(getf_zero, setf_zero, ZERO_FLAG_MASK);
    make_flag_getset!(getf_subtract, setf_subtract, SUBTRACT_FLAG_MASK);
    make_flag_getset!(getf_half_carry, setf_half_carry, HALF_CARRY_FLAG_MASK);
    make_flag_getset!(getf_carry, setf_carry, CARRY_FLAG_MASK);
}

// TESTS

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test_r16_getset {
        ($test_get_name:ident, $get_func:ident, $test_set_name:ident, $set_func:ident, $r1:ident, $r2:ident) => {
            #[test]
            fn $test_get_name() {
                let mut regs = Registers::default();
                regs.$r1 = 0xDE;
                regs.$r2 = 0xAD;
                assert_eq!(regs.$get_func(), 0xDEAD);
            }

            #[test]
            fn $test_set_name() {
                let mut regs = Registers::default();
                regs.$set_func(0xDEAD);
                assert_eq!(regs.$r1, 0xDE);
                assert_eq!(regs.$r2, 0xAD);
            }
        };
    }

    macro_rules! test_flag_getset {
        ($test_get_name:ident, $get_func:ident, $test_set_name:ident, $set_func:ident, $mask:expr) => {
            #[test]
            fn $test_get_name() {
                let mut regs = Registers::default();
                regs.f = $mask;
                assert!(regs.$get_func());
                regs.f = !$mask;
                assert!(!regs.$get_func())
            }

            #[test]
            fn $test_set_name() {
                let mut regs = Registers::default();
                regs.f = !$mask;
                regs.$set_func(true);
                assert_eq!(regs.f, 0xFF);
                regs.f = $mask;
                regs.$set_func(false);
                assert_eq!(regs.f, 0);
            }
        };
    }

    test_r16_getset!(test_get_af, get_af, test_set_af, set_af, a, f);
    test_r16_getset!(test_get_bc, get_bc, test_set_bc, set_bc, b, c);
    test_r16_getset!(test_get_de, get_de, test_set_de, set_de, d, e);
    test_r16_getset!(test_get_hl, get_hl, test_set_hl, set_hl, h, l);

    test_flag_getset!(test_getf_zero, getf_zero, test_setf_zero, setf_zero, ZERO_FLAG_MASK);
    test_flag_getset!(test_getf_subtract, getf_subtract, test_setf_subtract, setf_subtract, SUBTRACT_FLAG_MASK);
    test_flag_getset!(test_getf_half_carry, getf_half_carry, test_setf_half_carry, setf_half_carry, HALF_CARRY_FLAG_MASK);
    test_flag_getset!(test_getf_carry, getf_carry, test_setf_carry, setf_carry, CARRY_FLAG_MASK);
}
