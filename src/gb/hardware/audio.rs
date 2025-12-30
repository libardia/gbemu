use crate::{
    gb::{
        GameBoy,
        hardware::{HardwareInit, HardwareInterface, memory::OPEN_BUS_VALUE},
    },
    warn_unimplemented_read, warn_unimplemented_write,
};

#[derive(Debug, Default)]
pub struct Audio {
    // TODO: dummy Audio
}

impl HardwareInit for Audio {
    fn init(ctx: &mut GameBoy) {
        // TODO: dummy Audio init
    }
}

impl HardwareInterface for Audio {
    fn read(ctx: &GameBoy, address: u16) -> u8 {
        // TODO: Audio read
        warn_unimplemented_read!("Audio", address);
        OPEN_BUS_VALUE
    }

    fn write(ctx: &mut GameBoy, address: u16, value: u8) {
        // TODO: Audio write
        warn_unimplemented_write!("Audio", address, value);
    }
}
