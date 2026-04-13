use crate::gb::hardware_interface::{HardwareInterface, warn_todo_read, warn_todo_write};

#[derive(Debug, Default)]
pub struct SDU {
    // TODO: SDU struct
}

impl SDU {
    pub fn new() -> Self {
        Default::default()
    }
}

impl HardwareInterface for SDU {
    fn read(&mut self, address: u16) -> u8 {
        // TODO: read SDU
        warn_todo_read!("SDU", address)
    }

    fn write(&mut self, address: u16, byte: u8) {
        // TODO: write SDU
        warn_todo_write!("SDU", address, byte);
    }
}
