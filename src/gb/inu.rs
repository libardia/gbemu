use crate::gb::hardware_interface::{HardwareInterface, warn_todo_read, warn_todo_write};

#[derive(Debug, Default)]
pub struct INU {
    // TODO: INU struct
}

impl INU {
    pub fn new() -> Self {
        Default::default()
    }
}

impl HardwareInterface for INU {
    fn read(&mut self, address: u16) -> u8 {
        // TODO: read INU
        warn_todo_read!("INU", address)
    }

    fn write(&mut self, address: u16, byte: u8) {
        // TODO: write INU
        warn_todo_write!("INU", address, byte);
    }
}
