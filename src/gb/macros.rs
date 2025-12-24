macro_rules! new {
    () => {
        pub fn new() -> Self {
            Self::default()
        }
    };
    ( $( -> $arg:ident: $argt:ty; )* $( $field:ident = $value:expr; )* ) => {
        pub fn new($($arg: $argt),*) -> Self {
            Self {
                $($arg,)*
                $($field: $value,)*
            }
        }
    };
    ( $( -> $arg:ident: $argt:ty; )* $( $field:ident = $value:expr; )* ... ) => {
        pub fn new($($arg: $argt),*) -> Self {
            Self {
                $($arg,)*
                $($field: $value,)*
                ..Self::default()
            }
        }
    };
}
pub(crate) use new;

macro_rules! error_panic {
    ($($arg:tt)+) => ({
        log::error!($($arg)+);
        panic!($($arg)+)
    })
}
pub(crate) use error_panic;

macro_rules! byte_fmt {
    ($n:expr) => {
        std::format!("${:02X}", $n)
    };
}
pub(crate) use byte_fmt;

macro_rules! address_fmt {
    ($n:expr) => {
        std::format!("${:04X}", $n)
    };
}
pub(crate) use address_fmt;

macro_rules! byte_of {
    ($value:expr, $byte_index:expr) => {{
        let shift = $byte_index * 8;
        let mask = 0xFF << shift;
        (($value & mask) >> shift) as u8
    }};
}
pub(crate) use byte_of;

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
