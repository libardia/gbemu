use crate::gb::hardware_interface::{HardwareInterface, warn_todo_read, warn_todo_write};

#[derive(Debug, Default)]
pub struct TMU {
    // TODO: TMU struct
}

impl TMU {
    pub fn new() -> Self {
        Default::default()
    }
}

impl HardwareInterface for TMU {
    fn read(&mut self, address: u16) -> u8 {
        // TODO: read TMU
        warn_todo_read!("TMU", address)
    }

    fn write(&mut self, address: u16, byte: u8) {
        // TODO: write TMU
        warn_todo_write!("TMU", address, byte);
    }
}
