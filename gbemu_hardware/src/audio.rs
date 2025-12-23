use log::error;

use crate::{HardwareInterface, memory::OPEN_BUS_VALUE};

#[derive(Debug, Default)]
pub struct Audio {
    // TODO: Audio
}

impl HardwareInterface for Audio {
    fn read(&self, _: u16) -> u8 {
        error!("Read from a serial hardware register! Audio is unimplemented in this emulator.");
        OPEN_BUS_VALUE
    }

    fn write(&mut self, _: u16, _: u8) {
        error!("Wrote to a serial hardware register! Audio is unimplemented in this emulator.");
    }
}
