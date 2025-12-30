use crate::gb::{
    GameBoy,
    hardware::{HardwareInit, HardwareInterface, memory::OPEN_BUS_VALUE},
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
        OPEN_BUS_VALUE
    }

    fn write(ctx: &mut GameBoy, address: u16, value: u8) {
        // TODO: Timer write
    }
}

impl Timer {}
