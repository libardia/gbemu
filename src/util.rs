use std::io::{stdin, stdout, Write};

/* #region Hex types =========================================================================== */

make_number_type!(Hex8, u8);
impls_debug_as_hex!(Hex8, 2);
pub const HEX8_ZERO: Hex8 = Hex8(0);

make_number_type!(Hex16, u16);
impls_debug_as_hex!(Hex16, 4);
pub const HEX16_ZERO: Hex16 = Hex16(0);

macro_rules! impls_debug_as_hex {
    ($type:ty, $padding:expr) => {
        impl std::fmt::Debug for $type {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                let pad = $padding;
                write!(f, "0x{:0>pad$X}", self.0)
            }
        }
    };
}

pub(crate) use impls_debug_as_hex;

macro_rules! impls_debug_as_internal {
    ($type:ty) => {
        impl std::fmt::Debug for $type {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{:?}", self.0)
            }
        }
    };
}
pub(crate) use impls_debug_as_internal;

macro_rules! make_number_type {
    ($type:ident, $internal_type:ty) => {
        #[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
        pub struct $type($internal_type);

        impl std::ops::Add for $type {
            type Output = $type;

            fn add(self, rhs: Self) -> Self::Output {
                Self(self.0 + rhs.0)
            }
        }

        impl std::ops::AddAssign for $type {
            fn add_assign(&mut self, rhs: Self) {
                self.0 += rhs.0;
            }
        }

        impl std::ops::Sub for $type {
            type Output = $type;

            fn sub(self, rhs: Self) -> Self::Output {
                Self(self.0 - rhs.0)
            }
        }

        impl std::ops::SubAssign for $type {
            fn sub_assign(&mut self, rhs: Self) {
                self.0 -= rhs.0;
            }
        }

        impl std::ops::Rem for $type {
            type Output = $type;

            fn rem(self, rhs: Self) -> Self::Output {
                Self(self.0 % rhs.0)
            }
        }

        impl std::ops::RemAssign for $type {
            fn rem_assign(&mut self, rhs: Self) {
                self.0 %= rhs.0;
            }
        }

        #[allow(dead_code)]
        impl $type {
            pub const fn make(value: $internal_type) -> Self {
                Self(value)
            }

            pub const fn to(&self) -> $internal_type {
                self.0
            }
        }

        impl From<$internal_type> for $type {
            fn from(value: $internal_type) -> Self {
                Self(value)
            }
        }

        impl Into<$internal_type> for $type {
            fn into(self) -> $internal_type {
                self.0
            }
        }
    };
}
pub(crate) use make_number_type;

/* #endregion */

/* #region Util macros & functions ============================================================= */

pub fn input(prompt: &str) -> String {
    let mut input = String::new();
    print!("{prompt}");
    stdout().flush().ok();
    stdin().read_line(&mut input).ok();
    println!();
    input
}

macro_rules! error_and_panic {
    ($($format_arg:expr),+) => {
        let msg = format!($($format_arg),+);
        log::error!("{msg}");
        panic!("{msg}");
    };
}
pub(crate) use error_and_panic;

macro_rules! either {
    ($condition:expr => $when_true:expr; $when_false:expr) => {
        if $condition {
            $when_true
        } else {
            $when_false
        }
    };
}
pub(crate) use either;

macro_rules! byte_of {
    ($value:expr, $byte_index:expr) => {{
        let shift = $byte_index * 8;
        let mask = 0xFF << shift;
        (($value & mask) >> shift) as u8
    }};
}
pub(crate) use byte_of;

macro_rules! min {
    ($($item:expr),+) => {{
        let items = [$($item),+];

        let mut smallest = items[0];
        for i in items {
            if i < smallest {
                smallest = i;
            }
        }

        smallest
    }};
}
pub(crate) use min;

macro_rules! new {
    () => {
        pub fn new() -> Self { Self::default() }
    };
    ($body:block) => {
        pub fn new() -> Self $body
    };
    ($($arg:ident: $at:ty),+; $body:block) => {
        pub fn new($($arg: $at),+) -> Self $body
    };
    ($($arg:ident: $at:ty),+) => {
        pub fn new($($arg: $at),+) -> Self {
            Self { $($arg),+, ..Self::default() }
        }
    };
}
pub(crate) use new;

macro_rules! bit_flag {
    (get => $name:ident, $byte:ident, $bit_position:expr) => {
        pub fn $name(&self) -> bool {
            const MASK: u8 = 1 << $bit_position;
            self.$byte & MASK != 0
        }
    };
    (set => $name:ident, $byte:ident, $bit_position:expr) => {
        pub fn $name(&mut self, value: bool) {
            const MASK: u8 = 1 << $bit_position;
            const INV_MASK: u8 = !MASK;
            if value {
                self.$byte |= MASK;
            } else {
                self.$byte &= INV_MASK;
            }
        }
    };
    ($getname:ident, $setname:ident, $byte:ident, $bit_position:expr) => {
        bit_flag!(get => $getname, $byte, $bit_position);
        bit_flag!(set => $setname, $byte, $bit_position);
    };
}
pub(crate) use bit_flag;

/* #endregion */
