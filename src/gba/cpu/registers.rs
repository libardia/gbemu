pub mod flag_masks {
    pub const F_ZERO: u8 = 0b1000_0000;
    pub const F_SUB: u8 = 0b0100_0000;
    pub const F_HCARRY: u8 = 0b0010_0000;
    pub const F_CARRY: u8 = 0b0001_0000;
}

use flag_masks::*;

#[derive(Debug)]
pub struct Registers {
    pub a: u8,
    pub f: u8,

    pub b: u8,
    pub c: u8,

    pub d: u8,
    pub e: u8,

    pub h: u8,
    pub l: u8,

    pub ime: bool,
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
    pub fn new() -> Self {
        Registers {
            a: 0,
            f: 0,
            b: 0,
            c: 0,
            d: 0,
            e: 0,
            h: 0,
            l: 0,
            ime: false,
        }
    }

    // Getters and setters for the combined "16 bit" registers
    // NOTE: The bytes in the two component registers are in BIG-ENDIAN order. This is very
    // important to note because the memory, and the order two-byte instruction arguments are read,
    // are in LITTLE-ENDIAN order.
    make_r16_getset!(get_af, set_af, a, f);
    make_r16_getset!(get_bc, set_bc, b, c);
    make_r16_getset!(get_de, set_de, d, e);
    make_r16_getset!(get_hl, set_hl, h, l);

    // Getters and setters for the flags stored in the F register
    make_flag_getset!(getf_zero, setf_zero, F_ZERO);
    make_flag_getset!(getf_subtract, setf_subtract, F_SUB);
    make_flag_getset!(getf_half_carry, setf_half_carry, F_HCARRY);
    make_flag_getset!(getf_carry, setf_carry, F_CARRY);

    pub fn set_all_flags(&mut self, zero: bool, subtract: bool, half_carry: bool, carry: bool) {
        self.f = if zero { F_ZERO } else { 0 }
            | if subtract { F_SUB } else { 0 }
            | if half_carry { F_HCARRY } else { 0 }
            | if carry { F_CARRY } else { 0 };
    }
}
