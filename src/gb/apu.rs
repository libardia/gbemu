use crate::gb::hardware_interface::{HardwareInterface, warn_todo_read, warn_todo_write};

#[derive(Debug, Default)]
pub struct APU {
    // TODO: APU struct
}

impl APU {
    pub fn new() -> Self {
        Default::default()
    }
}

impl HardwareInterface for APU {
    fn read(&mut self, address: u16) -> u8 {
        // TODO: read APU
        warn_todo_read!("APU", address)
    }

    fn write(&mut self, address: u16, byte: u8) {
        // TODO: write APU
        warn_todo_write!("APU", address, byte);
    }
}
