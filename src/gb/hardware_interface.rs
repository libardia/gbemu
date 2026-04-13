pub trait HardwareInterface {
    fn read(&mut self, address: u16) -> u8;
    fn write(&mut self, address: u16, byte: u8);
}

macro_rules! warn_todo_read {
    ($src:expr, $address:expr) => {{
        log::warn!(
            "{}: read from {}: read not yet implemented, returning 0xFF",
            $src,
            crate::macros::hex!($address, 4),
        );
        0xFF
    }};
}
pub(crate) use warn_todo_read;

macro_rules! warn_todo_write {
    ($src:expr, $address:expr, $byte:expr) => {
        log::warn!(
            "{}: write {} to {}: write not yet implemented, ignoring",
            $src,
            crate::macros::hex!($byte, 2),
            crate::macros::hex!($address, 4),
        )
    };
}
pub(crate) use warn_todo_write;
