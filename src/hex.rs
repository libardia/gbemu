use std::fmt::Debug;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct HexU8(pub u8);

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct HexU16(pub u16);

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct HexI8(pub i8);

macro_rules! implements_for_number_type {
    ($type:ident, $internal_type:ident, $key:expr, $padding:expr) => {
        impl Debug for $type {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                let pad = $padding;
                write!(f, "0{}x{:0>pad$X}", $key, self.0)
            }
        }

        impl From<$internal_type> for $type {
            fn from(value: $internal_type) -> Self {
                $type(value)
            }
        }
    };
}

implements_for_number_type!(HexU8, u8, "", 2);
implements_for_number_type!(HexU16, u16, "", 4);
implements_for_number_type!(HexI8, i8, "i", 2);
