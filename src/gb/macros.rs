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
