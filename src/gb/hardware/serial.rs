use crate::gb::{
    GameBoy,
    hardware::{HardwareInit, HardwareInterface, memory::OPEN_BUS_VALUE},
};
use log::error;

#[derive(Debug, Default)]
pub struct Serial {
    // TODO: dummy Serial
}

impl HardwareInit for Serial {
    fn init(ctx: &mut GameBoy) {
        // TODO: dummy Serial init
    }
}

impl HardwareInterface for Serial {
    fn read(ctx: &GameBoy, address: u16) -> u8 {
        // TODO: dummy Serial read
        error!(
            "Read from a serial hardware register! Serial data is unimplemented in this emulator."
        );
        OPEN_BUS_VALUE
    }

    fn write(ctx: &mut GameBoy, address: u16, value: u8) {
        // TODO: dummy Serial write
        error!(
            "Wrote to a serial hardware register! Serial data is unimplemented in this emulator."
        );
    }
}
