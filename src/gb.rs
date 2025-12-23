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
use std::{io::Result, path::Path};

mod hardware;
mod macros;
mod regions;
mod registers;

pub fn start(rom_path: &str) -> Result<()> {
    let cart = load_cart(Path::new(rom_path))?;
    let mut gb = GameBoy::new(cart);
    gb.run();
    Ok(())
}

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
    pub fn new(cart: Box<dyn Cartridge>) -> Self {
        Self {
            cart,
            cpu: Processor::default(),
            mem: Memory::default(),
            gfx: Graphics::default(),
            timer: Timer::default(),
            input: Input::default(),
            aud: Audio::default(),
            serial: Serial::default(),
        }
    }

    pub fn run(&mut self) {
        //TODO: run
    }
}
