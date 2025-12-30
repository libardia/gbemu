use crate::{
    gb::{
        GameBoy,
        hardware::{HardwareInit, HardwareInterface, memory::OPEN_BUS_VALUE},
    },
    warn_unimplemented_interface, warn_unimplemented_read, warn_unimplemented_write,
};

#[derive(Debug, Default)]
pub struct Input {
    // TODO: Input
}

impl HardwareInit for Input {
    fn init(ctx: &mut GameBoy) {
        warn_unimplemented_interface!("Input");
        // TODO: Input init
    }
}

impl HardwareInterface for Input {
    fn read(ctx: &GameBoy, address: u16) -> u8 {
        // TODO: Input read
        warn_unimplemented_read!(ctx, "Input", address);
        OPEN_BUS_VALUE
    }

    fn write(ctx: &mut GameBoy, address: u16, value: u8) {
        // TODO: Input write
        warn_unimplemented_write!(ctx, "Input", address, value);
    }
}
