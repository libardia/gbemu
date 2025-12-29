#[macro_export]
macro_rules! wrapping_add_warn {
    ($orig:expr, $add:expr, $($arg:tt)*) => {{
        let (result, overflow) = $orig.overflowing_add($add);
        if overflow {
            log::warn!($($arg)*);
        }
        result
    }};
}

#[macro_export]
macro_rules! wrapping_sub_warn {
    ($orig:expr, $sub:expr, $($arg:tt)*) => {{
        let (result, overflow) = $orig.overflowing_sub($sub);
        if overflow {
            log::warn!($($arg)*);
        }
        result
    }};
}

#[macro_export]
macro_rules! error_panic {
    ($($arg:tt)*) => {{
        log::error!($($arg)*);
        panic!($($arg)*);
    }};
}

#[macro_export]
macro_rules! word_fmt {
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
        ((($value) & ($mask)) | !($mask))
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
                crate::word_fmt!($region.begin),
                crate::word_fmt!($region.end),
                crate::word_fmt!($address)
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
        crate::error_panic!("[ {} > {:?} ] {}", crate::word_fmt!($ctx.cpu.this_inst_pc), $ctx.cpu.this_inst, format!($($arg)*))
    };
    ($level:ident, $ctx:expr, $($arg:tt)*) => {
        log::$level!("[ {} > {:?} ] {}", crate::word_fmt!($ctx.cpu.this_inst_pc), $ctx.cpu.this_inst, format!($($arg)*))
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
        // Get
        let n1 = 0b_1101_0011_u8; // Orig
        let m1 = 0b_1110_0110_u8; // Mask
        let r1 = 0b_1101_1011_u8; // Result

        // Set
        let n2 = 0b_1111_0000_u8; // Orig
        let o1 = 0b_0010_0100_u8; // Overlay
        let m2 = 0b_0111_0010_u8; // Mask
        let r2 = 0b_1010_0000_u8; // Result

        assert_eq!(get_bits_of!(n1, m1), r1);
        assert_eq!(set_bits_of!(n2, o1, m2), r2);
    }
}
