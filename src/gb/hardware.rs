use crate::gb::GameBoy;

pub mod audio;
pub mod cartridge;
pub mod graphics;
pub mod input;
pub mod memory;
pub mod processor;
pub mod serial;
pub mod timer;

pub trait HardwareInterface {
    fn init(ctx: &mut GameBoy);
    fn read(ctx: &GameBoy, address: u16) -> u8;
    fn write(ctx: &mut GameBoy, address: u16, value: u8);
}
