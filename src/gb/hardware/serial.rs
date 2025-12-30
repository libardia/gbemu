use crate::{
    gb::{
        GameBoy,
        hardware::{HardwareInit, HardwareInterface, memory::OPEN_BUS_VALUE},
    },
    warn_unimplemented_read, warn_unimplemented_write,
};

#[derive(Debug, Default)]
pub struct Serial {
    // TODO: dummy Serial
}

impl HardwareInit for Serial {
    fn init(ctx: &mut GameBoy) {
        // TODO: dummy Serial init
    }
}

impl HardwareInterface for Serial {
    fn read(ctx: &GameBoy, address: u16) -> u8 {
        // TODO: dummy Serial read
        warn_unimplemented_read!("Serial", address);
        OPEN_BUS_VALUE
    }

    fn write(ctx: &mut GameBoy, address: u16, value: u8) {
        // TODO: dummy Serial write
        warn_unimplemented_write!("Serial", address, value);
    }
}
