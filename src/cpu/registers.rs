const ZERO_FLAG_MASK: u8 = 0x80; // 1000 0000
const SUBTRACT_FLAG_MASK: u8 = 0x40; // 0100 0000
const HALF_CARRY_FLAG_MASK: u8 = 0x20; // 0010 0000
const CARRY_FLAG_MASK: u8 = 0x10; // 0001 0000

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
