use crate::{gb::hw::HardwareInterface, macros::hex};

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
        todo!("TMU: read {}", hex!(address, 4));
    }

    fn write(&mut self, address: u16, byte: u8) {
        // TODO: write TMU
        todo!("TMU: write {} to {}", hex!(byte, 2), hex!(address, 4));
    }
}
