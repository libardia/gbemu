pub mod audio;
pub mod cartridge;
pub mod graphics;
pub mod input;
pub mod memory;
pub mod processor;
pub mod serial;
pub mod timer;

pub trait HardwareInterface {
    fn read(&self, address: u16) -> u8;
    fn write(&mut self, address: u16, value: u8);
}
