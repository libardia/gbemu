use crate::gb::{
    GameBoy,
    hardware::{HardwareInit, HardwareInterface, memory::OPEN_BUS_VALUE},
};

#[derive(Debug, Default)]
pub struct Input {
    // TODO: Input
}

impl HardwareInit for Input {
    fn init(ctx: &mut GameBoy) {
        // TODO: Input init
    }
}

impl HardwareInterface for Input {
    fn read(ctx: &GameBoy, address: u16) -> u8 {
        // TODO: Input read
        OPEN_BUS_VALUE
    }

    fn write(ctx: &mut GameBoy, address: u16, value: u8) {
        // TODO: Input write
    }
}
