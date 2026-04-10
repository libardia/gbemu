use crate::{gb::hardware_interface::HardwareInterface, macros::hex};

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
        todo!("APU: read {}", hex!(address, 4));
    }

    fn write(&mut self, address: u16, byte: u8) {
        // TODO: write APU
        todo!("APU: write {} to {}", hex!(byte, 2), hex!(address, 4));
    }
}
