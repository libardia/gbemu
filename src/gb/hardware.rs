use crate::gb::GameBoy;

pub mod audio;
pub mod cartridge;
pub mod graphics;
pub mod input;
pub mod memory;
pub mod processor;
pub mod serial;
pub mod timer;

pub trait HardwareInit {
    fn init(ctx: &mut GameBoy);
}

pub trait HardwareInterface {
    fn read(ctx: &GameBoy, address: u16) -> u8;
    fn write(ctx: &mut GameBoy, address: u16, value: u8);
}

#[macro_export]
macro_rules! warn_unimplemented_interface {
    ($interface:expr) => {
        log::warn!("{} is not currently implemented.", $interface)
    };
}

#[macro_export]
macro_rules! warn_unimplemented_read {
    ($ctx:expr, $interface:expr, $address:expr) => {
        crate::cpu_log!(
            warn,
            $ctx,
            "Read from {0} register at {1}. {0} is not implemented yet, so {2} was returned!",
            $interface,
            crate::word_fmt!($address),
            crate::byte_fmt!(crate::gb::hardware::memory::OPEN_BUS_VALUE),
        )
    };
}

#[macro_export]
macro_rules! warn_unimplemented_write {
    ($ctx:expr, $interface:expr, $address:expr, $value:expr) => {
        crate::cpu_log!(
            warn,
            $ctx,
            "Wrote {1} to {0} register at {2}. {0} is not implemented yet!",
            $interface,
            crate::byte_fmt!($value),
            crate::word_fmt!($address),
        )
    };
}
