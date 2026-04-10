use crate::{gb::hw::HardwareInterface, macros::hex};

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
        todo!("INU: read {}", hex!(address, 4));
    }

    fn write(&mut self, address: u16, byte: u8) {
        // TODO: write INU
        todo!("INU: write {} to {}", hex!(byte, 2), hex!(address, 4));
    }
}
