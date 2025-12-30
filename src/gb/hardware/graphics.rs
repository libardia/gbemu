use crate::gb::{
    GameBoy,
    hardware::{HardwareInit, HardwareInterface, memory::OPEN_BUS_VALUE},
};

#[derive(Debug, Default)]
pub struct Graphics {
    // TODO: Graphics
}

impl HardwareInit for Graphics {
    fn init(ctx: &mut GameBoy) {
        // TODO: Graphics init
    }
}

impl HardwareInterface for Graphics {
    fn read(ctx: &GameBoy, address: u16) -> u8 {
        // TODO: Graphics read
        OPEN_BUS_VALUE
    }

    fn write(ctx: &mut GameBoy, address: u16, value: u8) {
        // TODO: Graphics write
    }
}
