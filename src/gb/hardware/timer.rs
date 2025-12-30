use crate::{
    gb::{
        GameBoy,
        hardware::{HardwareInit, HardwareInterface, memory::OPEN_BUS_VALUE},
    },
    warn_unimplemented_read, warn_unimplemented_write,
};

#[derive(Debug, Default)]
pub struct Timer {
    // TODO: Timer
    system_timer: u16,
}

impl HardwareInit for Timer {
    fn init(ctx: &mut GameBoy) {
        // TODO: Timer init
    }
}

impl HardwareInterface for Timer {
    fn read(ctx: &GameBoy, address: u16) -> u8 {
        // TODO: Timer read
        warn_unimplemented_read!("Timer", address);
        OPEN_BUS_VALUE
    }

    fn write(ctx: &mut GameBoy, address: u16, value: u8) {
        // TODO: Timer write
        warn_unimplemented_write!("Timer", address, value);
    }
}

impl Timer {}
