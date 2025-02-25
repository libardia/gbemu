// ============================================================================
// Registers
// ============================================================================
pub struct Registers {
    a: u8,
    f: u8,
    
    b: u8,
    c: u8,
    
    d: u8,
    e: u8,
    
    h: u8,
    l: u8,
}

macro_rules! make_r16_getset {
    ($get_name:ident, $set_name:ident, $r1:ident, $r2:ident) => {
        pub fn $get_name(&self) -> u16 {
            ((self.$r1 as u16) << 8) | (self.$r2 as u16)
        }

        pub fn $set_name(&mut self, value: u16) {
            self.$r1 = ((value & 0xFF00) >> 8) as u8;
            self.$r1 = (value & 0x00FF) as u8;
        }
    };
}

impl Registers {
    make_r16_getset!(get_af, set_af, a, f);
    make_r16_getset!(get_bc, set_bc, b, c);
    make_r16_getset!(get_de, set_de, d, e);
    make_r16_getset!(get_hl, set_hl, h, l);
}

// ============================================================================
// Flag register
// ============================================================================
const ZERO_FLAG_BYTE_POSITION: u8 = 7;
const SUBTRACT_FLAG_BYTE_POSITION: u8 = 6;
const HALF_CARRY_FLAG_BYTE_POSITION: u8 = 5;
const CARRY_FLAG_BYTE_POSITION: u8 = 4;

pub struct FlagsRegister {
    zero: bool,
    subtract: bool,
    half_carry: bool,
    carry: bool,
}

impl std::convert::From<FlagsRegister> for u8 {
    fn from(flag: FlagsRegister) -> u8 {
        (if flag.zero       { 1 } else { 0 }) << ZERO_FLAG_BYTE_POSITION |
        (if flag.subtract   { 1 } else { 0 }) << SUBTRACT_FLAG_BYTE_POSITION |
        (if flag.half_carry { 1 } else { 0 }) << HALF_CARRY_FLAG_BYTE_POSITION |
        (if flag.carry      { 1 } else { 0 }) << CARRY_FLAG_BYTE_POSITION
    }
}

impl std::convert::From<u8> for FlagsRegister {
    fn from(byte: u8) -> Self {
        let zero        = ((byte >> ZERO_FLAG_BYTE_POSITION) & 1)       != 0;
        let subtract    = ((byte >> SUBTRACT_FLAG_BYTE_POSITION) & 1)   != 0;
        let half_carry  = ((byte >> HALF_CARRY_FLAG_BYTE_POSITION) & 1) != 0;
        let carry       = ((byte >> CARRY_FLAG_BYTE_POSITION) & 1)      != 0;

        FlagsRegister {
            zero,
            subtract,
            half_carry,
            carry,
        }
    }
}
