use crate::gb::{
    GameBoy,
    hardware::{HardwareInit, HardwareInterface, memory::OPEN_BUS_VALUE},
};
use log::error;

#[derive(Debug, Default)]
pub struct Audio {
    // TODO: dummy Audio
}

impl HardwareInit for Audio {
    fn init(ctx: &mut GameBoy) {
        // TODO: dummy Audio init
    }
}

impl HardwareInterface for Audio {
    fn read(ctx: &GameBoy, address: u16) -> u8 {
        // TODO: Audio read
        error!("Read from a serial hardware register! Audio is unimplemented in this emulator.");
        OPEN_BUS_VALUE
    }

    fn write(ctx: &mut GameBoy, address: u16, value: u8) {
        // TODO: Audio write
        error!("Wrote to a serial hardware register! Audio is unimplemented in this emulator.");
    }
}
