macro_rules! error_panic {
    ($($arg:tt)*) => {{
        log::error!($($arg)*);
        panic!($($arg)*);
    }};
}
pub(crate) use error_panic;

macro_rules! address_fmt {
    ($address:expr) => {
        format!("${:0>4X}", $address)
    };
}
pub(crate) use address_fmt;

macro_rules! byte_fmt {
    ($byte:expr) => {
        format!("${:0>2X}", $byte)
    };
}
pub(crate) use byte_fmt;
