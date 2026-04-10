use crate::{gb::hardware_interface::HardwareInterface, macros::hex};

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
        todo!("SDU: read {}", hex!(address, 4));
    }

    fn write(&mut self, address: u16, byte: u8) {
        // TODO: write SDU
        todo!("SDU: write {} to {}", hex!(byte, 2), hex!(address, 4));
    }
}
