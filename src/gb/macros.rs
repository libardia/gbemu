#[macro_export]
macro_rules! error_panic {
    ($($arg:tt)*) => {{
        log::error!($($arg)*);
        panic!($($arg)*);
    }};
}

#[macro_export]
macro_rules! address_fmt {
    ($address:expr) => {
        format!("${:0>4X}", $address)
    };
}

#[macro_export]
macro_rules! byte_fmt {
    ($byte:expr) => {
        format!("${:0>2X}", $byte)
    };
}

#[macro_export]
macro_rules! get_bits_of {
    ($value:expr, $mask:expr) => {
        (($value) & ($mask))
    };
}

#[macro_export]
macro_rules! set_bits_of {
    ($target:expr, $value:expr, $mask:expr) => {
        ((($target) & !($mask)) | (($value) & ($mask)))
    };
}

#[macro_export]
macro_rules! region_guard {
    ($address:tt in $region:ident) => {
        if !$region.contains($address) {
            crate::error_panic!(
                "Region gaurd failed! Address should be {}-{} but was {}.",
                crate::address_fmt!($region.begin),
                crate::address_fmt!($region.end),
                crate::address_fmt!($address)
            );
        }
    };
}

#[macro_export]
macro_rules! number_type {
    ($vis:vis $name:ident: $inner:ty) => {
        #[derive(Clone, Copy, PartialEq, Eq, Hash)]
        $vis struct $name(pub $inner);

        impl From<$inner> for $name {
            fn from(value: $inner) -> Self {
                Self(value)
            }
        }

        impl Into<$inner> for $name {
            fn into(self) -> $inner {
                self.0
            }
        }
    };
}

#[macro_export]
macro_rules! unwrap_or_log {
    ($thing:expr) => {
        $thing.unwrap_or_else(|e| {
            crate::error_panic!("{e}");
        })
    };
}

#[macro_export]
macro_rules! cpu_log {
    (error_panic, $ctx:expr, $($arg:tt)*) => {
        crate::error_panic!("[ {} > {:?} ] {}", crate::address_fmt!($ctx.cpu.this_inst_pc), $ctx.cpu.this_inst, format!($($arg)*))
    };
    ($level:ident, $ctx:expr, $($arg:tt)*) => {
        log::$level!("[ {} > {:?} ] {}", crate::address_fmt!($ctx.cpu.this_inst_pc), $ctx.cpu.this_inst, format!($($arg)*))
    };
}

#[cfg(test)]
mod tests {
    use test_log::test;

    number_type!(PrivTestNumberType: u8);

    #[test]
    fn number_type_cast_compiles() {
        let n: PrivTestNumberType = 0.into();
        let b: u8 = n.into();
    }

    #[test]
    fn test_getset_bits() {
        assert_eq!(get_bits_of!(0b_1110_1011, 0b_0111_0101), 0b_0110_0001);
        assert_eq!(
            set_bits_of!(0b_1110_1011, 0b_0000_0100, 0b_0000_1110),
            0b_1110_0101
        );
    }
}
