use crate::gb::{GameBoy, hardware::HardwareInterface};

#[derive(Debug, Default)]
pub struct Timer {
    // TODO: Timer
    system_timer: u16,
}

impl HardwareInterface for Timer {
    fn init(ctx: &mut GameBoy) {
        todo!()
    }

    fn read(ctx: &GameBoy, address: u16) -> u8 {
        todo!()
    }

    fn write(ctx: &mut GameBoy, address: u16, value: u8) {
        todo!()
    }
}

impl Timer {}
