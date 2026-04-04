#[macro_export]
macro_rules! hex {
    ($byte:expr, $width:literal) => {
        format_args!(concat!("${:>0", $width, "X}"), $byte)
    };
}
