use crate::gb::hardware::{
    audio::Audio,
    cartridge::{Cartridge, load_cart},
    graphics::Graphics,
    input::Input,
    memory::Memory,
    processor::Processor,
    serial::Serial,
    timer::Timer,
};
use std::io::Result;

mod hardware;
mod macros;
mod regions;
mod registers;

pub struct GameBoy {
    cart: Box<dyn Cartridge>,
    cpu: Processor,
    mem: Memory,
    gfx: Graphics,
    timer: Timer,
    input: Input,
    aud: Audio,
    serial: Serial,
}

impl GameBoy {
    pub fn new(rom_path: &str) -> Result<Self> {
        Ok(Self {
            cart: load_cart(rom_path)?,
            cpu: Processor::default(),
            mem: Memory::default(),
            gfx: Graphics::default(),
            timer: Timer::default(),
            input: Input::default(),
            aud: Audio::default(),
            serial: Serial::default(),
        })
    }

    pub fn run(&mut self) {
        //TODO: run
    }
}
