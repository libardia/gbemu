use crate::gb::hardware::{HardwareInterface, memory::OPEN_BUS_VALUE};

#[derive(Debug, Default)]
pub struct Graphics {
    // TODO: Graphics
}

impl HardwareInterface for Graphics {
    fn read(&self, address: u16) -> u8 {
        // TODO: Graphics read
        OPEN_BUS_VALUE
    }

    fn write(&mut self, address: u16, value: u8) {
        // TODO: Graphics write
    }
}
