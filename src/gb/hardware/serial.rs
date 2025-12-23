use crate::gb::hardware::{HardwareInterface, memory::OPEN_BUS_VALUE};
use log::error;

#[derive(Debug, Default)]
pub struct Serial {
    // Unimplemented
}

impl HardwareInterface for Serial {
    fn read(&self, _: u16) -> u8 {
        error!(
            "Read from a serial hardware register! Serial data is unimplemented in this emulator."
        );
        OPEN_BUS_VALUE
    }

    fn write(&mut self, _: u16, _: u8) {
        error!(
            "Wrote to a serial hardware register! Serial data is unimplemented in this emulator."
        );
    }
}
