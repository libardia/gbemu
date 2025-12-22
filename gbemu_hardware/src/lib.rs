#![allow(dead_code)]
#![allow(unused_variables)]

use crate::{
    cartridge::{Cartridge, load_cart},
    graphics::Graphics,
    memory::Memory,
    processor::Processor,
    timer::Timer,
};
use std::{io::Result, path::Path};

mod cartridge;
mod graphics;
mod macros;
mod memory;
mod processor;
mod regions;
mod registers;
mod timer;

pub fn start(rom_path: &str) -> Result<()> {
    let cart = load_cart(Path::new(rom_path))?;
    let mut gb = GameBoy::new(cart);
    gb.run();
    Ok(())
}

pub struct GameBoy {
    cart: Box<dyn Cartridge>,
    cpu: Processor,
    gfx: Graphics,
    mem: Memory,
    timer: Timer,
}

pub trait HardwareInterface {
    fn read(&self, address: u16) -> u8;
    fn write(&mut self, address: u16, value: u8);
}

impl GameBoy {
    pub fn new(cart: Box<dyn Cartridge>) -> Self {
        Self {
            cart,
            cpu: Processor::default(),
            gfx: Graphics::default(),
            mem: Memory::default(),
            timer: Timer::default(),
        }
    }

    pub fn run(&mut self) {
        //TODO: run
    }
}
