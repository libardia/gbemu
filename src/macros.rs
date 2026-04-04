#[macro_export]
macro_rules! hex {
    ($byte:expr, $width:literal) => {{ format!(concat!("${:>0", $width, "X}"), $byte) }};
}
