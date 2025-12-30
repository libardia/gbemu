use crate::gb::{
    GameBoy,
    hardware::{HardwareInterface, memory::OPEN_BUS_VALUE},
};

#[derive(Debug, Default)]
pub struct Input {
    // TODO: Graphics
}

impl HardwareInterface for Input {
    fn init(ctx: &mut GameBoy) {
        todo!()
    }

    fn read(ctx: &GameBoy, address: u16) -> u8 {
        // TODO: Graphics read
        OPEN_BUS_VALUE
    }

    fn write(ctx: &mut GameBoy, address: u16, value: u8) {
        // TODO: Graphics write
    }
}
