use crate::{gb::hardware_interface::HardwareInterface, macros::hex};

#[derive(Debug, Default)]
pub struct PPU {
    // TODO: PPU struct
}

impl PPU {
    pub fn new() -> Self {
        Default::default()
    }
}

impl HardwareInterface for PPU {
    fn read(&mut self, address: u16) -> u8 {
        // TODO: read PPU
        todo!("PPU: read {}", hex!(address, 4));
    }

    fn write(&mut self, address: u16, byte: u8) {
        // TODO: write PPU
        todo!("PPU: write {} to {}", hex!(byte, 2), hex!(address, 4));
    }
}
