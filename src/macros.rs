macro_rules! select {
    ($cond:expr; $true:expr, $false:expr) => {
        if $cond { $true } else { $false }
    };
}
pub(crate) use select;

macro_rules! hex {
    ($byte:expr, $width:literal) => {
        format_args!(concat!("${:>0", $width, "X}"), $byte)
    };
}
pub(crate) use hex;

macro_rules! get_masked {
    ($value:expr; $mask:expr) => {
        ((($value) & ($mask)) | !($mask))
    };
}
pub(crate) use get_masked;

macro_rules! set_masked {
    ($target:expr, $value:expr; $mask:expr) => {
        ((($target) & !($mask)) | (($value) & ($mask)))
    };
}
pub(crate) use set_masked;
