use crate::gb::{
    GameBoy,
    hardware_interface::{HardwareInterface, warn_todo_read, warn_todo_write},
};

#[derive(Debug, Default)]
pub struct PPU {
    // TODO: PPU struct
}

impl PPU {
    pub fn new() -> Self {
        Default::default()
    }
}

impl PPU {
    pub fn t_tick(_ctx: &mut GameBoy) {
        // TODO: PPU tick
    }
}

impl HardwareInterface for PPU {
    fn read(&mut self, address: u16) -> u8 {
        // TODO: read PPU
        warn_todo_read!("PPU", address)
    }

    fn write(&mut self, address: u16, byte: u8) {
        // TODO: write PPU
        warn_todo_write!("PPU", address, byte);
    }
}
