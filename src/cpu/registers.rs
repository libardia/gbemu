pub const ZERO_FLAG_MASK: u8        = 0b1000_0000;
pub const SUBTRACT_FLAG_MASK: u8    = 0b0100_0000;
pub const HALF_CARRY_FLAG_MASK: u8  = 0b0010_0000;
pub const CARRY_FLAG_MASK: u8       = 0b0001_0000;

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

    pub fn calculate_flags(&mut self, added_value: u8, new_value: u8, did_overflow: bool, was_subtraction: bool) {
        self.set_all_flags(
            // Set if the result of the operation was zero
            new_value == 0,
            // Set if the operation was a subtraction
            was_subtraction,
            // Set if adding the lower nibbles of the value and register A together result in a
            // value bigger than 0xF. If the result is larger than 0xF than the addition caused a
            // carry from the lower nibble to the upper nibble.
            (self.a & 0xF) + (added_value & 0xF) > 0xF,
            // Set if the operation fully overflowed a u8
            did_overflow
        );
    }

    pub fn set_all_flags(&mut self, zero: bool, subtract: bool, half_carry: bool, carry: bool) {
        self.f = if zero        { ZERO_FLAG_MASK        } else { 0 }
               | if subtract    { SUBTRACT_FLAG_MASK    } else { 0 }
               | if half_carry  { HALF_CARRY_FLAG_MASK  } else { 0 }
               | if carry       { CARRY_FLAG_MASK       } else { 0 };
    }

    pub fn clear_flags(&mut self) {
        self.f = 0;
    }
}

// Tests ==========================================================================================
#[cfg(test)]
mod tests;
