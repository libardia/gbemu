use crate::gb::{
    GameBoy,
    hardware::{HardwareInterface, memory::OPEN_BUS_VALUE},
};
use log::error;

#[derive(Debug, Default)]
pub struct Serial {
    // Unimplemented
}

impl HardwareInterface for Serial {
    fn init(ctx: &mut GameBoy) {
        todo!()
    }

    fn read(ctx: &GameBoy, address: u16) -> u8 {
        error!(
            "Read from a serial hardware register! Serial data is unimplemented in this emulator."
        );
        OPEN_BUS_VALUE
    }

    fn write(ctx: &mut GameBoy, address: u16, value: u8) {
        error!(
            "Wrote to a serial hardware register! Serial data is unimplemented in this emulator."
        );
    }
}
