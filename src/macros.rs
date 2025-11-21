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
        std::format!("{:#04X}", $n)
    };
}
pub(crate) use byte_fmt;

macro_rules! address_fmt {
    ($n:expr) => {
        std::format!("{:#06X}", $n)
    };
}
pub(crate) use address_fmt;

macro_rules! either {
    ($cond:expr, $a:expr, $b:expr) => {
        if $cond {
            $a
        } else {
            $b
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
